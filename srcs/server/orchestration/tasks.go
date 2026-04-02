package orchestration

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"time"

	"github.com/google/uuid"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	tasksMeter         = otel.Meter("github.com/onehumancorp/mono/srcs/server/orchestration/tasks")
	tasksCompleted, _  = tasksMeter.Int64Counter(
		"ohc_swarm_tasks_completed",
		metric.WithDescription("Total number of swarm tasks completed"),
	)
)

// SwarmTask represents a discrete task unit in the Teammate Mesh Shared Task List.
type SwarmTask struct {
	ID              string          `json:"id"`
	MissionID       string          `json:"mission_id"`
	Title           string          `json:"title"`
	Status          string          `json:"status"` // PENDING, IN_PROGRESS, COMPLETED, FAILED
	AssignedAgentID string          `json:"assigned_agent_id,omitempty"`
	LockedUntil     time.Time       `json:"locked_until,omitempty"`
	Payload         json.RawMessage `json:"payload"`
	CreatedAt       time.Time       `json:"created_at"`
	UpdatedAt       time.Time       `json:"updated_at"`
}

// PublishTask inserts a new task into the Shared Task List and broadcasts it via the Mesh.
func (h *Hub) PublishTask(ctx context.Context, missionID, title string, payload json.RawMessage, fromAgentID string) (string, error) {
	sipDB := h.GetSIPDB()
	if sipDB == nil {
		return "", errors.New("SIPDB is not initialized")
	}

	payloadStr := string(payload)

	taskID := uuid.New().String()
	err := withRetry(ctx, func() error {
		// Insert task into DB with pre-generated UUID to avoid RETURNING race conditions in SQLite
		if sipDB.db.IsSQLite() {
			_, err := sipDB.db.Exec(ctx,
				`INSERT INTO swarm_tasks (id, mission_id, title, status, payload, created_at, updated_at)
				 VALUES (?, ?, ?, 'PENDING', ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)`,
				taskID, missionID, title, payloadStr,
			)
			return err
		}
		_, err := sipDB.db.Exec(ctx,
			`INSERT INTO swarm_tasks (id, mission_id, title, status, payload, created_at, updated_at)
			 VALUES ($1, $2, $3, 'PENDING', $4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)`,
			taskID, missionID, title, payloadStr,
		)
		return err
	})
	if err != nil {
		return "", fmt.Errorf("failed to create task: %w", err)
	}

	// Broadcast to mesh
	broadcastMsg := Message{
		ID:         "task-" + taskID,
		FromAgent:  fromAgentID,
		ToAgent:    "all",
		Type:       EventTaskBroadcast,
		Content:    title,
		OccurredAt: time.Now().UTC(),
	}

	// Add payload to broadcast if possible, although message content is usually just string.
	// For now we just send the title, but agents can fetch the task by ID.
	_ = h.Publish(broadcastMsg)

	return taskID, nil
}

// ClaimTask attempts to atomically claim a task from the PENDING queue.
// It uses row-level locking or distributed locks (via the DB).
func (h *Hub) ClaimTask(ctx context.Context, taskID, agentID string) (bool, error) {
	sipDB := h.GetSIPDB()
	if sipDB == nil {
		return false, errors.New("SIPDB is not initialized")
	}

	isStandalone := envBoolDefault("OHC_STANDALONE", false)

	if isStandalone {
		select {
		case throttleSemaphore <- struct{}{}:
			defer func() { <-throttleSemaphore }()
		case <-ctx.Done():
			return false, ctx.Err()
		}
	}

	var claimed bool
	err := withRetry(ctx, func() error {
		// Try to claim the task if it's PENDING, or if IN_PROGRESS but lock has expired.
		// PostgreSQL uses row-level locks, SQLite uses standard transactions since it's already serialized in standalone
		query := `
			UPDATE swarm_tasks
			SET status = 'IN_PROGRESS',
				assigned_agent_id = $1,
				locked_until = $2,
				updated_at = CURRENT_TIMESTAMP
			WHERE id = $3
			  AND (status = 'PENDING' OR (status = 'IN_PROGRESS' AND locked_until < CURRENT_TIMESTAMP))
		`

		// Lock for 30 minutes
		lockedUntil := time.Now().UTC().Add(30 * time.Minute)

		// Provide parameter fallback formatting
		var rowsAffected int64
		var err error

		if sipDB.db.IsSQLite() {
			query = `
				UPDATE swarm_tasks
				SET status = 'IN_PROGRESS',
					assigned_agent_id = ?,
					locked_until = ?,
					updated_at = CURRENT_TIMESTAMP
				WHERE id = ?
				  AND (status = 'PENDING' OR (status = 'IN_PROGRESS' AND locked_until < CURRENT_TIMESTAMP))
			`
			rowsAffected, err = sipDB.db.Exec(ctx, query, agentID, lockedUntil, taskID)
		} else {
			rowsAffected, err = sipDB.db.Exec(ctx, query, agentID, lockedUntil, taskID)
		}

		if err != nil {
			return err
		}

		claimed = rowsAffected > 0
		return nil
	})

	return claimed, err
}

// CompleteTask marks a task as COMPLETED.
func (h *Hub) CompleteTask(ctx context.Context, taskID, agentID string) error {
	sipDB := h.GetSIPDB()
	if sipDB == nil {
		return errors.New("SIPDB is not initialized")
	}

	var rowsAffected int64
	err := withRetry(ctx, func() error {
		var err error
		if sipDB.db.IsSQLite() {
			rowsAffected, err = sipDB.db.Exec(ctx,
				"UPDATE swarm_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ? AND assigned_agent_id = ?",
				taskID, agentID,
			)
		} else {
			rowsAffected, err = sipDB.db.Exec(ctx,
				"UPDATE swarm_tasks SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND assigned_agent_id = $2",
				taskID, agentID,
			)
		}
		return err
	})

	if err != nil {
		return err
	}
	if rowsAffected == 0 {
		return errors.New("task not found or not assigned to this agent")
	}

	if tasksCompleted != nil {
		tasksCompleted.Add(ctx, 1)
	}

	return nil
}

// GetCompletedTasksForAutoDream returns up to `limit` recently completed tasks for memory consolidation.
func (h *Hub) GetCompletedTasksForAutoDream(ctx context.Context, limit int) ([]SwarmTask, error) {
	sipDB := h.GetSIPDB()
	if sipDB == nil {
		return nil, errors.New("SIPDB is not initialized")
	}

	var tasks []SwarmTask
	err := withRetry(ctx, func() error {
		query := fmt.Sprintf("SELECT id, mission_id, title, status, payload, created_at, updated_at FROM swarm_tasks WHERE status = 'COMPLETED' ORDER BY updated_at DESC LIMIT %d", limit)
		rows, err := sipDB.db.Query(ctx, query)
		if err != nil {
			return err
		}
		defer rows.Close()

		for rows.Next() {
			var t SwarmTask
			var payloadStr string
			var createdAt, updatedAt string

			// Simple parsing
			if err := rows.Scan(&t.ID, &t.MissionID, &t.Title, &t.Status, &payloadStr, &createdAt, &updatedAt); err != nil {
				return err
			}
			t.Payload = json.RawMessage(payloadStr)
			t.CreatedAt, _ = time.Parse(time.RFC3339, createdAt)
			t.UpdatedAt, _ = time.Parse(time.RFC3339, updatedAt)
			tasks = append(tasks, t)
		}
		return nil
	})

	return tasks, err
}

// StoreAutoDreamMemory saves a generated vector embedding memory.
func (h *Hub) StoreAutoDreamMemory(ctx context.Context, topic, summary string, embedding []byte) error {
	sipDB := h.GetSIPDB()
	if sipDB == nil {
		return errors.New("SIPDB is not initialized")
	}

	return withRetry(ctx, func() error {
		if sipDB.db.IsSQLite() {
			_, err := sipDB.db.Exec(ctx,
				"INSERT INTO swarm_long_term_memory (topic, summary, embedding, created_at) VALUES (?, ?, ?, CURRENT_TIMESTAMP)",
				topic, summary, embedding,
			)
			return err
		}
		_, err := sipDB.db.Exec(ctx,
			"INSERT INTO swarm_long_term_memory (topic, summary, embedding, created_at) VALUES ($1, $2, $3, CURRENT_TIMESTAMP)",
			topic, summary, embedding,
		)
		return err
	})
}

// FailTask marks a task as FAILED so it can be retried or investigated.
func (h *Hub) FailTask(ctx context.Context, taskID, agentID string) error {
	sipDB := h.GetSIPDB()
	if sipDB == nil {
		return errors.New("SIPDB is not initialized")
	}

	err := withRetry(ctx, func() error {
		var err error
		if sipDB.db.IsSQLite() {
			_, err = sipDB.db.Exec(ctx,
				"UPDATE swarm_tasks SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = ? AND assigned_agent_id = ?",
				taskID, agentID,
			)
		} else {
			_, err = sipDB.db.Exec(ctx,
				"UPDATE swarm_tasks SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND assigned_agent_id = $2",
				taskID, agentID,
			)
		}
		return err
	})

	return err
}

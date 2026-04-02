package orchestration

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
	"os"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

// FlexTime handles scanning of timestamps from both PostgreSQL and SQLite.
type FlexTime struct {
	time.Time
}

// Scan implements the sql.Scanner interface.
func (f *FlexTime) Scan(value interface{}) error {
	if value == nil {
		return nil
	}
	switch v := value.(type) {
	case time.Time:
		f.Time = v
	case string:
		parsed, err := time.Parse(time.RFC3339Nano, v)
		if err != nil {
			parsed, err = time.Parse("2006-01-02 15:04:05.999999-07", v)
			if err != nil {
				parsed, err = time.Parse("2006-01-02 15:04:05.999999", v)
				if err != nil {
					parsed, err = time.Parse("2006-01-02 15:04:05-07:00", v)
					if err != nil {
						parsed, err = time.Parse("2006-01-02 15:04:05", v)
						if err != nil {
							parsed, err = time.Parse(time.RFC3339, v)
							if err != nil {
								return fmt.Errorf("failed to parse timestamp string: %s", v)
							}
						}
					}
				}
			}
		}
		f.Time = parsed
	case []byte:
		s := string(v)
		parsed, err := time.Parse(time.RFC3339Nano, s)
		if err != nil {
			parsed, err = time.Parse("2006-01-02 15:04:05.999999-07", s)
			if err != nil {
				parsed, err = time.Parse("2006-01-02 15:04:05.999999", s)
				if err != nil {
					parsed, err = time.Parse("2006-01-02 15:04:05-07:00", s)
					if err != nil {
						parsed, err = time.Parse("2006-01-02 15:04:05", s)
						if err != nil {
							parsed, err = time.Parse(time.RFC3339, s)
							if err != nil {
								return fmt.Errorf("failed to parse timestamp string: %s", s)
							}
						}
					}
				}
			}
		}
		f.Time = parsed
	default:
		return fmt.Errorf("unknown timestamp type: %T", value)
	}
	return nil
}

// NullFlexTime represents a time.Time that may be null.
type NullFlexTime struct {
	Time  time.Time
	Valid bool
}

// Scan implements the sql.Scanner interface.
func (n *NullFlexTime) Scan(value interface{}) error {
	if value == nil {
		n.Time, n.Valid = time.Time{}, false
		return nil
	}
	n.Valid = true
	var f FlexTime
	err := f.Scan(value)
	if err == nil {
		n.Time = f.Time
	}
	return err
}

// SharedTask represents a shared task distributed across agents.
type SharedTask struct {
	ID              string
	MissionID       string
	Title           string
	Status          string // PENDING, IN_PROGRESS, COMPLETED, FAILED
	AssignedAgentID sql.NullString
	LockedUntil     NullFlexTime
	Payload         string
	CreatedAt       time.Time
	UpdatedAt       time.Time
}

// TaskManager manages the shared tasks list
type TaskManager struct {
	db          db.Provider
	redisClient rueidis.Client
}

// NewTaskManager creates a new TaskManager.
func NewTaskManager(provider db.Provider) *TaskManager {
	tm := &TaskManager{db: provider}

	if os.Getenv("OHC_MULTITENANT") == "true" {
		redisURL := os.Getenv("REDIS_URL")
		if redisURL != "" {
			c, err := rueidis.NewClient(rueidis.ClientOption{
				InitAddress: []string{redisURL},
			})
			if err == nil {
				tm.redisClient = c
			}
		}
	}
	return tm
}

// CreateTask creates a new shared task.
func (tm *TaskManager) CreateTask(ctx context.Context, missionID, title, payload string) (*SharedTask, error) {
	var task SharedTask
	var query string
	var err error

	var createdAt FlexTime
	var updatedAt FlexTime

	if tm.db.IsSQLite() {
		id := generateID() // Helper func
		query = `
			INSERT INTO swarm_tasks (id, mission_id, title, payload, status, created_at, updated_at)
			VALUES ($1, $2, $3, $4, 'PENDING', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
			RETURNING id, mission_id, title, status, assigned_agent_id, locked_until, payload, created_at, updated_at
		`
		err = tm.db.QueryRow(ctx, query, id, missionID, title, payload).Scan(
			&task.ID, &task.MissionID, &task.Title, &task.Status, &task.AssignedAgentID, &task.LockedUntil, &task.Payload, &createdAt, &updatedAt,
		)
	} else {
		query = `
			INSERT INTO swarm_tasks (mission_id, title, payload, status)
			VALUES ($1, $2, $3, 'PENDING')
			RETURNING id, mission_id, title, status, assigned_agent_id, locked_until, payload, created_at, updated_at
		`
		err = tm.db.QueryRow(ctx, query, missionID, title, payload).Scan(
			&task.ID, &task.MissionID, &task.Title, &task.Status, &task.AssignedAgentID, &task.LockedUntil, &task.Payload, &createdAt, &updatedAt,
		)
	}

	if err != nil {
		return nil, fmt.Errorf("failed to create task: %w", err)
	}

	task.CreatedAt = createdAt.Time
	task.UpdatedAt = updatedAt.Time

	return &task, nil
}

// ClaimTask attempts to claim a specific PENDING task for the given agentID.
// It uses row-level locking (FOR UPDATE) in Postgres, and relies on SQLite's lock mechanism
// to prevent race conditions.
// In Multi-tenant cloud mode, it attempts to acquire a distributed Redis lock.
func (tm *TaskManager) ClaimTask(ctx context.Context, taskID, agentID string) (*SharedTask, error) {
	if tm.redisClient != nil {
		// Acquire Redis-backed distributed lock with 30s TTL
		lockKey := "lock:task:" + taskID
		cmd := tm.redisClient.B().Set().Key(lockKey).Value(agentID).Nx().Ex(30 * time.Second).Build()
		err := tm.redisClient.Do(ctx, cmd).Error()
		if err != nil {
			if rueidis.IsRedisNil(err) {
				return nil, nil // Lock could not be acquired (task is locked)
			}
			return nil, fmt.Errorf("failed to acquire distributed lock: %w", err)
		}
	}

	tx, err := tm.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var task SharedTask
	var errQuery error
	var createdAt FlexTime
	var updatedAt FlexTime

	if tm.db.IsSQLite() {
		// SQLite doesn't support FOR UPDATE, but `Begin` handles concurrent writes lock.
		query := `
			SELECT id, mission_id, title, status, assigned_agent_id, locked_until, payload, created_at, updated_at
			FROM swarm_tasks
			WHERE id = $1 AND status = 'PENDING'
			ORDER BY created_at ASC
			LIMIT 1
		`
		errQuery = tx.QueryRow(ctx, query, taskID).Scan(
			&task.ID, &task.MissionID, &task.Title, &task.Status, &task.AssignedAgentID, &task.LockedUntil, &task.Payload, &createdAt, &updatedAt,
		)
	} else {
		// PostgreSQL with SKIP LOCKED
		query := `
			SELECT id, mission_id, title, status, assigned_agent_id, locked_until, payload, created_at, updated_at
			FROM swarm_tasks
			WHERE id = $1 AND status = 'PENDING'
			ORDER BY created_at ASC
			LIMIT 1
			FOR UPDATE SKIP LOCKED
		`
		errQuery = tx.QueryRow(ctx, query, taskID).Scan(
			&task.ID, &task.MissionID, &task.Title, &task.Status, &task.AssignedAgentID, &task.LockedUntil, &task.Payload, &createdAt, &updatedAt,
		)
	}

	if errQuery != nil {
		if errors.Is(errQuery, sql.ErrNoRows) {
			return nil, nil // No task available
		}
		return nil, fmt.Errorf("failed to find pending task: %w", errQuery)
	}
	task.CreatedAt = createdAt.Time
	task.UpdatedAt = updatedAt.Time

	// Update task status to IN_PROGRESS
	updateQuery := `
		UPDATE swarm_tasks
		SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2 AND status = 'PENDING'
	`
	rowsAffected, err := tx.Exec(ctx, updateQuery, agentID, task.ID)
	if err != nil {
		return nil, fmt.Errorf("failed to update task status: %w", err)
	}

	if rowsAffected == 0 {
		// Task was likely claimed by another worker concurrently.
		return nil, nil
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	task.Status = "IN_PROGRESS"
	task.AssignedAgentID = sql.NullString{String: agentID, Valid: true}
	return &task, nil
}

// CompleteTask marks a task as completed.
func (tm *TaskManager) CompleteTask(ctx context.Context, taskID, agentID string) error {
	query := `
		UPDATE swarm_tasks
		SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
		WHERE id = $1 AND assigned_agent_id = $2 AND status = 'IN_PROGRESS'
	`
	res, err := tm.db.Exec(ctx, query, taskID, agentID)
	if err != nil {
		return fmt.Errorf("failed to complete task: %w", err)
	}

	if res == 0 {
		return errors.New("task not found or not assigned to agent")
	}

	return nil
}

// generateID generates a pseudo-uuid for SQLite compatibility.
func generateID() string {
	return fmt.Sprintf("%d", time.Now().UnixNano())
}

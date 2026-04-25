package kairos

import (
	"context"
	"database/sql"
	"encoding/json"
	"sync"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
)

// SharedTask models the shared_tasks phase1 database table.
type SharedTask struct {
	ID        string          `json:"id"`
	AgentID   string          `json:"agent_id"`
	Status    string          `json:"status"`
	Payload   json.RawMessage `json:"payload"`
	CreatedAt time.Time       `json:"created_at"`
}

type SharedTaskRepo struct {
	provider db.Provider
	mu       sync.Mutex
}

func NewSharedTaskRepo(provider db.Provider) *SharedTaskRepo {
	return &SharedTaskRepo{provider: provider}
}

func (r *SharedTaskRepo) Insert(ctx context.Context, task *SharedTask) error {
	query := `INSERT INTO shared_tasks (id, agent_id, status, payload, created_at) VALUES ($1, $2, $3, $4, $5)`
	payloadStr := "{}"
	if task.Payload != nil && len(task.Payload) > 0 {
		payloadStr = string(task.Payload)
	}
	createdAt := task.CreatedAt.Format(time.RFC3339)
	_, err := r.provider.Exec(ctx, query, task.ID, task.AgentID, task.Status, payloadStr, createdAt)
	return err
}

func (r *SharedTaskRepo) Get(ctx context.Context, id string) (*SharedTask, error) {
	query := `SELECT id, agent_id, status, payload, created_at FROM shared_tasks WHERE id = $1`
	var task SharedTask
	var payloadStr string
	var createdAt string

	row := r.provider.QueryRow(ctx, query, id)
	if r.provider.IsSQLite() {
		err := row.Scan(&task.ID, &task.AgentID, &task.Status, &payloadStr, &createdAt)
		if err != nil {
			return nil, err
		}
		task.CreatedAt, _ = time.Parse(time.RFC3339, createdAt)
	} else {
		var t time.Time
		err := row.Scan(&task.ID, &task.AgentID, &task.Status, &payloadStr, &t)
		if err != nil {
			return nil, err
		}
		task.CreatedAt = t
	}

	if payloadStr != "" {
		task.Payload = json.RawMessage(payloadStr)
	}
	return &task, nil
}

// ClaimTask atomically claims a pending task.
func (r *SharedTaskRepo) ClaimTask(ctx context.Context, agentID string) (*SharedTask, error) {
	var task SharedTask
	var payloadStr string

	if r.provider.IsSQLite() {
		r.mu.Lock()
		defer r.mu.Unlock()

		tx, err := r.provider.Begin(ctx)
		if err != nil {
			return nil, err
		}
		defer tx.Rollback(ctx)

		var createdAt string
		query := `SELECT id, agent_id, status, payload, created_at FROM shared_tasks WHERE status = 'PENDING' LIMIT 1`
		err = tx.QueryRow(ctx, query).Scan(&task.ID, &task.AgentID, &task.Status, &payloadStr, &createdAt)
		if err != nil {
			if err == sql.ErrNoRows {
				return nil, nil // No task available
			}
			return nil, err
		}

		_, err = tx.Exec(ctx, `UPDATE shared_tasks SET status = 'IN_PROGRESS', agent_id = $1 WHERE id = $2`, agentID, task.ID)
		if err != nil {
			return nil, err
		}

		err = tx.Commit(ctx)
		if err != nil {
			return nil, err
		}

		task.AgentID = agentID
		task.Status = "IN_PROGRESS"
		task.CreatedAt, _ = time.Parse(time.RFC3339, createdAt)

	} else {
		// Postgres mode: enforce FOR UPDATE SKIP LOCKED
		query := `
		UPDATE shared_tasks
		SET status = 'IN_PROGRESS', agent_id = $1
		WHERE id = (
			SELECT id FROM shared_tasks
			WHERE status = 'PENDING'
			FOR UPDATE SKIP LOCKED
			LIMIT 1
		)
		RETURNING id, agent_id, status, payload, created_at
		`
		var t time.Time
		err := r.provider.QueryRow(ctx, query, agentID).Scan(&task.ID, &task.AgentID, &task.Status, &payloadStr, &t)
		if err != nil {
			if err == sql.ErrNoRows {
				return nil, nil // No task available
			}
			return nil, err
		}
		task.CreatedAt = t
	}

	if payloadStr != "" {
		task.Payload = json.RawMessage(payloadStr)
	}

	return &task, nil
}

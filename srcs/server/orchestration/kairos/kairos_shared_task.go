package kairos

import (
	"context"
	"encoding/json"
	"github.com/onehumancorp/mono/srcs/server/db"
	"time"

	"fmt"
	"github.com/google/uuid"
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
	provider      db.Provider
	mutexProvider MutexProvider
}

func NewSharedTaskRepo(provider db.Provider, mutexProvider MutexProvider) *SharedTaskRepo {
	return &SharedTaskRepo{provider: provider, mutexProvider: mutexProvider}
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

func (r *SharedTaskRepo) insertTransition(ctx context.Context, tx db.Tx, taskID, fromState, toState, agentID, reason string) error {
	id := uuid.New().String()
	_, err := tx.Exec(ctx, `
        INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason, occurred_at)
        VALUES ($1, $2, 'task', $3, $4, $5, $6, CURRENT_TIMESTAMP)
    `, id, taskID, fromState, toState, agentID, reason)
	return err
}

func (r *SharedTaskRepo) ClaimTask(ctx context.Context, agentID string) (*SharedTask, error) {
	if r.provider.IsSQLite() && r.mutexProvider != nil {
		mu := r.mutexProvider.NewMutex("shared_task_claim")
		if err := mu.Lock(ctx, 10*time.Second); err != nil {
			return nil, err
		}
		defer mu.Unlock(ctx)
	}

	tx, err := r.provider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var id string
	if r.provider.IsSQLite() {
		query := `
            SELECT id FROM shared_tasks
            WHERE status = 'PENDING'
            LIMIT 1
        `
		err = tx.QueryRow(ctx, query).Scan(&id)
	} else {
		query := `
            SELECT id FROM shared_tasks
            WHERE status = 'PENDING'
            LIMIT 1
            FOR UPDATE SKIP LOCKED
        `
		err = tx.QueryRow(ctx, query).Scan(&id)
	}

	if err != nil {
		if err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
			return nil, nil
		}
		return nil, err
	}

	_, err = tx.Exec(ctx, "UPDATE shared_tasks SET status = 'IN_PROGRESS', agent_id = $1 WHERE id = $2", agentID, id)
	if err != nil {
		return nil, err
	}

	if err := r.insertTransition(ctx, tx, id, "PENDING", "IN_PROGRESS", agentID, "Task claimed by agent"); err != nil {
		return nil, fmt.Errorf("failed to insert transition: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	return r.Get(ctx, id)
}

func (r *SharedTaskRepo) TransitionTask(ctx context.Context, taskID, agentID, fromState, toState, reason string) error {
	tx, err := r.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	res, err := tx.Exec(ctx, "UPDATE shared_tasks SET status = $1, agent_id = $2 WHERE id = $3 AND status = $4", toState, agentID, taskID, fromState)
	if err != nil {
		return err
	}
	rowsAffected := res
	if rowsAffected == 0 {
		return fmt.Errorf("task %s is not in expected state %s or does not exist", taskID, fromState)
	}

	if err := r.insertTransition(ctx, tx, taskID, fromState, toState, agentID, reason); err != nil {
		return err
	}

	return tx.Commit(ctx)
}

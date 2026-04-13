package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type SubAgentJob struct {
	ID             string
	OrganizationID string
	ParentTaskID   string
	Payload        string // JSON
	Status         string
	WorkerID       *string
	CreatedAt      string
	UpdatedAt      string
}

type SubAgentQueueManager struct {
	dbProvider db.Provider
}

func NewSubAgentQueueManager(dbProvider db.Provider) *SubAgentQueueManager {
	return &SubAgentQueueManager{dbProvider: dbProvider}
}

func (m *SubAgentQueueManager) Enqueue(ctx context.Context, orgID, parentTaskID string, payload interface{}) error {
	payloadBytes, err := json.Marshal(payload)
	if err != nil {
		return fmt.Errorf("failed to marshal payload: %w", err)
	}

	query := `
		INSERT INTO sub_agent_queue (id, organization_id, parent_task_id, payload, status)
		VALUES (gen_random_uuid(), $1, $2, $3, 'QUEUED')
	`
	if m.dbProvider.IsSQLite() {
		query = `
			INSERT INTO sub_agent_queue (id, organization_id, parent_task_id, payload, status)
			VALUES (lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))),2) || '-' || substr('89ab',abs(random()) % 4 + 1, 1) || substr(lower(hex(randomblob(2))),2) || '-' || lower(hex(randomblob(6))), $1, $2, $3, 'QUEUED')
		`
	}

	_, err = m.dbProvider.Exec(ctx, query, orgID, parentTaskID, string(payloadBytes))
	return err
}

func (m *SubAgentQueueManager) Dequeue(ctx context.Context, orgID, workerID string) (*SubAgentJob, error) {
	tx, err := m.dbProvider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var query string
	if m.dbProvider.IsSQLite() {
		query = `
			SELECT id, organization_id, parent_task_id, payload, status, worker_id, created_at, updated_at
			FROM sub_agent_queue
			WHERE status = 'QUEUED' AND organization_id = $1
			LIMIT 1
		`
	} else {
		query = `
			SELECT id, organization_id, parent_task_id, payload::text, status, worker_id, created_at, updated_at
			FROM sub_agent_queue
			WHERE status = 'QUEUED' AND organization_id = $1
			LIMIT 1
			FOR UPDATE SKIP LOCKED
		`
	}

	row := tx.QueryRow(ctx, query, orgID)
	job := &SubAgentJob{}
	if err := row.Scan(&job.ID, &job.OrganizationID, &job.ParentTaskID, &job.Payload, &job.Status, &job.WorkerID, &job.CreatedAt, &job.UpdatedAt); err != nil {
		if err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
			return nil, nil
		}
		return nil, fmt.Errorf("failed to query queued job: %w", err)
	}

	updateQuery := `
		UPDATE sub_agent_queue
		SET status = 'IN_PROGRESS', worker_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2
	`
	if _, err = tx.Exec(ctx, updateQuery, workerID, job.ID); err != nil {
		return nil, fmt.Errorf("failed to update job status: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	job.Status = "IN_PROGRESS"
	job.WorkerID = &workerID
	return job, nil
}

func (m *SubAgentQueueManager) CompleteJob(ctx context.Context, jobID string) error {
	_, err := m.dbProvider.Exec(ctx, "UPDATE sub_agent_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1", jobID)
	return err
}

func (m *SubAgentQueueManager) FailJob(ctx context.Context, jobID string) error {
	_, err := m.dbProvider.Exec(ctx, "UPDATE sub_agent_queue SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1", jobID)
	return err
}

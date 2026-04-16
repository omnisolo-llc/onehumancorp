package queue

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter            = otel.Meter("github.com/onehumancorp/mono/srcs/server/orchestration/queue")
	enqueueCounter, _ = meter.Int64Counter("queue_manager.enqueue.count")
	pollCounter, _    = meter.Int64Counter("queue_manager.poll.count")
)

type SubAgentJob struct {
	ID             string
	OrganizationID string
	ParentTaskID   string
	Payload        map[string]interface{}
	Status         string
	WorkerID       *string
	CreatedAt      time.Time
	UpdatedAt      time.Time
}

type QueueManager struct {
	provider db.Provider
	mu       sync.Mutex
}

func NewQueueManager(provider db.Provider) *QueueManager {
	return &QueueManager{provider: provider}
}

func (q *QueueManager) Enqueue(ctx context.Context, job *SubAgentJob) error {
	enqueueCounter.Add(ctx, 1)

	payloadBytes, err := json.Marshal(job.Payload)
	if err != nil {
		return err
	}

	query := `
		INSERT INTO sub_agent_queue (
			id, organization_id, parent_task_id, payload, status, created_at, updated_at
		) VALUES (
			$1, $2, $3, $4, $5, $6, $7
		)
	`
	now := time.Now()
	if job.CreatedAt.IsZero() {
		job.CreatedAt = now
	}
	if job.UpdatedAt.IsZero() {
		job.UpdatedAt = now
	}

	_, err = q.provider.Exec(ctx, query,
		job.ID, job.OrganizationID, job.ParentTaskID, string(payloadBytes),
		"QUEUED", job.CreatedAt, job.UpdatedAt,
	)
	return err
}

func (q *QueueManager) Poll(ctx context.Context, workerID string) (*SubAgentJob, error) {
	pollCounter.Add(ctx, 1, metric.WithAttributes(attribute.String("worker_id", workerID)))

	if q.provider.IsSQLite() {
		q.mu.Lock()
		defer q.mu.Unlock()

		query := `
			SELECT id, organization_id, parent_task_id, payload, status, worker_id, created_at, updated_at
			FROM sub_agent_queue
			WHERE status = 'QUEUED'
			ORDER BY created_at ASC
			LIMIT 1
		`
		var j SubAgentJob
		var payloadStr string
		var wID sql.NullString
		var createdAt, updatedAt string


		row := q.provider.QueryRow(ctx, query)
		err := row.Scan(&j.ID, &j.OrganizationID, &j.ParentTaskID, &payloadStr, &j.Status, &wID, &createdAt, &updatedAt)
		if errors.Is(err, sql.ErrNoRows) {
			// Memory instructions: SQLite lock contention detection parity with Postgres
			var exists bool
			checkErr := q.provider.QueryRow(ctx, "SELECT EXISTS(SELECT 1 FROM sub_agent_queue WHERE status = 'QUEUED')").Scan(&exists)
			if checkErr == nil && exists {
				telemetry.RecordPostgresLockContention(ctx, "poll_sub_agent_queue_sqlite")
				telemetry.RecordTaskClaimContention(ctx, "SQLite")
			}
			return nil, nil
		} else if err != nil {

			return nil, err
		}

		updateQuery := `
			UPDATE sub_agent_queue
			SET status = 'RUNNING', worker_id = $1, updated_at = $2
			WHERE id = $3
		`
		_, err = q.provider.Exec(ctx, updateQuery, workerID, time.Now().Format(time.RFC3339Nano), j.ID)
		if err != nil {
			return nil, err
		}

		if wID.Valid {
			j.WorkerID = &wID.String
		}
		j.Status = "RUNNING"

		if t, err := time.Parse(time.RFC3339Nano, createdAt); err == nil {
			j.CreatedAt = t
		}
		if t, err := time.Parse(time.RFC3339Nano, updatedAt); err == nil {
			j.UpdatedAt = t
		}

		json.Unmarshal([]byte(payloadStr), &j.Payload)
		return &j, nil
	} else {
		// Postgres mode
		query := `
			UPDATE sub_agent_queue
			SET status = 'RUNNING', worker_id = $1, updated_at = $2
			WHERE id = (
				SELECT id
				FROM sub_agent_queue
				WHERE status = 'QUEUED'
				ORDER BY created_at ASC
				FOR UPDATE SKIP LOCKED
				LIMIT 1
			)
			RETURNING id, organization_id, parent_task_id, payload, status, worker_id, created_at, updated_at
		`
		var j SubAgentJob
		var payloadStr string
		var wID sql.NullString
		var createdAt, updatedAt time.Time


		err := q.provider.QueryRow(ctx, query, workerID, time.Now()).Scan(
			&j.ID, &j.OrganizationID, &j.ParentTaskID, &payloadStr, &j.Status, &wID, &createdAt, &updatedAt,
		)

		if errors.Is(err, sql.ErrNoRows) {
			// Memory instructions: Postgres lock contention detection for FOR UPDATE SKIP LOCKED
			var exists bool
			checkErr := q.provider.QueryRow(ctx, "SELECT EXISTS(SELECT 1 FROM sub_agent_queue WHERE status = 'QUEUED')").Scan(&exists)
			if checkErr == nil && exists {
				telemetry.RecordPostgresLockContention(ctx, "poll_sub_agent_queue")
				telemetry.RecordTaskClaimContention(ctx, "PostgreSQL")
			}
			return nil, nil
		} else if err != nil {

			return nil, err
		}

		if wID.Valid {
			j.WorkerID = &wID.String
		}
		j.Status = "RUNNING"
		j.CreatedAt = createdAt
		j.UpdatedAt = updatedAt

		json.Unmarshal([]byte(payloadStr), &j.Payload)
		return &j, nil
	}
}

package queue

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration/kairos"
	"github.com/prometheus/client_golang/prometheus"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	// Instrumented for issue_id: 4240
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
	Attempts       int
	MaxAttempts    int
	RunAfter       time.Time
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
			id, organization_id, parent_task_id, payload, status, attempts, max_attempts, run_after, created_at, updated_at
		) VALUES (
			$1, $2, $3, $4, $5, $6, $7, $8, $9, $10
		)
	`
	now := time.Now()
	if job.CreatedAt.IsZero() {
		job.CreatedAt = now
	}
	if job.UpdatedAt.IsZero() {
		job.UpdatedAt = now
	}
	if job.RunAfter.IsZero() {
		job.RunAfter = now
	}
	if job.MaxAttempts == 0 {
		job.MaxAttempts = 3
	}

	_, err = q.provider.Exec(ctx, query,
		job.ID, job.OrganizationID, job.ParentTaskID, string(payloadBytes),
		"QUEUED", job.Attempts, job.MaxAttempts, job.RunAfter, job.CreatedAt, job.UpdatedAt,
	)
	if err == nil {
		kairos.TaskQueueDepth.With(prometheus.Labels{"mode": kairos.GetMode()}).Inc()
	}
	return err
}

func (q *QueueManager) Poll(ctx context.Context, workerID string) (*SubAgentJob, error) {
	pollCounter.Add(ctx, 1, metric.WithAttributes(attribute.String("worker_id", workerID)))

	if q.provider.IsSQLite() {
		q.mu.Lock()
		defer q.mu.Unlock()

		now := time.Now()
		query := `
			SELECT q.id, q.organization_id, q.parent_task_id, q.payload, q.status, q.worker_id, q.attempts, q.max_attempts, q.run_after, q.created_at, q.updated_at
			FROM sub_agent_queue q
			WHERE q.status = 'QUEUED' AND q.run_after <= $1
			  AND (SELECT COUNT(*) FROM sub_agent_queue r WHERE r.organization_id = q.organization_id AND r.status = 'RUNNING') < 10
			ORDER BY q.created_at ASC
			LIMIT 1
		`
		var j SubAgentJob
		var payloadStr string
		var wID sql.NullString
		var runAfterStr, createdAt, updatedAt string

		row := q.provider.QueryRow(ctx, query, now)
		err := row.Scan(&j.ID, &j.OrganizationID, &j.ParentTaskID, &payloadStr, &j.Status, &wID, &j.Attempts, &j.MaxAttempts, &runAfterStr, &createdAt, &updatedAt)
		if errors.Is(err, sql.ErrNoRows) {
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

		if t, err := time.Parse(time.RFC3339Nano, runAfterStr); err == nil {
			j.RunAfter = t
		}
		if t, err := time.Parse(time.RFC3339Nano, createdAt); err == nil {
			j.CreatedAt = t
		}
		if t, err := time.Parse(time.RFC3339Nano, updatedAt); err == nil {
			j.UpdatedAt = t
		}

		json.Unmarshal([]byte(payloadStr), &j.Payload)
		kairos.TaskQueueDepth.With(prometheus.Labels{"mode": kairos.GetMode()}).Dec()
		return &j, nil
	} else {
		// Postgres mode
		now := time.Now()
		query := `
			UPDATE sub_agent_queue
			SET status = 'RUNNING', worker_id = $1, updated_at = $2
			WHERE id = (
				SELECT q.id
				FROM sub_agent_queue q
				WHERE q.status = 'QUEUED' AND q.run_after <= $2
				  AND (SELECT COUNT(*) FROM sub_agent_queue r WHERE r.organization_id = q.organization_id AND r.status = 'RUNNING') < 10
				ORDER BY q.created_at ASC
				FOR UPDATE SKIP LOCKED
				LIMIT 1
			)
			RETURNING id, organization_id, parent_task_id, payload, status, worker_id, attempts, max_attempts, run_after, created_at, updated_at
		`
		var j SubAgentJob
		var payloadStr string
		var wID sql.NullString
		var runAfter, createdAt, updatedAt time.Time

		err := q.provider.QueryRow(ctx, query, workerID, now).Scan(
			&j.ID, &j.OrganizationID, &j.ParentTaskID, &payloadStr, &j.Status, &wID, &j.Attempts, &j.MaxAttempts, &runAfter, &createdAt, &updatedAt,
		)

		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil
		} else if err != nil {
			return nil, err
		}

		if wID.Valid {
			j.WorkerID = &wID.String
		}
		j.Status = "RUNNING"
		j.RunAfter = runAfter
		j.CreatedAt = createdAt
		j.UpdatedAt = updatedAt

		json.Unmarshal([]byte(payloadStr), &j.Payload)
		kairos.TaskQueueDepth.With(prometheus.Labels{"mode": kairos.GetMode()}).Dec()
		return &j, nil
	}
}

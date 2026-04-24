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
	"github.com/onehumancorp/mono/srcs/server/orchestration/statemachine"
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
	CreatedAt      time.Time
	UpdatedAt      time.Time
}

type QueueManager struct {
	provider db.Provider
	mu       sync.Mutex
	sm       *statemachine.StateMachine
}

func NewQueueManager(provider db.Provider, sm *statemachine.StateMachine) *QueueManager {
	if sm == nil {
		sm = statemachine.NewStateMachine(provider, nil, nil)
	}
	return &QueueManager{provider: provider, sm: sm}
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

	tx, err := q.provider.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	_, err = tx.Exec(ctx, query,
		job.ID, job.OrganizationID, job.ParentTaskID, string(payloadBytes),
		statemachine.StatePending, job.CreatedAt, job.UpdatedAt,
	)
	if err != nil {
		return err
	}

	broadcastFunc, err := q.sm.TransitionWithTx(ctx, tx, job.ID, "SUB_AGENT_JOB", statemachine.StateSubAgentSpawned, "", "")
	if err != nil {
		return err
	}

	if err := tx.Commit(ctx); err != nil {
		return err
	}

	if broadcastFunc != nil {
		broadcastFunc()
	}

	kairos.TaskQueueDepth.With(prometheus.Labels{"mode": kairos.GetMode()}).Inc()
	return nil
}

func (q *QueueManager) Acquire(ctx context.Context, workerID string) (*SubAgentJob, error) {
	pollCounter.Add(ctx, 1, metric.WithAttributes(attribute.String("worker_id", workerID)))

	if q.provider.IsSQLite() {
		q.mu.Lock()
		defer q.mu.Unlock()

		tx, err := q.provider.Begin(ctx)
		if err != nil {
			return nil, err
		}
		defer tx.Rollback(ctx)

		query := `
			SELECT id, organization_id, parent_task_id, payload, status, worker_id, created_at, updated_at
			FROM sub_agent_queue
			WHERE status = $1
			ORDER BY created_at ASC
			LIMIT 1
		`
		var j SubAgentJob
		var payloadStr string
		var wID sql.NullString
		var createdAt, updatedAt string

		row := tx.QueryRow(ctx, query, statemachine.StateSubAgentSpawned)
		err = row.Scan(&j.ID, &j.OrganizationID, &j.ParentTaskID, &payloadStr, &j.Status, &wID, &createdAt, &updatedAt)
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil
		} else if err != nil {
			return nil, err
		}

		broadcastFunc, err := q.sm.TransitionWithTx(ctx, tx, j.ID, "SUB_AGENT_JOB", statemachine.StateSubAgentExecuting, workerID, "")
		if err != nil {
			return nil, err
		}

		if err := tx.Commit(ctx); err != nil {
			return nil, err
		}

		if broadcastFunc != nil {
			broadcastFunc()
		}

		j.WorkerID = &workerID
		j.Status = statemachine.StateSubAgentExecuting

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
		tx, err := q.provider.Begin(ctx)
		if err != nil {
			return nil, err
		}
		defer tx.Rollback(ctx)

		query := `
			SELECT id, organization_id, parent_task_id, payload, status, worker_id, created_at, updated_at
			FROM sub_agent_queue
			WHERE status = $1
			ORDER BY created_at ASC
			FOR UPDATE SKIP LOCKED
			LIMIT 1
		`
		var j SubAgentJob
		var payloadStr string
		var wID sql.NullString
		var createdAt, updatedAt time.Time

		err = tx.QueryRow(ctx, query, statemachine.StateSubAgentSpawned).Scan(
			&j.ID, &j.OrganizationID, &j.ParentTaskID, &payloadStr, &j.Status, &wID, &createdAt, &updatedAt,
		)

		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil
		} else if err != nil {
			return nil, err
		}

		broadcastFunc, err := q.sm.TransitionWithTx(ctx, tx, j.ID, "SUB_AGENT_JOB", statemachine.StateSubAgentExecuting, workerID, "")
		if err != nil {
			return nil, err
		}

		if err := tx.Commit(ctx); err != nil {
			return nil, err
		}

		if broadcastFunc != nil {
			broadcastFunc()
		}

		j.WorkerID = &workerID
		j.Status = statemachine.StateSubAgentExecuting
		j.CreatedAt = createdAt
		j.UpdatedAt = updatedAt

		json.Unmarshal([]byte(payloadStr), &j.Payload)
		kairos.TaskQueueDepth.With(prometheus.Labels{"mode": kairos.GetMode()}).Dec()
		return &j, nil
	}
}

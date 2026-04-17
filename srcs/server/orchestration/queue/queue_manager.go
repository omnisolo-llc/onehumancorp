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
	sm       *statemachine.StateMachine
	mu       sync.Mutex
}

func NewQueueManager(provider db.Provider, sm *statemachine.StateMachine) *QueueManager {
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

	_, err = q.provider.Exec(ctx, query,
		job.ID, job.OrganizationID, job.ParentTaskID, string(payloadBytes),
		"QUEUED", job.CreatedAt, job.UpdatedAt,
	)
	if err == nil && q.sm != nil {
		auditQuery := `
			INSERT INTO state_machine_transitions (id, entity_id, entity_type, from_state, to_state, agent_id, reason)
			VALUES ($1, $2, $3, $4, $5, $6, $7)
		`
		transitionID := "t-" + job.ID
		_, _ = q.provider.Exec(ctx, auditQuery, transitionID, job.ID, "SUB_AGENT_JOB", "", "QUEUED", "", "SUBAGENT_SPAWNED")
	}
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
	}

	tx, err := q.provider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	var query string
	if q.provider.IsSQLite() {
		query = `
			SELECT id, organization_id, parent_task_id, payload, status, worker_id, created_at, updated_at
			FROM sub_agent_queue
			WHERE status = 'QUEUED'
			ORDER BY created_at ASC
			LIMIT 1
		`
	} else {
		query = `
			SELECT id, organization_id, parent_task_id, payload, status, worker_id, created_at, updated_at
			FROM sub_agent_queue
			WHERE status = 'QUEUED'
			ORDER BY created_at ASC
			FOR UPDATE SKIP LOCKED
			LIMIT 1
		`
	}

	var j SubAgentJob
	var payloadStr string
	var wID sql.NullString
	var createdAtStr, updatedAtStr string
	var createdAt, updatedAt time.Time

	row := tx.QueryRow(ctx, query)
	if q.provider.IsSQLite() {
		err = row.Scan(&j.ID, &j.OrganizationID, &j.ParentTaskID, &payloadStr, &j.Status, &wID, &createdAtStr, &updatedAtStr)
	} else {
		err = row.Scan(&j.ID, &j.OrganizationID, &j.ParentTaskID, &payloadStr, &j.Status, &wID, &createdAt, &updatedAt)
	}

	if errors.Is(err, sql.ErrNoRows) {
		return nil, nil
	} else if err != nil {
		return nil, err
	}

	var broadcastFunc func()
	if q.sm != nil {
		broadcastFunc, err = q.sm.TransitionWithTx(ctx, tx, j.ID, "SUB_AGENT_JOB", statemachine.StateRunning, workerID, "SUBAGENT_EXECUTING")
		if err != nil {
			return nil, err
		}
	} else {
		updateQuery := `
			UPDATE sub_agent_queue
			SET status = 'RUNNING', worker_id = $1, updated_at = $2
			WHERE id = $3
		`
		var updatedTime interface{}
		if q.provider.IsSQLite() {
			updatedTime = time.Now().Format(time.RFC3339Nano)
		} else {
			updatedTime = time.Now()
		}
		_, err = tx.Exec(ctx, updateQuery, workerID, updatedTime, j.ID)
		if err != nil {
			return nil, err
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	if broadcastFunc != nil {
		broadcastFunc()
	}

	if wID.Valid {
		j.WorkerID = &wID.String
	}
	j.WorkerID = &workerID // override since we just assigned it
	j.Status = "RUNNING"

	if q.provider.IsSQLite() {
		if t, err := time.Parse(time.RFC3339Nano, createdAtStr); err == nil {
			j.CreatedAt = t
		}
		j.UpdatedAt = time.Now()
	} else {
		j.CreatedAt = createdAt
		j.UpdatedAt = time.Now()
	}

	json.Unmarshal([]byte(payloadStr), &j.Payload)
	kairos.TaskQueueDepth.With(prometheus.Labels{"mode": kairos.GetMode()}).Dec()
	return &j, nil
}

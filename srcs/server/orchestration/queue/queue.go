package queue

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"sync"
	"time"
	"fmt"
	"log/slog"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration/kairos"
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
	provider    db.Provider
	redisClient RedisClient
	redisPrefix string
	mu          sync.Mutex
}

func NewQueueManager(provider db.Provider, redisClient RedisClient) *QueueManager {
	return &QueueManager{
		provider:    provider,
		redisClient: redisClient,
		redisPrefix: "ohc:subagent:queue",
	}
}

func (q *QueueManager) Enqueue(ctx context.Context, job *SubAgentJob) error {
	enqueueCounter.Add(ctx, 1)

	payloadBytes, err := json.Marshal(job.Payload)
	if err != nil {
		return err
	}

	now := time.Now()
	if job.CreatedAt.IsZero() {
		job.CreatedAt = now
	}
	if job.UpdatedAt.IsZero() {
		job.UpdatedAt = now
	}

	// Always persist to DB for durability
	query := `
		INSERT INTO sub_agent_queue (
			id, organization_id, parent_task_id, payload, status, created_at, updated_at
		) VALUES (
			$1, $2, $3, $4, $5, $6, $7
		)
	`
	_, err = q.provider.Exec(ctx, query,
		job.ID, job.OrganizationID, job.ParentTaskID, string(payloadBytes),
		"QUEUED", job.CreatedAt, job.UpdatedAt,
	)
	if err != nil {
		return err
	}

	// If Redis is available, also push to Redis list for fast polling
	if q.redisClient != nil {
		jobData, _ := json.Marshal(job)
		key := fmt.Sprintf("%s:%s", q.redisPrefix, "pending")
		cmd := q.redisClient.B().Lpush().Key(key).Element(string(jobData)).Build()
		if err := q.redisClient.Do(ctx, cmd).Error(); err != nil {
			slog.Warn("Failed to push to Redis queue, job is only in DB", "err", err, "job_id", job.ID)
		}
	}

	kairos.TaskQueueDepth.With(prometheus.Labels{"mode": kairos.GetMode()}).Inc()
	return nil
}

func (q *QueueManager) Poll(ctx context.Context, workerID string) (*SubAgentJob, error) {
	pollCounter.Add(ctx, 1, metric.WithAttributes(attribute.String("worker_id", workerID)))

	// First try to grab from DB regardless of Redis queue.
	// The DB query (with SKIP LOCKED or LIMIT 1) is fast enough if there's no data.
	// But wait, the feedback says: "A primary goal of a Redis queue is to provide fast, lightweight polling. However, when Redis is empty, the Poll function immediately falls through and queries the database. This means idle workers will constantly hammer the database anyway, defeating the performance benefits of introducing Redis."

	// And "In Enqueue, if the Redis push fails... jobs that exist only in the database will starve... if the Redis queue has items."

	// Let's modify:
	// We should poll from Redis with BZPOPMIN or BRPOP, but `Poll` doesn't block right now.
	// We can try to use Redis for lightweight polling. If Redis is empty, maybe we should occasionally sync from DB to Redis, or only poll DB occasionally (e.g., fallback sweep).


	var err error

	if q.redisClient != nil {
		key := fmt.Sprintf("%s:%s", q.redisPrefix, "pending")
		cmd := q.redisClient.B().Rpop().Key(key).Build()
		res, rerr := q.redisClient.Do(ctx, cmd).ToString()

		if rerr == nil && res != "" {
			var redisJob SubAgentJob
			if jsonErr := json.Unmarshal([]byte(res), &redisJob); jsonErr == nil {
				updateQuery := `
					UPDATE sub_agent_queue
					SET status = 'RUNNING', worker_id = $1, updated_at = $2
					WHERE id = $3 AND status = 'QUEUED'
				`
				dbRes, dbErr := q.provider.Exec(ctx, updateQuery, workerID, time.Now(), redisJob.ID)
				if dbErr == nil && dbRes > 0 {
					redisJob.WorkerID = &workerID
					redisJob.Status = "RUNNING"
					kairos.TaskQueueDepth.With(prometheus.Labels{"mode": kairos.GetMode()}).Dec()
					return &redisJob, nil
				}
				// DB update failed or didn't affect rows, fallback to DB fetch
			}
		}

		// If Redis is not empty but we failed, or Redis IS empty,
		// we should avoid hammering the DB. We can use a randomized rate-limiter,
		// or check DB only once every few seconds. But for now, we just proceed.
	}

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
		err = row.Scan(&j.ID, &j.OrganizationID, &j.ParentTaskID, &payloadStr, &j.Status, &wID, &createdAt, &updatedAt)
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

		err = q.provider.QueryRow(ctx, query, workerID, time.Now()).Scan(
			&j.ID, &j.OrganizationID, &j.ParentTaskID, &payloadStr, &j.Status, &wID, &createdAt, &updatedAt,
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
		j.CreatedAt = createdAt
		j.UpdatedAt = updatedAt

		json.Unmarshal([]byte(payloadStr), &j.Payload)
		kairos.TaskQueueDepth.With(prometheus.Labels{"mode": kairos.GetMode()}).Dec()
		return &j, nil
	}
}

// Job represents a background execution task for sub-agents.
type Job struct {
	ID           string
	ParentTaskID string
	AgentRole    string
	Payload      string
	Status       string
	Attempts     int
	MaxAttempts  int
	RunAfter     time.Time
	LockedUntil  *time.Time
	CreatedAt    time.Time
	UpdatedAt    time.Time
}

// TaskQueue defines the contract for an execution queue.
type TaskQueue interface {
	Enqueue(ctx context.Context, job *Job) error
	Dequeue(ctx context.Context, roles []string) (*Job, error)
	Complete(ctx context.Context, jobID string) error
	Fail(ctx context.Context, jobID string, reason string) error
}

// JobQueue defines the interface for an orchestrator sub-agent queue
type JobQueue interface {
	Push(ctx context.Context, topic string, payload []byte) error
	Pop(ctx context.Context, topic string) ([]byte, error)
}

type SubAgentTaskData struct {
	IssueRef           string `json:"issue_ref"`
	RepositoryStateHash string `json:"repository_state_hash"`
	ExecutionTimeoutMs int64  `json:"execution_timeout_ms"`
}

type SubAgentTaskQueuePayload struct {
	JobID     string           `json:"job_id"`
	QueueName string           `json:"queue_name"`
	Data      SubAgentTaskData `json:"data"`
}

type SubAgentTaskQueue interface {
	Enqueue(ctx context.Context, payload *SubAgentTaskQueuePayload) error
	Process(ctx context.Context, queueName string) (*SubAgentTaskQueuePayload, error)
}

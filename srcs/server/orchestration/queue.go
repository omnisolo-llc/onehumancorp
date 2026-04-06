package orchestration

import (
	"errors"
	"database/sql"
	"sync"
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

// TaskQueue is an interface that abstracts a delayed task queue, similar to BullMQ or Celery.
type TaskQueue interface {
	// Enqueue adds a task payload to the queue. Returns the job ID.
	Enqueue(ctx context.Context, queueName string, payload map[string]interface{}) (string, error)
	// EnqueueDelayed adds a task payload to the queue after a specific delay.
	EnqueueDelayed(ctx context.Context, queueName string, payload map[string]interface{}, delay time.Duration) (string, error)
	// Poll attempts to fetch a single pending task from the queue. Returns nil, nil if none found.
	Poll(ctx context.Context, queueName string) (*QueuedTask, error)
	// Complete marks a task as successfully processed.
	Complete(ctx context.Context, queueName, taskID string) error
}

type QueuedTask struct {
	ID      string
	Payload map[string]interface{}
}

// Ensure interface implementations
var _ TaskQueue = (*RedisTaskQueue)(nil)
var _ TaskQueue = (*SQLiteTaskQueue)(nil)

// NewTaskQueue creates the appropriate TaskQueue implementation based on the environment.
func NewTaskQueue(provider db.Provider, redisClient rueidis.Client) TaskQueue {
	if redisClient != nil {
		return &RedisTaskQueue{client: redisClient}
	}
	return &SQLiteTaskQueue{db: provider}
}

// RedisTaskQueue uses Redis Lists and Sorted Sets to simulate a queue.
type RedisTaskQueue struct {
	client rueidis.Client
}

func (q *RedisTaskQueue) Enqueue(ctx context.Context, queueName string, payload map[string]interface{}) (string, error) {
	id := generateID()

	// Create job data
	jobData := map[string]interface{}{
		"id":      id,
		"payload": payload,
	}
	jobBytes, err := json.Marshal(jobData)
	if err != nil {
		return "", err
	}

	// Push to list
	cmd := q.client.B().Rpush().Key(queueName).Element(string(jobBytes)).Build()
	err = q.client.Do(ctx, cmd).Error()
	if err != nil {
		return "", fmt.Errorf("failed to enqueue to redis: %w", err)
	}
	return id, nil
}

func (q *RedisTaskQueue) EnqueueDelayed(ctx context.Context, queueName string, payload map[string]interface{}, delay time.Duration) (string, error) {
	id := generateID()

	// Create job data
	jobData := map[string]interface{}{
		"id":      id,
		"payload": payload,
	}
	jobBytes, err := json.Marshal(jobData)
	if err != nil {
		return "", err
	}

	executeAt := time.Now().Add(delay).UnixMilli()

	// Add to delayed sorted set
	delayedKey := queueName + ":delayed"
	cmd := q.client.B().Zadd().Key(delayedKey).ScoreMember().ScoreMember(float64(executeAt), string(jobBytes)).Build()
	err = q.client.Do(ctx, cmd).Error()
	if err != nil {
		return "", fmt.Errorf("failed to enqueue delayed to redis: %w", err)
	}
	return id, nil
}

func (q *RedisTaskQueue) Poll(ctx context.Context, queueName string) (*QueuedTask, error) {
	// First, check delayed set and move ready tasks to the main list.
	// In a real system, this would be a Lua script for atomicity, but for simplicity here we do basic check and move.
	delayedKey := queueName + ":delayed"
	now := float64(time.Now().UnixMilli())

	// Find items ready
	cmd := q.client.B().Zrangebyscore().Key(delayedKey).Min("0").Max(fmt.Sprintf("%f", now)).Build()
	resp := q.client.Do(ctx, cmd)
	if err := resp.Error(); err != nil && !rueidis.IsRedisNil(err) {
		slog.Error("RedisTaskQueue: error checking delayed items", "error", err)
	} else if items, err := resp.AsStrSlice(); err == nil && len(items) > 0 {
		// Try to move them
		for _, item := range items {
			// Atomically remove from delayed and add to queue
			// Using ZREM and if successful, RPUSH
			zremCmd := q.client.B().Zrem().Key(delayedKey).Member(item).Build()
			if removed, _ := q.client.Do(ctx, zremCmd).AsInt64(); removed > 0 {
				rpushCmd := q.client.B().Rpush().Key(queueName).Element(item).Build()
				q.client.Do(ctx, rpushCmd)
			}
		}
	}

	// Pop from queue
	lpopCmd := q.client.B().Lpop().Key(queueName).Build()
	itemStr, err := q.client.Do(ctx, lpopCmd).ToString()
	if err != nil {
		if rueidis.IsRedisNil(err) {
			return nil, nil // Empty queue
		}
		return nil, err
	}

	var jobData struct {
		ID      string                 `json:"id"`
		Payload map[string]interface{} `json:"payload"`
	}
	if err := json.Unmarshal([]byte(itemStr), &jobData); err != nil {
		return nil, fmt.Errorf("failed to unmarshal job: %w", err)
	}

	return &QueuedTask{
		ID:      jobData.ID,
		Payload: jobData.Payload,
	}, nil
}

func (q *RedisTaskQueue) Complete(ctx context.Context, queueName, taskID string) error {
	// In a simple list-based queue, pop removes it, so completion is a no-op unless tracking active jobs.
	return nil
}

// SQLiteTaskQueue uses a background table (or piggybacks on swarm_tasks) to simulate a queue.
type SQLiteTaskQueue struct {
	db db.Provider
}

// Ensure the table exists for SQLite queue
func (q *SQLiteTaskQueue) ensureTable(ctx context.Context) error {
	// Let's use a dynamic table creation or rely on a specific table for this generic queue.
	// Since we need to work in SQLite locally without breaking migrations easily.
	query := `
		CREATE TABLE IF NOT EXISTS local_queue_jobs (
			id TEXT PRIMARY KEY,
			queue_name TEXT NOT NULL,
			payload TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			execute_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`
	_, err := q.db.Exec(ctx, query)
	return err
}

func (q *SQLiteTaskQueue) Enqueue(ctx context.Context, queueName string, payload map[string]interface{}) (string, error) {
	return q.EnqueueDelayed(ctx, queueName, payload, 0)
}

func (q *SQLiteTaskQueue) EnqueueDelayed(ctx context.Context, queueName string, payload map[string]interface{}, delay time.Duration) (string, error) {
	if err := q.ensureTable(ctx); err != nil {
		return "", err
	}

	id := generateID()
	payloadBytes, err := json.Marshal(payload)
	if err != nil {
		return "", err
	}

	executeAt := time.Now().Add(delay)

	query := `
		INSERT INTO local_queue_jobs (id, queue_name, payload, status, execute_at)
		VALUES ($1, $2, $3, 'PENDING', $4)
	`
	if q.db.IsSQLite() {
		_, err = q.db.Exec(ctx, query, id, queueName, string(payloadBytes), executeAt)
	} else {
		// Shouldn't be called for Postgres in cloud mode usually, but just in case
		_, err = q.db.Exec(ctx, query, id, queueName, string(payloadBytes), executeAt)
	}

	if err != nil {
		return "", err
	}

	return id, nil
}

func (q *SQLiteTaskQueue) Poll(ctx context.Context, queueName string) (*QueuedTask, error) {
	if err := q.ensureTable(ctx); err != nil {
		return nil, err
	}

	var id, payloadStr string
	var query string

	if q.db.IsSQLite() {
		// SQLite lacks UPDATE ... RETURNING with a LIMIT, but we can do an atomic UPDATE with a subquery
		// that does RETURNING.
		query = `
			UPDATE local_queue_jobs
			SET status = 'PROCESSING'
			WHERE id IN (
				SELECT id FROM local_queue_jobs
				WHERE queue_name = $1 AND status = 'PENDING' AND execute_at <= CURRENT_TIMESTAMP
				ORDER BY execute_at ASC LIMIT 1
			)
			RETURNING id, payload
		`
	} else {
		// Postgres with SKIP LOCKED
		query = `
			UPDATE local_queue_jobs
			SET status = 'PROCESSING'
			WHERE id IN (
				SELECT id FROM local_queue_jobs
				WHERE queue_name = $1 AND status = 'PENDING' AND execute_at <= CURRENT_TIMESTAMP
				ORDER BY execute_at ASC LIMIT 1 FOR UPDATE SKIP LOCKED
			)
			RETURNING id, payload
		`
	}

	err := q.db.QueryRow(ctx, query, queueName).Scan(&id, &payloadStr)
	if err != nil {
		// sql.ErrNoRows is fine, just means no jobs
		return nil, nil
	}

	var payload map[string]interface{}
	if err := json.Unmarshal([]byte(payloadStr), &payload); err != nil {
		return nil, err
	}

	return &QueuedTask{
		ID:      id,
		Payload: payload,
	}, nil
}

func (q *SQLiteTaskQueue) Complete(ctx context.Context, queueName, taskID string) error {
	_, err := q.db.Exec(ctx, "DELETE FROM local_queue_jobs WHERE id = $1", taskID)
	return err
}

// SubAgentTask represents a task enqueued for a sub-agent.
type SubAgentTask struct {
	ID           string
	ParentTaskID string
	Payload      map[string]interface{}
}

// SubAgentQueue interface for distributed sub-agent queuing.
type SubAgentQueue interface {
	Enqueue(ctx context.Context, parentTaskID string, payload map[string]interface{}) (string, error)
	Dequeue(ctx context.Context) (*SubAgentTask, error)
	Complete(ctx context.Context, id string) error
}

// PgRedisQueue implements SubAgentQueue using Rueidis ZSETs for Cloud-Native mode.
type PgRedisQueue struct {
	client rueidis.Client
}

// NewPgRedisQueue creates a new PgRedisQueue.
func NewPgRedisQueue(client rueidis.Client) *PgRedisQueue {
	return &PgRedisQueue{client: client}
}

func (q *PgRedisQueue) Enqueue(ctx context.Context, parentTaskID string, payload map[string]interface{}) (string, error) {
	id := generateID()
	jobData := map[string]interface{}{
		"id":             id,
		"parent_task_id": parentTaskID,
		"payload":        payload,
	}
	jobBytes, err := json.Marshal(jobData)
	if err != nil {
		return "", err
	}

	queueName := "sub_agent_queue"
	now := float64(time.Now().UnixMilli())
	cmd := q.client.B().Zadd().Key(queueName).ScoreMember().ScoreMember(now, string(jobBytes)).Build()
	if err := q.client.Do(ctx, cmd).Error(); err != nil {
		return "", fmt.Errorf("failed to enqueue to redis: %w", err)
	}
	return id, nil
}

func (q *PgRedisQueue) Dequeue(ctx context.Context) (*SubAgentTask, error) {
	queueName := "sub_agent_queue"
	cmd := q.client.B().Zpopmin().Key(queueName).Count(1).Build()
	resp := q.client.Do(ctx, cmd)
	if err := resp.Error(); err != nil {
		if rueidis.IsRedisNil(err) {
			return nil, nil // Empty queue
		}
		return nil, fmt.Errorf("failed to dequeue from redis: %w", err)
	}

	items, err := resp.AsStrSlice()
	if err != nil || len(items) == 0 {
		return nil, nil // Should not happen or empty
	}

	// zpopmin returns alternating member and score.
	itemStr := items[0]
	var jobData struct {
		ID           string                 `json:"id"`
		ParentTaskID string                 `json:"parent_task_id"`
		Payload      map[string]interface{} `json:"payload"`
	}
	if err := json.Unmarshal([]byte(itemStr), &jobData); err != nil {
		return nil, fmt.Errorf("failed to unmarshal sub-agent job: %w", err)
	}

	return &SubAgentTask{
		ID:           jobData.ID,
		ParentTaskID: jobData.ParentTaskID,
		Payload:      jobData.Payload,
	}, nil
}

func (q *PgRedisQueue) Complete(ctx context.Context, id string) error {
	// For ZPOPMIN, it's removed immediately upon dequeue, so this is a no-op unless tracking active jobs.
	return nil
}

// SqliteQueue implements SubAgentQueue using a local SQLite table.
type SqliteQueue struct {
	db db.Provider
	mu sync.Mutex
}

// NewSqliteQueue creates a new SqliteQueue.
func NewSqliteQueue(provider db.Provider) *SqliteQueue {
	return &SqliteQueue{db: provider}
}

// ensureTable creates the sub_agent_queue table if it doesn't exist.
func (q *SqliteQueue) ensureTable(ctx context.Context) error {
	q.mu.Lock()
	defer q.mu.Unlock()
	query := `
		CREATE TABLE IF NOT EXISTS sub_agent_queue (
			id TEXT PRIMARY KEY,
			parent_task_id TEXT NOT NULL,
			payload TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			scheduled_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			completed_at DATETIME
		)
	`
	_, err := q.db.Exec(ctx, query)
	return err
}

func (q *SqliteQueue) Enqueue(ctx context.Context, parentTaskID string, payload map[string]interface{}) (string, error) {
	if err := q.ensureTable(ctx); err != nil {
		return "", err
	}

	id := generateID()
	payloadBytes, err := json.Marshal(payload)
	if err != nil {
		return "", err
	}

	q.mu.Lock()
	defer q.mu.Unlock()

	query := `
		INSERT INTO sub_agent_queue (id, parent_task_id, payload, status)
		VALUES ($1, $2, $3, 'PENDING')
	`
	_, err = q.db.Exec(ctx, query, id, parentTaskID, string(payloadBytes))
	if err != nil {
		return "", err
	}
	return id, nil
}

func (q *SqliteQueue) Dequeue(ctx context.Context) (*SubAgentTask, error) {
	if err := q.ensureTable(ctx); err != nil {
		return nil, err
	}

	q.mu.Lock()
	defer q.mu.Unlock()

	var id, parentTaskID, payloadStr string
	query := `
		UPDATE sub_agent_queue
		SET status = 'PROCESSING'
		WHERE id IN (
			SELECT id FROM sub_agent_queue
			WHERE status = 'PENDING' AND scheduled_at <= CURRENT_TIMESTAMP
			ORDER BY scheduled_at ASC LIMIT 1
		)
		RETURNING id, parent_task_id, payload
	`

	err := q.db.QueryRow(ctx, query).Scan(&id, &parentTaskID, &payloadStr)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) || err.Error() == "sql: no rows in result set" {
			return nil, nil
		}
		return nil, fmt.Errorf("sqlite dequeue err: %w", err)
	}

	var payload map[string]interface{}
	if err := json.Unmarshal([]byte(payloadStr), &payload); err != nil {
		return nil, err
	}

	return &SubAgentTask{
		ID:           id,
		ParentTaskID: parentTaskID,
		Payload:      payload,
	}, nil
}

func (q *SqliteQueue) Complete(ctx context.Context, id string) error {
	q.mu.Lock()
	defer q.mu.Unlock()
	_, err := q.db.Exec(ctx, "UPDATE sub_agent_queue SET status = 'COMPLETED', completed_at = CURRENT_TIMESTAMP WHERE id = $1", id)
	return err
}

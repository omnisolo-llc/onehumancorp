package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"sync"
	"time"

	"github.com/redis/rueidis"
)

type SubAgentJob struct {
	ID           string
	ParentTaskID string
	Payload      json.RawMessage
	Status       string
	ScheduledAt  time.Time
	CompletedAt  *time.Time
}

type SubAgentQueue interface {
	Enqueue(ctx context.Context, job SubAgentJob) error
	Dequeue(ctx context.Context) (*SubAgentJob, error)
	Complete(ctx context.Context, jobID string) error
}

type PgRedisQueue struct {
	client rueidis.Client
	prefix string
}

func NewPgRedisQueue(client rueidis.Client) *PgRedisQueue {
	return &PgRedisQueue{
		client: client,
		prefix: "subagent:queue",
	}
}

func (q *PgRedisQueue) Enqueue(ctx context.Context, job SubAgentJob) error {
	data, err := json.Marshal(job)
	if err != nil {
		return err
	}

	cmd := q.client.B().Zadd().Key(q.prefix).ScoreMember().ScoreMember(float64(job.ScheduledAt.UnixNano()), string(data)).Build()
	err = q.client.Do(ctx, cmd).Error()
	if err != nil {
		return fmt.Errorf("failed to enqueue to redis: %w", err)
	}
	return nil
}

func (q *PgRedisQueue) Dequeue(ctx context.Context) (*SubAgentJob, error) {
	now := float64(time.Now().UnixNano())

	rangeCmd := q.client.B().Zrangebyscore().Key(q.prefix).Min("-inf").Max(fmt.Sprintf("%f", now)).Limit(0, 1).Build()
	resp := q.client.Do(ctx, rangeCmd)
	items, err := resp.AsStrSlice()
	if err != nil {
		return nil, err
	}

	if len(items) == 0 {
		return nil, nil // Nothing to dequeue
	}

	itemStr := items[0]

	// Try to remove it to claim the job
	remCmd := q.client.B().Zrem().Key(q.prefix).Member(itemStr).Build()

	remResp := q.client.Do(ctx, remCmd)
	if remResp.Error() != nil {
		return nil, remResp.Error()
	}

	count, err := remResp.AsInt64()
	if err != nil {
		return nil, err
	}

	if count == 0 {
		// Another worker claimed it via ZREM
		return nil, nil
	}

	var job SubAgentJob
	if err := json.Unmarshal([]byte(itemStr), &job); err != nil {
		return nil, err
	}

	return &job, nil
}

func (q *PgRedisQueue) Complete(ctx context.Context, jobID string) error {
	return nil
}

type SqliteQueue struct {
	db *sql.DB
	mu sync.Mutex
}

func NewSqliteQueue(db *sql.DB) *SqliteQueue {
	return &SqliteQueue{
		db: db,
	}
}

func (q *SqliteQueue) Enqueue(ctx context.Context, job SubAgentJob) error {
	q.mu.Lock()
	defer q.mu.Unlock()

	_, err := q.db.ExecContext(ctx, "INSERT INTO sub_agent_queue (id, parent_task_id, payload, status, scheduled_at) VALUES (?, ?, ?, 'QUEUED', ?)",
		job.ID, job.ParentTaskID, string(job.Payload), job.ScheduledAt)
	return err
}

func (q *SqliteQueue) Dequeue(ctx context.Context) (*SubAgentJob, error) {
	q.mu.Lock()
	defer q.mu.Unlock()

	tx, err := q.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	var job SubAgentJob
	var payloadStr string

	err = tx.QueryRowContext(ctx, "SELECT id, parent_task_id, payload, status, scheduled_at FROM sub_agent_queue WHERE status = 'QUEUED' AND scheduled_at <= ? ORDER BY scheduled_at ASC LIMIT 1", time.Now()).Scan(
		&job.ID, &job.ParentTaskID, &payloadStr, &job.Status, &job.ScheduledAt)

	if err == sql.ErrNoRows {
		return nil, nil
	} else if err != nil {
		return nil, err
	}

	job.Payload = json.RawMessage(payloadStr)

	_, err = tx.ExecContext(ctx, "UPDATE sub_agent_queue SET status = 'IN_PROGRESS' WHERE id = ?", job.ID)
	if err != nil {
		return nil, err
	}

	err = tx.Commit()
	if err != nil {
		return nil, err
	}

	return &job, nil
}

func (q *SqliteQueue) Complete(ctx context.Context, jobID string) error {
	q.mu.Lock()
	defer q.mu.Unlock()

	_, err := q.db.ExecContext(ctx, "UPDATE sub_agent_queue SET status = 'COMPLETED', completed_at = ? WHERE id = ?", time.Now(), jobID)
	return err
}

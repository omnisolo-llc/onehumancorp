package queue

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type SQLiteSubAgentTaskQueue struct {
	provider db.Provider
	opts     QueueOptions
}

type sqlitePayload struct {
	SubAgentTaskQueuePayload
	Retries int `json:"retries"`
}

func NewSQLiteSubAgentTaskQueue(provider db.Provider, opts QueueOptions) *SQLiteSubAgentTaskQueue {
	return &SQLiteSubAgentTaskQueue{provider: provider, opts: opts}
}

func (q *SQLiteSubAgentTaskQueue) Enqueue(ctx context.Context, payload *SubAgentTaskQueuePayload) error {
	sp := sqlitePayload{SubAgentTaskQueuePayload: *payload, Retries: 0}
	data, err := json.Marshal(sp)
	if err != nil {
		return err
	}
	query := "INSERT INTO sub_agent_tasks (job_id, queue_name, payload, status, created_at) VALUES ($1, $2, $3, 'QUEUED', $4)"
	_, err = q.provider.Exec(ctx, query, payload.JobID, payload.QueueName, string(data), time.Now())
	return err
}

func (q *SQLiteSubAgentTaskQueue) Process(ctx context.Context, queueName string) (*SubAgentTaskQueuePayload, error) {
	for {
		var query string
		var jobID, payloadStr string
		var err error
		if queueName == "" {
			query = "SELECT job_id, payload FROM sub_agent_tasks WHERE status = 'QUEUED' ORDER BY created_at ASC LIMIT 1"
			err = q.provider.QueryRow(ctx, query).Scan(&jobID, &payloadStr)
		} else {
			query = "SELECT job_id, payload FROM sub_agent_tasks WHERE status = 'QUEUED' AND queue_name = $1 ORDER BY created_at ASC LIMIT 1"
			err = q.provider.QueryRow(ctx, query, queueName).Scan(&jobID, &payloadStr)
		}

		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil
		} else if err != nil {
			return nil, err
		}

		updateQuery := "UPDATE sub_agent_tasks SET status = 'RUNNING' WHERE job_id = $1 AND status = 'QUEUED'"
		res, err := q.provider.Exec(ctx, updateQuery, jobID)
		if err != nil {
			return nil, err
		}
		if res == 0 {
			continue
		}

		var sp sqlitePayload
		if err := json.Unmarshal([]byte(payloadStr), &sp); err != nil {
			return nil, err
		}

		// Optional: We do not sleep for rate limit in SQLite to avoid blocking background threads indefinitely.
		return &sp.SubAgentTaskQueuePayload, nil
	}
}

func (q *SQLiteSubAgentTaskQueue) Complete(ctx context.Context, jobID string, queueName string) error {
	query := "UPDATE sub_agent_tasks SET status = 'COMPLETED' WHERE job_id = $1"
	_, err := q.provider.Exec(ctx, query, jobID)
	return err
}

func (q *SQLiteSubAgentTaskQueue) Fail(ctx context.Context, jobID string, queueName string, reason string) error {
	var payloadStr string
	err := q.provider.QueryRow(ctx, "SELECT payload FROM sub_agent_tasks WHERE job_id = $1", jobID).Scan(&payloadStr)
	if err != nil {
		return err
	}

	var sp sqlitePayload
	json.Unmarshal([]byte(payloadStr), &sp)

	sp.Retries++

	if sp.Retries <= q.opts.MaxRetries {
		newData, _ := json.Marshal(sp)
		query := "UPDATE sub_agent_tasks SET status = 'QUEUED', payload = $1 WHERE job_id = $2"
		_, err := q.provider.Exec(ctx, query, string(newData), jobID)
		return err
	}

	var status = "FAILED"
	if q.opts.DLQName != "" {
		status = "DLQ"
	}
	query := "UPDATE sub_agent_tasks SET status = $1 WHERE job_id = $2"
	_, err = q.provider.Exec(ctx, query, status, jobID)
	return err
}

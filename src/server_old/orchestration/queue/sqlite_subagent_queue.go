package queue

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
)

type SQLiteSubAgentTaskQueue struct {
	provider db.Provider
}

func NewSQLiteSubAgentTaskQueue(provider db.Provider) *SQLiteSubAgentTaskQueue {
	return &SQLiteSubAgentTaskQueue{provider: provider}
}

func (q *SQLiteSubAgentTaskQueue) Enqueue(ctx context.Context, payload *SubAgentTaskQueuePayload) error {
	data, err := json.Marshal(payload)
	if err != nil {
		return err
	}
	query := "INSERT INTO sub_agent_tasks (job_id, queue_name, payload, status, created_at) VALUES ($1, $2, $3, 'QUEUED', $4)"
	_, err = q.provider.Exec(ctx, query, payload.JobID, payload.QueueName, string(data), time.Now())
	return err
}

func (q *SQLiteSubAgentTaskQueue) Process(ctx context.Context, queueName string) (*SubAgentTaskQueuePayload, error) {
	for {
		query := `
			UPDATE sub_agent_tasks
			SET status = 'RUNNING'
			WHERE job_id = (
				SELECT job_id FROM sub_agent_tasks
				WHERE status = 'QUEUED' AND queue_name = $1
				ORDER BY created_at ASC
				LIMIT 1
			)
			RETURNING job_id, payload
		`
		var jobID, payloadStr string
		err := q.provider.QueryRow(ctx, query, queueName).Scan(&jobID, &payloadStr)
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil
		} else if err != nil {
			return nil, err
		}

		var payload SubAgentTaskQueuePayload
		if err := json.Unmarshal([]byte(payloadStr), &payload); err != nil {
			return nil, err
		}
		return &payload, nil
	}
}

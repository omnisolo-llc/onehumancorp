package db

import "context"

type TaskProvider interface {
	IsSQLite() bool
	CreateTask(ctx context.Context, task *Task) error
	ClaimTask(ctx context.Context, taskID string) error
	SearchSimilarMemoriesQuery(orgID string, query string, embeddingBytes []byte, topK int) (string, []interface{})
	AutoDreamInsertQuery() string
}

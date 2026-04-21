package autodream

import (
	"context"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type DBTraceQueue struct {
	dbProvider db.Provider
}

func NewDBTraceQueue(provider db.Provider) *DBTraceQueue {
	return &DBTraceQueue{
		dbProvider: provider,
	}
}

func (q *DBTraceQueue) FetchNextTrace(ctx context.Context) (string, string, map[string]any, error) {
	// First check shared_tasks
	queryShared := `
		SELECT id, title, payload
		FROM shared_tasks
		WHERE status IN ('DONE', 'COMPLETED')
		  AND id NOT IN (SELECT id FROM knowledge_embeddings)
		LIMIT 1
	`

	row := q.dbProvider.QueryRow(ctx, queryShared)
	var id, title, payload string
	err := row.Scan(&id, &title, &payload)

	if err == nil {
		content := fmt.Sprintf("Task: %s\nDetails: %s", title, payload)
		metadata := map[string]any{"source": "shared_tasks", "status": "DONE", "title": title}
		return id, content, metadata, nil
	}

	// Then check swarm_tasks
	querySwarm := `
		SELECT id, title, payload
		FROM swarm_tasks
		WHERE status IN ('DONE', 'COMPLETED')
		  AND id NOT IN (SELECT id FROM knowledge_embeddings)
		LIMIT 1
	`

	row = q.dbProvider.QueryRow(ctx, querySwarm)
	err = row.Scan(&id, &title, &payload)

	if err == nil {
		content := fmt.Sprintf("Task: %s\nDetails: %s", title, payload)
		metadata := map[string]any{"source": "swarm_tasks", "status": "DONE", "title": title}
		return id, content, metadata, nil
	}

	return "", "", nil, fmt.Errorf("no unextracted finalized tasks found")
}

func (q *DBTraceQueue) MarkTraceComplete(ctx context.Context, id string) error {
	// Our strategy uses NOT IN (SELECT id FROM knowledge_embeddings)
	// Therefore, as soon as ProcessTrace finishes and stores it, it's marked "complete"
	// and won't be fetched again. We don't need a separate column/flag.
	return nil
}

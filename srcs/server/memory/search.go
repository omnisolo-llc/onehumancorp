package memory

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"onehumancorp/srcs/server/db"
)

type Memory struct {
	ID             string
	OrganizationID string
	TaskID         string
	Content        string
	Embedding      []float32
}

func (d *AutoDreamDaemon) SearchSimilarMemories(ctx context.Context, query string, topK int, orgID string) ([]Memory, error) {
	// Generate query embedding
	queryEmbedding, err := d.llmClient.GenerateEmbedding(ctx, query)
	if err != nil {
		return nil, fmt.Errorf("failed to generate query embedding: %w", err)
	}

	embeddingBytes, err := json.Marshal(queryEmbedding)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal query embedding: %w", err)
	}

	var queryStr string
	var args []interface{}

	if db.GlobalProvider.IsSQLite() {
		// Fallback to text-based recency logic in SQLite Standalone mode
		queryStr = `
			SELECT id, organization_id, task_id, content
			FROM autodream_memories
			WHERE organization_id = ? AND content LIKE ?
			ORDER BY created_at DESC
			LIMIT ?
		`
		args = []interface{}{orgID, "%" + query + "%", topK}
	} else {
		// Exact Nearest Neighbor search in Postgres Cloud mode
		queryStr = `
			SELECT id, organization_id, task_id, content
			FROM autodream_memories
			WHERE organization_id = $1
			ORDER BY embedding <-> $2
			LIMIT $3
		`
		args = []interface{}{orgID, string(embeddingBytes), topK}
	}

	rows, err := d.db.QueryContext(ctx, queryStr, args...)
	if err != nil {
		return nil, fmt.Errorf("failed to execute search query: %w", err)
	}
	defer rows.Close()

	var memories []Memory
	for rows.Next() {
		var mem Memory
		var taskID sql.NullString
		if err := rows.Scan(&mem.ID, &mem.OrganizationID, &taskID, &mem.Content); err != nil {
			return nil, fmt.Errorf("failed to scan memory row: %w", err)
		}
		if taskID.Valid {
			mem.TaskID = taskID.String
		}
		memories = append(memories, mem)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("error iterating memory rows: %w", err)
	}

	return memories, nil
}

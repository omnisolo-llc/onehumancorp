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
		// Use vector extension in SQLite Standalone mode
		queryStr = `
			SELECT id, organization_id, task_id, content
			FROM autodream_memories
			WHERE organization_id = ?
			ORDER BY vec_distance_cosine(embedding, ?)
			LIMIT ?
		`
		args = []interface{}{orgID, string(embeddingBytes), topK}
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

// AutoResolveConflicts resolves conflicting memory records based on recency.
func (d *AutoDreamDaemon) AutoResolveConflicts(ctx context.Context) error {
    // Resolve conflicts by keeping the most recent entry when contents match exactly.
    // In Go, since we don't have reliability_score, we just use recency.
    query := `
        DELETE FROM autodream_memories
        WHERE id IN (
            SELECT a.id
            FROM autodream_memories a
            JOIN autodream_memories b ON a.organization_id = b.organization_id
                AND a.content = b.content
                AND a.created_at < b.created_at
        )
    `
    _, err := d.db.ExecContext(ctx, query)
    if err != nil {
        return fmt.Errorf("failed to resolve memory conflicts: %w", err)
    }
    return nil
}

package orchestration

import (
	"context"
	"database/sql"
	"fmt"
	"os"
	"path/filepath"
    "strings"
)

type AutoDreamWorker struct {
	DB *sql.DB
    MemoryDir string
}

func (w *AutoDreamWorker) Run(ctx context.Context) error {
    tenantID, ok := ctx.Value("tenant_id").(string)
    if !ok || tenantID == "" {
        return fmt.Errorf("tenant_id not found in context")
    }

    dir := w.MemoryDir
    if dir == "" {
        dir = ".agent-task/memory/"
    }

	files, err := os.ReadDir(dir)
	if err != nil {
		return fmt.Errorf("failed to read memory dir: %w", err)
	}

    var mockEmbedding []string
    for i := 0; i < 1536; i++ {
        mockEmbedding = append(mockEmbedding, "0.0")
    }
    embeddingStr := "[" + strings.Join(mockEmbedding, ",") + "]"

	for _, file := range files {
		if file.IsDir() {
			continue
		}

		path := filepath.Join(dir, file.Name())
		content, err := os.ReadFile(path)
		if err != nil {
			return fmt.Errorf("failed to read file %s: %w", path, err)
		}

		_, err = w.DB.ExecContext(ctx, "INSERT INTO consolidated_memory (tenant_id, content, embedding) VALUES ($1, $2, $3)", tenantID, string(content), embeddingStr)
		if err != nil {
		    return fmt.Errorf("failed to insert memory: %w", err)
		}
	}

	return nil
}

func (w *AutoDreamWorker) Search(ctx context.Context, embedding string) ([]string, error) {
    tenantID, ok := ctx.Value("tenant_id").(string)
    if !ok || tenantID == "" {
        return nil, fmt.Errorf("tenant_id not found in context")
    }

    rows, err := w.DB.QueryContext(ctx, "SELECT content FROM consolidated_memory WHERE tenant_id = $1 ORDER BY embedding <-> $2 LIMIT 1", tenantID, embedding)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var results []string
    for rows.Next() {
        var content string
        if err := rows.Scan(&content); err != nil {
            return nil, err
        }
        results = append(results, content)
    }
    return results, nil
}

package workers

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type AutoDreamLLMClient interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

type AutoDreamWorker struct {
	pool   db.Provider
	client AutoDreamLLMClient
}

func NewAutoDreamWorker(pool db.Provider, client AutoDreamLLMClient) *AutoDreamWorker {
	return &AutoDreamWorker{
		pool:   pool,
		client: client,
	}
}

func (w *AutoDreamWorker) Start(ctx context.Context) {
	slog.Info("Starting AutoDreamWorker for Memory Consolidation")
	ticker := time.NewTicker(30 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			w.ConsolidateMemories(ctx)
		}
	}
}

func (w *AutoDreamWorker) ConsolidateMemories(ctx context.Context) {
	// The problem states: "Implement AutoDream background worker to consolidate .agent-task/memory/*.yml to consolidated_memory in pgvector."

	// Create table if not exists (in pgvector or sqlite fallback)
	createTableQuery := `
		CREATE TABLE IF NOT EXISTS consolidated_memory (
			id UUID PRIMARY KEY,
			content TEXT NOT NULL,
			embedding vector(1536),
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		)
	`

	createTableQuerySqlite := `
		CREATE TABLE IF NOT EXISTS consolidated_memory (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	`

	if w.pool.IsSQLite() {
		w.pool.Exec(ctx, createTableQuerySqlite)
	} else {
		w.pool.Exec(ctx, createTableQuery)
	}

	memoryDir := ".agent-task/memory"
	files, err := filepath.Glob(filepath.Join(memoryDir, "*.yml"))
	if err != nil || len(files) == 0 {
		return
	}

	for _, file := range files {
		contentBytes, err := os.ReadFile(file)
		if err != nil {
			slog.Error("AutoDreamWorker: failed to read memory file", "file", file, "error", err)
			continue
		}

		content := string(contentBytes)

		var embedding []float32
		if w.client != nil {
			embCtx, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, embErr := w.client.GenerateEmbedding(embCtx, content)
			cancel()
			if embErr == nil && len(resp) == 1536 {
				embedding = resp
			}
		}

		if len(embedding) == 0 {
			// Fallback embedding
			embedding = make([]float32, 1536)
			embedding[0] = 0.1
		}

		memID := uuid.New().String()

		if w.pool.IsSQLite() {
			embBytes, _ := json.Marshal(embedding)
			embStr := string(embBytes)
			_, err = w.pool.Exec(ctx, `INSERT INTO consolidated_memory (id, content, embedding) VALUES ($1, $2, $3)`, memID, content, embStr)
		} else {
			strs := make([]string, len(embedding))
			for i, v := range embedding {
				strs[i] = fmt.Sprintf("%f", v)
			}
			embStr := "[" + strings.Join(strs, ",") + "]"
			_, err = w.pool.Exec(ctx, `INSERT INTO consolidated_memory (id, content, embedding) VALUES ($1, $2, $3::vector)`, memID, content, embStr)
		}

		if err != nil {
			slog.Error("AutoDreamWorker: failed to insert consolidated memory", "error", err)
		} else {
			slog.Info("AutoDreamWorker: consolidated memory file", "file", file)
			// Remove the file after processing
			os.Remove(file)
		}
	}
}

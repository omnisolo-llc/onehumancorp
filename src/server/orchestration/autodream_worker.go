package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/src/server/db"
)

type WorkerLLMClient interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

type AutoDreamWorker2 struct {
	db  db.Provider
	llm WorkerLLMClient
}

func NewAutoDreamWorkerPipeline(dbProvider db.Provider, llm WorkerLLMClient) *AutoDreamWorker2 {
	return &AutoDreamWorker2{
		db:  dbProvider,
		llm: llm,
	}
}

func (w *AutoDreamWorker2) Run(ctx context.Context) error {
	slog.Info("AutoDreamWorker: starting memory consolidation")

	memoryDir := os.Getenv("AGENT_TASK_MEMORY_DIR")
	if memoryDir == "" {
		memoryDir = ".agent-task/memory/"
	}

	if _, err := os.Stat(memoryDir); os.IsNotExist(err) {
		slog.Warn("AutoDreamWorker: memory directory does not exist", "dir", memoryDir)
		return nil
	}

	files, err := os.ReadDir(memoryDir)
	if err != nil {
		return fmt.Errorf("failed to read memory directory: %w", err)
	}

	for _, file := range files {
		if file.IsDir() {
			continue
		}

		filePath := filepath.Join(memoryDir, file.Name())
		content, err := os.ReadFile(filePath)
		if err != nil {
			slog.Warn("AutoDreamWorker: failed to read fs memory file", "file", filePath, "error", err)
			continue
		}

		if err := w.embedAndStore(ctx, "system", "autodream_worker", "fs_runtime", string(content)); err != nil {
			slog.Warn("AutoDreamWorker: failed to embed fs memory", "file", filePath, "error", err)
			continue
		}

		// Delete after successful processing
		if err := os.Remove(filePath); err != nil {
			slog.Warn("AutoDreamWorker: failed to delete fs memory file", "file", filePath, "error", err)
		}
	}

	slog.Info("AutoDreamWorker: memory consolidation completed")
	return nil
}

func (w *AutoDreamWorker2) embedAndStore(ctx context.Context, orgID, agentID, sourceType, content string) error {
	// Generate Embedding
	var embedding []float32
	var err error
	if w.llm != nil {
		embedding, err = w.llm.GenerateEmbedding(ctx, content)
		if err != nil {
			return fmt.Errorf("failed to generate embedding: %w", err)
		}
	} else {
		// fallback
		embedding = make([]float32, 1536)
		embedding[0] = 0.5
	}

	memID := uuid.New().String()
	var query string

	if w.db.IsSQLite() {
		query = `INSERT INTO consolidated_memory (id, organization_id, tenant_id, agent_id, source_type, content, embedding) VALUES (?, ?, ?, ?, ?, ?, ?)`
		embStr := fmt.Sprintf("%v", embedding)
		tenantID := "default"
		_, err = w.db.Exec(ctx, query, memID, orgID, tenantID, agentID, sourceType, content, embStr)
	} else {
		query = `INSERT INTO consolidated_memory (id, organization_id, tenant_id, agent_id, source_type, content, embedding) VALUES ($1, $2, $3, $4, $5, $6, $7::vector)`

		embBytes, _ := json.Marshal(embedding)
		// pgvector string format: [1,2,3]
		embStr := string(embBytes)
		tenantID := "default"
		_, err = w.db.Exec(ctx, query, memID, orgID, tenantID, agentID, sourceType, content, embStr)
	}

	if err != nil {
		return fmt.Errorf("failed to insert memory embedding: %w", err)
	}

	return nil
}

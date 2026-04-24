package autodream

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type WorkerLLMClient interface {
	Reason(ctx context.Context, prompt string) (string, error)
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

type AutoDreamWorker struct {
	db  db.Provider
	llm WorkerLLMClient
}

func NewAutoDreamWorker(dbProvider db.Provider, llm WorkerLLMClient) *AutoDreamWorker {
	return &AutoDreamWorker{
		db:  dbProvider,
		llm: llm,
	}
}

func (w *AutoDreamWorker) RunConsolidation(ctx context.Context) error {
	slog.Info("AutoDreamWorker: starting memory consolidation")

	// 1. Process from agent_session_data
	if err := w.processDBMemories(ctx); err != nil {
		slog.Error("AutoDreamWorker: error processing db memories", "error", err)
	}

	// 2. Process from OHC_MEMORY_DIR/*.yml
	if err := w.processFSMemories(ctx); err != nil {
		slog.Error("AutoDreamWorker: error processing fs memories", "error", err)
	}

	slog.Info("AutoDreamWorker: memory consolidation completed")
	return nil
}

func (w *AutoDreamWorker) processDBMemories(ctx context.Context) error {
	start := time.Now()
	mode := os.Getenv("OHC_SOURCE_MODE")
	if mode == "" {
		mode = "standalone"
	}


	query := "SELECT session_id, agent_id, context_data FROM agent_session_data ORDER BY last_accessed ASC LIMIT 100"
	if !w.db.IsSQLite() {
		query += " FOR UPDATE SKIP LOCKED"
	}

	rows, err := w.db.Query(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to query agent_session_data: %w", err)
	}
	defer rows.Close()

	type sessionData struct {
		sessionID string
		agentID   string
		context   string
	}

	var sessions []sessionData
	for rows.Next() {
		var s sessionData
		if err := rows.Scan(&s.sessionID, &s.agentID, &s.context); err != nil {
			return fmt.Errorf("failed to scan session data: %w", err)
		}
		sessions = append(sessions, s)
	}
	rows.Close() // Close early before processing

	// Record queue depth and latency

	for _, s := range sessions {
		if err := w.embedAndStore(ctx, "system", s.agentID, "agent_session", s.context); err != nil {
			slog.Warn("AutoDreamWorker: failed to embed session", "session_id", s.sessionID, "error", err)
			continue
		}

		// Delete from agent_session_data after successful processing
		delQuery := "DELETE FROM agent_session_data WHERE session_id = $1"
		if w.db.IsSQLite() {
			delQuery = "DELETE FROM agent_session_data WHERE session_id = ?"
		}
		if _, err := w.db.Exec(ctx, delQuery, s.sessionID); err != nil {
			slog.Warn("AutoDreamWorker: failed to delete processed session", "session_id", s.sessionID, "error", err)
		}
	}

	// Approximate queue depth by the batch size if it's 100, otherwise it's just the batch length.
	// This avoids expensive COUNT(*). We just report the batch size as the queue depth sample.

	if depth, err := w.getQueueDepth(ctx); err == nil {
		telemetry.RecordAutoDreamQueueDepth(ctx, depth, mode)
	}
	telemetry.RecordAutoDreamJobLatency(ctx, time.Since(start).Seconds(), mode)

	return nil
}

func (w *AutoDreamWorker) processFSMemories(ctx context.Context) error {
	memoryDir := os.Getenv("OHC_MEMORY_DIR")
	if memoryDir == "" {
		memoryDir = ".ohc/runtime/memory"
	}

	if _, err := os.Stat(memoryDir); os.IsNotExist(err) {
		return nil
	}

	files, err := os.ReadDir(memoryDir)
	if err != nil {
		return fmt.Errorf("failed to read memory directory: %w", err)
	}

	for _, file := range files {
		if file.IsDir() || filepath.Ext(file.Name()) != ".yml" {
			continue
		}

		filePath := filepath.Join(memoryDir, file.Name())
		content, err := os.ReadFile(filePath)
		if err != nil {
			slog.Warn("AutoDreamWorker: failed to read fs memory file", "file", filePath, "error", err)
			continue
		}

		if err := w.embedAndStore(ctx, "system", "fs-agent", "fs_runtime", string(content)); err != nil {
			slog.Warn("AutoDreamWorker: failed to embed fs memory", "file", filePath, "error", err)
			continue
		}

		// Delete after successful processing
		if err := os.Remove(filePath); err != nil {
			slog.Warn("AutoDreamWorker: failed to delete fs memory file", "file", filePath, "error", err)
		}
	}

	return nil
}

func (w *AutoDreamWorker) embedAndStore(ctx context.Context, orgID, agentID, memType, content string) error {
	// Generate Embedding
	embedding, err := w.llm.GenerateEmbedding(ctx, content)
	if err != nil {
		return fmt.Errorf("failed to generate embedding: %w", err)
	}



	query := `INSERT INTO agent_memory_embeddings (organization_id, agent_id, memory_type, content, embedding) VALUES ($1, $2, $3, $4, $5)`

	if w.db.IsSQLite() {
		query = `INSERT INTO agent_memory_embeddings (organization_id, agent_id, memory_type, content, embedding) VALUES (?, ?, ?, ?, ?)`
		embStr := fmt.Sprintf("%v", embedding)
		_, err = w.db.Exec(ctx, query, orgID, agentID, memType, content, embStr)
	} else {
		embBytes, _ := json.Marshal(embedding)
		// pgvector string format: [1,2,3]
		embStr := string(embBytes)
		_, err = w.db.Exec(ctx, query, orgID, agentID, memType, content, embStr)
	}

	if err != nil {
		return fmt.Errorf("failed to insert memory embedding: %w", err)
	}

	return nil
}


func (w *AutoDreamWorker) getQueueDepth(ctx context.Context) (int, error) {
	if w.db.IsSQLite() {
		var count int
		err := w.db.QueryRow(ctx, "SELECT COUNT(*) FROM agent_session_data").Scan(&count)
		return count, err
	}
	var count int
	err := w.db.QueryRow(ctx, "SELECT reltuples::bigint FROM pg_class WHERE relname = 'agent_session_data'").Scan(&count)
	if err != nil {
		return 0, err
	}
	return count, nil
}

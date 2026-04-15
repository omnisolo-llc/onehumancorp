package autodream

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type VectorRepository interface {
	Insert(ctx context.Context, mem *Memory) error
}

type AutoDreamPipeline struct {
	dbProvider db.Provider
	llm        LLMClient
	vectorRepo VectorRepository
	memoryDir  string
}

func NewAutoDreamPipeline(provider db.Provider, llm LLMClient, repo VectorRepository, memDir string) *AutoDreamPipeline {
	if memDir == "" {
		memDir = ".agent-task/memory"
	}
	return &AutoDreamPipeline{
		dbProvider: provider,
		llm:        llm,
		vectorRepo: repo,
		memoryDir:  memDir,
	}
}

func (p *AutoDreamPipeline) RunConsolidationCycle() error {
	ctx := context.Background()

	// 1. Extract from DB (agent_session_data)
	var dbChunks []string
	var sessionIDs []string
	rows, err := p.dbProvider.Query(ctx, `SELECT session_id, context_data FROM agent_session_data`)
	if err == nil {
		for rows.Next() {
			var sessionID, data string
			if err := rows.Scan(&sessionID, &data); err == nil {
				dbChunks = append(dbChunks, fmt.Sprintf("Session %s: %s", sessionID, data))
				sessionIDs = append(sessionIDs, sessionID)
			}
		}
		rows.Close() // Close early before network calls
	}

	// 2. Extract from filesystem directory
	var fileChunks []string
	var processedFiles []string
	files, err := filepath.Glob(filepath.Join(p.memoryDir, "*.yml"))
	if err == nil {
		for _, f := range files {
			data, err := os.ReadFile(f)
			if err == nil {
				fileChunks = append(fileChunks, fmt.Sprintf("File %s:\n%s", filepath.Base(f), string(data)))
				processedFiles = append(processedFiles, f)
			}
		}
	}

	if len(dbChunks) == 0 && len(fileChunks) == 0 {
		return nil
	}

	// Process DB chunks
	for i, chunk := range dbChunks {
		err := p.processChunk(ctx, chunk, fmt.Sprintf("db-%d", i))
		if err != nil {
			return fmt.Errorf("failed to process db chunk %d: %w", i, err)
		}
		// Cleanup immediately after successful processing
		_, _ = p.dbProvider.Exec(ctx, `DELETE FROM agent_session_data WHERE session_id = $1`, sessionIDs[i])
	}

	// Process File chunks
	for i, chunk := range fileChunks {
		err := p.processChunk(ctx, chunk, fmt.Sprintf("file-%d", i))
		if err != nil {
			return fmt.Errorf("failed to process file chunk %d: %w", i, err)
		}
		// Cleanup immediately after successful processing
		_ = os.Remove(processedFiles[i])
	}

	return nil
}

func (p *AutoDreamPipeline) processChunk(ctx context.Context, chunk string, sourceID string) error {
	prompt := fmt.Sprintf("Summarize the key technical decisions, user preferences, and permanent facts from these logs:\n%s", chunk)
	summary, err := p.llm.Reason(ctx, prompt)
	if err != nil {
		return err
	}

	embedding, err := p.llm.GenerateEmbedding(ctx, summary)
	if err != nil {
		return err
	}

	mem := &Memory{
		ID:        fmt.Sprintf("autodream-%d-%s", time.Now().UnixNano(), sourceID),
		TaskID:    "autodream-consolidation",
		Content:   summary,
		Embedding: embedding,
		CreatedAt: time.Now().Format(time.RFC3339),
	}

	return p.vectorRepo.Insert(ctx, mem)
}

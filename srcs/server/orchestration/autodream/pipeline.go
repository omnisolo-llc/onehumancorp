package autodream

import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"time"

	"gopkg.in/yaml.v3"

	"github.com/onehumancorp/mono/srcs/server/agents/local"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// VectorRepository defines the interface for persisting memory embeddings.
type VectorRepository interface {
	Insert(ctx context.Context, mem *Memory) error
}

// AutoDreamPipeline is the background worker for memory consolidation.
type AutoDreamPipeline struct {
	pool       db.Provider
	llmClient  local.LLMClient
	vectorRepo VectorRepository
}

// NewAutoDreamPipeline creates a new pipeline instance.
func NewAutoDreamPipeline(pool db.Provider, llmClient local.LLMClient, vectorRepo VectorRepository) *AutoDreamPipeline {
	return &AutoDreamPipeline{
		pool:       pool,
		llmClient:  llmClient,
		vectorRepo: vectorRepo,
	}
}

// RunConsolidationCycle coordinates extraction, compression, and storage of context.
func (p *AutoDreamPipeline) RunConsolidationCycle(ctx context.Context) {
	slog.Info("AutoDreamPipeline: starting consolidation cycle")

	p.processDatabaseSessions(ctx)
	p.processFilesystemMemories(ctx)

	slog.Info("AutoDreamPipeline: consolidation cycle completed")
}

func (p *AutoDreamPipeline) processDatabaseSessions(ctx context.Context) {
	threshold := time.Now().Add(-1 * time.Hour).UTC()
	var query string
	if p.pool.IsSQLite() {
		query = "SELECT session_id, agent_id, context_data FROM agent_session_data WHERE last_accessed < ? LIMIT 50"
	} else {
		// Postgres mode: use SKIP LOCKED to prevent concurrent processing by other pods
		query = "SELECT session_id, agent_id, context_data FROM agent_session_data WHERE last_accessed < $1 LIMIT 50 FOR UPDATE SKIP LOCKED"
	}

	// We only hold the transaction long enough to fetch the data to process.
	// In Postgres, we should ideally lock row-by-row, but for simplicity, we
	// fetch and process. A better way is to lock, update status, then commit.
	// Since we can't easily change the schema to add a "processing" state, we'll
	// fetch IDs and attempt to delete them safely.

	rows, err := p.pool.Query(ctx, query, threshold)
	if err != nil {
		slog.Error("AutoDreamPipeline: failed to fetch stale sessions", "error", err)
		return
	}

	type Session struct {
		ID          string
		AgentID     string
		ContextData string
	}

	var sessions []Session
	for rows.Next() {
		var s Session
		if err := rows.Scan(&s.ID, &s.AgentID, &s.ContextData); err == nil {
			sessions = append(sessions, s)
		}
	}
	rows.Close()

	for _, s := range sessions {
		// Acquire an isolated transaction to lock this specific row
		err := func() error {
			tx, err := p.pool.Begin(ctx)
			if err != nil {
				return err
			}
			defer tx.Rollback(ctx)

			var checkQuery string
			var exists bool
			if p.pool.IsSQLite() {
				checkQuery = "SELECT 1 FROM agent_session_data WHERE session_id = ?"
			} else {
				checkQuery = "SELECT 1 FROM agent_session_data WHERE session_id = $1 FOR UPDATE SKIP LOCKED"
			}
			err = tx.QueryRow(ctx, checkQuery, s.ID).Scan(&exists)
			if err != nil {
				return err // Not found or locked by someone else
			}

			// We have the lock, so we can now release the transaction lock safely,
			// But wait, we can't release it and then try to delete it later because someone else might grab it.
			// Let's hold the transaction for the LLM call but just for ONE item, limiting lock scope.

			prompt := fmt.Sprintf("Summarize and consolidate this agent session memory:\n%s", s.ContextData)
			req := local.CompletionRequest{
				SystemPrompt: "You are an AI Memory Consolidator.",
				Messages: []local.ConversationMessage{
					{Role: "user", Content: []local.ContentPart{{Type: "text", Text: prompt}}},
				},
				MaxTokens:    500,
			}

			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, llmErr := p.llmClient.Complete(ctxTimeout, req)
			cancel()

			if llmErr != nil || resp == nil || resp.Text == "" {
				slog.Warn("AutoDreamPipeline: LLM summarization failed, aborting processing for session", "id", s.ID, "error", llmErr)
				return fmt.Errorf("llm error: %w", llmErr)
			}

			summary := resp.Text
			var vec []float32
			for i := 0; i < 1536; i++ {
				vec = append(vec, 0.0)
			}

			mem := &Memory{
				ID:        fmt.Sprintf("db-%s-%d", s.ID, time.Now().UnixNano()),
				TaskID:    s.ID,
				Content:   summary,
				Embedding: vec,
				CreatedAt: time.Now().Format(time.RFC3339),
			}

			if err := p.vectorRepo.Insert(ctx, mem); err != nil {
				slog.Error("AutoDreamPipeline: failed to insert consolidated memory", "error", err)
				return err
			}

			telemetry.RecordAutoDreamMemoryCompressed(ctx, s.AgentID)

			var delQuery string
			if p.pool.IsSQLite() {
				delQuery = "DELETE FROM agent_session_data WHERE session_id = ?"
			} else {
				delQuery = "DELETE FROM agent_session_data WHERE session_id = $1"
			}
			if _, err := tx.Exec(ctx, delQuery, s.ID); err != nil {
				return err
			}

			return tx.Commit(ctx)
		}()

		if err != nil {
			slog.Debug("AutoDreamPipeline: skipped or failed processing session", "id", s.ID, "error", err)
		}
	}
}

func (p *AutoDreamPipeline) processFilesystemMemories(ctx context.Context) {
	memoryDir := os.Getenv("OHC_MEMORY_DIR")
	if memoryDir == "" {
		memoryDir = ".agent-task/memory"
	}

	files, err := os.ReadDir(memoryDir)
	if err != nil {
		if !os.IsNotExist(err) {
			slog.Error("AutoDreamPipeline: failed to read memory directory", "error", err)
		}
		return
	}

	for _, file := range files {
		// Include .processing files in case of crash recovery
		isProcessing := filepath.Ext(file.Name()) == ".processing"
		isYaml := filepath.Ext(file.Name()) == ".yml" || filepath.Ext(file.Name()) == ".yaml"

		if file.IsDir() || (!isYaml && !isProcessing) {
			continue
		}

		filePath := filepath.Join(memoryDir, file.Name())
		var processingPath string

		if isProcessing {
			processingPath = filePath
			filePath = strings.TrimSuffix(filePath, ".processing")
		} else {
			processingPath = filePath + ".processing"
			// Use atomic rename to claim the file
			if err := os.Rename(filePath, processingPath); err != nil {
				continue
			}
		}

		contentBytes, err := os.ReadFile(processingPath)
		if err != nil {
			slog.Error("AutoDreamPipeline: failed to read memory file", "file", processingPath, "error", err)
			os.Rename(processingPath, filePath) // rollback
			continue
		}

		var memFile struct {
			Content string `yaml:"content"`
			AgentID string `yaml:"agent_id"`
			TaskID  string `yaml:"task_id"`
		}

		if err := yaml.Unmarshal(contentBytes, &memFile); err != nil {
			slog.Error("AutoDreamPipeline: failed to parse memory file", "file", processingPath, "error", err)
			os.Rename(processingPath, filePath)
			continue
		}

		contentToEmbed := memFile.Content
		if contentToEmbed == "" {
			contentToEmbed = string(contentBytes)
		}

		prompt := fmt.Sprintf("Summarize and consolidate this file memory:\n%s", contentToEmbed)
		req := local.CompletionRequest{
			SystemPrompt: "You are an AI Memory Consolidator.",
			Messages: []local.ConversationMessage{
				{Role: "user", Content: []local.ContentPart{{Type: "text", Text: prompt}}},
			},
			MaxTokens: 500,
		}

		ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
		resp, err := p.llmClient.Complete(ctxTimeout, req)
		cancel()

		if err != nil || resp == nil || resp.Text == "" {
			slog.Warn("AutoDreamPipeline: LLM summarization failed, aborting processing for file", "file", file.Name(), "error", err)
			os.Rename(processingPath, filePath) // rollback
			continue
		}

		summary := resp.Text

		var vec []float32
		for i := 0; i < 1536; i++ {
			vec = append(vec, 0.0)
		}

		taskID := memFile.TaskID
		if taskID == "" {
			taskID = strings.TrimSuffix(strings.TrimSuffix(file.Name(), ".processing"), filepath.Ext(file.Name()))
		}

		mem := &Memory{
			ID:        fmt.Sprintf("file-%s-%d", taskID, time.Now().UnixNano()),
			TaskID:    taskID,
			Content:   summary,
			Embedding: vec,
			CreatedAt: time.Now().Format(time.RFC3339),
		}

		if err := p.vectorRepo.Insert(ctx, mem); err != nil {
			slog.Error("AutoDreamPipeline: failed to insert consolidated file memory", "error", err)
			os.Rename(processingPath, filePath) // rollback
			continue
		}

		agentID := memFile.AgentID
		if agentID == "" {
			agentID = "system"
		}
		telemetry.RecordAutoDreamMemoryIngested(ctx, agentID)

		os.Remove(processingPath)
	}
}

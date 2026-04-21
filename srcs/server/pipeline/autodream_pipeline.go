package pipeline

import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/agents/builtin"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"github.com/redis/rueidis"
)

// AutoDreamPipeline orchestrates the background processing of agent memories.
// It extracts raw memory, consolidates it, embeds it via LLM clients, and loads it into pgvector.
type AutoDreamPipeline struct {
	pool          db.Provider
	worker        *orchestration.AutoDreamWorker
	llm           builtin.LLMClient
	minimaxClient orchestration.MinimaxClient
}

// NewAutoDreamPipeline creates a new AutoDreamPipeline instance.
func NewAutoDreamPipeline(pool db.Provider, redisClient rueidis.Client) *AutoDreamPipeline {
	worker := orchestration.NewAutoDreamWorker(pool)

	// Determine LLM client based on env vars
	var llmClient builtin.LLMClient
	if key := os.Getenv("ANTHROPIC_API_KEY"); key != "" {
		llmClient = builtin.NewAnthropicClient(key)
	} else if key := os.Getenv("OPENAI_API_KEY"); key != "" {
		llmClient = builtin.NewOpenAIClient(key)
	} else {
		llmClient = builtin.NewOllamaClient("")
	}


	var mClient orchestration.MinimaxClient
	if minimaxKey := os.Getenv("MINIMAX_API_KEY"); minimaxKey != "" {
		mClient = orchestration.NewCachedMinimaxClient(orchestration.NewMinimaxClient(minimaxKey), pool, redisClient)
	}

	return &AutoDreamPipeline{
		pool:          pool,
		worker:        worker,
		llm:           llmClient,
		minimaxClient: mClient,
	}
}

// Start runs the pipeline in the background on a ticker.
func (p *AutoDreamPipeline) Start(ctx context.Context) {
	ticker := time.NewTicker(2 * time.Minute)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			p.processBatch(ctx)
			p.processFiles(ctx)
			p.resolveConflicts(ctx)
		}
	}
}

// resolveConflicts finds vector embeddings that are similar but have conflicting contexts,
// similar to orchestration.AutoDreamWorker but targeting consolidated_memory.
func (p *AutoDreamPipeline) resolveConflicts(ctx context.Context) {
	if p.pool.IsSQLite() {
		// SQLite standalone fallback doesn't support pgvector
		return
	}

	query := `
		SELECT a.id, a.content, b.id, b.content
		FROM consolidated_memory a
		JOIN consolidated_memory b ON a.id < b.id
		WHERE a.embedding <=> b.embedding < 0.05
		LIMIT 10
	`

	rows, err := p.pool.Query(ctx, query)
	if err != nil {
		slog.Error("AutoDreamPipeline: failed to query embeddings with pgvector", "error", err)
		return
	}
	defer rows.Close()

	type Conflict struct {
		ID1      string
		Content1 string
		ID2      string
		Content2 string
	}

	var conflicts []Conflict
	for rows.Next() {
		var c Conflict
		if err := rows.Scan(&c.ID1, &c.Content1, &c.ID2, &c.Content2); err != nil {
			continue
		}
		conflicts = append(conflicts, c)
	}

	for _, c := range conflicts {
		conflictID := fmt.Sprintf("conflict-%s-%s", c.ID1, c.ID2)

		insertQuery := "INSERT INTO memory_conflicts (conflict_id, memory_id_1, memory_id_2, resolution_status) VALUES ($1, $2, $3, 'PENDING') ON CONFLICT DO NOTHING"
		_, err := p.pool.Exec(ctx, insertQuery, conflictID, c.ID1, c.ID2)
		if err != nil {
			continue
		}

		slog.Info("AutoDreamPipeline: detected memory conflict via pgvector", "id1", c.ID1, "id2", c.ID2)

		prompt := fmt.Sprintf("You are an AI Memory Consolidator. Resolve these two conflicting memories into a single truth.\nMemory 1: %s\nMemory 2: %s", c.Content1, c.Content2)
		req := builtin.ChatRequest{
			System: "You are an AI Memory Consolidator.",
			Messages: []builtin.Message{
				{Role: builtin.RoleUser, Content: prompt},
			},
			MaxTokens: 500,
		}

		ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
		resp, llmErr := p.llm.Chat(ctxTimeout, req)
		cancel()

		resolvedContext := "Consolidated memory: " + c.Content1 + " & " + c.Content2
		if llmErr == nil && resp.Message.Content != "" && resp.Message.Content != "" {
			resolvedContext = resp.Message.Content
		}

		resolvedID := fmt.Sprintf("resolved-%s", conflictID)

		_ = func() error {
			tx, txErr := p.pool.Begin(ctx)
			if txErr != nil {
				return txErr
			}
			defer tx.Rollback(ctx)

			_, _ = tx.Exec(ctx, "INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type) SELECT $1, organization_id, agent_id, $2, embedding, source_type FROM consolidated_memory WHERE id = $3", resolvedID, resolvedContext, c.ID1)
			_, _ = tx.Exec(ctx, "DELETE FROM consolidated_memory WHERE id IN ($1, $2)", c.ID1, c.ID2)
			_, _ = tx.Exec(ctx, "UPDATE memory_conflicts SET resolution_status = 'RESOLVED', resolved_memory_id = $1 WHERE conflict_id = $2", resolvedID, conflictID)

			if err := tx.Commit(ctx); err == nil {
				slog.Info("AutoDreamPipeline: resolved conflict via LLM synthesis", "conflict_id", conflictID, "resolved_id", resolvedID)
			}
			return nil
		}()
	}
}

// SearchTruth queries the consolidated vector database for the closest semantic embeddings.
func (p *AutoDreamPipeline) SearchTruth(ctx context.Context, embedding string, limit int) ([]orchestration.TruthSearchResult, error) {
	if p.pool.IsSQLite() {
		query := `
			SELECT id, content, 0 as distance
			FROM consolidated_memory
			ORDER BY created_at DESC
			LIMIT ?
		`
		rows, err := p.pool.Query(ctx, query, limit)
		if err != nil {
			return nil, fmt.Errorf("failed to search truth with SQLite fallback: %w", err)
		}
		defer rows.Close()

		var results []orchestration.TruthSearchResult
		for rows.Next() {
			var res orchestration.TruthSearchResult
			if err := rows.Scan(&res.MemoryID, &res.Context, &res.Distance); err != nil {
				continue
			}
			results = append(results, res)
		}
		return results, nil
	}

	query := `
		SELECT id, content, embedding <=> $1::vector as distance
		FROM consolidated_memory
		ORDER BY distance ASC
		LIMIT $2
	`
	rows, err := p.pool.Query(ctx, query, embedding, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to search truth with pgvector: %w", err)
	}
	defer rows.Close()

	var results []orchestration.TruthSearchResult
	for rows.Next() {
		var res orchestration.TruthSearchResult
		if err := rows.Scan(&res.MemoryID, &res.Context, &res.Distance); err != nil {
			continue
		}
		results = append(results, res)
	}
	return results, nil
}

func (p *AutoDreamPipeline) processBatch(ctx context.Context) {
	slog.Info("AutoDreamPipeline: starting memory consolidation batch")

	// 1. Extraction: Poll recent agent_session_data
	threshold := time.Now().Add(-1 * time.Hour).UTC()
	var query string
	if p.pool.IsSQLite() {
		query = "SELECT session_id, agent_id, context_data FROM agent_session_data WHERE last_accessed < ? LIMIT 50"
	} else {
		query = "SELECT session_id, agent_id, context_data FROM agent_session_data WHERE last_accessed < $1 LIMIT 50 FOR UPDATE SKIP LOCKED"
	}

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

	if len(sessions) == 0 {
		return
	}

	// 2. Consolidation & Embedding
	for _, s := range sessions {
		prompt := fmt.Sprintf("Summarize and consolidate this agent session memory:\n%s", s.ContextData)

		req := builtin.ChatRequest{
			System: "You are an AI Memory Consolidator.",
			Messages: []builtin.Message{
				{Role: builtin.RoleUser, Content: prompt},
			},
			MaxTokens: 500,
		}

		ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
		resp, err := p.llm.Chat(ctxTimeout, req)
		cancel()

		summary := "Summarized context from session " + s.ID
		if err == nil && resp.Message.Content != "" && resp.Message.Content != "" {
			summary = resp.Message.Content
		} else {
			slog.Warn("AutoDreamPipeline: LLM summarization failed", "error", err)
			telemetry.RecordAutoDreamCompressionError(ctx, s.AgentID, "llm_summarization_failed")
		}

		var embeddingStr string
		if p.minimaxClient != nil {
			ctxTimeout, cancel := context.WithTimeout(ctx, 15*time.Second)
			embedding, embedErr := p.minimaxClient.GenerateEmbedding(ctxTimeout, summary)
			cancel()
			if embedErr != nil {
				telemetry.RecordAutoDreamCompressionError(ctx, s.AgentID, "embedding_failed")
			} else if len(embedding) > 0 {
				str := fmt.Sprintf("%v", embedding)
				str = strings.ReplaceAll(strings.Trim(str, "[]"), " ", ",")
				embeddingStr = "[" + str + "]"
			}
		}

		if embeddingStr == "" {
			// Fallback vector for tests or missing API keys to allow insertion
			var vec []string
			for i := 0; i < 1536; i++ {
				vec = append(vec, "0.0")
			}
			embeddingStr = "[" + strings.Join(vec, ",") + "]"
		}

		err = func() error {
			tx, err := p.pool.Begin(ctx)
			if err != nil {
				return err
			}
			defer tx.Rollback(ctx)

			// 3. Loading: Upsert into consolidated_memory using pgvector
			var insertQuery string
			if p.pool.IsSQLite() {
				insertQuery = "INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type) VALUES (?, ?, ?, ?, ?, ?)"
				id := fmt.Sprintf("%d", time.Now().UnixNano())
				_, err = tx.Exec(ctx, insertQuery, id, "system", s.AgentID, summary, embeddingStr, "session_compression")
			} else {
				insertQuery = "INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type) VALUES ($1, $2, $3, $4, $5::vector, $6)"
				id := fmt.Sprintf("%d", time.Now().UnixNano())
				_, err = tx.Exec(ctx, insertQuery, id, "system", s.AgentID, summary, embeddingStr, "session_compression")
			}

			if err != nil {
				slog.Error("AutoDreamPipeline: failed to insert consolidated memory", "error", err)
				telemetry.RecordAutoDreamCompressionError(ctx, s.AgentID, "db_insert_failed")
				return err
			}

		telemetry.RecordAutoDreamMemoryCompressed(ctx, s.AgentID)

			// Delete from agent_session_data
			var delQuery string
			if p.pool.IsSQLite() {
				delQuery = "DELETE FROM agent_session_data WHERE session_id = ?"
			} else {
				delQuery = "DELETE FROM agent_session_data WHERE session_id = $1"
			}
			_, err = tx.Exec(ctx, delQuery, s.ID)
			if err != nil {
				slog.Error("AutoDreamPipeline: failed to delete session data after compression", "error", err)
				return err
			}

			return tx.Commit(ctx)
		}()
	}

	slog.Info("AutoDreamPipeline: batch completed", "count", len(sessions))
}

func (p *AutoDreamPipeline) processFiles(ctx context.Context) {
	memoryDir := os.Getenv("OHC_MEMORY_DIR")
	if memoryDir == "" {
		return // file-based memory ingestion disabled; use DB-backed memory instead
	}
	files, err := os.ReadDir(memoryDir)
	if err != nil {
		if !os.IsNotExist(err) {
			slog.Error("AutoDreamPipeline: failed to read memory directory", "error", err)
		}
		return
	}

	for _, file := range files {
		if file.IsDir() || filepath.Ext(file.Name()) != ".yml" && filepath.Ext(file.Name()) != ".yaml" {
			continue
		}

		filePath := filepath.Join(memoryDir, file.Name())
		processingPath := filePath + ".processing"

		// Use atomic rename to claim the file and prevent race conditions with other workers
		if err := os.Rename(filePath, processingPath); err != nil {
			// Another worker claimed it or file missing
			continue
		}

		contentBytes, err := os.ReadFile(processingPath)
		if err != nil {
			slog.Error("AutoDreamPipeline: failed to read memory file", "file", processingPath, "error", err)
			os.Rename(processingPath, filePath) // rollback claim
			continue
		}
		content := string(contentBytes)

		prompt := fmt.Sprintf("Summarize and consolidate this file memory:\n%s", content)

		req := builtin.ChatRequest{
			System: "You are an AI Memory Consolidator.",
			Messages: []builtin.Message{
				{Role: builtin.RoleUser, Content: prompt},
			},
			MaxTokens: 500,
		}

		ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
		resp, err := p.llm.Chat(ctxTimeout, req)
		cancel()

		summary := "Summarized context from file " + file.Name()
		if err == nil && resp.Message.Content != "" && resp.Message.Content != "" {
			summary = resp.Message.Content
		} else {
			slog.Warn("AutoDreamPipeline: LLM summarization failed", "error", err)
			telemetry.RecordAutoDreamIngestionError(ctx, "system", "llm_summarization_failed")
		}

		var embeddingStr string
		if p.minimaxClient != nil {
			ctxTimeout, cancel := context.WithTimeout(ctx, 15*time.Second)
			embedding, embedErr := p.minimaxClient.GenerateEmbedding(ctxTimeout, summary)
			cancel()
			if embedErr != nil {
				telemetry.RecordAutoDreamIngestionError(ctx, "system", "embedding_failed")
			} else if len(embedding) > 0 {
				str := fmt.Sprintf("%v", embedding)
				str = strings.ReplaceAll(strings.Trim(str, "[]"), " ", ",")
				embeddingStr = "[" + str + "]"
			}
		}

		if embeddingStr == "" {
			// Fallback vector for tests or missing API keys to allow insertion
			var vec []string
			for i := 0; i < 1536; i++ {
				vec = append(vec, "0.0")
			}
			embeddingStr = "[" + strings.Join(vec, ",") + "]"
		}

		var insertQuery string
		if p.pool.IsSQLite() {
			insertQuery = "INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type) VALUES (?, ?, ?, ?, ?, ?)"
			id := fmt.Sprintf("%d", time.Now().UnixNano())
			_, err = p.pool.Exec(ctx, insertQuery, id, "system", "system", summary, embeddingStr, "file_ingestion")
		} else {
			insertQuery = "INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type) VALUES ($1, $2, $3, $4, $5::vector, $6)"
			id := fmt.Sprintf("%d", time.Now().UnixNano())
			_, err = p.pool.Exec(ctx, insertQuery, id, "system", "system", summary, embeddingStr, "file_ingestion")
		}

		if err != nil {
			slog.Error("AutoDreamPipeline: failed to insert consolidated file memory", "error", err)
			telemetry.RecordAutoDreamIngestionError(ctx, "system", "db_insert_failed")
			// rename back to retry later
			os.Rename(processingPath, filePath)
			continue
		}

		telemetry.RecordAutoDreamMemoryIngested(ctx, "system")

		// Try to delete file
		os.Remove(processingPath)
	}
}

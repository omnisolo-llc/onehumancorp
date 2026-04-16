package pipeline

import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/agents/local"
	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"github.com/redis/rueidis"
)

// TruthSearchResult represents a semantic search result.
type TruthSearchResult struct {
	MemoryID string
	Context  string
	Distance float64
}

// AutoDreamPipeline orchestrates the background processing of agent memories.
// It extracts raw memory, consolidates it, embeds it via LLM clients, and loads it into pgvector.
// Master table: autodream_memories_master
type AutoDreamPipeline struct {
	pool          db.Provider
	llm           local.LLMClient
	minimaxClient interface {
		GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
	}
}

// NewAutoDreamPipeline creates a new AutoDreamPipeline instance.
func NewAutoDreamPipeline(pool db.Provider, redisClient rueidis.Client) *AutoDreamPipeline {
	var llmClient local.LLMClient
	if key := os.Getenv("ANTHROPIC_API_KEY"); key != "" {
		llmClient = local.NewAnthropicClient(key, "", "")
	} else if key := os.Getenv("OPENAI_API_KEY"); key != "" {
		endpoint := os.Getenv("OPENAI_API_BASE")
		model := os.Getenv("OHC_LOCAL_AGENT_MODEL")
		llmClient = local.NewOpenAICompatClient(endpoint, key, model)
	} else {
		llmClient = local.NewOllamaClient("", "")
	}

	llmClient = local.NewCachedLLMClient(llmClient, pool, redisClient)

	return &AutoDreamPipeline{
		pool:          pool,
		llm:           llmClient,
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
			p.pruneStaleData(ctx)
		}
	}
}

// pruneStaleData removes very old session data that might have been skipped or failed consolidation.
func (p *AutoDreamPipeline) pruneStaleData(ctx context.Context) {
	threshold := time.Now().Add(-7 * 24 * time.Hour).UTC()
	var query string
	if p.pool.IsSQLite() {
		query = "DELETE FROM agent_session_data WHERE last_accessed < ?"
	} else {
		query = "DELETE FROM agent_session_data WHERE last_accessed < $1"
	}

	res, err := p.pool.Exec(ctx, query, threshold)
	if err == nil && res > 0 {
		slog.Info("AutoDreamPipeline: pruned stale session data", "count", res)
	}
}

// resolveConflicts finds vector embeddings that are similar but have conflicting contexts.
func (p *AutoDreamPipeline) resolveConflicts(ctx context.Context) {
	if p.pool.IsSQLite() {
		return
	}

	query := `
		SELECT a.id, a.content, b.id, b.content, a.organization_id, a.memory_type, a.source_task_id
		FROM autodream_memories_master a
		JOIN autodream_memories_master b ON a.id < b.id AND a.organization_id = b.organization_id
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
		ID1            string
		Content1       string
		ID2            string
		Content2       string
		OrganizationID string
		MemoryType     string
		SourceTaskID   *string
	}

	var conflicts []Conflict
	for rows.Next() {
		var c Conflict
		if err := rows.Scan(&c.ID1, &c.Content1, &c.ID2, &c.Content2, &c.OrganizationID, &c.MemoryType, &c.SourceTaskID); err != nil {
			continue
		}
		conflicts = append(conflicts, c)
	}

	for _, c := range conflicts {
		conflictID := fmt.Sprintf("conflict-%s-%s", c.ID1, c.ID2)

		insertConflictQuery := "INSERT INTO memory_conflicts (conflict_id, memory_id_1, memory_id_2, resolution_status) VALUES ($1, $2, $3, 'PENDING') ON CONFLICT DO NOTHING"
		_, err := p.pool.Exec(ctx, insertConflictQuery, conflictID, c.ID1, c.ID2)
		if err != nil {
			continue
		}

		slog.Info("AutoDreamPipeline: detected memory conflict via pgvector", "id1", c.ID1, "id2", c.ID2, "org", c.OrganizationID)

		prompt := fmt.Sprintf("You are an AI Memory Consolidator. Resolve these two conflicting memories into a single truth.\nMemory 1: %s\nMemory 2: %s", c.Content1, c.Content2)
		req := local.CompletionRequest{
			SystemPrompt: "You are an AI Memory Consolidator. Provide a 'resolution' and a 'justification' for the resolution.",
			Messages: []local.ConversationMessage{
				{Role: "user", Content: []local.ContentPart{{Type: "text", Text: prompt}}},
			},
			MaxTokens: 500,
		}

		ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
		resp, llmErr := p.llm.Complete(ctxTimeout, req)
		cancel()

		resolvedContext := "Consolidated memory: " + c.Content1 + " & " + c.Content2
		if llmErr == nil && resp != nil && resp.Text != "" {
			resolvedContext = resp.Text
		}

		resolvedID := fmt.Sprintf("resolved-%s", conflictID)

		_ = func() error {
			tx, txErr := p.pool.Begin(ctx)
			if txErr != nil {
				slog.Error("AutoDreamPipeline: failed to begin tx for conflict resolution", "error", txErr)
				return txErr
			}
			defer tx.Rollback(ctx)

			_, err = tx.Exec(ctx, "INSERT INTO autodream_memories_master (id, organization_id, memory_type, content, embedding, source_task_id) SELECT $1, organization_id, memory_type, $2, embedding, source_task_id FROM autodream_memories_master WHERE id = $3", resolvedID, resolvedContext, c.ID1)
			if err != nil {
				slog.Error("AutoDreamPipeline: failed to insert resolved memory", "error", err)
				return err
			}

			_, err = tx.Exec(ctx, "DELETE FROM autodream_memories_master WHERE id IN ($1, $2)", c.ID1, c.ID2)
			if err != nil {
				slog.Error("AutoDreamPipeline: failed to delete conflicting memories", "error", err)
				return err
			}

			_, err = tx.Exec(ctx, "UPDATE memory_conflicts SET resolution_status = 'RESOLVED', resolved_memory_id = $1 WHERE conflict_id = $2", resolvedID, conflictID)
			if err != nil {
				slog.Error("AutoDreamPipeline: failed to update memory_conflicts table", "error", err)
				return err
			}

			if err := tx.Commit(ctx); err == nil {
				slog.Info("AutoDreamPipeline: resolved conflict via LLM synthesis", "conflict_id", conflictID, "resolved_id", resolvedID)
			} else {
				slog.Error("AutoDreamPipeline: failed to commit conflict resolution tx", "error", err)
			}
			return nil
		}()
	}
}

// SearchTruth queries the consolidated vector database for the closest semantic embeddings.
func (p *AutoDreamPipeline) SearchTruth(ctx context.Context, embedding string, limit int) ([]TruthSearchResult, error) {
	claims := auth.ClaimsFromContext(ctx)
	orgID := "system"
	if claims != nil && claims.OrganizationID != "" {
		orgID = claims.OrganizationID
	}

	if p.pool.IsSQLite() {
		query := `
			SELECT id, content, 0 as distance
			FROM autodream_memories_master
			WHERE organization_id = ?
			ORDER BY created_at DESC
			LIMIT ?
		`
		rows, err := p.pool.Query(ctx, query, orgID, limit)
		if err != nil {
			return nil, fmt.Errorf("failed to search truth with SQLite fallback: %w", err)
		}
		defer rows.Close()

		var results []TruthSearchResult
		for rows.Next() {
			var res TruthSearchResult
			if err := rows.Scan(&res.MemoryID, &res.Context, &res.Distance); err != nil {
				continue
			}
			results = append(results, res)
		}
		return results, nil
	}

	query := `
		SELECT id, content, embedding <=> $1::vector as distance
		FROM autodream_memories_master
		WHERE organization_id = $2
		ORDER BY distance ASC
		LIMIT $3
	`
	rows, err := p.pool.Query(ctx, query, embedding, orgID, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to search truth with pgvector: %w", err)
	}
	defer rows.Close()

	var results []TruthSearchResult
	for rows.Next() {
		var res TruthSearchResult
		if err := rows.Scan(&res.MemoryID, &res.Context, &res.Distance); err != nil {
			continue
		}
		results = append(results, res)
	}
	return results, nil
}

// InjectTruth directly inserts a pre-computed embedding into the master store.
func (p *AutoDreamPipeline) InjectTruth(ctx context.Context, memoryID, orgID, content string, embedding string) error {
	var query string
	if p.pool.IsSQLite() {
		query = "INSERT INTO autodream_memories_master (id, organization_id, memory_type, content, embedding) VALUES (?, ?, 'injected', ?, ?) ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, embedding=EXCLUDED.embedding"
	} else {
		query = "INSERT INTO autodream_memories_master (id, organization_id, memory_type, content, embedding) VALUES ($1, $2, 'injected', $3, $4::vector) ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, embedding=EXCLUDED.embedding"
	}

	_, err := p.pool.Exec(ctx, query, memoryID, orgID, content, embedding)
	return err
}

func (p *AutoDreamPipeline) processBatch(ctx context.Context) {
	slog.Info("AutoDreamPipeline: starting memory consolidation batch")

	threshold := time.Now().Add(-1 * time.Hour).UTC()
	var query string
	if p.pool.IsSQLite() {
		query = "SELECT session_id, agent_id, context_data FROM agent_session_data WHERE last_accessed < ? LIMIT 50"
	} else {
		query = "SELECT session_id, agent_id, context_data FROM agent_session_data WHERE last_accessed < $1 LIMIT 50 FOR UPDATE SKIP LOCKED"
	}

	// We open a short-lived transaction just to claim the rows
	tx, err := p.pool.Begin(ctx)
	if err != nil {
		slog.Error("AutoDreamPipeline: failed to begin claim tx", "error", err)
		return
	}

	rows, err := tx.Query(ctx, query, threshold)
	if err != nil {
		slog.Error("AutoDreamPipeline: failed to fetch stale sessions", "error", err)
		tx.Rollback(ctx)
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

	// Release the claim transaction BEFORE starting LLM calls to avoid holding locks.
	// This is slightly risky as other workers might pick them up, but better than holding locks for minutes.
	tx.Rollback(ctx)

	if len(sessions) == 0 {
		return
	}

	for _, s := range sessions {
		p.processSession(ctx, s)
	}

	slog.Info("AutoDreamPipeline: batch completed", "count", len(sessions))
}

func (p *AutoDreamPipeline) processSession(ctx context.Context, s Session) {
	prompt := fmt.Sprintf("Summarize and consolidate this agent session memory:\n%s", s.ContextData)

	req := local.CompletionRequest{
		SystemPrompt: "You are an AI Memory Consolidator.",
		Messages: []local.ConversationMessage{
			{Role: "user", Content: []local.ContentPart{{Type: "text", Text: prompt}}},
		},
		MaxTokens: 500,
	}

	ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
	resp, err := p.llm.Complete(ctxTimeout, req)
	cancel()

	summary := "Summarized context from session " + s.ID
	if err == nil && resp != nil && resp.Text != "" {
		summary = resp.Text
	} else {
		slog.Warn("AutoDreamPipeline: LLM summarization failed", "error", err)
		telemetry.RecordAutoDreamCompressionError(ctx, s.AgentID, "llm_summarization_failed")
	}

	embeddingStr := "[0.0]"
	if p.minimaxClient != nil {
		ctxTimeout, cancel := context.WithTimeout(ctx, 15*time.Second)
		embedding, embedErr := p.minimaxClient.GenerateEmbedding(ctxTimeout, summary)
		cancel()
		if embedErr == nil && len(embedding) > 0 {
			str := fmt.Sprintf("%v", embedding)
			str = strings.ReplaceAll(strings.Trim(str, "[]"), " ", ",")
			embeddingStr = "[" + str + "]"
		}
	}

	if embeddingStr == "[0.0]" {
		var vec []string
		for i := 0; i < 1536; i++ {
			vec = append(vec, "0.0")
		}
		embeddingStr = "[" + strings.Join(vec, ",") + "]"
	}

	_ = func() error {
		tx, err := p.pool.Begin(ctx)
		if err != nil {
			return err
		}
		defer tx.Rollback(ctx)

		id := uuid.New().String()
		orgID := "system"

		var insertQuery string
		if p.pool.IsSQLite() {
			insertQuery = "INSERT INTO autodream_memories_master (id, organization_id, memory_type, content, embedding) VALUES (?, ?, ?, ?, ?)"
			_, err = tx.Exec(ctx, insertQuery, id, orgID, "session_compression", summary, embeddingStr)
		} else {
			insertQuery = "INSERT INTO autodream_memories_master (id, organization_id, memory_type, content, embedding) VALUES ($1, $2, $3, $4, $5::vector)"
			_, err = tx.Exec(ctx, insertQuery, id, orgID, "session_compression", summary, embeddingStr)
		}

		if err != nil {
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
		_, err = tx.Exec(ctx, delQuery, s.ID)
		if err != nil {
			slog.Error("AutoDreamPipeline: failed to delete session data", "error", err)
			return err
		}

		return tx.Commit(ctx)
	}()
}

func (p *AutoDreamPipeline) processFiles(ctx context.Context) {
	memoryDir := os.Getenv("OHC_MEMORY_DIR")
	if memoryDir == "" {
		return
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

		if err := os.Rename(filePath, processingPath); err != nil {
			continue
		}

		contentBytes, err := os.ReadFile(processingPath)
		if err != nil {
			slog.Error("AutoDreamPipeline: failed to read memory file", "file", processingPath, "error", err)
			os.Rename(processingPath, filePath)
			continue
		}
		content := string(contentBytes)

		prompt := fmt.Sprintf("Summarize and consolidate this file memory:\n%s", content)

		req := local.CompletionRequest{
			SystemPrompt: "You are an AI Memory Consolidator.",
			Messages: []local.ConversationMessage{
				{Role: "user", Content: []local.ContentPart{{Type: "text", Text: prompt}}},
			},
			MaxTokens: 500,
		}

		ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
		resp, err := p.llm.Complete(ctxTimeout, req)
		cancel()

		summary := "Summarized context from file " + file.Name()
		if err == nil && resp != nil && resp.Text != "" {
			summary = resp.Text
		}

		embeddingStr := "[0.0]"
		if p.minimaxClient != nil {
			ctxTimeout, cancel := context.WithTimeout(ctx, 15*time.Second)
			embedding, embedErr := p.minimaxClient.GenerateEmbedding(ctxTimeout, summary)
			cancel()
			if embedErr == nil && len(embedding) > 0 {
				str := fmt.Sprintf("%v", embedding)
				str = strings.ReplaceAll(strings.Trim(str, "[]"), " ", ",")
				embeddingStr = "[" + str + "]"
			}
		}

		if embeddingStr == "[0.0]" {
			var vec []string
			for i := 0; i < 1536; i++ {
				vec = append(vec, "0.0")
			}
			embeddingStr = "[" + strings.Join(vec, ",") + "]"
		}

		var insertQuery string
		id := uuid.New().String()
		orgID := "system"

		if p.pool.IsSQLite() {
			insertQuery = "INSERT INTO autodream_memories_master (id, organization_id, memory_type, content, embedding) VALUES (?, ?, ?, ?, ?)"
			_, err = p.pool.Exec(ctx, insertQuery, id, orgID, "file_ingestion", summary, embeddingStr)
		} else {
			insertQuery = "INSERT INTO autodream_memories_master (id, organization_id, memory_type, content, embedding) VALUES ($1, $2, $3, $4, $5::vector)"
			_, err = p.pool.Exec(ctx, insertQuery, id, orgID, "file_ingestion", summary, embeddingStr)
		}

		if err != nil {
			slog.Error("AutoDreamPipeline: failed to insert consolidated file memory", "error", err)
			os.Rename(processingPath, filePath)
			continue
		}

		telemetry.RecordAutoDreamMemoryIngested(ctx, "system")
		os.Remove(processingPath)
	}
}

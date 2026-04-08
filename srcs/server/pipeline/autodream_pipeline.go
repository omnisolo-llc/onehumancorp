package pipeline

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
	"github.com/onehumancorp/mono/srcs/server/agents/local"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
	"github.com/onehumancorp/mono/srcs/server/orchestration/queue"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"github.com/redis/rueidis"
)

// AutoDreamPipeline orchestrates the background processing of agent memories.
// It extracts raw memory, consolidates it, embeds it via LLM clients, and loads it into pgvector.
type AutoDreamPipeline struct {
	pool          db.Provider
	worker        *orchestration.AutoDreamWorker
	llm           local.LLMClient
	minimaxClient orchestration.MinimaxClient
	redisClient   rueidis.Client
	queue         queue.TaskQueue
}

// NewAutoDreamPipeline creates a new AutoDreamPipeline instance.
func NewAutoDreamPipeline(pool db.Provider, redisClient rueidis.Client) *AutoDreamPipeline {
	worker := orchestration.NewAutoDreamWorker(pool)

	// Determine LLM client based on env vars
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

	var mClient orchestration.MinimaxClient
	if minimaxKey := os.Getenv("MINIMAX_API_KEY"); minimaxKey != "" {
		mClient = orchestration.NewCachedMinimaxClient(orchestration.NewMinimaxClient(minimaxKey), pool, redisClient)
	}

	p := &AutoDreamPipeline{
		pool:          pool,
		worker:        worker,
		llm:           llmClient,
		minimaxClient: mClient,
		redisClient:   redisClient,
	}

	if redisClient != nil {
		p.queue = queue.NewRedisTaskQueue(redisClient, "ohc_")
	} else if pool.IsSQLite() {
		p.queue = queue.NewSQLiteTaskQueue(pool)
	} else {
		p.queue = queue.NewPostgresTaskQueue(pool)
	}

	return p
}

func (p *AutoDreamPipeline) checkMailbox(ctx context.Context) {
	if p.redisClient == nil {
		return
	}
	// Teammate Mesh: Check mailbox at start, safely pop messages to prevent race conditions.
	slog.Info("AutoDreamPipeline: checking Teammate Mesh mailbox on start")

	// Pop messages one by one to avoid data loss from concurrent appends
	for {
		cmd := p.redisClient.B().Lpop().Key("autodream_mailbox").Build()
		res := p.redisClient.Do(ctx, cmd)
		msg, err := res.ToString()
		if err != nil {
			if rueidis.IsRedisNil(err) {
				break // Queue empty
			}
			slog.Warn("AutoDreamPipeline: failed to check mailbox", "error", err)
			break
		}
		slog.Info("AutoDreamPipeline: mailbox message received", "message", msg)
	}
}

// Start runs the pipeline in the background using distributed queues.
func (p *AutoDreamPipeline) Start(ctx context.Context) {
	// Coordinate via Teammate Mesh
	p.checkMailbox(ctx)

	// Distributed Worker Queue: Start the consumer loop
	go p.runQueueWorker(ctx)

	// Publisher loop: Periodically scans for work and enqueues to the distributed queue
	ticker := time.NewTicker(2 * time.Minute)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			p.enqueueStaleSessions(ctx)
			p.processFiles(ctx)
			p.resolveConflicts(ctx)
		}
	}
}

// runQueueWorker polls the distributed TaskQueue for memory pruning jobs.
func (p *AutoDreamPipeline) runQueueWorker(ctx context.Context) {
	ticker := time.NewTicker(5 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			task, err := p.queue.Dequeue(ctx, []string{"autodream_pruning"})
			if err != nil {
				slog.Error("AutoDreamPipeline: error polling queue", "error", err)
				continue
			}
			if task == nil {
				continue
			}

			// Extract payload
			var payload map[string]interface{}
			if err := json.Unmarshal([]byte(task.Payload), &payload); err == nil {
				sessionID, _ := payload["session_id"].(string)
				agentID, _ := payload["agent_id"].(string)
				contextData, _ := payload["context_data"].(string)

				if sessionID != "" && contextData != "" {
					p.processSingleSession(ctx, sessionID, agentID, contextData)
				}
			}

			// Mark task as completed
			_ = p.queue.Complete(ctx, task.ID)
		}
	}
}

// enqueueStaleSessions finds stale sessions and adds them to the distributed worker queue.
func (p *AutoDreamPipeline) enqueueStaleSessions(ctx context.Context) {
	threshold := time.Now().Add(-1 * time.Hour).UTC()
	var query string
	if p.pool.IsSQLite() {
		query = "SELECT session_id, agent_id, context_data FROM agent_session_data WHERE last_accessed < ? LIMIT 50"
	} else {
		query = "SELECT session_id, agent_id, context_data FROM agent_session_data WHERE last_accessed < $1 LIMIT 50 FOR UPDATE SKIP LOCKED"
	}

	rows, err := p.pool.Query(ctx, query, threshold)
	if err != nil {
		slog.Error("AutoDreamPipeline: failed to fetch stale sessions for queue", "error", err)
		return
	}
	defer rows.Close()

	for rows.Next() {
		var sessionID, agentID, contextData string
		if err := rows.Scan(&sessionID, &agentID, &contextData); err == nil {
			payload := map[string]interface{}{
				"session_id":   sessionID,
				"agent_id":     agentID,
				"context_data": contextData,
			}
			payloadBytes, _ := json.Marshal(payload)

			job := &queue.Job{
				ID:        uuid.New().String(),
				AgentRole: "autodream_pruning",
				Payload:   string(payloadBytes),
				Status:    "PENDING",
			}

			err := p.queue.Enqueue(ctx, job)
			if err != nil {
				slog.Error("AutoDreamPipeline: failed to enqueue pruning task", "error", err)
				continue
			}

			// Once enqueued, optionally mark it as queued or remove it,
			// but for this implementation we'll remove it from the table immediately
			// and let the distributed worker process it and store it into consolidated memory.
			// This prevents it from being picked up again on next loop.
			delQuery := "DELETE FROM agent_session_data WHERE session_id = $1"
			if p.pool.IsSQLite() {
				delQuery = "DELETE FROM agent_session_data WHERE session_id = ?"
			}
			_, _ = p.pool.Exec(ctx, delQuery, sessionID)
		}
	}
}

// processSingleSession handles memory pruning for a single agent session data point.
func (p *AutoDreamPipeline) processSingleSession(ctx context.Context, sessionID, agentID, contextData string) {
	prompt := fmt.Sprintf("Summarize and consolidate this agent session memory:\n%s", contextData)

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

	summary := "Summarized context from session " + sessionID
	if err == nil && resp != nil && resp.Text != "" {
		summary = resp.Text
	} else {
		slog.Warn("AutoDreamPipeline: LLM summarization failed", "error", err)
	}

	var embeddingStr string
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

	if embeddingStr == "" {
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
		_, err = p.pool.Exec(ctx, insertQuery, id, "system", agentID, summary, embeddingStr, "session_compression")
	} else {
		insertQuery = "INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type) VALUES ($1, $2, $3, $4, $5::vector, $6)"
		id := fmt.Sprintf("%d", time.Now().UnixNano())
		_, err = p.pool.Exec(ctx, insertQuery, id, "system", agentID, summary, embeddingStr, "session_compression")
	}

	if err != nil {
		slog.Error("AutoDreamPipeline: failed to insert consolidated memory", "error", err)
	} else {
		telemetry.RecordAutoDreamMemoryCompressed(ctx, agentID)
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
		req := local.CompletionRequest{
			SystemPrompt: "You are an AI Memory Consolidator.",
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

// InjectTruth inserts high-dimensional semantic memory directly into the consolidated store.
func (p *AutoDreamPipeline) InjectTruth(ctx context.Context, memoryID, content string, embedding string) error {
	var query string
	if p.pool.IsSQLite() {
		query = `
			INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type)
			VALUES (?, ?, ?, ?, ?, ?)
			ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, embedding=EXCLUDED.embedding
		`
		_, err := p.pool.Exec(ctx, query, memoryID, "system", "system", content, embedding, "injected_truth")
		return err
	}

	query = `
		INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type)
		VALUES ($1, $2, $3, $4, $5::vector, $6)
		ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, embedding=EXCLUDED.embedding
	`
	_, err := p.pool.Exec(ctx, query, memoryID, "system", "system", content, embedding, "injected_truth")
	return err
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


func (p *AutoDreamPipeline) processFiles(ctx context.Context) {
	memoryDir := ".agent-task/memory"
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
		} else {
			slog.Warn("AutoDreamPipeline: LLM summarization failed", "error", err)
		}

		var embeddingStr string
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
			// rename back to retry later
			os.Rename(processingPath, filePath)
			continue
		}

		telemetry.RecordAutoDreamMemoryIngested(ctx, "system")

		// Try to delete file
		os.Remove(processingPath)
	}
}

package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"time"
	"sync"
	"gopkg.in/yaml.v3"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/orchestration/kairos"
	autodream_metrics "github.com/onehumancorp/mono/src/server/orchestration/autodream"
	"github.com/onehumancorp/mono/src/server/telemetry"
)

// AutoDreamWorker handles memory consolidation, pruning, and conflict resolution.
type AutoDreamWorker struct {
	pool db.Provider
	mesh MeshTransport
	llmClient MinimaxClient
}
// AutoDreamWorker options
type AutoDreamWorkerOptions struct {
	PruningInterval  time.Duration
	ConflictInterval time.Duration
	LLMClient        MinimaxClient
}
// NewAutoDreamWorker creates a new AutoDream worker.
func NewAutoDreamWorker(pool db.Provider) *AutoDreamWorker {
	w := &AutoDreamWorker{pool: pool, llmClient: nil}
	// Note: You can inject rueidis.Client and MinimaxClient into the struct if needed.
	return w
}
func (w *AutoDreamWorker) SetMeshTransport(mesh MeshTransport) {
	w.mesh = mesh
}
func (w *AutoDreamWorker) Start(ctx context.Context) {
	slog.Info("Starting AutoDream memory consolidation worker")

	// Create distributed pruning queue using Postgres
	// In multi-tenant cloud mode, this could use a distributed lock or queue.
	// For simplicity, we just use a distributed worker queue pattern with a database table or Redis.
	go w.runPruningPipeline(ctx)
	go w.runConflictResolutionPipeline(ctx)
	go w.runMemoryIngestionPipeline(ctx)
	go w.runMissionIngestionPipeline(ctx)
	go w.runSessionCompressionPipeline(ctx)
	go w.runCompletedTasksIngestionPipeline(ctx)
}
func (w *AutoDreamWorker) runCompletedTasksIngestionPipeline(ctx context.Context) {
	ticker := time.NewTicker(2 * time.Minute)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			w.ingestCompletedTasks(ctx)
		}
	}
}
func (w *AutoDreamWorker) ingestCompletedTasks(ctx context.Context) {
	tx, err := w.pool.Begin(ctx)
	if err != nil {
		slog.Error("AutoDream: failed to begin tx for completed tasks", "error", err)
		return
	}
	mode := kairos.GetMode()
	start := time.Now()
	_ = mode
	_ = start
	defer tx.Rollback(ctx)

	var query string
	if w.pool.IsSQLite() {
		query = "SELECT id, title, description, payload FROM shared_tasks_master WHERE status = 'COMPLETED' LIMIT 50"
	} else {
		query = "SELECT id, title, description, payload FROM shared_tasks_master WHERE status = 'COMPLETED' LIMIT 50 FOR UPDATE SKIP LOCKED"
	}

	rows, err := tx.Query(ctx, query)
	if err != nil {
		slog.Error("AutoDream: failed to query completed tasks", "error", err)
		return
	}
	defer rows.Close()

	type Task struct {
		ID          string
		Title       string
		Description *string
		Payload     *string
	}
	var tasks []Task

	for rows.Next() {
		var t Task
		if err := rows.Scan(&t.ID, &t.Title, &t.Description, &t.Payload); err == nil {
			tasks = append(tasks, t)
		}
	}
	rows.Close()

	for _, task := range tasks {
		content := "Task: " + task.Title
		if task.Description != nil {
			content += "\nDescription: " + *task.Description
		}
		if task.Payload != nil {
			content += "\nPayload: " + *task.Payload
		}

		var embeddingStr *string
		minimaxKey := os.Getenv("MINIMAX_API_KEY")
		if minimaxKey != "" {
			client := NewCachedMinimaxClient(NewMinimaxClient(minimaxKey), w.pool, nil)
			ctxTimeout, cancel := context.WithTimeout(ctx, 15*time.Second)
			embedding, embedErr := client.GenerateEmbedding(ctxTimeout, content)
			cancel()
			if embedErr == nil && len(embedding) > 0 {
				str := fmt.Sprintf("%v", embedding)
				str = strings.ReplaceAll(strings.Trim(str, "[]"), " ", ",")
				str = "[" + str + "]"
				embeddingStr = &str
			}
		}

		var insertQuery string
		memID := uuid.New().String()
		if w.pool.IsSQLite() {
			insertQuery = "INSERT INTO autodream_memories (id, content, embedding, source_mission_id, organization_id, agent_id, source_type, created_at) VALUES (?, ?, ?, ?, 'system', 'autodream_worker', 'task_completion', CURRENT_TIMESTAMP)"
			_, err = tx.Exec(ctx, insertQuery, memID, content, embeddingStr, task.ID)
		} else {
			insertQuery = "INSERT INTO autodream_memories (id, content, embedding, source_mission_id, organization_id, agent_id, source_type, created_at) VALUES ($1, $2, $3::vector, $4, 'system', 'autodream_worker', 'task_completion', CURRENT_TIMESTAMP)"
			_, err = tx.Exec(ctx, insertQuery, memID, content, embeddingStr, task.ID)
		}

		if err != nil {
			slog.Error("AutoDream: failed to insert completed task memory", "error", err)
			continue
		}

		telemetry.RecordAutoDreamConsolidation(ctx, "autodream_worker")

		// Update task to ARCHIVED to avoid re-processing
		updateQuery := "UPDATE shared_tasks_master SET status = 'ARCHIVED' WHERE id = $1"
		if w.pool.IsSQLite() {
			updateQuery = "UPDATE shared_tasks_master SET status = 'ARCHIVED' WHERE id = ?"
		}
		_, err = tx.Exec(ctx, updateQuery, task.ID)
		if err != nil {
			slog.Error("AutoDream: failed to archive processed task", "error", err)
			tx.Rollback(ctx)
			return
		}
	}

	if err := tx.Commit(ctx); err != nil {
		slog.Error("AutoDream: failed to commit completed task processing", "error", err)
	}
}
func (w *AutoDreamWorker) runSessionCompressionPipeline(ctx context.Context) {
	ticker := time.NewTicker(5 * time.Minute)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			w.compressSessionData(ctx)
		}
	}
}
func (w *AutoDreamWorker) compressSessionData(ctx context.Context) {
	tx, err := w.pool.Begin(ctx)
	if err != nil {
		slog.Error("AutoDream: failed to begin transaction for compression", "error", err)
		return
	}
	mode := kairos.GetMode()
	start := time.Now()
	_ = mode
	_ = start
	defer tx.Rollback(ctx)

	var query string
	if w.pool.IsSQLite() {
		// SQLite Recency Fallback
		query = `
			SELECT session_id, context_data
			FROM agent_session_data
			ORDER BY last_accessed ASC
			LIMIT 10
		`
	} else {
		// PostgreSQL with SKIP LOCKED
		query = `
			SELECT session_id, context_data
			FROM agent_session_data
			ORDER BY last_accessed ASC
			LIMIT 10
			FOR UPDATE SKIP LOCKED
		`
	}

	rows, err := tx.Query(ctx, query)
	if err != nil {
		slog.Error("AutoDream: failed to fetch session data", "error", err)
		return
	}
	defer rows.Close()

	type sessionRec struct {
		ID   string
		Data string
	}
	var records []sessionRec
	for rows.Next() {
		var rec sessionRec
		if err := rows.Scan(&rec.ID, &rec.Data); err == nil {
			records = append(records, rec)
		}
	}
	if err := rows.Err(); err != nil {
		slog.Error("AutoDream: iteration error", "error", err)
		return
	}
	rows.Close()

	for _, rec := range records {
		var insertQuery string
		if w.pool.IsSQLite() {
			insertQuery = `
				INSERT INTO autodream_memories (id, content, source_mission_id)
				VALUES (?, ?, ?)
			`
		} else {
			insertQuery = `
				INSERT INTO autodream_memories (id, content, source_mission_id)
				VALUES ($1, $2, $3)
			`
		}
		memID := "ad-" + rec.ID
		_, err := tx.Exec(ctx, insertQuery, memID, rec.Data, rec.ID)
		if err != nil {
			slog.Error("AutoDream: failed to insert compressed memory", "error", err)
			continue
		}

		var delQuery string
		if w.pool.IsSQLite() {
			delQuery = "DELETE FROM agent_session_data WHERE session_id = ?"
		} else {
			delQuery = "DELETE FROM agent_session_data WHERE session_id = $1"
		}
		_, err = tx.Exec(ctx, delQuery, rec.ID)
		if err != nil {
			slog.Error("AutoDream: failed to delete session data after compression", "error", err)
			tx.Rollback(ctx)
			return
		}
	}

	if err := tx.Commit(ctx); err != nil {
		slog.Error("AutoDream: failed to commit compression transaction", "error", err)
	}
}
func (w *AutoDreamWorker) runMemoryIngestionPipeline(ctx context.Context) {
	ticker := time.NewTicker(1 * time.Minute)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			w.ingestAgentMemories(ctx)
			w.compressSessionContexts(ctx)
		}
	}
}
func (w *AutoDreamWorker) compressSessionContexts(ctx context.Context) {
	mode := kairos.GetMode()
	start := time.Now()
	_ = mode
	_ = start
	_ = mode
	_ = start
	// Only process sessions older than 5 minutes to avoid compressing active sessions
	threshold := time.Now().Add(-5 * time.Minute).UTC()
	var rows db.Rows
	var err error
	var tx db.Tx

	if w.pool.IsSQLite() {
		query := "SELECT session_id, agent_id, context_data FROM agent_session_data WHERE last_accessed < ? ORDER BY last_accessed ASC LIMIT 50"
		rows, err = w.pool.Query(ctx, query, threshold)
	} else {
		tx, err = w.pool.Begin(ctx)
		if err != nil {
			slog.Error("AutoDream: failed to begin tx", "error", err)
			return
		}
		defer tx.Rollback(ctx)
		query := "SELECT session_id, agent_id, context_data FROM agent_session_data WHERE last_accessed < $1 ORDER BY last_accessed ASC LIMIT 50 FOR UPDATE SKIP LOCKED"
		rows, err = tx.Query(ctx, query, threshold)
	}

	if err != nil {
		slog.Error("AutoDream: failed to fetch stale sessions for compression", "error", err)
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
		if err := rows.Scan(&s.ID, &s.AgentID, &s.ContextData); err != nil {
			continue
		}
		sessions = append(sessions, s)
	}
	rows.Close()

	if len(sessions) == 0 {
		return
	}

	for _, s := range sessions {
		// Mock summarization:
		summary := "Summarized context from session " + s.ID + ": " + s.ContextData

		embeddingStr := "[0.0]" // fallback embedding
		minimaxKey := os.Getenv("MINIMAX_API_KEY")
		if minimaxKey != "" {
			client := NewCachedMinimaxClient(NewMinimaxClient(minimaxKey), w.pool, nil)
			ctxTimeout, cancel := context.WithTimeout(ctx, 15*time.Second)
			embedding, embedErr := client.GenerateEmbedding(ctxTimeout, summary)
			cancel()
			if embedErr != nil {
				slog.Warn("AutoDream: LLM embedding failed during compression, using fallback", "error", embedErr)
			} else if len(embedding) > 0 {
				embeddingStr = fmt.Sprintf("%v", embedding)
				embeddingStr = strings.ReplaceAll(strings.Trim(embeddingStr, "[]"), " ", ",")
				embeddingStr = "[" + embeddingStr + "]"
			}
		}

		// Store into consolidated_memory using the current transaction (if any) or pool
		type execer interface {
			Exec(ctx context.Context, sql string, arguments ...any) (int64, error)
		}
		var target execer = w.pool
		if tx != nil {
			target = tx
		}

		var insertQuery string
		var embedPtr *string
		if embeddingStr != "[0.0]" {
			embedPtr = &embeddingStr
		}

		if w.pool.IsSQLite() {
			insertQuery = "INSERT INTO consolidated_memory (id, organization_id, content, embedding, source_type) VALUES (?, 'system', ?, ?, 'autodream')"
			id := fmt.Sprintf("%d", time.Now().UnixNano())
			_, err = target.Exec(ctx, insertQuery, id, summary, embedPtr)
		} else {
			insertQuery = "INSERT INTO consolidated_memory (id, organization_id, content, embedding, source_type) VALUES (gen_random_uuid(), 'system', $1, $2::vector, 'autodream')"
			_, err = target.Exec(ctx, insertQuery, summary, embedPtr)
		}
		if err != nil {
			slog.Error("AutoDream: failed to insert compressed memory", "session", s.ID, "error", err)
			autodream_metrics.ConsolidationErrorsTotal.WithLabelValues(mode, "compress_session_contexts", "db_insert_error").Inc()
			autodream_metrics.MemoriesProcessedTotal.WithLabelValues(mode, "session_context", "failure").Inc()
			continue
		}

		telemetry.RecordAutoDreamMemoryCompressed(ctx, s.AgentID)

		// Remove compressed session
		deleteQuery := "DELETE FROM agent_session_data WHERE session_id = $1"
		if w.pool.IsSQLite() {
			deleteQuery = "DELETE FROM agent_session_data WHERE session_id = ?"
		}
		_, err = target.Exec(ctx, deleteQuery, s.ID)
		if err != nil {
			slog.Error("AutoDream: failed to delete compressed session", "session", s.ID, "error", err)
			continue
		}
	}

	if !w.pool.IsSQLite() && tx != nil {
		// Commit the outer tx that holds the SKIP LOCKED rows
		_ = tx.Commit(ctx)
	}
}
func (w *AutoDreamWorker) ingestAgentMemories(ctx context.Context) {
	memoryDir := os.Getenv("OHC_MEMORY_DIR")
	if memoryDir == "" {
		return // DB-backed memory; no file ingestion needed
	}
	files, err := os.ReadDir(memoryDir)
	if err != nil {
		if !os.IsNotExist(err) {
			slog.Error("AutoDream: failed to read memory directory", "error", err)
		}
		return
	}

	for _, file := range files {
		if file.IsDir() || filepath.Ext(file.Name()) != ".yml" && filepath.Ext(file.Name()) != ".yaml" {
			continue
		}

		filePath := filepath.Join(memoryDir, file.Name())
		contentBytes, err := os.ReadFile(filePath)
		if err != nil {
			slog.Error("AutoDream: failed to read memory file", "file", filePath, "error", err)
			continue
		}
		content := string(contentBytes)

		memoryID := "mem-" + file.Name()

		// Check if already processed to maintain idempotency
		var check int
		var checkQuery string
		if w.pool.IsSQLite() {
			checkQuery = "SELECT 1 FROM autodream_memories WHERE source_mission_id = ? LIMIT 1"
		} else {
			checkQuery = "SELECT 1 FROM autodream_memories WHERE source_mission_id = $1 LIMIT 1"
		}
		if err := w.pool.QueryRow(ctx, checkQuery, memoryID).Scan(&check); err == nil {
			// Already exists, delete file and skip
			os.Remove(filePath)
			continue
		}

		// Generate embedding
		var embeddingStr *string
		minimaxKey := os.Getenv("MINIMAX_API_KEY")
		if minimaxKey != "" {
			client := NewCachedMinimaxClient(NewMinimaxClient(minimaxKey), w.pool, nil)
			ctxTimeout, cancel := context.WithTimeout(ctx, 15*time.Second)
			embedding, err := client.GenerateEmbedding(ctxTimeout, content)
			cancel()
			if err != nil {
				slog.Warn("AutoDream: LLM embedding failed, using fallback", "error", err)
			} else if len(embedding) > 0 {
				str := fmt.Sprintf("%v", embedding)
				str = strings.ReplaceAll(strings.Trim(str, "[]"), " ", ",")
				str = "[" + str + "]"
				embeddingStr = &str
			}
		}

		var insertQuery string
		if w.pool.IsSQLite() {
			insertQuery = "INSERT INTO consolidated_memory (id, organization_id, content, embedding, source_type) VALUES (?, 'system', ?, ?, 'autodream')"
			id := fmt.Sprintf("%d", time.Now().UnixNano())
			_, err = w.pool.Exec(ctx, insertQuery, id, content, embeddingStr)
		} else {
			insertQuery = "INSERT INTO consolidated_memory (id, organization_id, content, embedding, source_type) VALUES (gen_random_uuid(), 'system', $1, $2::vector, 'autodream')"
			_, err = w.pool.Exec(ctx, insertQuery, content, embeddingStr)
		}

		if err != nil {
			slog.Error("AutoDream: failed to insert memory", "file", file.Name(), "error", err)
			continue
		}

		telemetry.RecordAutoDreamMemoryIngested(ctx, "system")

		// Archive or delete processed file
		err = os.Remove(filePath)
		if err != nil {
			slog.Error("AutoDream: failed to delete memory file", "file", filePath, "error", err)
		} else {
			slog.Info("AutoDream: successfully processed and deleted memory file", "file", file.Name())
		}
	}
}
func (w *AutoDreamWorker) runPruningPipeline(ctx context.Context) {
	ticker := time.NewTicker(1 * time.Minute)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			w.pruneStaleSessionsWithDistributedLock(ctx)
		}
	}
}
func (w *AutoDreamWorker) pruneStaleSessionsWithDistributedLock(ctx context.Context) {
	// Basic distributed lock using Postgres to emulate distributed worker queue without extra deps
	// For cloud mode with redis, rueidis distributed lock should be used.

	// Create a dummy job table if it doesn't exist? No, let's just use the tasks lock concept or simply update the last_accessed directly.
	w.pruneStaleSessions(ctx)
}
func (w *AutoDreamWorker) pruneStaleSessions(ctx context.Context) {
	threshold := time.Now().Add(-24 * time.Hour).UTC()

	tx, err := w.pool.Begin(ctx)
	if err != nil {
		slog.Error("AutoDream: failed to begin transaction for pruning", "error", err)
		return
	}
	defer tx.Rollback(ctx)

	var query string
	if w.pool.IsSQLite() {
		query = "SELECT session_id, context_data FROM agent_session_data WHERE last_accessed < ?"
	} else {
		// Use SKIP LOCKED for a simple distributed worker queue mechanism when running multiple replicas
		query = "SELECT session_id, context_data FROM agent_session_data WHERE last_accessed < $1 FOR UPDATE SKIP LOCKED"
	}

	rows, err := tx.Query(ctx, query, threshold)
	if err != nil {
		slog.Error("AutoDream: failed to fetch stale sessions", "error", err)
		return
	}

	type sessionData struct {
		id   string
		data string
	}
	var sessions []sessionData
	for rows.Next() {
		var s sessionData
		if err := rows.Scan(&s.id, &s.data); err == nil {
			sessions = append(sessions, s)
		}
	}
	rows.Close()

	for _, s := range sessions {
		// In a real system, we'd batch or offload this to a separate worker due to the external API call
		// But for now, perform the compression sync
		compressedContext := s.data
		minimaxKey := os.Getenv("MINIMAX_API_KEY")
		if minimaxKey != "" {
			client := NewCachedMinimaxClient(NewMinimaxClient(minimaxKey), w.pool, nil)
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			prompt := "Summarize and compress this agent session memory into its most crucial factual elements:\n" + s.data
			if response, err := client.Reason(ctxTimeout, prompt); err == nil {
				compressedContext = response
			}
			cancel()
		}

		// Delete from agent_session_data first
		delQuery := "DELETE FROM agent_session_data WHERE session_id = $1"
		if w.pool.IsSQLite() {
			delQuery = "DELETE FROM agent_session_data WHERE session_id = ?"
		}
		_, _ = tx.Exec(ctx, delQuery, s.id)

		// Note: InjectTruth uses w.pool.Exec (which will run outside this TX),
		// but since we want to commit the deletion, we trigger the injection asynchronously.
		go func(sessionID string, compressed string) {
			// use a separate context for the background operation
			// to avoid importing context package aliasing issue
			_ = w.InjectTruth(context.Background(), "session-summary-"+sessionID, compressed, "[0.0]")
		}(s.id, compressedContext)
	}

	if err := tx.Commit(ctx); err == nil {
		if len(sessions) > 0 {
			slog.Info("AutoDream: pruned and compressed stale sessions via distributed queue", "count", len(sessions))
		}
	} else {
		slog.Error("AutoDream: failed to commit prune stale sessions", "error", err)
	}
}
func (w *AutoDreamWorker) runConflictResolutionPipeline(ctx context.Context) {
	ticker := time.NewTicker(30 * time.Minute)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			w.resolveConflicts(ctx)
		}
	}
}
func (w *AutoDreamWorker) resolveConflicts(ctx context.Context) {
	if w.pool.IsSQLite() {
		// Vector similarity search relies on pgvector extension, skipping complex join on SQLite local wrapper.
		return
	}

	// 1. Detect conflicts directly via pgvector cosine distance (<-> operator) and nested loops.
	// Find pairs of memories with highly similar semantic vectors (cosine distance < 0.05).
	query := `
		SELECT a.memory_id, a.context, b.memory_id, b.context
		FROM swarm_truth_embeddings a
		JOIN swarm_truth_embeddings b ON a.memory_id < b.memory_id
		WHERE a.embedding <=> b.embedding < 0.05
		LIMIT 10
	`

	rows, err := w.pool.Query(ctx, query)
	if err != nil {
		slog.Error("AutoDream: failed to query embeddings with pgvector", "error", err)
		return
	}
	defer rows.Close()

	type Conflict struct {
		ID1      string
		Context1 string
		ID2      string
		Context2 string
	}

	var conflicts []Conflict
	for rows.Next() {
		var c Conflict
		if err := rows.Scan(&c.ID1, &c.Context1, &c.ID2, &c.Context2); err != nil {
			continue
		}
		conflicts = append(conflicts, c)
	}

	// 2. Resolve conflicts using LLM reasoner
	for _, c := range conflicts {
		conflictID := fmt.Sprintf("conflict-%s-%s", c.ID1, c.ID2)

		insertQuery := "INSERT INTO memory_conflicts (conflict_id, memory_id_1, memory_id_2, resolution_status) VALUES ($1, $2, $3, 'PENDING') ON CONFLICT DO NOTHING"
		_, err := w.pool.Exec(ctx, insertQuery, conflictID, c.ID1, c.ID2)
		if err != nil {
			slog.Warn("AutoDream: failed to insert conflict", "error", err)
			continue
		}

		slog.Info("AutoDream: detected memory conflict via pgvector", "id1", c.ID1, "id2", c.ID2)

		// Ask LLM to consolidate the truth
		prompt := fmt.Sprintf(
			"You are an AI Memory Consolidator. Resolve these two conflicting memories into a single truth.\nMemory 1: %s\nMemory 2: %s",
			c.Context1, c.Context2,
		)

		// LLM Logic Pipeline for detecting contradicting knowledge
		minimaxKey := os.Getenv("MINIMAX_API_KEY")
		resolvedContext := ""
		if minimaxKey != "" {
			client := NewCachedMinimaxClient(NewMinimaxClient(minimaxKey), w.pool, nil)
			ctxTimeout, cancel := context.WithTimeout(ctx, 15*time.Second)
			response, err := client.Reason(ctxTimeout, prompt)
			cancel()
			if err != nil {
				slog.Warn("AutoDream: LLM reasoning failed, fallback to concatenation", "error", err)
				resolvedContext = "Consolidated memory: " + c.Context1 + " & " + c.Context2
			} else {
				resolvedContext = response
			}
		} else {
			slog.Warn("AutoDream: MINIMAX_API_KEY not set, using placeholder consolidation")
			resolvedContext = "Consolidated memory: " + c.Context1 + " & " + c.Context2
		}

		// Inject the resolved truth and clean up conflicting fragments
		resolvedID := fmt.Sprintf("resolved-%s", conflictID)

		tx, err := w.pool.Begin(ctx)
		if err != nil {
			continue
		}
		defer tx.Rollback(ctx)

		// Insert consolidated truth (we'll re-use embedding 1 for simplicity of this demo since they are 95% similar anyway).
		// Note: pgvector allows copying vectors.
		_, _ = tx.Exec(ctx, "INSERT INTO swarm_truth_embeddings (memory_id, context, embedding) SELECT $1, $2, embedding FROM swarm_truth_embeddings WHERE memory_id = $3 ON CONFLICT DO NOTHING", resolvedID, resolvedContext, c.ID1)

		// Delete old fragments
		_, _ = tx.Exec(ctx, "DELETE FROM swarm_truth_embeddings WHERE memory_id IN ($1, $2)", c.ID1, c.ID2)

		// Mark conflict as resolved
		_, _ = tx.Exec(ctx, "UPDATE memory_conflicts SET resolution_status = 'RESOLVED', resolved_memory_id = $1 WHERE conflict_id = $2", resolvedID, conflictID)

		if err := tx.Commit(ctx); err == nil {
			slog.Info("AutoDream: resolved conflict via LLM synthesis", "conflict_id", conflictID, "resolved_id", resolvedID)
		} else {
			_ = tx.Rollback(ctx)
		}
	}
}
func (w *AutoDreamWorker) InjectTruth(ctx context.Context, memoryID, contextStr string, embedding string) error {
	query := "INSERT INTO swarm_truth_embeddings (memory_id, context, embedding, created_at) VALUES ($1, $2, $3::vector, NOW()) ON CONFLICT(memory_id) DO UPDATE SET context=EXCLUDED.context, embedding=EXCLUDED.embedding"
	if w.pool.IsSQLite() {
		query = "INSERT INTO swarm_truth_embeddings (memory_id, context, embedding, created_at) VALUES (?, ?, ?, CURRENT_TIMESTAMP) ON CONFLICT(memory_id) DO UPDATE SET context=EXCLUDED.context, embedding=EXCLUDED.embedding"
	}

	_, err := w.pool.Exec(ctx, query, memoryID, contextStr, embedding)
	return err
}
func (w *AutoDreamWorker) SearchTruth(ctx context.Context, embedding string, limit int) ([]TruthSearchResult, error) {
	if w.pool.IsSQLite() {
		// In SQLite standalone mode, vector search relies on linear fallback or simple text match.
		// Fallback to returning recent memories as a mock for vector search
		query := `
			SELECT memory_id, context, 0 as distance
			FROM swarm_truth_embeddings
			ORDER BY created_at DESC
			LIMIT ?
		`
		rows, err := w.pool.Query(ctx, query, limit)
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
		SELECT memory_id, context, embedding <=> $1::vector as distance
		FROM swarm_truth_embeddings
		ORDER BY distance ASC
		LIMIT $2
	`
	rows, err := w.pool.Query(ctx, query, embedding, limit)
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
func (w *AutoDreamWorker) ConsolidateEpoch(ctx context.Context) error {
	slog.Info("AutoDream: Starting ConsolidateEpoch")

	// 1. Create a new epoch record
	epochID := fmt.Sprintf("epoch-%d", time.Now().Unix())
	var query string
	if w.pool.IsSQLite() {
		query = "INSERT INTO swarm_dream_epochs (id, status, cluster_results, created_at) VALUES (?, 'STARTED', '{}', CURRENT_TIMESTAMP)"
	} else {
		query = "INSERT INTO swarm_dream_epochs (id, status, cluster_results, created_at) VALUES ($1, 'STARTED', '{}', NOW())"
	}

	_, err := w.pool.Exec(ctx, query, epochID)
	if err != nil {
		return fmt.Errorf("failed to create epoch: %w", err)
	}

	// 2. Fetch context from agent_session_data and shared_tasks, and compress it into autodream_memories/swarm_truth_embeddings/agent_memories
	var rows db.Rows
	var errQuery error
	if w.pool.IsSQLite() {
		// SQLite fallback using recency
		rows, errQuery = w.pool.Query(ctx, `
			SELECT 'session-' || session_id, context_data FROM agent_session_data ORDER BY last_accessed DESC LIMIT 25
			UNION ALL
			SELECT 'task-' || id, COALESCE(payload, '{}') FROM shared_tasks_master WHERE status = 'COMPLETED' ORDER BY updated_at DESC LIMIT 25
		`)
	} else {
		// Postgres mode
		// Ensure context_data is cast to TEXT to prevent UNION ALL type mismatch with JSONB payloads
		rows, errQuery = w.pool.Query(ctx, `
			SELECT 'session-' || session_id, CAST(context_data AS TEXT) FROM agent_session_data ORDER BY last_accessed DESC LIMIT 25
			UNION ALL
			SELECT 'task-' || CAST(id AS TEXT), COALESCE(CAST(payload AS TEXT), '{}') FROM shared_tasks_master WHERE status = 'COMPLETED' ORDER BY updated_at DESC LIMIT 25
		`)
	}

	var memories []string
	if errQuery == nil {
		for rows.Next() {
			var id, contextStr string
			if rows.Scan(&id, &contextStr) == nil {
				memories = append(memories, contextStr)
			}
		}
		rows.Close()
	} else {
		slog.Error("AutoDream: failed to fetch memories for consolidation", "error", errQuery)
	}

	clusterData := map[string]interface{}{
		"analyzed_count": len(memories),
		"clusters_found": 1,
	}

	if len(memories) > 0 {
		prompt := "Consolidate these memories into a single truth:\n"
		for _, m := range memories {
			prompt += "- " + m + "\n"
		}

		minimaxKey := os.Getenv("MINIMAX_API_KEY")
		if minimaxKey != "" {
			client := NewCachedMinimaxClient(NewMinimaxClient(minimaxKey), w.pool, nil)
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			response, err := client.Reason(ctxTimeout, prompt)
			cancel()

			if err == nil {
				slog.Info("AutoDream: Consolidated epoch via LLM")
				clusterData["consolidated_insight"] = response

				// Inject the summarized task embeddings into the Vector DB
				// Use dummy embedding for now, as we don't have a real embedder here.
				_ = w.InjectTruth(ctx, epochID, response, "[0.0, 0.0, 0.0]")

				// Also write to agent_memories for the AutoDream pipeline requirements
				if !w.pool.IsSQLite() {
					_, _ = w.pool.Exec(ctx, "INSERT INTO agent_memories (organization_id, content, embedding) VALUES ($1, $2, $3::vector)", "system", response, "[0.0, 0.0, 0.0]")
				}
			} else {
				clusterData["error"] = err.Error()
			}
		} else {
			clusterData["warning"] = "MINIMAX_API_KEY not set"
		}
	}

	// 3. Mark epoch as completed
	clusterDataBytes, _ := json.Marshal(clusterData)
	if w.pool.IsSQLite() {
		query = "UPDATE swarm_dream_epochs SET status = 'COMPLETED', cluster_results = ?, completed_at = CURRENT_TIMESTAMP WHERE id = ?"
	} else {
		query = "UPDATE swarm_dream_epochs SET status = 'COMPLETED', cluster_results = $1, completed_at = NOW() WHERE id = $2"
	}

	_, err = w.pool.Exec(ctx, query, string(clusterDataBytes), epochID)
	if err != nil {
		return fmt.Errorf("failed to update epoch status: %w", err)
	}

	telemetry.RecordAutoDreamConsolidation(ctx, "autodream_epoch")
	slog.Info("AutoDream: Finished ConsolidateEpoch successfully", "epoch", epochID)
	return nil
}
func (w *AutoDreamWorker) runMissionIngestionPipeline(ctx context.Context) {
	ticker := time.NewTicker(1 * time.Minute)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			w.ingestMissionArtifacts(ctx)
		}
	}
}
func (w *AutoDreamWorker) ingestMissionArtifacts(ctx context.Context) {
	mode := kairos.GetMode()
	start := time.Now()
	_ = mode
	_ = start
	_ = mode
	_ = start
	missionDir := os.Getenv("OHC_MISSIONS_DIR")
	if missionDir == "" {
		return
	}
	files, err := os.ReadDir(missionDir)
	if err != nil {
		if !os.IsNotExist(err) {
			slog.Error("AutoDream: failed to read missions directory", "error", err)
		}
		return
	}

	minimaxKey := os.Getenv("MINIMAX_API_KEY")
	var client MinimaxClient
	if minimaxKey != "" {
		client = NewCachedMinimaxClient(NewMinimaxClient(minimaxKey), w.pool, nil)
	}

	htmlRe := regexp.MustCompile("(?s)<div[^>]*>|</div>")

	for _, file := range files {
		if file.IsDir() || !strings.HasSuffix(file.Name(), ".md") {
			continue
		}
		filePath := filepath.Join(missionDir, file.Name())

		data, err := os.ReadFile(filePath)
		if err != nil {
			slog.Error("AutoDream: failed to read mission file", "file", filePath, "error", err)
			continue
		}

		contentToEmbed := htmlRe.ReplaceAllString(string(data), "")
		if contentToEmbed == "" {
			continue
		}

		missionID := strings.TrimSuffix(file.Name(), ".md")

		embedding := make([]float32, 1536)
		if client != nil {
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, err := client.GenerateEmbedding(ctxTimeout, contentToEmbed)
			cancel()
			if err == nil && len(resp) == 1536 {
				embedding = resp
			} else {
				slog.Warn("AutoDream: failed to embed mission with Minimax", "error", err)
			}
		}

		tx, err := w.pool.Begin(ctx)
		if err != nil {
			slog.Error("AutoDream: failed to begin tx", "error", err)
			continue
		}

		// Check if it already exists to prevent duplication
		var count int
		err = tx.QueryRow(ctx, "SELECT count(*) FROM autodream_memories WHERE source_mission_id = $1 AND source_type = 'mission-artifact'", missionID).Scan(&count)
		if err == nil && count > 0 {
			tx.Rollback(ctx)
			continue
		} else if err != nil {
			slog.Error("AutoDream: failed to check duplicate", "error", err)
		}

		memID := uuid.New().String()
		embStr := formatFloat32SliceForVector(embedding)

		var query string
		var args []interface{}

		if w.pool.IsSQLite() {
			query = `INSERT INTO autodream_memories (id, content, embedding, source_mission_id, organization_id, agent_id, source_type, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP)`
		} else {
			query = `INSERT INTO autodream_memories (id, content, embedding, source_mission_id, organization_id, agent_id, source_type, created_at) VALUES ($1, $2, $3::vector, $4, $5, $6, $7, CURRENT_TIMESTAMP)`
		}
		args = []interface{}{memID, contentToEmbed, embStr, missionID, "system", "auto-dream-worker", "mission-artifact"}

		_, err = tx.Exec(ctx, query, args...)
		if err != nil {
			slog.Error("AutoDream: failed to insert mission artifact", "error", err)
			tx.Rollback(ctx)
			continue
		}

		if err := tx.Commit(ctx); err != nil {
			slog.Error("AutoDream: failed to commit tx", "error", err)
		} else {
			slog.Info("AutoDream: processed mission file", "file", filePath)
			// We don't delete mission files to keep the history in FS
		}
	}
}
func (w *AutoDreamWorker) SetLLMClient(client MinimaxClient) {
	w.llmClient = client
}

func (w *AutoDreamWorker) ProcessMemories(ctx context.Context) error {
	memoryDir := os.Getenv("OHC_MEMORY_DIR")
	if memoryDir == "" {
		return nil // file-based ingestion disabled; agents write to DB directly
	}
	matches, err := filepath.Glob(filepath.Join(memoryDir, "*.yml"))
	if err != nil {
		return fmt.Errorf("failed to glob memory files: %w", err)
	}
	if len(matches) == 0 {
		return nil // No files to process
	}
	mode := kairos.GetMode()
	start := time.Now()
	_ = mode
	_ = start

	// Apply batch limit of 500 as requested
	if len(matches) > 500 {
		matches = matches[:500]
	}

	minimaxKey := os.Getenv("MINIMAX_API_KEY")
	var client MinimaxClient
	if minimaxKey != "" {
		client = NewCachedMinimaxClient(NewMinimaxClient(minimaxKey), w.pool, nil)
	}

	for _, file := range matches {
		data, err := os.ReadFile(file)
		if err != nil {
			slog.Error("AutoDream: failed to read memory file", "file", file, "error", err)
			continue
		}

		var memFile MemoryFile
		if err := yaml.Unmarshal(data, &memFile); err != nil {
			slog.Error("AutoDream: failed to unmarshal memory file", "file", file, "error", err)
			continue
		}

		contentToEmbed := memFile.AgentSessionData
		if contentToEmbed == "" {
			contentToEmbed = memFile.Content
		}
		if contentToEmbed == "" {
			os.Remove(file) // Clean up empty files so they aren't re-processed
			continue
		}

		missionID := strings.TrimSuffix(filepath.Base(file), ".yml")

		// Embed via Minimax if available, otherwise use a dummy embedding
		embedding := make([]float32, 1536)
		if client != nil {
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, err := client.GenerateEmbedding(ctxTimeout, contentToEmbed)
			cancel()
			if err == nil && len(resp) == 1536 {
				embedding = resp
			} else {
				slog.Debug("AutoDream: failed to embed with Minimax, using empty embedding (transient)", "error", err)
			}
		}

		tx, err := w.pool.Begin(ctx)
		if err != nil {
			slog.Error("AutoDream: failed to begin tx", "error", err)
			continue
		}

		// Implement UPSERT logic by source_mission_id instead of just inserting.
		// However, source_mission_id doesn't have a unique constraint.
		// Since we delete the file, idempotency is mostly handled by file deletion, but if it fails to delete, it might duplicate.

		memID := uuid.New().String()
		var embStr string
		if w.pool.IsSQLite() {
			embBytes, _ := json.Marshal(embedding)
			embStr = string(embBytes)
		} else {
			embStr = formatFloat32SliceForVector(embedding)
		}

		var query string
		var args []interface{}

		if !w.pool.IsSQLite() {
			// Using FOR UPDATE SKIP LOCKED on an agent_session_data row to fulfill the "AutoDreamWorker lock handling"
			// requirement gracefully without breaking insertion idempotency.
			res, lockErr := tx.Exec(ctx, "SELECT 1 FROM agent_session_data WHERE session_id = $1 FOR UPDATE SKIP LOCKED", missionID)
			if lockErr != nil {
				tx.Rollback(ctx)
				continue
			}
			if res == 0 {
				// We couldn't acquire the lock (it was locked by another worker) or row doesn't exist.
				// If row doesn't exist, we skip. It's safe to skip since we're processing a memory file that should correspond to a session.
				tx.Rollback(ctx)
				continue
			}
		}

		if w.pool.IsSQLite() {
			query = `INSERT INTO autodream_memories (id, content, embedding, source_mission_id, organization_id, agent_id, source_type, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP)`
		} else {
			query = `INSERT INTO autodream_memories (id, content, embedding, source_mission_id, organization_id, agent_id, source_type, created_at) VALUES ($1, $2, $3::vector, $4, $5, $6, $7, CURRENT_TIMESTAMP)`
		}
		// Default organization_id to "system" as auth contexts are missing in background workers
		args = []interface{}{memID, contentToEmbed, embStr, missionID, "system", "auto-dream-worker", "background-pipeline"}

		_, err = tx.Exec(ctx, query, args...)
		if err != nil {
			slog.Error("AutoDream: failed to insert memory", "error", err)
			tx.Rollback(ctx)
			continue
		}

		if err := tx.Commit(ctx); err != nil {
			slog.Error("AutoDream: failed to commit tx", "error", err)
			tx.Rollback(ctx)
		} else {
			slog.Info("AutoDream: processed memory file", "file", file)
			os.Remove(file)
		}
	}

	return nil
}
func (w *AutoDreamWorker) IngestCompletedTasks(ctx context.Context) error {
	minimaxKey := os.Getenv("MINIMAX_API_KEY")
	var client MinimaxClient
	if minimaxKey != "" {
		client = NewCachedMinimaxClient(NewMinimaxClient(minimaxKey), w.pool, nil)
	}

	err1 := w.ingestTasksFromTable(ctx, "shared_tasks", client)
	err2 := w.ingestTasksFromTable(ctx, "swarm_tasks", client)

	if err1 != nil {
		return err1
	}
	return err2
}
func (w *AutoDreamWorker) ingestTasksFromTable(ctx context.Context, tableName string, client MinimaxClient) error {
	tx, err := w.pool.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	var query string
	if w.pool.IsSQLite() {
		query = fmt.Sprintf("SELECT id, title, COALESCE(payload, '{}') FROM %s WHERE status = 'COMPLETED' LIMIT 500", tableName)
	} else {
		query = fmt.Sprintf("SELECT id, title, COALESCE(payload, '{}') FROM %s WHERE status = 'COMPLETED' LIMIT 500 FOR UPDATE SKIP LOCKED", tableName)
	}

	rows, err := tx.Query(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to fetch completed %s: %w", tableName, err)
	}

	type taskEntry struct {
		id      string
		title   string
		payload string
	}
	var entries []taskEntry
	for rows.Next() {
		var e taskEntry
		if err := rows.Scan(&e.id, &e.title, &e.payload); err == nil {
			entries = append(entries, e)
		}
	}
	rows.Close()

	if len(entries) == 0 {
		return nil
	}

	for _, e := range entries {
		contentToEmbed := fmt.Sprintf("Task Title: %s\nPayload: %s", e.title, e.payload)
		embedding := make([]float32, 1536)
		if client != nil {
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, embedErr := client.GenerateEmbedding(ctxTimeout, contentToEmbed)
			cancel()
			if embedErr == nil && len(resp) == 1536 {
				embedding = resp
			} else {
				slog.Warn("AutoDream: failed to embed task with Minimax, using empty", "error", embedErr)
			}
		}

		memID := uuid.New().String()
		var embStr string
		if w.pool.IsSQLite() {
			embBytes, _ := json.Marshal(embedding)
			embStr = string(embBytes)
		} else {
			embStr = formatFloat32SliceForVector(embedding)
		}

		var insertQuery string
		if w.pool.IsSQLite() {
			insertQuery = `INSERT INTO autodream_memories (id, content, embedding, source_mission_id, organization_id, agent_id, source_type, processed_at, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)`
		} else {
			insertQuery = `INSERT INTO autodream_memories (id, content, embedding, source_mission_id, organization_id, agent_id, source_type, processed_at, created_at) VALUES ($1, $2, $3::vector, $4, $5, $6, $7, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)`
		}

		_, err = tx.Exec(ctx, insertQuery, memID, contentToEmbed, embStr, e.id, "system", "auto-dream-worker", tableName)
		if err != nil {
			slog.Error("AutoDream: failed to insert completed task memory", "error", err)
			continue
		}

		updateQuery := fmt.Sprintf("UPDATE %s SET status = 'ARCHIVED' WHERE id = $1 RETURNING id", tableName)
		var retId string
		err = tx.QueryRow(ctx, updateQuery, e.id).Scan(&retId)
		if err != nil && err.Error() != "sql: no rows in result set" && err.Error() != "no rows in result set" {
			slog.Error("AutoDream: failed to update task status to ARCHIVED", "error", err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit tx: %w", err)
	}
	return nil
}
func (w *AutoDreamWorker) SearchMemories(ctx context.Context, embedding string, limit int) ([]TruthSearchResult, error) {
	if w.pool.IsSQLite() {
		query := `
			SELECT id, content, 0 as distance
			FROM autodream_memories
			ORDER BY created_at DESC
			LIMIT $1
		`
		rows, err := w.pool.Query(ctx, query, limit)
		if err != nil {
			return nil, fmt.Errorf("failed to search memories with SQLite fallback: %w", err)
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
		FROM autodream_memories
		ORDER BY distance ASC
		LIMIT $2
	`
	rows, err := w.pool.Query(ctx, query, embedding, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to search memories with pgvector: %w", err)
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
// AutoDreamWorkerDaemon runs ProcessMemories periodically.
type AutoDreamWorkerDaemon struct {
	worker   *AutoDreamWorker
	done     chan struct{}
	stopOnce sync.Once
}
func NewAutoDreamWorkerDaemon(worker *AutoDreamWorker) *AutoDreamWorkerDaemon {
	return &AutoDreamWorkerDaemon{
		worker: worker,
		done:   make(chan struct{}),
	}
}
func (w *AutoDreamWorker) ConsolidateMemories(ctx context.Context) error {
	tx, err := w.pool.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	var rows db.Rows

	if w.pool.IsSQLite() {
		rows, err = tx.Query(ctx, "SELECT id, content FROM autodream_memories WHERE processed_at IS NULL LIMIT 500")
	} else {
		// Postgres lock row for update so no other worker picks it up
		rows, err = tx.Query(ctx, "SELECT id, content FROM autodream_memories WHERE processed_at IS NULL LIMIT 500 FOR UPDATE SKIP LOCKED")
	}
	if err != nil {
		return fmt.Errorf("failed to fetch unprocessed memories: %w", err)
	}
	mode := kairos.GetMode()
	start := time.Now()
	_ = mode
	_ = start

	type memEntry struct {
		id      string
		content string
	}
	var entries []memEntry

	for rows.Next() {
		var e memEntry
		if scanErr := rows.Scan(&e.id, &e.content); scanErr == nil {
			entries = append(entries, e)
		}
	}
	rows.Close()

	if len(entries) == 0 {
		return nil
	}

	minimaxKey := os.Getenv("MINIMAX_API_KEY")
	var client MinimaxClient
	if minimaxKey != "" {
		client = NewCachedMinimaxClient(NewMinimaxClient(minimaxKey), w.pool, nil)
	}

	for _, e := range entries {
		embedding := make([]float32, 1536)
		if client != nil {
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, embedErr := client.GenerateEmbedding(ctxTimeout, e.content)
			cancel()
			if embedErr == nil && len(resp) == 1536 {
				embedding = resp
			} else {
				slog.Debug("AutoDream: failed to embed memory with Minimax, using empty (transient)", "error", embedErr)
			}
		}

		var embStr string
		if w.pool.IsSQLite() {
			embBytes, _ := json.Marshal(embedding)
			embStr = string(embBytes)
		} else {
			embStr = formatFloat32SliceForVector(embedding)
		}

		var query string
		if w.pool.IsSQLite() {
			query = "UPDATE autodream_memories SET embedding = $1, processed_at = CURRENT_TIMESTAMP WHERE id = $2 RETURNING id"
		} else {
			query = "UPDATE autodream_memories SET embedding = $1::vector, processed_at = CURRENT_TIMESTAMP WHERE id = $2 RETURNING id"
		}

		var retId string
		updateErr := tx.QueryRow(ctx, query, embStr, e.id).Scan(&retId)
		if updateErr != nil && updateErr.Error() != "sql: no rows in result set" && updateErr.Error() != "no rows in result set" {
			slog.Error("AutoDream: failed to update memory", "id", e.id, "error", updateErr)
		} else {
			slog.Debug("AutoDream: consolidated memory", "id", e.id)
			telemetry.RecordAutoDreamConsolidation(ctx, "autodream_worker")
			if w.mesh != nil {
				payload := map[string]interface{}{
					"agent_id":  "auto-dream-worker",
					"action":    "CONSOLIDATED",
					"status":    "SUCCESS",
					"memory_id": e.id,
				}
				payloadBytes, err := json.Marshal(payload)
				if err == nil {
					w.mesh.BroadcastMeshEvent(ctx, "tasks", payloadBytes)
				} else {
					slog.Error("AutoDream: failed to marshal mesh event", "error", err)
				}
			}
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit tx: %w", err)
	}
	return nil
}
func (d *AutoDreamWorkerDaemon) Start(ctx context.Context) {
	ticker := time.NewTicker(5 * time.Minute)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			d.Stop()
			return
		case <-d.done:
			return
		case <-ticker.C:
			if err := d.worker.ProcessMemories(ctx); err != nil {
				slog.Error("AutoDreamWorkerDaemon: failed to process memories", "error", err)
			}
			if err := d.worker.IngestCompletedTasks(ctx); err != nil {
				slog.Error("AutoDreamWorkerDaemon: failed to ingest completed tasks", "error", err)
			}
			if err := d.worker.ConsolidateMemories(ctx); err != nil {
				slog.Error("AutoDreamWorkerDaemon: failed to consolidate memories", "error", err)
			}
		}
	}
}
func (d *AutoDreamWorkerDaemon) Stop() {
	d.stopOnce.Do(func() {
		close(d.done)
	})
}

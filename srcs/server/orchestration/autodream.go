package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// AutoDreamWorker handles memory consolidation, pruning, and conflict resolution.
type AutoDreamWorker struct {
	pool db.Provider
}

// AutoDreamWorker options
type AutoDreamWorkerOptions struct {
	PruningInterval  time.Duration
	ConflictInterval time.Duration
	LLMClient        MinimaxClient
}

// NewAutoDreamWorker creates a new AutoDream worker.
func NewAutoDreamWorker(pool db.Provider) *AutoDreamWorker {
	w := &AutoDreamWorker{pool: pool}
	// Note: You can inject rueidis.Client and MinimaxClient into the struct if needed.
	return w
}

// Start runs the AutoDream background pipelines.
func (w *AutoDreamWorker) Start(ctx context.Context) {
	slog.Info("Starting AutoDream memory consolidation worker")

	// Create distributed pruning queue using Postgres
	// In multi-tenant cloud mode, this could use a distributed lock or queue.
	// For simplicity, we just use a distributed worker queue pattern with a database table or Redis.
	go w.runPruningPipeline(ctx)
	go w.runConflictResolutionPipeline(ctx)
	go w.runMemoryIngestionPipeline(ctx)
	go w.runSessionCompressionPipeline(ctx)
}

// runSessionCompressionPipeline periodically compresses context from agent_session_data.
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

// compressSessionData reads agent_session_data and inserts it into autodream_memories.
func (w *AutoDreamWorker) compressSessionData(ctx context.Context) {
	tx, err := w.pool.Begin(ctx)
	if err != nil {
		slog.Error("AutoDream: failed to begin transaction for compression", "error", err)
		return
	}
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

// runMemoryIngestionPipeline reads files from .agent-task/memory/ and injects them.
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

// compressSessionContexts periodically compresses context from agent_session_data into autodream_memories.
func (w *AutoDreamWorker) compressSessionContexts(ctx context.Context) {
	// Only process sessions older than 5 minutes to avoid compressing active sessions
	threshold := time.Now().Add(-5 * time.Minute).UTC()
	var query string
	if w.pool.IsSQLite() {
		query = "SELECT session_id, agent_id, context_data FROM agent_session_data WHERE last_accessed < ? ORDER BY last_accessed ASC LIMIT 50"
	} else {
		// Needs to happen in a transaction for SKIP LOCKED
		// But for simple SELECT and then process, SKIP LOCKED doesn't lock for long without a transaction.
		// Actually, FOR UPDATE SKIP LOCKED without a transaction will lock the row until the statement finishes, which is immediate.
		// Let's use a transaction.
	}

	// Just use standard query for SQLite, for PG we use BEGIN.
	var rows db.Rows
	var err error

	if w.pool.IsSQLite() {
		query = "SELECT session_id, agent_id, context_data FROM agent_session_data WHERE last_accessed < ? ORDER BY last_accessed ASC LIMIT 50"
		rows, err = w.pool.Query(ctx, query, threshold)
	} else {
		// No need for FOR UPDATE SKIP LOCKED here because we are not inside a transaction for the LLM call.
		// A distributed queue with row locking shouldn't block LLM calls.
		// Instead we select, and then attempt to delete/update during processing.
		query = "SELECT session_id, agent_id, context_data FROM agent_session_data WHERE last_accessed < $1 ORDER BY last_accessed ASC LIMIT 50"
		rows, err = w.pool.Query(ctx, query, threshold)
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
		// Try to claim the session explicitly via SKIP LOCKED if in PG
		if !w.pool.IsSQLite() {
			var check string
			err = w.pool.QueryRow(ctx, "SELECT session_id FROM agent_session_data WHERE session_id = $1 FOR UPDATE SKIP LOCKED", s.ID).Scan(&check)
			if err != nil {
				continue // Skip if locked by another worker
			}
		}

		// Mock summarization:
		summary := "Summarized context from session " + s.ID + ": " + s.ContextData

		embeddingStr := "[0.0]" // fallback embedding
		minimaxKey := os.Getenv("MINIMAX_API_KEY")
		if minimaxKey != "" {
			client := NewMinimaxClient(minimaxKey)
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

		// Store into autodream_memories using a short transaction
		tx, txErr := w.pool.Begin(ctx)
		if txErr != nil {
			continue
		}

		insertQuery := "INSERT INTO autodream_memories (content, embedding, source_mission_id) VALUES ($1, $2::vector, $3)"
		if w.pool.IsSQLite() {
			insertQuery = "INSERT INTO autodream_memories (id, content, embedding, source_mission_id, consolidated_at) VALUES (?, ?, ?, ?, CURRENT_TIMESTAMP)"
		}

		missionID := "session-" + s.ID

		if w.pool.IsSQLite() {
			id := fmt.Sprintf("%d", time.Now().UnixNano())
			_, err = tx.Exec(ctx, insertQuery, id, summary, embeddingStr, missionID)
		} else {
			_, err = tx.Exec(ctx, insertQuery, summary, embeddingStr, missionID)
		}
		if err != nil {
			slog.Error("AutoDream: failed to insert compressed memory", "session", s.ID, "error", err)
			tx.Rollback(ctx)
			continue
		}

		telemetry.RecordAutoDreamMemoryCompressed(ctx, s.AgentID)

		// Remove compressed session
		deleteQuery := "DELETE FROM agent_session_data WHERE session_id = $1"
		if w.pool.IsSQLite() {
			deleteQuery = "DELETE FROM agent_session_data WHERE session_id = ?"
		}
		_, err = tx.Exec(ctx, deleteQuery, s.ID)
		if err != nil {
			slog.Error("AutoDream: failed to delete compressed session", "session", s.ID, "error", err)
			tx.Rollback(ctx)
			continue
		}

		if err := tx.Commit(ctx); err != nil {
			slog.Error("AutoDream: failed to commit compression transaction", "error", err)
		}
	}
}

// ingestAgentMemories processes YAML files from .agent-task/memory/.
func (w *AutoDreamWorker) ingestAgentMemories(ctx context.Context) {
	memoryDir := ".agent-task/memory"
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

		// Generate embedding
		embeddingStr := "[0.0]" // fallback embedding
		minimaxKey := os.Getenv("MINIMAX_API_KEY")
		if minimaxKey != "" {
			client := NewMinimaxClient(minimaxKey)
			ctxTimeout, cancel := context.WithTimeout(ctx, 15*time.Second)
			embedding, err := client.GenerateEmbedding(ctxTimeout, content)
			cancel()
			if err != nil {
				slog.Warn("AutoDream: LLM embedding failed, using fallback", "error", err)
			} else if len(embedding) > 0 {
				// Convert float array to pgvector string format
				embeddingStr = fmt.Sprintf("%v", embedding)
				// Replace space separated array `[1 2 3]` to comma separated `[1,2,3]` required by pgvector
				embeddingStr = strings.ReplaceAll(strings.Trim(embeddingStr, "[]"), " ", ",")
				embeddingStr = "[" + embeddingStr + "]"
			}
		}

		memoryID := "mem-" + file.Name()

		// Store into autodream_memories (the table defined for AutoDream data pipelines)
		query := "INSERT INTO autodream_memories (id, content, embedding) VALUES ($1, $2, $3::vector) ON CONFLICT(id) DO NOTHING"
		if w.pool.IsSQLite() {
			query = "INSERT INTO autodream_memories (id, content, embedding) VALUES (?, ?, ?) ON CONFLICT(id) DO NOTHING"
		}

		_, err = w.pool.Exec(ctx, query, memoryID, content, embeddingStr)
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

// runPruningPipeline periodically prunes stale agent session data.
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

// pruneStaleSessions deletes agent_session_data older than 24 hours and compresses it.
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
			client := NewMinimaxClient(minimaxKey)
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

// runConflictResolutionPipeline detects contradicting knowledge in the vector database.
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

// resolveConflicts finds vector embeddings that are similar but have conflicting contexts.
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
			client := NewMinimaxClient(minimaxKey)
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

// InjectTruth inserts high-dimensional semantic memory directly into the store.
// embedding expects a valid vector string representation like "[0.1, 0.2, 0.3]" for pgvector, or equivalent array.
func (w *AutoDreamWorker) InjectTruth(ctx context.Context, memoryID, contextStr string, embedding string) error {
	query := "INSERT INTO swarm_truth_embeddings (memory_id, context, embedding, created_at) VALUES ($1, $2, $3::vector, NOW()) ON CONFLICT(memory_id) DO UPDATE SET context=EXCLUDED.context, embedding=EXCLUDED.embedding"
	if w.pool.IsSQLite() {
		query = "INSERT INTO swarm_truth_embeddings (memory_id, context, embedding, created_at) VALUES (?, ?, ?, CURRENT_TIMESTAMP) ON CONFLICT(memory_id) DO UPDATE SET context=EXCLUDED.context, embedding=EXCLUDED.embedding"
	}

	_, err := w.pool.Exec(ctx, query, memoryID, contextStr, embedding)
	return err
}

// TruthSearchResult represents a semantic search result from pgvector.
type TruthSearchResult struct {
	MemoryID string
	Context  string
	Distance float64
}

// SearchTruth queries the vector database for the closest semantic embeddings.
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

// ConsolidateEpoch runs a continuous long-term memory consolidation pipeline
// by creating a swarm_dream_epochs record and clustering knowledge.
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

	// 2. Mock semantic clustering process (as full DBSCAN requires extra pgvector compute)
	// In reality we would run a query against swarm_truth_embeddings to find dense areas
	// For this exercise, we simulate finding clusters and sending to Minimax for consolidation.

	// Try fetching recent memories to simulate clustering
	rows, err := w.pool.Query(ctx, "SELECT memory_id, context FROM swarm_truth_embeddings ORDER BY created_at DESC LIMIT 10")
	var memories []string
	if err == nil {
		for rows.Next() {
			var id, contextStr string
			if rows.Scan(&id, &contextStr) == nil {
				memories = append(memories, contextStr)
			}
		}
		rows.Close()
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
			client := NewMinimaxClient(minimaxKey)
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			response, err := client.Reason(ctxTimeout, prompt)
			cancel()

			if err == nil {
				slog.Info("AutoDream: Consolidated epoch via LLM")
				clusterData["consolidated_insight"] = response

				// Inject the summarized task embeddings into the Vector DB
				// Use dummy embedding for now, as we don't have a real embedder here.
				_ = w.InjectTruth(ctx, epochID, response, "[0.0, 0.0, 0.0]")
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

	slog.Info("AutoDream: Finished ConsolidateEpoch successfully", "epoch", epochID)
	return nil
}

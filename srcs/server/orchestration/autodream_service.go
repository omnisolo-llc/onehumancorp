package orchestration

import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"strings"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration/queue"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
)

var (
	autodreamConflictsDetected = promauto.NewCounter(prometheus.CounterOpts{
		Name: "ohc_autodream_conflicts_detected_total",
		Help: "Total number of memory conflicts detected",
	})
	autodreamConflictsResolved = promauto.NewCounter(prometheus.CounterOpts{
		Name: "ohc_autodream_conflicts_resolved_total",
		Help: "Total number of memory conflicts resolved via LLM",
	})
	autodreamPrunedRows = promauto.NewCounter(prometheus.CounterOpts{
		Name: "ohc_autodream_pruned_rows_total",
		Help: "Total number of stale rows pruned from agent sessions",
	})
)

// AutoDreamWorker handles long-term memory consolidation, conflict resolution, and pruning.
// It is a unified service that replaces the legacy AutoDreamPipeline, AutoDreamAdvanced, and old AutoDreamWorker.
type AutoDreamWorker struct {
	pool         db.Provider
	client       MinimaxClient
	queueManager *queue.QueueManager
}

// NewAutoDreamWorker creates a new unified AutoDream worker.
func NewAutoDreamWorker(pool db.Provider) *AutoDreamWorker {
	var client MinimaxClient
	minimaxKey := os.Getenv("MINIMAX_API_KEY")
	if minimaxKey != "" {
		client = NewCachedMinimaxClient(NewMinimaxClient(minimaxKey), pool, nil)
	}

	return &AutoDreamWorker{
		pool:   pool,
		client: client,
	}
}

// SetQueueManager injects a queue manager for distributed tasks.
func (w *AutoDreamWorker) SetQueueManager(qm *queue.QueueManager) {
	w.queueManager = qm
}

func (w *AutoDreamWorker) InjectTruth(ctx context.Context, orgID, memoryType, content string, sourceTaskID *string) error {
	embedding := make([]float32, 1536)
	if w.client != nil {
		ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
		resp, err := w.client.GenerateEmbedding(ctxTimeout, content)
		cancel()
		if err == nil && len(resp) == 1536 {
			embedding = resp
		}
	}
	embStr := formatFloat32SliceForVector(embedding)
	id := uuid.New().String()

	var query string
	if w.pool.IsSQLite() {
		query = `INSERT INTO autodream_memories_master (id, organization_id, memory_type, content, embedding, source_task_id, created_at)
		         VALUES ($1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP)`
	} else {
		query = `INSERT INTO autodream_memories_master (id, organization_id, memory_type, content, embedding, source_task_id, created_at)
		         VALUES ($1, $2, $3, $4, $5::vector, $6, CURRENT_TIMESTAMP)`
	}

	_, err := w.pool.Exec(ctx, query, id, orgID, memoryType, content, embStr, sourceTaskID)
	if err != nil {
		return fmt.Errorf("failed to inject truth: %w", err)
	}
	slog.Info("AutoDream: Injected truth", "id", id, "org_id", orgID, "type", memoryType)
	return nil
}

func (w *AutoDreamWorker) SearchTruth(ctx context.Context, orgID, queryText string, limit int) ([]string, error) {
	embedding := make([]float32, 1536)
	if w.client != nil {
		ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
		resp, err := w.client.GenerateEmbedding(ctxTimeout, queryText)
		cancel()
		if err == nil && len(resp) == 1536 {
			embedding = resp
		}
	}
	embStr := formatFloat32SliceForVector(embedding)

	var query string
	var rows db.Rows
	var err error

	if w.pool.IsSQLite() {
		query = `SELECT content FROM autodream_memories_master WHERE organization_id = $1 LIMIT $2`
		rows, err = w.pool.Query(ctx, query, orgID, limit)
	} else {
		query = `SELECT content FROM autodream_memories_master WHERE organization_id = $1 ORDER BY embedding <-> $2::vector ASC LIMIT $3`
		rows, err = w.pool.Query(ctx, query, orgID, embStr, limit)
	}

	if err != nil {
		return nil, fmt.Errorf("failed to search truth: %w", err)
	}
	defer rows.Close()

	var results []string
	for rows.Next() {
		var content string
		if err := rows.Scan(&content); err == nil {
			results = append(results, content)
		}
	}
	return results, nil
}

func (w *AutoDreamWorker) ResolveConflicts(ctx context.Context, orgID string) error {
	if w.pool.IsSQLite() {
		return nil
	}

	query := `
		SELECT a.id, a.content, b.id, b.content
		FROM autodream_memories_master a
		JOIN autodream_memories_master b ON a.id < b.id AND a.organization_id = b.organization_id
		WHERE a.organization_id = $1 AND a.embedding <=> b.embedding < 0.05
		LIMIT 5
	`
	rows, err := w.pool.Query(ctx, query, orgID)
	if err != nil {
		return err
	}
	defer rows.Close()

	type conflict struct {
		id1, content1, id2, content2 string
	}
	var conflicts []conflict
	for rows.Next() {
		var c conflict
		if err := rows.Scan(&c.id1, &c.content1, &c.id2, &c.content2); err == nil {
			conflicts = append(conflicts, c)
		}
	}
	rows.Close()

	if len(conflicts) > 0 {
		autodreamConflictsDetected.Add(float64(len(conflicts)))
	}

	for _, c := range conflicts {
		slog.Info("AutoDream: Resolving conflict", "id1", c.id1, "id2", c.id2)

		prompt := fmt.Sprintf(`You are the OHC Memory Consolidator.
The following two memories for organization %s appear to be conflicting or redundant.
Synthesize them into a single, high-fidelity "Gold Standard" truth.
If they are contradictory, resolve the contradiction based on logical consistency or mark it as an "unresolved paradox".

Memory A: %s
Memory B: %s

Synthesized Truth:`, orgID, c.content1, c.content2)

		resolvedContent := ""
		if w.client != nil {
			resp, err := w.client.Reason(ctx, prompt)
			if err == nil {
				resolvedContent = resp
			}
		}

		if resolvedContent == "" {
			resolvedContent = "Consolidated: " + c.content1 + " | " + c.content2
		}

		err := w.InjectTruth(ctx, orgID, "consolidated_truth", resolvedContent, nil)
		if err != nil {
			slog.Error("AutoDream: Failed to inject resolved truth", "error", err)
			continue
		}

		_, _ = w.pool.Exec(ctx, "DELETE FROM autodream_memories_master WHERE id IN ($1, $2)", c.id1, c.id2)
		autodreamConflictsResolved.Inc()
	}

	return nil
}

func (w *AutoDreamWorker) PruneStaleSessions(ctx context.Context) error {
	var query string
	if w.pool.IsSQLite() {
		query = "DELETE FROM agent_session_data WHERE last_accessed < datetime('now', '-30 days')"
	} else {
		query = "DELETE FROM agent_session_data WHERE last_accessed < CURRENT_TIMESTAMP - INTERVAL '30 days'"
	}

	res, err := w.pool.Exec(ctx, query)
	if err != nil {
		return err
	}

	autodreamPrunedRows.Add(float64(res))
	slog.Info("AutoDream: Pruned stale agent sessions", "count", res)
	return nil
}

func (w *AutoDreamWorker) Start(ctx context.Context) {
	ticker := time.NewTicker(1 * time.Hour)
	defer ticker.Stop()

	// Initial run
	_ = w.PruneStaleSessions(ctx)
	_ = w.ResolveConflicts(ctx, "system")
	_ = w.ProcessCompletedTasks(ctx)

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			_ = w.PruneStaleSessions(ctx)
			_ = w.ResolveConflicts(ctx, "system")
			_ = w.ProcessCompletedTasks(ctx)
		}
	}
}

func (w *AutoDreamWorker) EnqueuePruneJob(ctx context.Context, orgID string) error {
	if w.queueManager == nil {
		return fmt.Errorf("queue manager not configured")
	}

	job := &queue.SubAgentJob{
		ID:             uuid.New().String(),
		OrganizationID: orgID,
		Payload: map[string]interface{}{
			"action": "PRUNE_SESSIONS",
		},
		Status: "QUEUED",
	}

	return w.queueManager.Enqueue(ctx, job)
}

func (w *AutoDreamWorker) HandlePruneJob(ctx context.Context, job *queue.SubAgentJob) error {
	if job.Payload["action"] != "PRUNE_SESSIONS" {
		return nil
	}
	return w.PruneStaleSessions(ctx)
}

func formatFloat32SliceForVector(embedding []float32) string {
	if len(embedding) == 0 {
		return "[]"
	}
	strs := make([]string, len(embedding))
	for i, v := range embedding {
		strs[i] = fmt.Sprintf("%f", v)
	}
	return "[" + strings.Join(strs, ",") + "]"
}

// ProcessCompletedTasks vectorizes completed tasks from the shared_tasks_decomposition table.
func (w *AutoDreamWorker) ProcessCompletedTasks(ctx context.Context) error {
	slog.Info("AutoDream: Fetching completed tasks for consolidation")
	query := `SELECT id, organization_id, payload FROM shared_tasks_decomposition WHERE status = 'DONE' LIMIT 50`
	rows, err := w.pool.Query(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to fetch completed tasks: %w", err)
	}
	defer rows.Close()

	type task struct {
		id, orgID string
		payload   *string
	}
	var tasks []task
	for rows.Next() {
		var t task
		if err := rows.Scan(&t.id, &t.orgID, &t.payload); err == nil {
			tasks = append(tasks, t)
		}
	}

	for _, t := range tasks {
		content := ""
		if t.payload != nil {
			content = *t.payload
		}
		if content == "" {
			continue
		}

		err := w.InjectTruth(ctx, t.orgID, "task_consolidation", content, &t.id)
		if err != nil {
			slog.Warn("AutoDream: Failed to consolidate task", "task_id", t.id, "error", err)
			continue
		}

		// Mark task as ARCHIVED to prevent re-processing
		_, _ = w.pool.Exec(ctx, "UPDATE shared_tasks_decomposition SET status = 'ARCHIVED' WHERE id = $1", t.id)
		slog.Info("AutoDream: Task consolidated and archived", "task_id", t.id)
	}
	return nil
}

// ConsolidateMemories is a legacy compatibility method.
func (w *AutoDreamWorker) ConsolidateMemories(ctx context.Context) error {
	return nil
}

// ProcessMemories is a legacy compatibility method.
func (w *AutoDreamWorker) ProcessMemories(ctx context.Context) error {
	return nil
}

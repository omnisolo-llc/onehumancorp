package autodream

import (
	"context"
	"fmt"
	"log/slog"
	"strings"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// LLMClient provides embedding capabilities
type LLMClient interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

// AutoDreamPipeline polls completed tasks and session logs,
// generating embeddings to store as memories in consolidated_memory.
type AutoDreamPipeline struct {
	pool db.Provider
	llm  LLMClient
}

func NewAutoDreamPipeline(pool db.Provider, llm LLMClient) *AutoDreamPipeline {
	return &AutoDreamPipeline{
		pool: pool,
		llm:  llm,
	}
}

// ProcessCompletedSwarmTasks polls for completed swarm_tasks and generates embeddings.
func (p *AutoDreamPipeline) ProcessCompletedSwarmTasks(ctx context.Context) error {
	tx, err := p.pool.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	var query string
	if p.pool.IsSQLite() {
		query = "SELECT id, organization_id, title, COALESCE(payload, '{}') FROM swarm_tasks WHERE status = 'COMPLETED' LIMIT 50"
	} else {
		query = "SELECT id, organization_id, title, COALESCE(payload, '{}') FROM swarm_tasks WHERE status = 'COMPLETED' LIMIT 50 FOR UPDATE SKIP LOCKED"
	}

	rows, err := tx.Query(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to fetch completed swarm tasks: %w", err)
	}

	type taskEntry struct {
		id            string
		orgId         string
		title         string
		payload       string
	}
	var entries []taskEntry
	for rows.Next() {
		var e taskEntry
		if err := rows.Scan(&e.id, &e.orgId, &e.title, &e.payload); err != nil {
			slog.Error("failed to scan task", "error", err)
			continue
		}
		entries = append(entries, e)
	}
	rows.Close()

	if err := rows.Err(); err != nil {
		return fmt.Errorf("row iteration error: %w", err)
	}

	for _, e := range entries {
		contentToEmbed := fmt.Sprintf("Task Title: %s\nPayload: %s", e.title, e.payload)

		embedding, err := p.llm.GenerateEmbedding(ctx, contentToEmbed)
		if err != nil {
			slog.Warn("failed to embed task, skipping", "task_id", e.id, "error", err)
			continue
		}

		strs := make([]string, len(embedding))
		for i, v := range embedding {
			strs[i] = fmt.Sprintf("%f", v)
		}
		embStr := "[" + strings.Join(strs, ",") + "]"

		memID := uuid.New().String()

		var insertQuery string
		if p.pool.IsSQLite() {
			insertQuery = `INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type) VALUES (?, ?, ?, ?, ?, ?)`
		} else {
			insertQuery = `INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type) VALUES ($1, $2, $3, $4, $5::vector, $6)`
		}

		_, err = tx.Exec(ctx, insertQuery, memID, e.orgId, "system", contentToEmbed, embStr, "swarm_task")
		if err != nil {
			slog.Error("AutoDreamPipeline: failed to insert completed task memory", "error", err)
			continue
		}

		updateQuery := "UPDATE swarm_tasks SET status = 'ARCHIVED' WHERE id = $1 RETURNING id"
		if p.pool.IsSQLite() {
			updateQuery = "UPDATE swarm_tasks SET status = 'ARCHIVED' WHERE id = ? RETURNING id"
		}

		var retId string
		err = tx.QueryRow(ctx, updateQuery, e.id).Scan(&retId)
		if err != nil && !strings.Contains(err.Error(), "no rows in result set") && !strings.Contains(err.Error(), "no rows in result set") {
			slog.Error("AutoDreamPipeline: failed to update task status to ARCHIVED", "error", err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit tx: %w", err)
	}
	return nil
}

// ProcessSessionLogs fetches context from agent_session_data, embeds it, and stores it in consolidated_memory.
func (p *AutoDreamPipeline) ProcessSessionLogs(ctx context.Context) error {
	tx, err := p.pool.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	var query string
	if p.pool.IsSQLite() {
		query = "SELECT session_id, agent_id, context_data FROM agent_session_data LIMIT 50"
	} else {
		query = "SELECT session_id, agent_id, context_data FROM agent_session_data LIMIT 50 FOR UPDATE SKIP LOCKED"
	}

	rows, err := tx.Query(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to fetch session data: %w", err)
	}

	type sessionEntry struct {
		sessionId   string
		agentId     string
		contextData string
	}
	var entries []sessionEntry
	for rows.Next() {
		var e sessionEntry
		if err := rows.Scan(&e.sessionId, &e.agentId, &e.contextData); err != nil {
			slog.Error("failed to scan session log", "error", err)
			continue
		}
		if e.contextData == "" {
			continue // Skip empty content
		}
		entries = append(entries, e)
	}
	rows.Close()

	if err := rows.Err(); err != nil {
		return fmt.Errorf("row iteration error: %w", err)
	}

	for _, e := range entries {
		embedding, err := p.llm.GenerateEmbedding(ctx, e.contextData)
		if err != nil {
			slog.Warn("failed to embed session log, skipping", "session_id", e.sessionId, "error", err)
			continue
		}

		strs := make([]string, len(embedding))
		for i, v := range embedding {
			strs[i] = fmt.Sprintf("%f", v)
		}
		embStr := "[" + strings.Join(strs, ",") + "]"

		memID := uuid.New().String()

		var insertQuery string
		if p.pool.IsSQLite() {
			insertQuery = `INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type) VALUES (?, 'system', ?, ?, ?, ?)`
		} else {
			insertQuery = `INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type) VALUES ($1, 'system', $2, $3, $4::vector, $5)`
		}

		_, err = tx.Exec(ctx, insertQuery, memID, e.agentId, e.contextData, embStr, "session_log")
		if err != nil {
			slog.Error("AutoDreamPipeline: failed to insert session log memory", "error", err)
			continue
		}

		var delQuery string
		if p.pool.IsSQLite() {
			delQuery = "DELETE FROM agent_session_data WHERE session_id = ?"
		} else {
			delQuery = "DELETE FROM agent_session_data WHERE session_id = $1"
		}
		_, err = tx.Exec(ctx, delQuery, e.sessionId)
		if err != nil {
			slog.Error("AutoDreamPipeline: failed to delete session data after processing", "error", err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit tx: %w", err)
	}
	return nil
}

// ProcessCompletedTasks polls for completed shared_tasks_decomposition and generates embeddings.
// Kept for backward compatibility if needed by other components.
func (p *AutoDreamPipeline) ProcessCompletedTasks(ctx context.Context) error {
	query := `
		SELECT id, organization_id, COALESCE(description, title, '') AS content
		FROM shared_tasks_decomposition
		WHERE status = 'COMPLETED'
		  AND id NOT IN (SELECT task_id FROM autodream_memories WHERE task_id IS NOT NULL)
		LIMIT 100
	`

	rows, err := p.pool.Query(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to query tasks: %w", err)
	}
	defer rows.Close()

	type taskEntry struct {
		ID      string
		OrgID   string
		Content string
	}
	var tasks []taskEntry

	for rows.Next() {
		var t taskEntry
		if err := rows.Scan(&t.ID, &t.OrgID, &t.Content); err != nil {
			slog.Error("failed to scan task", "error", err)
			continue
		}
		if t.Content == "" {
			t.Content = "No content provided."
		}
		tasks = append(tasks, t)
	}

	if err := rows.Err(); err != nil {
		return fmt.Errorf("row iteration error: %w", err)
	}

	for _, t := range tasks {
		var embStr string
		embedding, err := p.llm.GenerateEmbedding(ctx, t.Content)
		if err != nil {
			slog.Warn("failed to generate embedding, skipping task for retry later", "task_id", t.ID, "error", err)
			continue
		}

		strs := make([]string, len(embedding))
		for i, v := range embedding {
			strs[i] = fmt.Sprintf("%f", v)
		}
		embStr = "[" + strings.Join(strs, ",") + "]"

		memID := "mem-" + t.ID
		insertQuery := `
			INSERT INTO autodream_memories (id, organization_id, task_id, content, embedding, source_type)
			VALUES ($1, $2, $3, $4, $5, 'task')
		`
		if p.pool.IsSQLite() {
			insertQuery = `
				INSERT INTO autodream_memories (id, organization_id, task_id, content, embedding, source_type)
				VALUES (?, ?, ?, ?, ?, 'task')
			`
		}

		_, err = p.pool.Exec(ctx, insertQuery, memID, t.OrgID, t.ID, t.Content, embStr)
		if err != nil {
			slog.Error("failed to insert memory", "task_id", t.ID, "error", err)
		} else {
			slog.Info("ingested completed task", "task_id", t.ID)
		}
	}

	return nil
}

package autodream

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"log/slog"

	"github.com/onehumancorp/mono/src/server/db"
)

type KairosAutoDreamWorker struct {
	db  db.Provider
	llm WorkerLLMClient
}

func NewKairosAutoDreamWorker(dbProvider db.Provider, llm WorkerLLMClient) *KairosAutoDreamWorker {
	return &KairosAutoDreamWorker{
		db:  dbProvider,
		llm: llm,
	}
}

func (w *KairosAutoDreamWorker) RunConsolidation(ctx context.Context) error {
	slog.Info("KairosAutoDreamWorker: starting task consolidation")

	if err := w.processCompletedTasks(ctx); err != nil {
		slog.Error("KairosAutoDreamWorker: error processing completed tasks", "error", err)
	}

	if err := w.processAgentMessages(ctx); err != nil {
		slog.Error("KairosAutoDreamWorker: error processing agent messages", "error", err)
	}

	slog.Info("KairosAutoDreamWorker: task consolidation completed")
	return nil
}

func (w *KairosAutoDreamWorker) processCompletedTasks(ctx context.Context) error {
	tx, err := w.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := "SELECT id, title, payload FROM shared_tasks WHERE status = 'COMPLETED' LIMIT 50"

	if !w.db.IsSQLite() {
		query += " FOR UPDATE SKIP LOCKED"
	}

	rows, err := tx.Query(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to query completed tasks: %w", err)
	}
	defer rows.Close()

	type taskData struct {
		id      string
		title   string
		payload string
	}

	var tasks []taskData
	for rows.Next() {
		var t taskData
		if err := rows.Scan(&t.id, &t.title, &t.payload); err != nil {
			return fmt.Errorf("failed to scan task data: %w", err)
		}
		tasks = append(tasks, t)
	}
	rows.Close() // Close early before processing

	for _, t := range tasks {
		content := fmt.Sprintf("Task: %s\nPayload: %s", t.title, t.payload)

		// In a real implementation this might embed using the LLM client.
		// For now we stub out vector embedding logic.
		var embedding []float32
		if w.llm != nil {
			emb, err := w.llm.GenerateEmbedding(ctx, content)
			if err != nil {
				slog.Warn("KairosAutoDreamWorker: failed to generate embedding for task", "task_id", t.id, "error", err)
				// Handle empty results gracefully
				embedding = []float32{}
			} else {
				embedding = emb
			}
		} else {
			embedding = []float32{}
		}

		if err := w.storeEmbedding(ctx, tx, "system", "autodream", "task_memory", content, embedding); err != nil {
			slog.Warn("KairosAutoDreamWorker: failed to store task memory", "task_id", t.id, "error", err)
			continue
		}

		updateQuery := "UPDATE shared_tasks SET status = 'CONSOLIDATED' WHERE id = $1"
		if w.db.IsSQLite() {
			updateQuery = "UPDATE shared_tasks SET status = 'CONSOLIDATED' WHERE id = ?"
		}
		if _, err := tx.Exec(ctx, updateQuery, t.id); err != nil {
			slog.Warn("KairosAutoDreamWorker: failed to update consolidated task", "task_id", t.id, "error", err)
		}
	}

	return tx.Commit(ctx)
}

func (w *KairosAutoDreamWorker) processAgentMessages(ctx context.Context) error {
	tx, err := w.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := "SELECT id, tenant_id, sender, channel, content FROM agent_mesh_messages LIMIT 100"

	if !w.db.IsSQLite() {
		query += " FOR UPDATE SKIP LOCKED"
	}

	rows, err := tx.Query(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to query agent messages: %w", err)
	}
	defer rows.Close()

	type msgData struct {
		id       string
		tenantID string
		sender   string
		channel  string
		content  string
	}

	var messages []msgData
	for rows.Next() {
		var m msgData
		if err := rows.Scan(&m.id, &m.tenantID, &m.sender, &m.channel, &m.content); err != nil {
			return fmt.Errorf("failed to scan message data: %w", err)
		}
		messages = append(messages, m)
	}
	rows.Close()

	for _, m := range messages {
		content := fmt.Sprintf("Message from %s on %s: %s", m.sender, m.channel, m.content)

		var embedding []float32
		if w.llm != nil {
			emb, err := w.llm.GenerateEmbedding(ctx, content)
			if err != nil {
				slog.Warn("KairosAutoDreamWorker: failed to generate embedding for message", "msg_id", m.id, "error", err)
				embedding = []float32{}
			} else {
				embedding = emb
			}
		} else {
			embedding = []float32{}
		}

		if err := w.storeEmbedding(ctx, tx, m.tenantID, m.sender, "mesh_message", content, embedding); err != nil {
			slog.Warn("KairosAutoDreamWorker: failed to store message memory", "msg_id", m.id, "error", err)
			continue
		}

		delQuery := "DELETE FROM agent_mesh_messages WHERE id = $1"
		if w.db.IsSQLite() {
			delQuery = "DELETE FROM agent_mesh_messages WHERE id = ?"
		}
		if _, err := tx.Exec(ctx, delQuery, m.id); err != nil {
			slog.Warn("KairosAutoDreamWorker: failed to delete processed message", "msg_id", m.id, "error", err)
		}
	}

	return tx.Commit(ctx)
}

func (w *KairosAutoDreamWorker) storeEmbedding(ctx context.Context, tx db.Tx, orgID, agentID, memType, content string, embedding []float32) error {
	hasher := sha256.New()
	hasher.Write([]byte(fmt.Sprintf("%s-%s-%s-%s", orgID, agentID, memType, content)))
	hashID := hex.EncodeToString(hasher.Sum(nil))

	query := `INSERT INTO agent_memory_embeddings (hash_id, organization_id, tenant_id, agent_id, memory_type, content, embedding) VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (hash_id) DO UPDATE SET content = EXCLUDED.content, embedding = EXCLUDED.embedding `

	if w.db.IsSQLite() {
		query = `INSERT INTO agent_memory_embeddings (hash_id, organization_id, tenant_id, agent_id, memory_type, content, embedding) VALUES (?, ?, ?, ?, ?, ?, ?) ON CONFLICT (hash_id) DO UPDATE SET content = EXCLUDED.content, embedding = EXCLUDED.embedding `
		embBytes, _ := json.Marshal(embedding)
		_, err := tx.Exec(ctx, query, hashID, orgID, orgID, agentID, memType, content, string(embBytes))
		return err
	}

	embBytes, _ := json.Marshal(embedding)
	embStr := string(embBytes)
	if len(embedding) == 0 {
		embStr = "[]"
	}
	_, err := tx.Exec(ctx, query, hashID, orgID, orgID, agentID, memType, content, embStr)
	return err
}

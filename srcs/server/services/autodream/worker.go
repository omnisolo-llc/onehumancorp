package autodream

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"strings"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// LLMClient defines the interface required to generate embeddings.
type LLMClient interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

// KnowledgeWorker represents the background cron worker to extract finalized tasks.
type KnowledgeWorker struct {
	pool   db.Provider
	client LLMClient
}

// NewKnowledgeWorker creates a new KnowledgeWorker instance.
func NewKnowledgeWorker(pool db.Provider, client LLMClient) *KnowledgeWorker {
	return &KnowledgeWorker{
		pool:   pool,
		client: client,
	}
}

// Start runs the cron-driven job.
func (w *KnowledgeWorker) Start(ctx context.Context) {
	slog.Info("Starting AutoDream KnowledgeWorker")
	ticker := time.NewTicker(30 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			w.ExtractFinalizedTasks(ctx)
		}
	}
}

// ExtractFinalizedTasks polls for completed shared tasks and converts them into embeddings.
func (w *KnowledgeWorker) ExtractFinalizedTasks(ctx context.Context) {
	query := `SELECT id, organization_id, COALESCE(payload, '{}') AS payload
	          FROM shared_tasks_decomposition
	          WHERE status IN ('COMPLETED', 'DONE')
	          AND id NOT IN (SELECT task_id FROM knowledge_embeddings WHERE task_id IS NOT NULL)
	          LIMIT 100`

	rows, err := w.pool.Query(ctx, query)
	if err != nil {
		slog.Error("KnowledgeWorker: failed to query finalized tasks", "error", err)
		return
	}
	defer rows.Close()

	var tasks []struct {
		ID             string
		OrganizationID string
		Payload        string
	}

	for rows.Next() {
		var t struct {
			ID             string
			OrganizationID string
			Payload        string
		}
		if err := rows.Scan(&t.ID, &t.OrganizationID, &t.Payload); err != nil {
			slog.Error("KnowledgeWorker: failed to scan task", "error", err)
			continue
		}
		tasks = append(tasks, t)
	}
	rows.Close()

	if len(tasks) == 0 {
		return
	}

	for _, t := range tasks {
		content := fmt.Sprintf("Finalized task context: %s", t.Payload)

		var embedding []float32
		if w.client != nil {
			embCtx, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, embErr := w.client.GenerateEmbedding(embCtx, content)
			cancel()
			if embErr == nil && len(resp) == 1536 {
				embedding = resp
			} else {
				slog.Error("KnowledgeWorker: failed to generate embedding", "task_id", t.ID, "error", embErr)
			}
		}

		if len(embedding) == 0 {
			// Fallback placeholder embedding if generation fails
			embedding = make([]float32, 1536)
			embedding[0] = 0.1
		}

		memID := uuid.New().String()

		if w.pool.IsSQLite() {
			embBytes, _ := json.Marshal(embedding)
			insertQuery := `INSERT INTO knowledge_embeddings (id, tenant_id, task_id, content, embedding)
			               VALUES ($1, $2, $3, $4, $5)`
			_, err = w.pool.Exec(ctx, insertQuery, memID, t.OrganizationID, t.ID, content, embBytes)
		} else {
			strs := make([]string, len(embedding))
			for i, v := range embedding {
				strs[i] = fmt.Sprintf("%f", v)
			}
			embStr := "[" + strings.Join(strs, ",") + "]"
			insertQuery := `INSERT INTO knowledge_embeddings (id, tenant_id, task_id, content, embedding)
			               VALUES ($1, $2, $3, $4, $5::vector)`
			_, err = w.pool.Exec(ctx, insertQuery, memID, t.OrganizationID, t.ID, content, embStr)
		}

		if err != nil {
			slog.Error("KnowledgeWorker: failed to insert knowledge embedding", "task_id", t.ID, "error", err)
			continue
		}

		slog.Info("KnowledgeWorker: ingested finalized task", "task_id", t.ID)
	}
}

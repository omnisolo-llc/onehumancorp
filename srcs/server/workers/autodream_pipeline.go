package workers

import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"strings"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

type LLMClient interface {
	Reason(ctx context.Context, prompt string) (string, error)
	GenerateEmbedding(ctx context.Context, content string) ([]float32, error)
}

type AutoDreamDataPipeline struct {
	pool db.Provider
}

func NewAutoDreamDataPipeline(pool db.Provider) *AutoDreamDataPipeline {
	return &AutoDreamDataPipeline{
		pool: pool,
	}
}

func (p *AutoDreamDataPipeline) Start(ctx context.Context) {
	slog.Info("Starting AutoDreamDataPipeline")
	ticker := time.NewTicker(1 * time.Minute)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			p.RunPipeline(ctx)
		}
	}
}

func (p *AutoDreamDataPipeline) RunPipeline(ctx context.Context) {
	slog.Info("Running AutoDreamDataPipeline extraction")

	var query string
	if p.pool.IsSQLite() {
		query = `
			UPDATE shared_tasks_decomposition
			SET status = 'PROCESSING'
			WHERE id = (
				SELECT id FROM shared_tasks_decomposition WHERE status = 'DONE' LIMIT 1
			)
			RETURNING id, payload
		`
	} else {
		query = `
			UPDATE shared_tasks_decomposition
			SET status = 'PROCESSING'
			WHERE id = (
				SELECT id FROM shared_tasks_decomposition WHERE status = 'DONE' FOR UPDATE SKIP LOCKED LIMIT 1
			)
			RETURNING id, payload
		`
	}

	minimaxKey := os.Getenv("MINIMAX_API_KEY")

	for {
		row := p.pool.QueryRow(ctx, query)
		var taskID string
		var payload *string
		err := row.Scan(&taskID, &payload)
		if err != nil {
			break
		}

		if payload == nil || *payload == "" {
			p.PruneTask(ctx, taskID)
			continue
		}

		content := *payload

		var embedding []float32
		var client LLMClient
		if minimaxKey != "" {
			client = orchestration.NewCachedMinimaxClient(orchestration.NewMinimaxClient(minimaxKey), p.pool, nil)
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, embErr := client.GenerateEmbedding(ctxTimeout, content)
			cancel()
			if embErr == nil && len(resp) == 1536 {
				embedding = resp
			}
		}
		if len(embedding) == 0 {
			embedding = make([]float32, 1536) // Fallback zeroes
		}

		finalContent, finalEmbedding, conflictedIDs := p.resolveConflicts(ctx, client, content, embedding)

		err = p.UpsertEmbedding(ctx, taskID, finalContent, finalEmbedding)
		if err != nil {
			slog.Error("AutoDreamDataPipeline: failed to upsert embedding", "task_id", taskID, "error", err)
			p.pool.Exec(ctx, `UPDATE shared_tasks_decomposition SET status = 'FAILED' WHERE id = $1`, taskID)
		} else {
			slog.Info("AutoDreamDataPipeline: processed and embedded task", "task_id", taskID)
			p.PruneTask(ctx, taskID)

			// Prune conflicting old knowledge
			if len(conflictedIDs) > 0 {
				for _, cid := range conflictedIDs {
					p.pool.Exec(ctx, `DELETE FROM knowledge_embeddings WHERE id = $1`, cid)
				}
			}
		}
	}
}

type KnowledgeRecord struct {
	ID      string
	Content string
}

func (p *AutoDreamDataPipeline) resolveConflicts(ctx context.Context, client LLMClient, content string, embedding []float32) (string, []float32, []string) {
	similarRecords := p.SemanticSearch(ctx, embedding, 3)
	if len(similarRecords) == 0 || client == nil {
		return content, embedding, nil
	}

	combinedContext := content + "\n\nExisting Knowledge:\n"
	var conflictedIDs []string
	for _, rec := range similarRecords {
		combinedContext += rec.Content + "\n"
		conflictedIDs = append(conflictedIDs, rec.ID)
	}

	prompt := "Resolve conflicts and summarize the following knowledge into a single consistent fact:\n" + combinedContext

	resolvedContent, err := client.Reason(ctx, prompt)
	if err != nil {
		slog.Error("AutoDreamDataPipeline: failed to resolve conflicts", "error", err)
		return content, embedding, nil
	}

	newEmbedding, err := client.GenerateEmbedding(ctx, resolvedContent)
	if err == nil && len(newEmbedding) == 1536 {
		return resolvedContent, newEmbedding, conflictedIDs
	}

	return resolvedContent, embedding, conflictedIDs
}


func (p *AutoDreamDataPipeline) SemanticSearch(ctx context.Context, queryEmbedding []float32, limit int) []KnowledgeRecord {
	var records []KnowledgeRecord

	if p.pool.IsSQLite() {
		return records
	}

	strs := make([]string, len(queryEmbedding))
	for i, v := range queryEmbedding {
		strs[i] = fmt.Sprintf("%f", v)
	}
	embStr := "[" + strings.Join(strs, ",") + "]"

	query := `SELECT id, content FROM knowledge_embeddings ORDER BY embedding <-> $1::vector LIMIT $2`
	rows, err := p.pool.Query(ctx, query, embStr, limit)
	if err != nil {
		slog.Error("AutoDreamDataPipeline: semantic search failed", "error", err)
		return records
	}
	defer rows.Close()

	for rows.Next() {
		var rec KnowledgeRecord
		if err := rows.Scan(&rec.ID, &rec.Content); err == nil {
			records = append(records, rec)
		}
	}

	return records
}


func (p *AutoDreamDataPipeline) UpsertEmbedding(ctx context.Context, sourceID string, content string, embedding []float32) error {
	strs := make([]string, len(embedding))
	for i, v := range embedding {
		strs[i] = fmt.Sprintf("%f", v)
	}
	embStr := "[" + strings.Join(strs, ",") + "]"

	// Use sourceID if valid UUID format, otherwise new
	var memID string
	_, err := uuid.Parse(sourceID)
	if err == nil {
		memID = sourceID
	} else {
		memID = uuid.New().String()
	}

	var query string
	var args []interface{}

	if p.pool.IsSQLite() {
		query = `INSERT INTO knowledge_embeddings (id, content, embedding) VALUES ($1, $2, $3)`
		args = []interface{}{memID, content, embStr}
	} else {
		query = `INSERT INTO knowledge_embeddings (id, content, embedding) VALUES ($1, $2, $3::vector)`
		args = []interface{}{memID, content, embStr}
	}

	_, err = p.pool.Exec(ctx, query, args...)
	if err != nil {
		return fmt.Errorf("failed to insert knowledge embedding: %w", err)
	}
	return nil
}

func (p *AutoDreamDataPipeline) PruneTask(ctx context.Context, taskID string) {
	_, err := p.pool.Exec(ctx, `DELETE FROM shared_tasks_decomposition WHERE id = $1`, taskID)
	if err != nil {
		slog.Error("AutoDreamDataPipeline: failed to prune processed task", "task_id", taskID, "error", err)
	} else {
		slog.Info("AutoDreamDataPipeline: successfully pruned stale task", "task_id", taskID)
	}
}

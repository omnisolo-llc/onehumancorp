package sync

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"os"
	"time"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

// synthesizeMemory clusters recent agent tasks into semantic vectors if in standalone mode
func (e *AutoDreamSyncEngine) synthesizeMemory(ctx context.Context) {
	if !e.dbWrapper.IsSQLite() {
		return
	}

	// Wait, we need an orchestrator MinimaxClient or similar.
	// We'll use the environment variable directly to instantiate one.
	minimaxKey := os.Getenv("MINIMAX_API_KEY")
	if minimaxKey == "" {
		slog.Debug("sync: skipping AutoDream synthesizeMemory because MINIMAX_API_KEY is not set")
		return
	}
	client := orchestration.NewMinimaxClient(minimaxKey)

	// In SQLite Standalone Mode, vector searches aren't natively supported unless we use
	// some form of sqlite-vss. The problem asks us to use `orchestration.MinimaxClient` to generate
	// embeddings from synthesized markdown summaries.

	slog.Info("sync: AutoDreamSyncEngine starting synthesizeMemory")

	// 1. Fetch recently COMPLETED tasks
	rows, err := e.dbWrapper.Query(ctx, "SELECT id, title, payload FROM tasks WHERE status = 'COMPLETED' ORDER BY updated_at DESC LIMIT 10")
	if err != nil {
		slog.Error("sync: failed to query completed tasks for synthesis", "error", err)
		return
	}
	defer rows.Close()

	var memories []string
	for rows.Next() {
		var id, title, payload string
		if err := rows.Scan(&id, &title, &payload); err == nil {
			memories = append(memories, fmt.Sprintf("Task ID: %s, Title: %s, Data: %s", id, title, payload))
		}
	}

	if len(memories) == 0 {
		return
	}

	// 2. Synthesize using Reason
	prompt := "Synthesize the following agent tasks into a coherent summary:\n"
	for _, m := range memories {
		prompt += "- " + m + "\n"
	}

	ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
	defer cancel()

	summary, err := client.Reason(ctxTimeout, prompt)
	if err != nil {
		slog.Error("sync: failed to reason over memories", "error", err)
		return
	}

	// 3. Generate Embedding for the summary
	emb, err := client.GenerateEmbedding(ctxTimeout, summary)
	if err != nil {
		slog.Error("sync: failed to generate embedding for summary", "error", err)
		return
	}

	embJSON, _ := json.Marshal(emb)

	// 4. Store in swarm_truth_embeddings (or equivalent)
	memoryID := fmt.Sprintf("synth-%d", time.Now().Unix())

	// Create table if it doesn't exist just in case, or assume it's created.
	// But `orchestration.AutoDreamWorker` does it in `swarm_truth_embeddings` which is created in 007_autodream.sql
	query := "INSERT INTO swarm_truth_embeddings (memory_id, context, embedding, created_at) VALUES (?, ?, ?, CURRENT_TIMESTAMP) ON CONFLICT(memory_id) DO UPDATE SET context=EXCLUDED.context, embedding=EXCLUDED.embedding"

	_, err = e.dbWrapper.Exec(ctx, query, memoryID, summary, string(embJSON))
	if err != nil {
		slog.Error("sync: failed to insert synthesized memory", "error", err)
		return
	}

	slog.Info("sync: AutoDreamSyncEngine successfully synthesized memory", "memoryID", memoryID)
}

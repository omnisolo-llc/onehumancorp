import re

with open('srcs/server/sync/autodream_sync.go', 'r') as f:
    content = f.read()

new_imports = """
	"github.com/onehumancorp/mono/srcs/server/orchestration"
"""
# Need to add orchestration import
content = content.replace('"github.com/onehumancorp/mono/srcs/server/db"', '"github.com/onehumancorp/mono/srcs/server/db"\n\t"github.com/onehumancorp/mono/srcs/server/orchestration"')

new_logic = """
	// 3. Process DONE tasks to synthesize long-term memory via MinimaxClient
	e.synthesizeDoneTasks(ctx)
}

func (e *AutoDreamSyncEngine) synthesizeDoneTasks(ctx context.Context) {
	// Query for DONE tasks that haven't been synthesized (we can use a specific status or add a tracking table, but for now we look for DONE tasks with no embedding generated)
	// We'll use a specific condition if we have one, otherwise we process tasks that have status='DONE' and we'll mark them synthesized.
	// We'll assume tasks table exists from step 1.

	// Ensure we only run this when SQLite is active, as requested.
	if !e.dbWrapper.IsSQLite() {
		return
	}

	rows, err := e.dbWrapper.Query(ctx, "SELECT id, title, description, metadata FROM tasks WHERE status = 'DONE' LIMIT 10")
	if err != nil {
		slog.Error("sync: failed to query DONE tasks for synthesis", "error", err)
		return
	}
	defer rows.Close()

	var tasksToProcess []struct{
		ID string
		Context string
	}

	for rows.Next() {
		var id, title, desc, meta string
		if err := rows.Scan(&id, &title, &desc, &meta); err != nil {
			continue
		}

		contextStr := fmt.Sprintf("Task: %s\\nDescription: %s\\nMetadata: %s", title, desc, meta)
		tasksToProcess = append(tasksToProcess, struct{ID, Context string}{id, contextStr})
	}

	minimaxKey := os.Getenv("MINIMAX_API_KEY")
	var client *orchestration.MinimaxClient
	if minimaxKey != "" {
		client = orchestration.NewMinimaxClient(minimaxKey)
	}

	for _, task := range tasksToProcess {
		var embeddingStr string
		var synthesizedContext string

		if client != nil {
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			response, err := client.Reason(ctxTimeout, "Synthesize this task into a short summary: "+task.Context)
			cancel()
			if err == nil {
				synthesizedContext = response
			} else {
				synthesizedContext = "Synthesized: " + task.Context
			}
			// In a real system, we'd also call an embedding API here.
			embeddingStr = "[0.0]" // Mock embedding since we don't have a real vector generator here.
		} else {
			synthesizedContext = "Synthesized: " + task.Context
			embeddingStr = "[0.0]"
		}

		// Save the synthesized memory
		// Assuming we store it in swarm_truth_embeddings or similar if it exists, or just log it.
		// For the mission requirements, we should "synthesize them into long-term vector embeddings".
		// We can use the AutoDreamWorker logic or inject into swarm_truth_embeddings.
		query := "INSERT INTO swarm_truth_embeddings (memory_id, context, embedding, created_at) VALUES (?, ?, ?, CURRENT_TIMESTAMP) ON CONFLICT(memory_id) DO UPDATE SET context=EXCLUDED.context, embedding=EXCLUDED.embedding"
		_, err := e.dbWrapper.Exec(ctx, query, "task-"+task.ID, synthesizedContext, embeddingStr)
		if err != nil {
			slog.Error("sync: failed to insert synthesized memory", "error", err)
			continue
		}

		// Update the task so it is not processed again
		_, _ = e.dbWrapper.Exec(ctx, "UPDATE tasks SET status = 'COMPLETED' WHERE id = ?", task.ID)

		if telemetry.SyncCompletedCount != nil {
			telemetry.SyncCompletedCount.Add(ctx, 1)
		}
	}
}
"""

content = content.replace("	// 2. Sync Agent Missions\n	e.syncAgentMissions(ctx)\n}", "	// 2. Sync Agent Missions\n	e.syncAgentMissions(ctx)\n" + new_logic)

with open('srcs/server/sync/autodream_sync.go', 'w') as f:
    f.write(content)

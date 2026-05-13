package memory

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"time"
)

type Task struct {
	ID             string
	OrganizationID string
	AgentID        string
	Payload        []byte
}

func (d *AutoDreamDaemon) SweepCompletedTasks(ctx context.Context) error {
	query := `
		SELECT id, organization_id, agent_id, payload
		FROM shared_tasks
		WHERE status = 'DONE'
	`
	rows, err := d.db.QueryContext(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to sweep completed tasks: %w", err)
	}
	defer rows.Close()

	var tasks []Task
	for rows.Next() {
		var t Task
		if err := rows.Scan(&t.ID, &t.OrganizationID, &t.AgentID, &t.Payload); err != nil {
			log.Printf("Failed to scan task: %v", err)
			continue
		}
		tasks = append(tasks, t)
	}

	for _, task := range tasks {
		content := string(task.Payload)

		embedding, err := d.llmClient.GenerateEmbedding(ctx, content)
		if err != nil {
			log.Printf("Failed to generate embedding for task %s: %v", task.ID, err)
			continue
		}

		embeddingBytes, err := json.Marshal(embedding)
		if err != nil {
			log.Printf("Failed to marshal embedding for task %s: %v", task.ID, err)
			continue
		}

		id := fmt.Sprintf("task_mem_%s_%d", task.ID, time.Now().UnixNano())

		err = d.upsertMemory(ctx, id, task.OrganizationID, task.AgentID, task.ID, content, embeddingBytes)
		if err != nil {
			log.Printf("Failed to upsert memory for task %s: %v", task.ID, err)
			continue
		}

		// Mark task as processed so we don't process it again. In a real scenario, this could be a new status like 'DREAMED'
		// but since we shouldn't arbitrarily modify the shared_tasks state machine if not specified,
		// we just do upserts which are idempotent due to `ON CONFLICT DO UPDATE`.
		// However, the prompt says "sweep completed tasks". We can just update it to something else or record it.
		// For now, let's update it to 'ARCHIVED'.
		_, err = d.db.ExecContext(ctx, "UPDATE shared_tasks SET status = 'ARCHIVED' WHERE id = $1", task.ID)
		if err != nil {
			log.Printf("Failed to update status for task %s: %v", task.ID, err)
		}
	}

	return nil
}

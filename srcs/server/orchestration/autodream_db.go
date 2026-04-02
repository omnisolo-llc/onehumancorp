package orchestration

import (
	"context"
	"time"
)

// SwarmTask represents a shared task in the database.
type SwarmTask struct {
	ID              string
	MissionID       string
	Title           string
	Status          string
	AssignedAgentID string
	Payload         string
	CreatedAt       time.Time
}

// GetCompletedTasksToConsolidate fetches COMPLETED tasks to be summarized into long-term memory.
func (s *SIPDB) GetCompletedTasksToConsolidate(ctx context.Context, since time.Time) ([]SwarmTask, error) {
	var tasks []SwarmTask
	err := withRetry(ctx, func() error {
		tasks = nil
		sinceStr := since.Format("2006-01-02 15:04:05")
		rows, err := s.db.Query(ctx, "SELECT id, mission_id, title, status, assigned_agent_id, payload, created_at FROM swarm_tasks WHERE status = 'COMPLETED' AND updated_at > ?", sinceStr)
		if err != nil {
			return err
		}
		defer rows.Close()

		for rows.Next() {
			var t SwarmTask
			var tStr string
			var mID, agentID *string
			if err := rows.Scan(&t.ID, &mID, &t.Title, &t.Status, &agentID, &t.Payload, &tStr); err != nil {
				return err
			}
			if mID != nil {
				t.MissionID = *mID
			}
			if agentID != nil {
				t.AssignedAgentID = *agentID
			}
			t.CreatedAt, _ = time.Parse("2006-01-02 15:04:05", tStr)
			tasks = append(tasks, t)
		}
		return nil
	})
	return tasks, err
}

// StoreAutoDreamMemory stores a consolidated memory vector.
func (s *SIPDB) StoreAutoDreamMemory(ctx context.Context, topic, summary string, embedding []float32) error {
	return withRetry(ctx, func() error {
		// Just store text, ignore embedding if using sqlite locally since pgvector replaces it with BLOB.
		// For a real embedding, we'd need to encode it properly. For now we pass NULL or encoded.
		_, err := s.db.Exec(ctx, "INSERT INTO swarm_long_term_memory (topic, summary, created_at) VALUES (?, ?, CURRENT_TIMESTAMP)", topic, summary)
		return err
	})
}

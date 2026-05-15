package orchestration

import (
	"encoding/json"
	"context"
	"time"
)

type DefaultTaskOrchestrator struct {
	mesh      MeshTransport
	db      TaskStore
	spawner SubAgentSpawner
}

func NewDefaultTaskOrchestrator(db TaskStore, spawner SubAgentSpawner, mesh MeshTransport) *DefaultTaskOrchestrator {
	return &DefaultTaskOrchestrator{
		db:      db,
		spawner: spawner,
		mesh: mesh,
	}
}

func (t *DefaultTaskOrchestrator) StartBackgroundWorker(ctx context.Context) {
	go func() {
		ticker := time.NewTicker(2 * time.Second)
		defer ticker.Stop()

		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				_ = t.PollTasks(ctx)
			}
		}
	}()
}

func (t *DefaultTaskOrchestrator) PollTasks(ctx context.Context) error {
	// Let's assume we can add a method PollDelegatedTasks to TaskStore
	// We'll define PollDelegatedTasks to fetch up to 10 delegated tasks.
	tasks, err := t.db.PollDelegatedTasks(ctx, 10)
	if err != nil {
		return err
	}

	for _, task := range tasks {
		_ = t.spawner.Spawn(ctx, task)
		if t.mesh != nil {
			var payload []byte
			if task.Payload != nil {
				payload = *task.Payload
			}
			agentID := ""
			if task.AgentID != nil {
				agentID = *task.AgentID
			}
			msg := MeshMessage{
				AgentID:   agentID,
				EventType: "TASK_SPAWNED",
				Channel:   "mesh:tasks",
			}
			if len(payload) > 0 {
				raw := json.RawMessage(payload)
				msg.Data = &raw
			}
			if msgBytes, err := json.Marshal(msg); err == nil {
				t.mesh.Publish(ctx, msg.Channel, msgBytes)
			}
		}
	}

	return nil
}

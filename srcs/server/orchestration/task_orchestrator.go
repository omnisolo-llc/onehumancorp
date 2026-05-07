package orchestration

import (
	"context"
	"time"
)

type DefaultTaskOrchestrator struct {
	db      TaskStore
	spawner SubAgentSpawner
}

func NewDefaultTaskOrchestrator(db TaskStore, spawner SubAgentSpawner) *DefaultTaskOrchestrator {
	return &DefaultTaskOrchestrator{
		db:      db,
		spawner: spawner,
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
	}

	return nil
}

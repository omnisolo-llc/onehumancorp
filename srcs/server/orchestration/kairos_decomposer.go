package orchestration

import (
	"context"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// KairosDecomposer is a background worker that reads missions and converts them to tasks.
type KairosDecomposer struct {
	taskManager *TaskManager
	provider    db.Provider
}

// NewKairosDecomposer initializes a new KAIROS decomposer worker.
func NewKairosDecomposer(tm *TaskManager, prov db.Provider) *KairosDecomposer {
	return &KairosDecomposer{
		taskManager: tm,
		provider:    prov,
	}
}

// Start begins the continuous polling process for new missions.
func (kd *KairosDecomposer) Start(ctx context.Context) {
	go kd.run(ctx)
}

func (kd *KairosDecomposer) run(ctx context.Context) {
	ticker := time.NewTicker(5 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			if err := kd.decomposePendingMissions(ctx); err != nil {
				slog.Error("kairos decomposer error", "err", err)
			}
		}
	}
}

func (kd *KairosDecomposer) decomposePendingMissions(ctx context.Context) error {
	// 1. In a full system, this would query agent_missions WHERE status = PENDING.
	// For now, since missions may just be files or partially defined in DB,
	// this is a placeholder representing the background loop that KAIROS requires.
	return nil
}

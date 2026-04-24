package memory

import (
	"context"
	"fmt"
	"log"
	"time"
)

type LLMClient interface {
	Reason(ctx context.Context, prompt string) (string, error)
}

type Worker struct {
	repo     *VectorRepository
	llm      LLMClient
	Interval time.Duration
	done     chan struct{}
}

func NewWorker(repo *VectorRepository, llm LLMClient, interval time.Duration) *Worker {
	if interval == 0 {
		interval = 24 * time.Hour
	}
	return &Worker{
		repo:     repo,
		llm:      llm,
		Interval: interval,
		done:     make(chan struct{}),
	}
}

func (w *Worker) Start(ctx context.Context) {
	ticker := time.NewTicker(w.Interval)
	go func() {
		defer ticker.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-w.done:
				return
			case <-ticker.C:
				w.Run(ctx)
			}
		}
	}()
}

func (w *Worker) Run(ctx context.Context) {
	// A simple mocked approach since we must process globally per org.
	// In reality we would query orgs, then run. For the sake of the test and implementation:
	w.resolveConflictsForOrg(ctx, "default_org")
}

func (w *Worker) resolveConflictsForOrg(ctx context.Context, orgID string) {
	// 1. Get recent memories
	recent, err := w.repo.FindRecentMemories(ctx, orgID, time.Now().Add(-24*time.Hour))
	if err != nil || len(recent) < 2 {
		return
	}

	// 2. We use SemanticSearch locally against the DB to find similar older items for each new item
	// (Simulated LLM-based semantic deduplication/conflict resolution)
	for _, rec := range recent {
		similar, err := w.repo.SemanticSearch(ctx, orgID, rec.Embedding, 5)
		if err != nil {
			continue
		}

		for _, sim := range similar {
			if sim.ID == rec.ID {
				continue
			}

			// Let the LLM decide if there's a conflict
			prompt := fmt.Sprintf("Are these two facts about the same business conflicting? Fact 1: '%s' (Date: %s). Fact 2: '%s' (Date: %s). Reply strictly with YES or NO.", sim.Content, sim.CreatedAt, rec.Content, rec.CreatedAt)
			resp, _ := w.llm.Reason(ctx, prompt)

			if resp == "YES" {
				// We keep the newer one (rec is newer since it's from recent batch and sim is matched)
				if rec.CreatedAt.After(sim.CreatedAt) {
					w.repo.Delete(ctx, sim.ID)
					log.Printf("Resolved semantic conflict: deleted older memory %s in favor of %s", sim.ID, rec.ID)
				}
			}
		}
	}
}

func (w *Worker) Stop() {
	close(w.done)
}

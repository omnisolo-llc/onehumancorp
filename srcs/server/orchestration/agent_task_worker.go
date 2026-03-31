package orchestration

import (
	"context"
	"log/slog"
	"math/rand"
	"time"

	"github.com/onehumancorp/mono/srcs/server/integrations/plane"
)

// TaskWorker periodically fetches open issues from the configured issue tracker (Plane)
// and randomly assigns an idle agent to them by injecting the task into their context.
type TaskWorker struct {
	planeClient *plane.Client
}

// NewTaskWorker creates a new TaskWorker.
func NewTaskWorker(pc *plane.Client) *TaskWorker {
	return &TaskWorker{
		planeClient: pc,
	}
}

// Start begins the background polling loop for the task worker.
func (tw *TaskWorker) Start(ctx context.Context) {
	ticker := time.NewTicker(30 * time.Second)
	go func() {
		defer ticker.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				tw.pollAndAssign()
			}
		}
	}()
}

// pollAndAssign polls the Plane REST API for open issues and assigns a random one.
func (tw *TaskWorker) pollAndAssign() {
	if !plane.IsEnabled() {
		return
	}

	issues, err := tw.planeClient.ListOpenIssues()
	if err != nil {
		slog.Error("failed to list open plane issues", "err", err)
		return
	}

	if len(issues) == 0 {
		return
	}

	// Pick a random issue
	randomIndex := rand.Intn(len(issues))
	issue := issues[randomIndex]

	slog.Info("agent task worker: randomly picked issue for execution", "issue_id", issue.ID, "title", issue.Name)

	// Mark it as in progress
	if err := tw.planeClient.UpdateIssueStatus(issue.ID, "in_progress"); err != nil {
		slog.Error("failed to update plane issue status", "err", err)
		return
	}

	// TODO: Inject issue descriptor into an available agent's prompt queue.
	slog.Info("agent task worker: issue marked in_progress, delegating to agent")
}

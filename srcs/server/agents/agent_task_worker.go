package agents

import (
	"context"
	"encoding/json"
	"log/slog"
	"math/rand"
	"time"

	"github.com/onehumancorp/mono/srcs/server/integrations/plane"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

// TaskWorker periodically fetches open issues from the configured issue tracker (Plane)
// and randomly assigns an idle agent to them by injecting the task into their context.
type TaskWorker struct {
	planeClient  *plane.Client
	hub          *orchestration.Hub
	pollInterval time.Duration
}

// NewTaskWorker creates a new TaskWorker.
func NewTaskWorker(pc *plane.Client, hub *orchestration.Hub) *TaskWorker {
	return &TaskWorker{
		planeClient:  pc,
		hub:          hub,
		pollInterval: 30 * time.Second,
	}
}

// Start begins the background polling loop for the task worker.
func (tw *TaskWorker) Start(ctx context.Context) {
	ticker := time.NewTicker(tw.pollInterval)
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

	agentFound := false
	if tw.hub != nil {
		agents := tw.hub.Agents()
		for _, a := range agents {
			if a.Status == orchestration.StatusActive || a.Status == orchestration.StatusWaitingForTools {
				// Encode the issue payload securely as JSON to prevent prompt injection.
				// The agent's framework is responsible for parsing this data blob
				// rather than blindly executing unstructured text.
				payload, _ := json.Marshal(map[string]string{
					"issue_id":   issue.ID,
					"issue_name": issue.Name,
					"directive":  "Please resolve the attached issue descriptor.",
				})

				msg := orchestration.Message{
					ID:         "task-" + issue.ID,
					FromAgent:  "SYSTEM",
					ToAgent:    a.ID,
					Type:       "TaskAssignment",
					Content:    string(payload),
					OccurredAt: time.Now().UTC(),
				}
				_ = tw.hub.Publish(msg)
				slog.Info("agent task worker: issue marked in_progress, delegating to agent", "agent_id", a.ID)
				agentFound = true
				break
			}
		}
	}

	if !agentFound {
		slog.Warn("agent task worker: issue marked in_progress but no available agents to delegate to")
	}
}

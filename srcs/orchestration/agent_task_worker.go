package orchestration

import (
	"context"
	"log/slog"
	"fmt"
	"math/rand"
	"time"

	"github.com/onehumancorp/mono/srcs/server/integrations/plane"
)

// TaskWorker periodically fetches open issues from the configured issue tracker (Plane)
// and randomly assigns an idle agent to them by injecting the task into their context.
type TaskWorker struct {
	planeClient *plane.Client
	pollInterval time.Duration
	hub *Hub
}

// NewTaskWorker creates a new TaskWorker.
func NewTaskWorker(pc *plane.Client, hub *Hub) *TaskWorker {
	return &TaskWorker{
		planeClient: pc,
		pollInterval: 30 * time.Second,
		hub: hub,
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

	var idleAgents []Agent
	for _, agent := range tw.hub.Agents() {
		if agent.Status == StatusIdle {
			idleAgents = append(idleAgents, agent)
		}
	}

	if len(idleAgents) == 0 {
		slog.Info("agent task worker: no idle agents available to take the issue")
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

	agent := idleAgents[rand.Intn(len(idleAgents))]

	msg := Message{
		ID:         fmt.Sprintf("task-%s-%d", issue.ID, time.Now().UnixNano()),
		FromAgent:  "system-task-worker",
		ToAgent:    agent.ID,
		Type:       EventTask,
		Content:    fmt.Sprintf("Please resolve Plane Issue: %s - %s", issue.Name, issue.Description),
		OccurredAt: time.Now().UTC(),
	}

	if err := tw.hub.Publish(msg); err != nil {
		slog.Error("failed to publish task to agent", "err", err, "agent_id", agent.ID)
		return
	}

	slog.Info("agent task worker: issue marked in_progress, delegating to agent", "agent_id", agent.ID)
}

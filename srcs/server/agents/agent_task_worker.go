package agents

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"math/rand"
	"time"

	"github.com/onehumancorp/mono/srcs/server/agents/builtin"
	"github.com/onehumancorp/mono/srcs/server/integrations/plane"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
	"os"
)

import (
	"sync"
)

// TaskWorker periodically fetches open issues from the configured issue tracker (Plane)
// and randomly assigns an idle agent to them by injecting the task into their context.
// It uses a worker pool to process multiple tasks in parallel for higher throughput.
type TaskWorker struct {
	planeClient  *plane.Client
	hub          *orchestration.Hub
	pollInterval time.Duration
	numWorkers   int
}

// NewTaskWorker creates a new TaskWorker with default single worker.
func NewTaskWorker(pc *plane.Client, hub *orchestration.Hub) *TaskWorker {
	return &TaskWorker{
		planeClient:  pc,
		hub:          hub,
		pollInterval: 30 * time.Second,
		numWorkers:   1, // Default to 1 for backward compatibility
	}
}

// Start begins the background polling loop for the task worker.
func (tw *TaskWorker) Start(ctx context.Context) {
	tw.StartWithWorkers(ctx, 3) // Default to 3 workers for better throughput
}

// StartWithWorkers begins the polling loop and worker pool with the specified number of workers.
func (tw *TaskWorker) StartWithWorkers(ctx context.Context, workers int) {
	tw.numWorkers = workers
	ticker := time.NewTicker(tw.pollInterval)

	// Create a buffered channel for tasks
	taskChan := make(chan plane.Issue, 100)

	// Start worker pool
	var wg sync.WaitGroup
	for i := 0; i < tw.numWorkers; i++ {
		wg.Add(1)
		go func(workerID int) {
			defer wg.Done()
			for {
				select {
				case <-ctx.Done():
					return
				case issue, ok := <-taskChan:
					if !ok {
						return
					}
					tw.processIssue(issue)
				}
			}
		}(i)
	}

	// Start polling loop
	go func() {
		defer ticker.Stop()
		defer close(taskChan)
		for {
			select {
			case <-ctx.Done():
				// Wait for workers to finish current tasks before returning
				wg.Wait()
				return
			case <-ticker.C:
				tw.pollAndDispatch(taskChan)
			}
		}
	}()
}

// pollAndDispatch polls the Plane REST API for open issues and sends them to the worker channel.
func (tw *TaskWorker) pollAndDispatch(taskChan chan<- plane.Issue) {
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

	// To improve throughput, dispatch multiple issues instead of randomly picking one
	// Dispatch up to tw.numWorkers issues per tick
	dispatchCount := tw.numWorkers
	if len(issues) < dispatchCount {
		dispatchCount = len(issues)
	}

	// Shuffle or pick random issues to avoid processing the same ones if workers are slow
	rand.Shuffle(len(issues), func(i, j int) {
		issues[i], issues[j] = issues[j], issues[i]
	})

	for i := 0; i < dispatchCount; i++ {
		select {
		case taskChan <- issues[i]:
			slog.Info("agent task worker: dispatched issue to worker pool", "issue_id", issues[i].ID)
		default:
			slog.Warn("agent task worker: task channel full, dropping issue dispatch")
		}
	}
}

// pollAndAssign polls the Plane REST API for open issues and assigns a random one.
// Kept for tests or manual single-task dispatch.
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

	tw.processIssue(issue)
}

// processIssue handles the actual processing and assignment of a single issue to an agent.
func (tw *TaskWorker) processIssue(issue plane.Issue) {
	slog.Info("agent task worker: processing issue", "issue_id", issue.ID, "title", issue.Name)

	// Mark it as in progress
	if err := tw.planeClient.UpdateIssueStatus(issue.ID, "in_progress"); err != nil {
		slog.Error("failed to update plane issue status", "err", err)
		return
	}

	agentFound := false
	if tw.hub != nil {
		agents := tw.hub.Agents()

		// To avoid multiple workers assigning tasks to the same first idle agent,
		// randomize agent traversal order
		agentIndices := rand.Perm(len(agents))

		for _, idx := range agentIndices {
			a := agents[idx]
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

				// Handle Builtin agent logic
				if a.ProviderType == string(ProviderTypeBuiltin) || a.ProviderType == "" {
					slog.Info("agent task worker: dispatching to builtin local runner", "agent_id", a.ID)
					go func(agent orchestration.Agent, payload string) {
						workDir, _ := os.Getwd()
						_, err := builtin.SpawnTask(
							context.Background(),
							fmt.Sprintf("plane issue %s", issue.ID),
							payload,
							workDir,
							builtin.AgentConfig{},
						)
						if err != nil {
							slog.Error("builtin agent run error", "err", err, "agent_id", agent.ID)
						}
					}(a, string(payload))
				}

				agentFound = true
				break
			}
		}
	}

	if !agentFound {
		slog.Warn("agent task worker: issue marked in_progress but no available agents to delegate to")
	}
}

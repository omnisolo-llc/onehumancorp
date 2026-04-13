package agents

import (
	"context"
	"encoding/json"
	"log/slog"
	"math/rand"
	"os"
	"sync"
	"time"

	pb "github.com/onehumancorp/mono/srcs/proto"
	"github.com/onehumancorp/mono/srcs/server/agents/builtin"
	agentruntime "github.com/onehumancorp/mono/srcs/server/agents/runtime"
	"github.com/onehumancorp/mono/srcs/server/integrations/plane"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
	"google.golang.org/protobuf/proto"
)

type builtinTaskLauncher interface {
	LaunchTask(context.Context, agentruntime.TaskRequest) error
	DefaultRegion() string
}

// TaskWorker periodically fetches open issues from the configured issue tracker (Plane)
// and randomly assigns an idle agent to them by injecting the task into their context.
// It uses a worker pool to process multiple tasks in parallel for higher throughput.
type TaskWorker struct {
	planeClient  *plane.Client
	hub          *orchestration.Hub
	pollInterval time.Duration
	numWorkers   int
	taskLauncher builtinTaskLauncher
}

// NewTaskWorker creates a new TaskWorker with default single worker.
func NewTaskWorker(pc *plane.Client, hub *orchestration.Hub) *TaskWorker {
	return &TaskWorker{
		planeClient:  pc,
		hub:          hub,
		pollInterval: 30 * time.Second,
		numWorkers:   1, // Default to 1 for backward compatibility
		taskLauncher: agentruntime.NewLauncherFromEnv(),
	}
}

func defaultAgentWorkDir() string {
	if configured := os.Getenv("OHC_AGENT_WORKDIR"); configured != "" {
		return configured
	}
	workDir, err := os.Getwd()
	if err != nil {
		return "."
	}
	return workDir
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
	if !plane.IsEnabled() || tw.planeClient == nil {
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
	if !plane.IsEnabled() || tw.planeClient == nil {
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
	if tw.planeClient != nil {
		if err := tw.planeClient.UpdateIssueStatus(issue.ID, "in_progress"); err != nil {
			slog.Error("failed to update plane issue status", "err", err)
			return
		}
	}

	agentFound := false
	if tw.hub != nil {
		agents := tw.hub.Agents()

		// To avoid multiple workers assigning tasks to the same first idle agent,
		// randomize agent traversal order
		agentIndices := rand.Perm(len(agents))

		for _, idx := range agentIndices {
			a := agents[idx]
			if a.Status != orchestration.StatusIdle {
				continue
			}

			// Encode the issue payload securely as JSON to prevent prompt injection.
			// The agent's framework is responsible for parsing this data blob
			// rather than blindly executing unstructured text.
			payload, _ := json.Marshal(map[string]string{
				"issue_id":   issue.ID,
				"issue_name": issue.Name,
				"directive":  "Please resolve the attached issue descriptor.",
			})
			content := string(payload)
			if IsManagedBuiltin(a) {
				encoded, err := builtin.EncodeTaskAssignment(workerTaskAssignment(issue))
				if err != nil {
					slog.Error("agent task worker: failed to encode task assignment", "issue_id", issue.ID, "agent_id", a.ID, "err", err)
					continue
				}
				content = encoded
			}

			msg := orchestration.Message{
				ID:         "task-" + issue.ID,
				FromAgent:  a.ID,
				ToAgent:    a.ID,
				Type:       "TaskAssignment",
				Content:    content,
				OccurredAt: time.Now().UTC(),
			}

			activeAgent := a
			activeAgent.Status = orchestration.StatusActive
			tw.hub.RegisterAgent(activeAgent)

			if err := tw.hub.Publish(msg); err != nil {
				slog.Error("agent task worker: failed to publish task assignment", "agent_id", activeAgent.ID, "err", err)
				tw.hub.RegisterAgent(a)
				continue
			}
			slog.Info("agent task worker: issue marked in_progress, delegating to agent", "agent_id", activeAgent.ID)

			if IsManagedBuiltin(activeAgent) {
				agentFound = true
				break
			}

			if activeAgent.ProviderType == string(ProviderTypeBuiltin) || activeAgent.ProviderType == "" {
				go tw.launchBuiltinTask(activeAgent, issue, string(payload))
			}

			agentFound = true
			break
		}
	}

	if !agentFound {
		slog.Warn("agent task worker: issue marked in_progress but no available agents to delegate to")
	}
}

func workerTaskAssignment(issue plane.Issue) *pb.TaskAssignment {
	return pb.TaskAssignment_builder{
		IssueId:   proto.String(issue.ID),
		IssueName: proto.String(issue.Name),
		Directive: proto.String("Please resolve the attached issue descriptor."),
		WorkDir:   proto.String(defaultAgentWorkDir()),
	}.Build()
}

func (tw *TaskWorker) launchBuiltinTask(agent orchestration.Agent, issue plane.Issue, payload string) {
	launcher := tw.taskLauncher
	if launcher == nil {
		launcher = agentruntime.NewLauncherFromEnv()
	}

	request := agentruntime.TaskRequest{
		AgentID:      agent.ID,
		AgentName:    agent.Name,
		Role:         agent.Role,
		ProviderType: agent.ProviderType,
		IssueID:      issue.ID,
		Description:  issue.Name,
		Prompt:       payload,
		WorkDir:      defaultAgentWorkDir(),
	}

	if err := launcher.LaunchTask(context.Background(), request); err != nil {
		slog.Error("builtin agent launch error", "err", err, "agent_id", agent.ID, "runtime", launcher.DefaultRegion())
	}

	agent.Status = orchestration.StatusIdle
	tw.hub.RegisterAgent(agent)
}

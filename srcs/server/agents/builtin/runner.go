package builtin

import (
	"context"
	"fmt"
	"log/slog"
	"sync"
	"time"

	pb "github.com/onehumancorp/mono/srcs/proto"
	agentservicepb "github.com/onehumancorp/mono/srcs/proto/agentservice"
	"github.com/onehumancorp/mono/srcs/server/agents/builtinclient"
	"google.golang.org/protobuf/proto"
)

type HubAgent struct {
	ID             string
	Name           string
	Role           string
	OrganizationID string
	Status         HubStatus
	ProviderType   string
	Region         string
	Managed        bool
}

type HubStatus string

const (
	HubStatusIdle   HubStatus = "IDLE"
	HubStatusActive HubStatus = "ACTIVE"
)

type HubMessage struct {
	ID        string
	FromAgent string
	ToAgent   string
	Type      string
	Content   string
}

type Hub interface {
	RegisterAgent(agent HubAgent)
	ReportWorkerState(state *pb.WorkerState)
	Subscribe(agentID string) (<-chan struct{}, func())
	Inbox(agentID string) []HubMessage
	Publish(msg HubMessage) error
}

type Runner struct {
	agent          HubAgent
	hub            Hub
	builtinAddress string

	mu     sync.Mutex
	active *activeTask
}

type activeTask struct {
	id          string
	description string
	origin      HubMessage
	startedAt   time.Time
	cancel      context.CancelFunc
}

type taskResult struct {
	id           string
	description  string
	status       pb.TaskStatus
	result       string
	errText      string
	toolUseCount int32
	startedAt    time.Time
	endedAt      time.Time
}

func NewRunner(hub Hub, agent HubAgent, builtinAddress string) *Runner {
	if agent.Name == "" {
		agent.Name = "builtin-agent"
	}
	if agent.Role == "" {
		agent.Role = "SOFTWARE_ENGINEER"
	}
	if agent.ProviderType == "" {
		agent.ProviderType = "builtin"
	}
	if agent.Status == "" {
		agent.Status = HubStatusIdle
	}
	if builtinAddress == "" {
		builtinAddress = builtinclient.AddressFromEnv()
	}
	return &Runner{agent: agent, hub: hub, builtinAddress: builtinAddress}
}

func (r *Runner) Start(ctx context.Context) {
	r.reportWorkerState(pb.WorkerPhase_WORKER_PHASE_STARTING, "worker starting")
	r.register(HubStatusIdle)
	r.reportWorkerState(pb.WorkerPhase_WORKER_PHASE_READY, "worker ready")

	notifyCh, unsubscribe := r.hub.Subscribe(r.agent.ID)
	defer unsubscribe()

	ticker := time.NewTicker(30 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			r.reportWorkerState(pb.WorkerPhase_WORKER_PHASE_STOPPING, "worker stopping")
			r.cancelActiveTask()
			return
		case <-notifyCh:
			r.drainInbox(ctx)
		case <-ticker.C:
			status := r.currentStatus()
			r.register(status)
			r.reportWorkerState(WorkerPhaseForStatus(status), "worker heartbeat")
			r.drainInbox(ctx)
		}
	}
}

func (r *Runner) currentStatus() HubStatus {
	r.mu.Lock()
	defer r.mu.Unlock()
	if r.active != nil {
		return HubStatusActive
	}
	return HubStatusIdle
}

func (r *Runner) cancelActiveTask() {
	r.mu.Lock()
	defer r.mu.Unlock()
	if r.active != nil {
		r.active.cancel()
	}
}

func (r *Runner) drainInbox(ctx context.Context) {
	messages := r.hub.Inbox(r.agent.ID)
	for _, msg := range messages {
		switch msg.Type {
		case "TaskAssignment":
			r.handleTaskAssignment(ctx, msg)
		case "Kill":
			r.handleKill(msg)
		default:
			slog.Debug("builtin runner: unhandled message type", "type", msg.Type, "agent_id", r.agent.ID)
		}
	}
}

func (r *Runner) handleTaskAssignment(ctx context.Context, msg HubMessage) {
	payload, err := DecodeTaskAssignment(msg.Content)
	if err != nil {
		slog.Error("builtin runner: failed to parse task assignment", "agent_id", r.agent.ID, "error", err)
		return
	}

	prompt := payload.GetPrompt()
	if prompt == "" {
		prompt = buildPrompt(payload)
	}
	description := payload.GetIssueName()
	if description == "" {
		description = payload.GetDirective()
	}
	taskID := firstNonEmpty(payload.GetIssueId(), msg.ID, fmt.Sprintf("task-%d", time.Now().UTC().UnixNano()))

	r.mu.Lock()
	if r.active != nil {
		r.mu.Unlock()
		slog.Warn("builtin runner: ignoring concurrent task assignment", "agent_id", r.agent.ID, "task_id", taskID)
		return
	}
	taskCtx, cancel := context.WithCancel(ctx)
	active := &activeTask{
		id:          taskID,
		description: description,
		origin:      msg,
		startedAt:   time.Now().UTC(),
		cancel:      cancel,
	}
	r.active = active
	r.mu.Unlock()

	r.register(HubStatusActive)
	r.reportWorkerState(pb.WorkerPhase_WORKER_PHASE_BUSY, description)
	go r.executeTask(taskCtx, active, prompt)
}

func (r *Runner) handleKill(msg HubMessage) {
	payload, err := DecodeKillTaskRequest(msg.Content)
	if err != nil {
		return
	}

	r.mu.Lock()
	defer r.mu.Unlock()
	if r.active == nil {
		return
	}
	if payload.GetTaskId() != "" && payload.GetTaskId() != r.active.id {
		return
	}
	r.active.cancel()
}

func (r *Runner) executeTask(ctx context.Context, active *activeTask, prompt string) {
	result := taskResult{
		id:          active.id,
		description: active.description,
		startedAt:   active.startedAt,
	}

	client, err := builtinclient.DialContext(ctx, r.builtinAddress)
	if err != nil {
		if ctx.Err() != nil {
			result.status = pb.TaskStatus_TASK_STATUS_KILLED
			result.errText = ctx.Err().Error()
		} else {
			result.status = pb.TaskStatus_TASK_STATUS_FAILED
			result.errText = err.Error()
		}
		r.finishTask(active, result)
		return
	}
	defer client.Close()

	output, err := client.RunTask(ctx, builtinclient.RunTaskRequest{Task: prompt}, func(event *agentservicepb.RunTaskEvent) {
		if event.GetType() == agentservicepb.EventType_TOOL_CALL {
			result.toolUseCount++
		}
	})
	result.endedAt = time.Now().UTC()

	if ctx.Err() != nil {
		result.status = pb.TaskStatus_TASK_STATUS_KILLED
		result.errText = ctx.Err().Error()
	} else if err != nil {
		result.status = pb.TaskStatus_TASK_STATUS_FAILED
		result.errText = err.Error()
	} else {
		result.status = pb.TaskStatus_TASK_STATUS_COMPLETED
		result.result = output
	}

	r.finishTask(active, result)
}

func (r *Runner) finishTask(active *activeTask, result taskResult) {
	if result.endedAt.IsZero() {
		result.endedAt = time.Now().UTC()
	}

	r.mu.Lock()
	if r.active != nil && r.active.id == active.id {
		r.active = nil
	}
	r.mu.Unlock()

	r.register(HubStatusIdle)
	r.reportWorkerState(pb.WorkerPhase_WORKER_PHASE_READY, "worker idle")
	r.publishCompletion(active.origin, result)
}

func (r *Runner) publishCompletion(originMsg HubMessage, result taskResult) {
	replyTo := originMsg.FromAgent
	if replyTo == "" || replyTo == "SYSTEM" || replyTo == r.agent.ID {
		slog.Info("builtin runner: task complete", "agent_id", r.agent.ID, "task_id", result.id, "status", result.status.String())
		return
	}
	status := result.status
	content, err := EncodeTaskResultEnvelope(pb.TaskResultEnvelope_builder{
		TaskId:        proto.String(result.id),
		Description:   proto.String(result.description),
		Status:        &status,
		Result:        proto.String(result.result),
		Error:         proto.String(result.errText),
		ToolUseCount:  proto.Int32(result.toolUseCount),
		StartedAtUnix: proto.Int64(result.startedAt.Unix()),
		EndedAtUnix:   proto.Int64(result.endedAt.Unix()),
	}.Build())
	if err != nil {
		slog.Error("builtin runner: failed to encode task result", "agent_id", r.agent.ID, "task_id", result.id, "error", err)
		return
	}

	if err := r.hub.Publish(HubMessage{
		ID:        "task-result-" + result.id,
		FromAgent: r.agent.ID,
		ToAgent:   replyTo,
		Type:      "TaskResult",
		Content:   content,
	}); err != nil {
		slog.Error("builtin runner: failed to publish task result", "agent_id", r.agent.ID, "task_id", result.id, "error", err)
	}
}

func (r *Runner) register(status HubStatus) {
	r.agent.Status = status
	r.hub.RegisterAgent(r.agent)
}

func (r *Runner) reportWorkerState(phase pb.WorkerPhase, detail string) {
	r.hub.ReportWorkerState(pb.WorkerState_builder{
		AgentId:        proto.String(r.agent.ID),
		Phase:          &phase,
		Runtime:        proto.String(firstNonEmpty(r.agent.Region, "process")),
		ObservedAtUnix: proto.Int64(time.Now().UTC().Unix()),
		Detail:         proto.String(detail),
	}.Build())
	if phase == pb.WorkerPhase_WORKER_PHASE_BUSY {
		return
	}
	if phase == pb.WorkerPhase_WORKER_PHASE_READY {
		if r.agent.Status != HubStatusIdle {
			r.agent.Status = HubStatusIdle
		}
	}
}

func buildPrompt(p *pb.TaskAssignment) string {
	prompt := p.GetDirective()
	if p.GetIssueName() != "" {
		prompt = fmt.Sprintf("Issue: %s\n\n%s", p.GetIssueName(), p.GetDirective())
	}
	return prompt
}

func firstNonEmpty(values ...string) string {
	for _, value := range values {
		if value != "" {
			return value
		}
	}
	return ""
}

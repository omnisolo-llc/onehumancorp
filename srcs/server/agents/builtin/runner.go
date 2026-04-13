package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"strings"
	"sync"
	"time"

	agentservicepb "github.com/onehumancorp/mono/srcs/proto/agentservice"
	"github.com/onehumancorp/mono/srcs/server/agents/builtinclient"
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
	Subscribe(agentID string) (<-chan struct{}, func())
	Inbox(agentID string) []HubMessage
	Publish(msg HubMessage) error
}

type TaskAssignmentPayload struct {
	IssueID   string `json:"issue_id"`
	IssueName string `json:"issue_name"`
	Directive string `json:"directive"`
	Prompt    string `json:"prompt,omitempty"`
	WorkDir   string `json:"work_dir,omitempty"`
}

type TaskStatus string

const (
	TaskStatusCompleted TaskStatus = "completed"
	TaskStatusFailed    TaskStatus = "failed"
	TaskStatusKilled    TaskStatus = "killed"
)

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
	status       TaskStatus
	result       string
	errText      string
	toolUseCount int
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
	r.register(HubStatusIdle)

	notifyCh, unsubscribe := r.hub.Subscribe(r.agent.ID)
	defer unsubscribe()

	ticker := time.NewTicker(30 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			r.cancelActiveTask()
			return
		case <-notifyCh:
			r.drainInbox(ctx)
		case <-ticker.C:
			r.register(r.currentStatus())
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
	var payload TaskAssignmentPayload
	if err := json.Unmarshal([]byte(msg.Content), &payload); err != nil {
		slog.Error("builtin runner: failed to parse task assignment", "agent_id", r.agent.ID, "error", err)
		return
	}

	prompt := payload.Prompt
	if prompt == "" {
		prompt = buildPrompt(payload)
	}
	description := payload.IssueName
	if description == "" {
		description = payload.Directive
	}
	taskID := firstNonEmpty(payload.IssueID, msg.ID, fmt.Sprintf("task-%d", time.Now().UTC().UnixNano()))

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
	go r.executeTask(taskCtx, active, prompt)
}

func (r *Runner) handleKill(msg HubMessage) {
	var payload struct {
		TaskID string `json:"task_id"`
	}
	if err := json.Unmarshal([]byte(msg.Content), &payload); err != nil {
		return
	}

	r.mu.Lock()
	defer r.mu.Unlock()
	if r.active == nil {
		return
	}
	if payload.TaskID != "" && payload.TaskID != r.active.id {
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
			result.status = TaskStatusKilled
			result.errText = ctx.Err().Error()
		} else {
			result.status = TaskStatusFailed
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
		result.status = TaskStatusKilled
		result.errText = ctx.Err().Error()
	} else if err != nil {
		result.status = TaskStatusFailed
		result.errText = err.Error()
	} else {
		result.status = TaskStatusCompleted
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
	r.publishCompletion(active.origin, result)
}

func (r *Runner) publishCompletion(originMsg HubMessage, result taskResult) {
	replyTo := originMsg.FromAgent
	if replyTo == "" || replyTo == "SYSTEM" || replyTo == r.agent.ID {
		slog.Info("builtin runner: task complete", "agent_id", r.agent.ID, "task_id", result.id, "status", result.status)
		return
	}

	if err := r.hub.Publish(HubMessage{
		ID:        "task-result-" + result.id,
		FromAgent: r.agent.ID,
		ToAgent:   replyTo,
		Type:      "TaskResult",
		Content:   buildNotification(result),
	}); err != nil {
		slog.Error("builtin runner: failed to publish task result", "agent_id", r.agent.ID, "task_id", result.id, "error", err)
	}
}

func (r *Runner) register(status HubStatus) {
	r.agent.Status = status
	r.hub.RegisterAgent(r.agent)
}

func buildPrompt(p TaskAssignmentPayload) string {
	prompt := p.Directive
	if p.IssueName != "" {
		prompt = fmt.Sprintf("Issue: %s\n\n%s", p.IssueName, p.Directive)
	}
	return prompt
}

func buildNotification(result taskResult) string {
	summary := fmt.Sprintf("Task %q completed.", result.description)
	if result.status == TaskStatusFailed {
		summary = fmt.Sprintf("Task %q failed: %s", result.description, result.errText)
	}
	if result.status == TaskStatusKilled {
		summary = fmt.Sprintf("Task %q was killed.", result.description)
	}
	if result.status == TaskStatusCompleted && result.result != "" {
		summary = summary + " Result: " + truncate(result.result, 500)
	}

	resultSection := ""
	if result.result != "" {
		resultSection = fmt.Sprintf("\n<result>%s</result>", escapeXML(truncate(result.result, 2000)))
	}

	durationMs := result.endedAt.Sub(result.startedAt).Milliseconds()
	if durationMs < 0 {
		durationMs = 0
	}

	errorSection := ""
	if result.errText != "" {
		errorSection = fmt.Sprintf("\n<error>%s</error>", escapeXML(truncate(result.errText, 1000)))
	}

	return fmt.Sprintf(`<task-notification>
<task-id>%s</task-id>
<output-file></output-file>
<status>%s</status>
<summary>%s</summary>%s%s
<usage><total_tokens>0</total_tokens><tool_uses>%d</tool_uses><duration_ms>%d</duration_ms></usage>
</task-notification>`,
		result.id,
		string(result.status),
		escapeXML(summary),
		resultSection,
		errorSection,
		result.toolUseCount,
		durationMs,
	)
}

func truncate(value string, max int) string {
	if len(value) <= max {
		return value
	}
	return value[:max] + "..."
}

func escapeXML(value string) string {
	replacer := strings.NewReplacer(
		"&", "&amp;",
		"<", "&lt;",
		">", "&gt;",
		`"`, "&quot;",
		"'", "&apos;",
	)
	return replacer.Replace(value)
}

func firstNonEmpty(values ...string) string {
	for _, value := range values {
		if value != "" {
			return value
		}
	}
	return ""
}

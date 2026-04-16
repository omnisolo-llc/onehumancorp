// Package local provides the default local agent runner.
//
// The Runner registers itself in the orchestration Hub, watches for incoming
// TaskAssignment messages, and executes each task using the full local agent
// loop (LLM + tool execution). It is selected as the default implementation
// when an agent has ProviderType == "" or "builtin".
package builtin

import (
"context"
"encoding/json"
"fmt"
"log/slog"
"os"
"sync"
"time"

"github.com/google/uuid"
)

// HubAgent is the minimal agent registration struct the Runner needs.
type HubAgent struct {
ID           string    `json:"id"`
Name         string    `json:"name"`
Role         string    `json:"role"`
Status       HubStatus `json:"status"`
ProviderType string    `json:"providerType,omitempty"`
}

// HubStatus mirrors orchestration.Status.
type HubStatus string

const (
HubStatusIdle   HubStatus = "IDLE"
HubStatusActive HubStatus = "ACTIVE"
)

// HubMessage is the minimal message struct the Runner reads from the inbox.
type HubMessage struct {
ID        string `json:"id"`
FromAgent string `json:"fromAgent"`
ToAgent   string `json:"toAgent"`
Type      string `json:"type"`
Content   string `json:"content"`
}

// Hub is the subset of orchestration.Hub that the Runner requires.
// Using an interface keeps the local package decoupled from the proto-generated
// code that is only available under Bazel builds.
type Hub interface {
// RegisterAgent registers (or upserts) the agent in the Hub.
RegisterAgent(agent HubAgent)
// Subscribe returns a channel that receives a signal when the agent's inbox
// has new messages, and an unsubscribe function to clean up.
Subscribe(agentID string) (<-chan struct{}, func())
// Inbox returns and clears all pending messages for the agent.
Inbox(agentID string) []HubMessage
// Publish sends a message from the runner back to other agents.
Publish(msg HubMessage) error
}

// TaskAssignmentPayload is the JSON structure of a TaskAssignment message.
// It matches the format written by agents.TaskWorker.processIssue.
type TaskAssignmentPayload struct {
IssueID   string `json:"issue_id"`
IssueName string `json:"issue_name"`
Directive string `json:"directive"`
// Optional extra fields
Prompt  string `json:"prompt,omitempty"`
WorkDir string `json:"work_dir,omitempty"`
}

// Runner listens on a registered Hub agent and dispatches local agent tasks.
// One Runner manages a single registered agent identity; multiple Runner
// instances can be created to provide a pool of local agents.
type Runner struct {
agentID string
hub     Hub
cfg     AgentConfig

mu     sync.Mutex
tasks  map[string]*TaskState
cancel context.CancelFunc
}

// NewRunner creates a Runner that will register itself in hub under the given
// agentID and name.  If agentID is empty a random UUID is generated.
func NewRunner(hub Hub, agentID, agentName, role string, cfg AgentConfig) *Runner {
if agentID == "" {
agentID = uuid.New().String()
}
if agentName == "" {
agentName = "local-agent"
}
if role == "" {
role = "SOFTWARE_ENGINEER"
}
return &Runner{
agentID: agentID,
hub:     hub,
cfg:     cfg,
tasks:   make(map[string]*TaskState),
}
}

// AgentID returns the agent identifier this runner registered under.
func (r *Runner) AgentID() string { return r.agentID }

// Start registers the agent in the Hub and begins processing messages.
// It blocks until ctx is cancelled.
func (r *Runner) Start(ctx context.Context) {
ctx, cancel := context.WithCancel(ctx)
r.cancel = cancel
defer cancel()

// Register in Hub.
r.hub.RegisterAgent(HubAgent{
ID:           r.agentID,
Name:         "local-agent",
Role:         "SOFTWARE_ENGINEER",
Status:       HubStatusIdle,
ProviderType: "builtin",
})

slog.Info("local agent runner: registered", "agent_id", r.agentID)

// Subscribe to message notifications.
notifyCh, unsubscribe := r.hub.Subscribe(r.agentID)
defer unsubscribe()

ticker := time.NewTicker(5 * time.Second)
defer ticker.Stop()

for {
select {
case <-ctx.Done():
return
case <-notifyCh:
r.drainInbox(ctx)
case <-ticker.C:
// Periodic drain in case notifications were missed.
r.drainInbox(ctx)
}
}
}

// Stop cancels the runner context, causing Start to return.
func (r *Runner) Stop() {
if r.cancel != nil {
r.cancel()
}
}

// ActiveTasks returns a snapshot of the currently managed tasks.
func (r *Runner) ActiveTasks() []*TaskState {
r.mu.Lock()
defer r.mu.Unlock()
out := make([]*TaskState, 0, len(r.tasks))
for _, ts := range r.tasks {
out = append(out, ts)
}
return out
}

// KillTask aborts a running task by ID.
func (r *Runner) KillTask(taskID string) bool {
r.mu.Lock()
ts, ok := r.tasks[taskID]
r.mu.Unlock()
if !ok {
return false
}
ts.Kill()
return true
}

// drainInbox reads all pending messages from the Hub and dispatches tasks.
func (r *Runner) drainInbox(ctx context.Context) {
messages := r.hub.Inbox(r.agentID)
for _, msg := range messages {
switch msg.Type {
case "TaskAssignment":
r.handleTaskAssignment(ctx, msg)
case "Kill":
var payload struct {
TaskID string `json:"task_id"`
}
if err := json.Unmarshal([]byte(msg.Content), &payload); err == nil && payload.TaskID != "" {
r.KillTask(payload.TaskID)
}
default:
slog.Debug("local agent runner: unhandled message type", "type", msg.Type, "from", msg.FromAgent)
}
}
}

// handleTaskAssignment decodes and spawns a local agent for a task assignment.
func (r *Runner) handleTaskAssignment(ctx context.Context, msg HubMessage) {
var payload TaskAssignmentPayload
if err := json.Unmarshal([]byte(msg.Content), &payload); err != nil {
slog.Error("local agent runner: failed to parse TaskAssignment", "err", err)
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
workDir := payload.WorkDir
if workDir == "" {
workDir, _ = os.Getwd()
}

// Update Hub status to active.
r.hub.RegisterAgent(HubAgent{
ID:           r.agentID,
Name:         "local-agent",
Role:         "SOFTWARE_ENGINEER",
Status:       HubStatusActive,
ProviderType: "builtin",
})

state, err := SpawnTask(ctx, description, prompt, workDir, r.cfg)
if err != nil {
slog.Error("local agent runner: failed to spawn task", "err", err)
r.hub.RegisterAgent(HubAgent{
ID:           r.agentID,
Name:         "local-agent",
Role:         "SOFTWARE_ENGINEER",
Status:       HubStatusIdle,
ProviderType: "builtin",
})
return
}

r.mu.Lock()
r.tasks[state.ID] = state
r.mu.Unlock()

slog.Info("local agent runner: task started", "task_id", state.ID, "description", description)

// Watch the task completion in a separate goroutine so Start doesn't block.
go r.watchTask(ctx, state, msg)
}

// watchTask polls the task state and sends a completion message back to the Hub.
func (r *Runner) watchTask(ctx context.Context, state *TaskState, originMsg HubMessage) {
ticker := time.NewTicker(1 * time.Second)
defer ticker.Stop()

for {
select {
case <-ctx.Done():
return
case <-ticker.C:
status := state.Status()
if !status.IsTerminal() {
continue
}

// Task has finished.
r.mu.Lock()
delete(r.tasks, state.ID)
r.mu.Unlock()

r.hub.RegisterAgent(HubAgent{
ID:           r.agentID,
Name:         "local-agent",
Role:         "SOFTWARE_ENGINEER",
Status:       HubStatusIdle,
ProviderType: "builtin",
})

r.publishCompletion(state, originMsg)
return
}
}
}

// publishCompletion sends the task result back to the originating agent/system via the Hub.
func (r *Runner) publishCompletion(state *TaskState, originMsg HubMessage) {
status := state.Status()
var summaryText string
switch status {
case TaskStatusCompleted:
summaryText = fmt.Sprintf("Task %q completed successfully.", state.Description)
if res := state.Result(); res != "" {
summaryText += " Result: " + truncate(res, 500)
}
case TaskStatusFailed:
summaryText = fmt.Sprintf("Task %q failed: %s", state.Description, state.Err())
case TaskStatusKilled:
summaryText = fmt.Sprintf("Task %q was killed.", state.Description)
}

notification := buildNotification(state, status, summaryText)

replyTo := originMsg.FromAgent
if replyTo == "" || replyTo == "SYSTEM" {
slog.Info("local agent runner: task complete", "task_id", state.ID, "status", status, "summary", summaryText)
return
}

reply := HubMessage{
ID:        "task-result-" + state.ID,
FromAgent: r.agentID,
ToAgent:   replyTo,
Type:      "TaskResult",
Content:   notification,
}
if err := r.hub.Publish(reply); err != nil {
slog.Error("local agent runner: failed to publish task result", "err", err, "task_id", state.ID)
}
}

// buildPrompt constructs a task prompt from the TaskAssignment payload.
func buildPrompt(p TaskAssignmentPayload) string {
prompt := p.Directive
if p.IssueName != "" {
prompt = fmt.Sprintf("Issue: %s\n\n%s", p.IssueName, p.Directive)
}
return prompt
}

// buildNotification constructs a JSON-encoded TaskNotification that is sent
// back to the parent agent via the Hub.  Replaces the legacy XML format.
func buildNotification(state *TaskState, status TaskStatus, summary string) string {
	progress := state.Progress()
	durationMs := int64(0)
	if v := state.endAt.Load(); v != nil {
		durationMs = v.(time.Time).Sub(state.startAt).Milliseconds()
	}

	result := ""
	if res := state.Result(); res != "" {
		result = truncate(res, 2000)
	}

	n := TaskNotificationPayload{
		TaskID:     state.ID,
		ToolUseID:  state.ToolUseID,
		OutputFile: state.OutputFile,
		Status:     string(status),
		Summary:    truncate(summary, 1000),
		Result:     result,
		TokenCount: progress.TokenCount,
		ToolUses:   int64(progress.ToolUseCount),
		DurationMs: durationMs,
	}

	b, err := json.Marshal(n)
	if err != nil {
		return fmt.Sprintf(`{"task_id":%q,"status":%q}`, state.ID, status)
	}
	return string(b)
}

func truncate(s string, max int) string {
	if len(s) <= max {
		return s
	}
	return s[:max] + "…"
}

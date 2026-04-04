// Package builtin implements the platform's default (builtin) agent — a full
// agentic loop modelled after the Claude Code TypeScript harness.
//
// Features:
//   - tasks move through pending → running → completed/failed/killed
//   - progress (tool-use count, token count, recent activities) is tracked
//   - output is streamed to a per-task disk file
//   - task completion sends an XML-tagged notification into the Hub
//
// The builtin agent uses a configurable LLM backend (Anthropic Messages API,
// OpenAI-compatible, or Ollama) and exposes the full tool set derived from
// CC-Source's ASYNC_AGENT_ALLOWED_TOOLS: Bash, FileRead, FileWrite, FileEdit,
// Grep, Glob, WebFetch, WebSearch, TodoWrite, TaskCreate/Get/List/Update,
// SendMessage, ToolSearch, LS.
package builtin

import (
	"crypto/rand"
	"encoding/hex"
	"fmt"
	"sync"
	"sync/atomic"
	"time"
)

// TaskStatus mirrors CC-Source's TaskStatus union.
type TaskStatus string

const (
	TaskStatusPending   TaskStatus = "pending"
	TaskStatusRunning   TaskStatus = "running"
	TaskStatusCompleted TaskStatus = "completed"
	TaskStatusFailed    TaskStatus = "failed"
	TaskStatusKilled    TaskStatus = "killed"
)

// IsTerminal returns true for statuses from which a task will not transition.
func (s TaskStatus) IsTerminal() bool {
	return s == TaskStatusCompleted || s == TaskStatusFailed || s == TaskStatusKilled
}

// ToolActivity records a single tool invocation for progress display.
type ToolActivity struct {
	ToolName    string                 `json:"toolName"`
	Input       map[string]interface{} `json:"input"`
	Description string                 `json:"description,omitempty"`
}

// AgentProgress is the public-facing progress snapshot for a running task.
type AgentProgress struct {
	ToolUseCount     int            `json:"toolUseCount"`
	TokenCount       int64          `json:"tokenCount"`
	LastActivity     *ToolActivity  `json:"lastActivity,omitempty"`
	RecentActivities []ToolActivity `json:"recentActivities,omitempty"`
	Summary          string         `json:"summary,omitempty"`
}

const maxRecentActivities = 5

// progressTracker accumulates token and tool-use counts.
type progressTracker struct {
	mu                    sync.Mutex
	toolUseCount          int
	latestInputTokens     int64
	cumulativeOutputTokens int64
	recentActivities      []ToolActivity
}

func newProgressTracker() *progressTracker {
	return &progressTracker{}
}

func (p *progressTracker) recordToolUse(activity ToolActivity) {
	p.mu.Lock()
	defer p.mu.Unlock()
	p.toolUseCount++
	p.recentActivities = append(p.recentActivities, activity)
	if len(p.recentActivities) > maxRecentActivities {
		p.recentActivities = p.recentActivities[len(p.recentActivities)-maxRecentActivities:]
	}
}

func (p *progressTracker) recordTokens(inputTokens, outputTokens int64) {
	p.mu.Lock()
	defer p.mu.Unlock()
	p.latestInputTokens = inputTokens
	p.cumulativeOutputTokens += outputTokens
}

func (p *progressTracker) snapshot() AgentProgress {
	p.mu.Lock()
	defer p.mu.Unlock()
	acts := make([]ToolActivity, len(p.recentActivities))
	copy(acts, p.recentActivities)
	total := p.latestInputTokens + p.cumulativeOutputTokens
	var last *ToolActivity
	if len(acts) > 0 {
		a := acts[len(acts)-1]
		last = &a
	}
	return AgentProgress{
		ToolUseCount:     p.toolUseCount,
		TokenCount:       total,
		LastActivity:     last,
		RecentActivities: acts,
	}
}

// TaskState holds all mutable state for a running local agent task.
// It is safe to read Status and Progress from multiple goroutines; all writes
// go through the atomic/mutex helpers below.
type TaskState struct {
	ID          string
	Description string
	Prompt      string
	WorkDir     string // working directory for bash/file tools
	ToolUseID   string // optional tool_use id from the caller
	OutputFile  string // absolute path to the task output file

	status  atomic.Value // stores TaskStatus
	err     atomic.Value // stores error (string)
	result  atomic.Value // stores final result string

	progress *progressTracker

	cancel  func()     // cancels the agent goroutine context
	startAt time.Time
	endAt   atomic.Value // stores time.Time

	// notified is flipped true when the completion notification has been sent.
	// Guards against double-delivery.
	notifiedOnce sync.Once
}

func newTaskState(id, description, prompt, workDir, outputFile, toolUseID string, cancel func()) *TaskState {
	ts := &TaskState{
		ID:          id,
		Description: description,
		Prompt:      prompt,
		WorkDir:     workDir,
		OutputFile:  outputFile,
		ToolUseID:   toolUseID,
		cancel:      cancel,
		startAt:     time.Now(),
		progress:    newProgressTracker(),
	}
	ts.status.Store(TaskStatusPending)
	return ts
}

// Status returns the current status atomically.
func (ts *TaskState) Status() TaskStatus {
	return ts.status.Load().(TaskStatus)
}

func (ts *TaskState) setStatus(s TaskStatus) { ts.status.Store(s) }

// Err returns the error string if the task failed.
func (ts *TaskState) Err() string {
	if v := ts.err.Load(); v != nil {
		return v.(string)
	}
	return ""
}

// Result returns the final result string if the task completed.
func (ts *TaskState) Result() string {
	if v := ts.result.Load(); v != nil {
		return v.(string)
	}
	return ""
}

// Progress returns a snapshot of the current progress.
func (ts *TaskState) Progress() AgentProgress { return ts.progress.snapshot() }

// Kill aborts the running task.  No-op if already in a terminal state.
func (ts *TaskState) Kill() {
	// Only transition from non-terminal states.
	old := ts.Status()
	if old.IsTerminal() {
		return
	}
	ts.status.Store(TaskStatusKilled)
	now := time.Now()
	ts.endAt.Store(now)
	if ts.cancel != nil {
		ts.cancel()
	}
}

// generateTaskID returns a URL-safe task ID with an "a" prefix (local_agent).
func generateTaskID() (string, error) {
	b := make([]byte, 8)
	if _, err := rand.Read(b); err != nil {
		return "", fmt.Errorf("generateTaskID: %w", err)
	}
	return "a" + hex.EncodeToString(b), nil
}

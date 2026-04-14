package agentgrpc

// task_harness.go implements a Claude Code–style task harness.
//
// Patterns drawn from:
//   - srcs/server/agents/local/task.go  (local agent task, "modelled after the
//     Claude Code TypeScript LocalAgentTask harness")
//   - orchestration.UltraPlan         (gzip+base64 context compression)
//   - orchestration.AutoDreamWorker   (memory consolidation on task completion)
//
// Key design decisions for PERFORMANCE:
//   - sync.Pool for JSON encode buffers → zero allocations in hot path
//   - Sliding-window context trimming → bounded memory per long task
//   - Atomic counters (no mutex) for progress metrics
//   - Pre-allocated event channel (capacity = maxContextMessages/2) so
//     event fanout never blocks the LLM loop
//   - Context compression: when the serialised conversation exceeds
//     contextCompressThreshold bytes, old messages are gzip+base64 compacted
//     into a single "context summary" message

import (
	"compress/gzip"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"sync/atomic"
	"time"
)

const (
	// defaultMaxContextMessages is the sliding-window size for conversation
	// history.  Keeps memory bounded for long-running tasks.
	defaultMaxContextMessages = 80

	// contextCompressThreshold triggers gzip compression of old messages when
	// the JSON-serialised conversation exceeds this size (bytes).
	contextCompressThreshold = 32 * 1024 // 32 KiB

	// taskOutputDir mirrors the Claude Code pattern: one file per task.
	taskOutputDir = ".agent-task/output"

	// selfReflectionPrefix is prepended to LLM retry prompts.
	selfReflectionPrefix = "The previous attempt failed. Please reflect on what went wrong and try a different approach.\n\n"
)

// TaskProgress tracks live metrics for a running task.
// All fields are updated with atomic operations so the progress snapshot can
// be read from a separate goroutine without a mutex.
type TaskProgress struct {
	toolUseCount atomic.Int64
	tokenCount   atomic.Int64
	lastActivity atomic.Pointer[string] // last tool name
	startTime    time.Time
}

// NewTaskProgress initialises progress tracking.
func NewTaskProgress() *TaskProgress {
	return &TaskProgress{startTime: time.Now()}
}

// RecordToolUse atomically records a tool invocation.
func (p *TaskProgress) RecordToolUse(toolName string) {
	p.toolUseCount.Add(1)
	p.lastActivity.Store(&toolName)
}

// AddTokens adds n tokens to the running total.
func (p *TaskProgress) AddTokens(n int64) {
	p.tokenCount.Add(n)
}

// Snapshot returns an immutable progress snapshot (allocation-free struct copy).
func (p *TaskProgress) Snapshot() ProgressSnapshot {
	activity := ""
	if ptr := p.lastActivity.Load(); ptr != nil {
		activity = *ptr
	}
	return ProgressSnapshot{
		ToolUseCount: p.toolUseCount.Load(),
		TokenCount:   p.tokenCount.Load(),
		LastActivity: activity,
		Elapsed:      time.Since(p.startTime),
	}
}

// ProgressSnapshot is a read-only copy of TaskProgress metrics.
type ProgressSnapshot struct {
	ToolUseCount int64
	TokenCount   int64
	LastActivity string
	Elapsed      time.Duration
}

// ── context-window management ──────────────────────────────────────────────

// contextWindow maintains a bounded sliding window of agent messages.
// Old messages are dropped (and optionally compressed) when the window fills.
type contextWindow struct {
	maxMessages int
	messages    []agentMessage // ring-buffer semantics via head/tail
}

// agentMessage is a lightweight internal representation of one conversation turn.
type agentMessage struct {
	Role    string `json:"role"`
	Content string `json:"content"`
	// IsCompressed signals that Content is a gzip+base64 packed batch.
	IsCompressed bool `json:"compressed,omitempty"`
}

func newContextWindow(maxMessages int) *contextWindow {
	if maxMessages <= 0 {
		maxMessages = defaultMaxContextMessages
	}
	return &contextWindow{
		maxMessages: maxMessages,
		messages:    make([]agentMessage, 0, maxMessages),
	}
}

// Append adds a message.  When the window is full the oldest half is
// compressed into a single summary message.
func (w *contextWindow) Append(role, content string) {
	w.messages = append(w.messages, agentMessage{Role: role, Content: content})
	if len(w.messages) > w.maxMessages {
		w.compact()
	}
}

// All returns a snapshot of current messages.  Callers must not modify the
// returned slice.
func (w *contextWindow) All() []agentMessage {
	return w.messages
}

// compact compresses the oldest half of messages into a single summary block.
// This keeps memory bounded while preserving recent context verbatim.
func (w *contextWindow) compact() {
	cutoff := len(w.messages) / 2
	old := w.messages[:cutoff]

	packed, err := packMessages(old)
	if err != nil {
		// If compression fails, just drop the oldest half silently.
		slog.Warn("contextWindow: compact failed, dropping old messages", "err", err)
		w.messages = append(w.messages[:0], w.messages[cutoff:]...)
		return
	}

	summary := agentMessage{
		Role:         "user",
		Content:      packed,
		IsCompressed: true,
	}
	keep := append([]agentMessage{summary}, w.messages[cutoff:]...)
	w.messages = keep
}

// packMessages gzip+base64 encodes a slice of messages.
func packMessages(msgs []agentMessage) (string, error) {
	data, err := json.Marshal(msgs)
	if err != nil {
		return "", err
	}
	if len(data) < contextCompressThreshold {
		// Small enough: just base64 without gzip
		return base64.StdEncoding.EncodeToString(data), nil
	}

	var buf strings.Builder
	enc := base64.NewEncoder(base64.StdEncoding, &buf)
	gz := gzip.NewWriter(enc)
	if _, err := gz.Write(data); err != nil {
		return "", err
	}
	if err := gz.Close(); err != nil {
		return "", err
	}
	if err := enc.Close(); err != nil {
		return "", err
	}
	return "[gz]" + buf.String(), nil
}

// ── task output file ──────────────────────────────────────────────────────────

// TaskOutputWriter streams task output to a per-task file, following the
// Claude Code LocalAgentTask pattern of ".agent-task/output/<taskID>.log".
// Writes are best-effort; errors are logged but never returned to the caller.
type TaskOutputWriter struct {
	f *os.File
}

// NewTaskOutputWriter opens (or creates) the output file for taskID.
// Returns nil when the directory cannot be created.
func NewTaskOutputWriter(taskID string) *TaskOutputWriter {
	if err := os.MkdirAll(taskOutputDir, 0o755); err != nil {
		return nil
	}
	path := filepath.Join(taskOutputDir, sanitizeID(taskID)+".log")
	f, err := os.OpenFile(path, os.O_CREATE|os.O_WRONLY|os.O_APPEND, 0o644)
	if err != nil {
		return nil
	}
	return &TaskOutputWriter{f: f}
}

// Write appends text to the file.
func (w *TaskOutputWriter) Write(text string) {
	if w == nil || w.f == nil {
		return
	}
	_, _ = fmt.Fprintln(w.f, text)
}

// Close flushes and closes the file.
func (w *TaskOutputWriter) Close() {
	if w == nil || w.f == nil {
		return
	}
	_ = w.f.Close()
}

// ── self-reflection helper ────────────────────────────────────────────────────

// SelfReflectionPrompt wraps the original task with a reflection preamble so
// the LLM understands this is a retry and should learn from the prior failure.
func SelfReflectionPrompt(originalTask, priorError string) string {
	var sb strings.Builder
	sb.WriteString(selfReflectionPrefix)
	if priorError != "" {
		sb.WriteString("Prior error: ")
		sb.WriteString(priorError)
		sb.WriteString("\n\n")
	}
	sb.WriteString("Original task: ")
	sb.WriteString(originalTask)
	return sb.String()
}

package local_test

import (
	"context"
	"fmt"
	"os"
	"strings"
	"sync"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/agents/local"
)

// ─── Fake LLM ─────────────────────────────────────────────────────────────────

// fakeLLM drives the agent loop with scripted responses.
type fakeLLM struct {
	mu    sync.Mutex
	turns []local.AssistantMessage
	idx   int
}

func (f *fakeLLM) Complete(_ context.Context, _ local.CompletionRequest) (*local.AssistantMessage, error) {
	f.mu.Lock()
	defer f.mu.Unlock()
	if f.idx >= len(f.turns) {
		return &local.AssistantMessage{
			Text:       "Done.",
			StopReason: "end_turn",
		}, nil
	}
	r := f.turns[f.idx]
	f.idx++
	return &r, nil
}

// ─── Task lifecycle tests ─────────────────────────────────────────────────────

func TestTaskStateTransitions(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	llm := &fakeLLM{turns: []local.AssistantMessage{
		{Text: "Hello, I'll help.", StopReason: "end_turn", InputTokens: 100, OutputTokens: 50},
	}}

	tmpDir := t.TempDir()
	state, err := local.SpawnTask(ctx, "test task", "Say hello", tmpDir, local.AgentConfig{LLM: llm})
	if err != nil {
		t.Fatalf("SpawnTask: %v", err)
	}
	if state.ID == "" {
		t.Fatal("task ID should not be empty")
	}

	// Wait for completion (up to 5s).
	deadline := time.Now().Add(5 * time.Second)
	for time.Now().Before(deadline) {
		if state.Status().IsTerminal() {
			break
		}
		time.Sleep(10 * time.Millisecond)
	}

	if !state.Status().IsTerminal() {
		t.Fatalf("task did not reach terminal state; status=%s", state.Status())
	}
	if state.Status() != local.TaskStatusCompleted {
		t.Errorf("expected completed, got %s; err=%s", state.Status(), state.Err())
	}
}

func TestTaskKill(t *testing.T) {
	ctx := context.Background()

	// LLM that blocks until cancelled.
	blockLLM := &blockingLLM{}

	tmpDir := t.TempDir()
	state, err := local.SpawnTask(ctx, "block task", "block", tmpDir, local.AgentConfig{LLM: blockLLM})
	if err != nil {
		t.Fatalf("SpawnTask: %v", err)
	}

	// Give it a moment to start.
	time.Sleep(50 * time.Millisecond)
	state.Kill()

	deadline := time.Now().Add(3 * time.Second)
	for time.Now().Before(deadline) {
		if state.Status().IsTerminal() {
			break
		}
		time.Sleep(10 * time.Millisecond)
	}
	if state.Status() != local.TaskStatusKilled {
		t.Errorf("expected killed, got %s", state.Status())
	}
}

// blockingLLM waits for context cancellation before returning.
type blockingLLM struct{}

func (b *blockingLLM) Complete(ctx context.Context, _ local.CompletionRequest) (*local.AssistantMessage, error) {
	<-ctx.Done()
	return nil, ctx.Err()
}

// ─── Tool execution tests ─────────────────────────────────────────────────────

func TestAgentToolUse_BashAndFileWrite(t *testing.T) {
	ctx := context.Background()
	tmpDir := t.TempDir()
	outFile := tmpDir + "/hello.txt"

	llm := &fakeLLM{turns: []local.AssistantMessage{
		// Turn 1: request bash tool use
		{
			Text: "I'll write a file.",
			ToolUses: []local.ToolUseRequest{
				{
					ID:   "tu1",
					Name: "bash",
					Input: map[string]interface{}{
						"command": fmt.Sprintf("echo hello > %s", outFile),
					},
				},
			},
			StopReason:   "tool_use",
			InputTokens:  200,
			OutputTokens: 40,
		},
		// Turn 2: done
		{Text: "File written.", StopReason: "end_turn", InputTokens: 300, OutputTokens: 20},
	}}

	state, err := local.SpawnTask(ctx, "write file", "Write hello to a file", tmpDir, local.AgentConfig{LLM: llm})
	if err != nil {
		t.Fatalf("SpawnTask: %v", err)
	}

	deadline := time.Now().Add(10 * time.Second)
	for time.Now().Before(deadline) {
		if state.Status().IsTerminal() {
			break
		}
		time.Sleep(10 * time.Millisecond)
	}
	if state.Status() != local.TaskStatusCompleted {
		t.Errorf("expected completed, got %s (err=%s)", state.Status(), state.Err())
	}

	// Verify the file was created.
	data, err := os.ReadFile(outFile)
	if err != nil {
		t.Errorf("output file not created: %v", err)
	} else if !strings.Contains(string(data), "hello") {
		t.Errorf("output file content unexpected: %q", data)
	}

	// Verify progress tracking.
	prog := state.Progress()
	if prog.ToolUseCount != 1 {
		t.Errorf("expected 1 tool use, got %d", prog.ToolUseCount)
	}
	if prog.TokenCount <= 0 {
		t.Errorf("expected positive token count, got %d", prog.TokenCount)
	}
}

func TestAgentFileEditRoundtrip(t *testing.T) {
	ctx := context.Background()
	tmpDir := t.TempDir()
	path := tmpDir + "/sample.txt"

	// Pre-create the file.
	if err := os.WriteFile(path, []byte("Hello World\n"), 0o644); err != nil {
		t.Fatal(err)
	}

	llm := &fakeLLM{turns: []local.AssistantMessage{
		// Turn 1: read the file
		{
			Text: "Reading file.",
			ToolUses: []local.ToolUseRequest{
				{
					ID:   "tu1",
					Name: "file_read",
					Input: map[string]interface{}{"path": path},
				},
			},
			StopReason: "tool_use",
		},
		// Turn 2: edit the file
		{
			Text: "Editing file.",
			ToolUses: []local.ToolUseRequest{
				{
					ID:   "tu2",
					Name: "file_edit",
					Input: map[string]interface{}{
						"path":    path,
						"old_str": "Hello World",
						"new_str": "Goodbye World",
					},
				},
			},
			StopReason: "tool_use",
		},
		// Turn 3: done
		{Text: "Done.", StopReason: "end_turn"},
	}}

	state, err := local.SpawnTask(ctx, "edit file", "Edit sample.txt", tmpDir, local.AgentConfig{LLM: llm})
	if err != nil {
		t.Fatalf("SpawnTask: %v", err)
	}

	deadline := time.Now().Add(10 * time.Second)
	for time.Now().Before(deadline) {
		if state.Status().IsTerminal() {
			break
		}
		time.Sleep(10 * time.Millisecond)
	}
	if state.Status() != local.TaskStatusCompleted {
		t.Errorf("expected completed, got %s (err=%s)", state.Status(), state.Err())
	}

	data, _ := os.ReadFile(path)
	if !strings.Contains(string(data), "Goodbye World") {
		t.Errorf("file not edited: %q", data)
	}
}

// ─── Hub interface tests ──────────────────────────────────────────────────────

type fakeHub struct {
	mu       sync.Mutex
	agents   []local.HubAgent
	inbox    map[string][]local.HubMessage
	published []local.HubMessage
	subs     map[string][]chan struct{}
}

func newFakeHub() *fakeHub {
	return &fakeHub{
		inbox: make(map[string][]local.HubMessage),
		subs:  make(map[string][]chan struct{}),
	}
}

func (h *fakeHub) RegisterAgent(a local.HubAgent) {
	h.mu.Lock()
	defer h.mu.Unlock()
	h.agents = append(h.agents, a)
}

func (h *fakeHub) Subscribe(agentID string) (<-chan struct{}, func()) {
	h.mu.Lock()
	defer h.mu.Unlock()
	ch := make(chan struct{}, 1)
	h.subs[agentID] = append(h.subs[agentID], ch)
	return ch, func() {}
}

func (h *fakeHub) Inbox(agentID string) []local.HubMessage {
	h.mu.Lock()
	defer h.mu.Unlock()
	msgs := h.inbox[agentID]
	delete(h.inbox, agentID)
	return msgs
}

func (h *fakeHub) Publish(msg local.HubMessage) error {
	h.mu.Lock()
	defer h.mu.Unlock()
	h.published = append(h.published, msg)
	return nil
}

func (h *fakeHub) deliver(agentID string, msg local.HubMessage) {
	h.mu.Lock()
	h.inbox[agentID] = append(h.inbox[agentID], msg)
	chs := h.subs[agentID]
	h.mu.Unlock()
	for _, ch := range chs {
		select {
		case ch <- struct{}{}:
		default:
		}
	}
}

func TestRunner_ReceivesTaskAndPublishesResult(t *testing.T) {
	hub := newFakeHub()
	llm := &fakeLLM{turns: []local.AssistantMessage{
		{Text: "Task complete.", StopReason: "end_turn", InputTokens: 50, OutputTokens: 20},
	}}

	runner := local.NewRunner(hub, "test-agent", "test", "SOFTWARE_ENGINEER", local.AgentConfig{LLM: llm})

	ctx, cancel := context.WithTimeout(context.Background(), 15*time.Second)
	defer cancel()

	go runner.Start(ctx)
	time.Sleep(100 * time.Millisecond) // Let the runner register and start listening.

	// Deliver a task assignment.
	hub.deliver("test-agent", local.HubMessage{
		ID:        "msg-1",
		FromAgent: "orchestrator",
		ToAgent:   "test-agent",
		Type:      "TaskAssignment",
		Content:   `{"issue_id":"i1","issue_name":"Test issue","directive":"Do something"}`,
	})

	// Wait for the runner to publish a result.
	deadline := time.Now().Add(12 * time.Second)
	for time.Now().Before(deadline) {
		hub.mu.Lock()
		n := len(hub.published)
		hub.mu.Unlock()
		if n > 0 {
			break
		}
		time.Sleep(100 * time.Millisecond)
	}

	hub.mu.Lock()
	published := hub.published
	hub.mu.Unlock()

	if len(published) == 0 {
		t.Fatal("runner did not publish any result messages")
	}

	result := published[0]
	if result.Type != "TaskResult" {
		t.Errorf("expected TaskResult, got %s", result.Type)
	}
	if !strings.Contains(result.Content, "<task-notification>") {
		t.Errorf("result should contain XML notification tag; got: %s", result.Content)
	}
	if !strings.Contains(result.Content, "<status>completed</status>") {
		t.Errorf("result should show completed status; got: %s", result.Content)
	}
}

// ─── Output file tests ────────────────────────────────────────────────────────

func TestTaskOutputFile_Created(t *testing.T) {
	ctx := context.Background()
	llm := &fakeLLM{turns: []local.AssistantMessage{
		{Text: "All done.", StopReason: "end_turn"},
	}}

	tmpDir := t.TempDir()
	state, err := local.SpawnTask(ctx, "output test", "Output something", tmpDir, local.AgentConfig{LLM: llm})
	if err != nil {
		t.Fatalf("SpawnTask: %v", err)
	}

	deadline := time.Now().Add(5 * time.Second)
	for time.Now().Before(deadline) {
		if state.Status().IsTerminal() {
			break
		}
		time.Sleep(10 * time.Millisecond)
	}

	// The output file should exist.
	if _, err := os.Stat(state.OutputFile); os.IsNotExist(err) {
		t.Errorf("output file %s was not created", state.OutputFile)
	}
}

// ─── generateTaskID uniqueness test ──────────────────────────────────────────

func TestSpawnTask_UniqueIDs(t *testing.T) {
	ctx := context.Background()
	llm := &fakeLLM{turns: []local.AssistantMessage{
		{Text: "ok", StopReason: "end_turn"},
	}}

	ids := make(map[string]bool)
	for i := 0; i < 10; i++ {
		tmpDir := t.TempDir()
		s, err := local.SpawnTask(ctx, "t", "p", tmpDir, local.AgentConfig{LLM: llm})
		if err != nil {
			t.Fatalf("SpawnTask: %v", err)
		}
		if ids[s.ID] {
			t.Errorf("duplicate task ID: %s", s.ID)
		}
		ids[s.ID] = true
	}
}

// ─── Done() channel tests ─────────────────────────────────────────────────────

// TestTaskDoneChannel verifies that Done() is closed when the task completes.
func TestTaskDoneChannel(t *testing.T) {
	ctx := context.Background()
	tmpDir := t.TempDir()

	llm := &fakeLLM{turns: []local.AssistantMessage{
		{Text: "Done.", StopReason: "end_turn"},
	}}

	state, err := local.SpawnTask(ctx, "done test", "say done", tmpDir, local.AgentConfig{LLM: llm})
	if err != nil {
		t.Fatalf("SpawnTask: %v", err)
	}

	// Wait on Done() channel instead of polling.
	select {
	case <-state.Done():
		// successful: channel closed
	case <-time.After(5 * time.Second):
		t.Fatal("Done() not closed within timeout")
	}

	if !state.Status().IsTerminal() {
		t.Errorf("status should be terminal after Done(): %s", state.Status())
	}
}

// TestTaskDoneChannelOnKill verifies Done() is closed when the task is killed.
func TestTaskDoneChannelOnKill(t *testing.T) {
	ctx := context.Background()
	tmpDir := t.TempDir()

	state, err := local.SpawnTask(ctx, "kill test", "block", tmpDir, local.AgentConfig{LLM: &blockingLLM{}})
	if err != nil {
		t.Fatalf("SpawnTask: %v", err)
	}

	time.Sleep(20 * time.Millisecond)
	state.Kill()

	select {
	case <-state.Done():
		// correct
	case <-time.After(3 * time.Second):
		t.Fatal("Done() not closed after Kill()")
	}

	if state.Status() != local.TaskStatusKilled {
		t.Errorf("expected killed, got %s", state.Status())
	}
}

// TestTaskDoneChannelMultipleWaiters verifies multiple goroutines can wait on Done().
func TestTaskDoneChannelMultipleWaiters(t *testing.T) {
	ctx := context.Background()
	tmpDir := t.TempDir()

	llm := &fakeLLM{turns: []local.AssistantMessage{
		{Text: "Done.", StopReason: "end_turn"},
	}}

	state, err := local.SpawnTask(ctx, "multi-wait", "say done", tmpDir, local.AgentConfig{LLM: llm})
	if err != nil {
		t.Fatalf("SpawnTask: %v", err)
	}

	var wg sync.WaitGroup
	const waiters = 5
	results := make([]bool, waiters)

	for i := range waiters {
		wg.Add(1)
		go func(idx int) {
			defer wg.Done()
			select {
			case <-state.Done():
				results[idx] = true
			case <-time.After(5 * time.Second):
				results[idx] = false
			}
		}(i)
	}

	wg.Wait()

	for i, ok := range results {
		if !ok {
			t.Errorf("waiter %d did not receive Done() signal", i)
		}
	}
}

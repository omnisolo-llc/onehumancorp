package builtin_test

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/agents/builtin"
)

// ─── FileReadTool line number tests ──────────────────────────────────────────

func TestFileReadTool_LineNumbers(t *testing.T) {
	dir := t.TempDir()
	path := filepath.Join(dir, "test.txt")
	if err := os.WriteFile(path, []byte("line one\nline two\nline three\n"), 0o644); err != nil {
		t.Fatal(err)
	}

	llm := &fakeLLM{turns: []builtin.AssistantMessage{
		{
			Text: "Reading file.",
			ToolUses: []builtin.ToolUseRequest{
				{ID: "r1", Name: "file_read", Input: map[string]interface{}{"path": path}},
			},
			StopReason: "tool_use",
		},
		{Text: "Done.", StopReason: "end_turn"},
	}}

	state, err := builtin.SpawnTask(context.Background(), "test", "read file", dir, builtin.AgentConfig{LLM: llm})
	if err != nil {
		t.Fatalf("SpawnTask: %v", err)
	}
	waitForTerminal(t, state, 5)

	// Verify the output file contains line numbers.
	data, _ := os.ReadFile(state.OutputFile)
	if !strings.Contains(string(data), "1. line one") {
		t.Errorf("expected '1. line one' in output; got: %s", string(data))
	}
}

// ─── LSTool tests ─────────────────────────────────────────────────────────────

func TestLSTool(t *testing.T) {
	dir := t.TempDir()
	if err := os.WriteFile(filepath.Join(dir, "hello.go"), []byte("package main"), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := os.Mkdir(filepath.Join(dir, "subdir"), 0o755); err != nil {
		t.Fatal(err)
	}

	llm := &fakeLLM{turns: []builtin.AssistantMessage{
		{
			ToolUses: []builtin.ToolUseRequest{
				{ID: "ls1", Name: "ls", Input: map[string]interface{}{"path": dir}},
			},
			StopReason: "tool_use",
		},
		{Text: "Saw files.", StopReason: "end_turn"},
	}}

	state, err := builtin.SpawnTask(context.Background(), "test", "list dir", dir, builtin.AgentConfig{LLM: llm})
	if err != nil {
		t.Fatalf("SpawnTask: %v", err)
	}
	waitForTerminal(t, state, 5)

	data, _ := os.ReadFile(state.OutputFile)
	output := string(data)
	if !strings.Contains(output, "hello.go") {
		t.Errorf("expected 'hello.go' in ls output; got: %s", output)
	}
	if !strings.Contains(output, "subdir") {
		t.Errorf("expected 'subdir' in ls output; got: %s", output)
	}
}

// ─── ToolSearchTool tests ─────────────────────────────────────────────────────

func TestToolSearchTool_ListsAllTools(t *testing.T) {
	dir := t.TempDir()

	llm := &fakeLLM{turns: []builtin.AssistantMessage{
		{
			ToolUses: []builtin.ToolUseRequest{
				{ID: "ts1", Name: "tool_search", Input: map[string]interface{}{}},
			},
			StopReason: "tool_use",
		},
		{Text: "Done.", StopReason: "end_turn"},
	}}

	state, err := builtin.SpawnTask(context.Background(), "test", "list tools", dir, builtin.AgentConfig{LLM: llm})
	if err != nil {
		t.Fatalf("SpawnTask: %v", err)
	}
	waitForTerminal(t, state, 5)

	data, _ := os.ReadFile(state.OutputFile)
	output := string(data)
	for _, toolName := range []string{"bash", "file_read", "file_write", "file_edit", "grep", "glob", "ls", "web_fetch", "web_search", "todo_write", "tool_search"} {
		if !strings.Contains(output, toolName) {
			t.Errorf("expected tool %q in tool_search output; got: %s", toolName, output)
		}
	}
}

// ─── GrepTool tests ──────────────────────────────────────────────────────────

func TestGrepTool_FilesWithMatches(t *testing.T) {
	dir := t.TempDir()
	if err := os.WriteFile(filepath.Join(dir, "a.go"), []byte("func Hello() {}\n"), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(filepath.Join(dir, "b.go"), []byte("func Goodbye() {}\n"), 0o644); err != nil {
		t.Fatal(err)
	}

	llm := &fakeLLM{turns: []builtin.AssistantMessage{
		{
			ToolUses: []builtin.ToolUseRequest{
				{ID: "g1", Name: "grep", Input: map[string]interface{}{
					"pattern": "Hello",
					"path":    dir,
				}},
			},
			StopReason: "tool_use",
		},
		{Text: "Found files.", StopReason: "end_turn"},
	}}

	state, err := builtin.SpawnTask(context.Background(), "test", "grep", dir, builtin.AgentConfig{LLM: llm})
	if err != nil {
		t.Fatalf("SpawnTask: %v", err)
	}
	waitForTerminal(t, state, 5)

	data, _ := os.ReadFile(state.OutputFile)
	if !strings.Contains(string(data), "a.go") {
		t.Errorf("expected 'a.go' in grep output; got: %s", string(data))
	}
	if strings.Contains(string(data), "b.go") {
		t.Errorf("unexpected 'b.go' in grep output; got: %s", string(data))
	}
}

// ─── TodoTool tests ───────────────────────────────────────────────────────────

func TestTodoWriteTool(t *testing.T) {
	dir := t.TempDir()

	todos := []map[string]interface{}{
		{"id": "1", "content": "First task", "status": "pending", "priority": "high"},
		{"id": "2", "content": "Second task", "status": "in_progress", "priority": "medium"},
		{"id": "3", "content": "Third task", "status": "completed", "priority": "low"},
	}
	todosJSON, _ := json.Marshal(todos)
	var todosInput []interface{}
	_ = json.Unmarshal(todosJSON, &todosInput)

	llm := &fakeLLM{turns: []builtin.AssistantMessage{
		{
			ToolUses: []builtin.ToolUseRequest{
				{ID: "tw1", Name: "todo_write", Input: map[string]interface{}{"todos": todosInput}},
			},
			StopReason: "tool_use",
		},
		{Text: "Done.", StopReason: "end_turn"},
	}}

	state, err := builtin.SpawnTask(context.Background(), "test", "todos", dir, builtin.AgentConfig{LLM: llm})
	if err != nil {
		t.Fatalf("SpawnTask: %v", err)
	}
	waitForTerminal(t, state, 5)

	data, _ := os.ReadFile(state.OutputFile)
	output := string(data)
	if !strings.Contains(output, "Todo list updated") {
		t.Errorf("expected 'Todo list updated' in output; got: %s", output)
	}
	if !strings.Contains(output, "First task") {
		t.Errorf("expected 'First task' in output; got: %s", output)
	}
}

// ─── Hub tool tests ───────────────────────────────────────────────────────────

func TestDefaultToolsWithHub_IncludesHubTools(t *testing.T) {
	hub := newFakeHub()
	tools := builtin.DefaultToolsWithHub(hub, "agent-1")
	names := make(map[string]bool)
	for _, tool := range tools {
		names[tool.Definition().Name] = true
	}
	for _, expected := range []string{
		"bash", "file_read", "file_write", "file_edit", "grep", "glob", "ls",
		"web_fetch", "web_search", "todo_write", "tool_search",
		"task_create", "task_get", "task_list", "task_update", "send_message",
	} {
		if !names[expected] {
			t.Errorf("expected tool %q in DefaultToolsWithHub; got: %v", expected, names)
		}
	}
}

// ─── Helper ───────────────────────────────────────────────────────────────────

func waitForTerminal(t *testing.T, state *builtin.TaskState, seconds int) {
	t.Helper()
	deadline := time.Now().Add(time.Duration(seconds) * time.Second)
	for time.Now().Before(deadline) {
		if state.Status().IsTerminal() {
			return
		}
		time.Sleep(10 * time.Millisecond)
	}
	if !state.Status().IsTerminal() {
		t.Fatalf("task did not reach terminal state within %ds; status=%s", seconds, state.Status())
	}
}

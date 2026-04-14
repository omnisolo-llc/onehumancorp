package builtin

import (
	"context"
	"encoding/json"
	"os"
	"strings"
	"testing"
	"time"
)

// ─── TodoWrite tests ──────────────────────────────────────────────────────────

func TestTodoWriteListAPI(t *testing.T) {
	ctx := context.Background()

	todos := []todoItem{
		{Content: "Write tests", Status: "in_progress"},
		{Content: "Deploy", Status: "pending"},
	}
	b, _ := json.Marshal(map[string]interface{}{"todos": todos})

	res, err := TodoWriteTool.Execute(ctx, b)
	if err != nil {
		t.Fatalf("TodoWrite: %v", err)
	}
	if !strings.Contains(res, "Write tests") {
		t.Errorf("TodoWrite output missing item: %q", res)
	}
}

func TestTodoWriteAllComplete(t *testing.T) {
	ctx := context.Background()
	todos := []todoItem{
		{Content: "Task A", Status: "completed"},
		{Content: "Task B", Status: "completed"},
		{Content: "Task C", Status: "completed"},
	}
	b, _ := json.Marshal(map[string]interface{}{"todos": todos})
	res, err := TodoWriteTool.Execute(ctx, b)
	if err != nil {
		t.Fatalf("TodoWrite: %v", err)
	}
	if !strings.Contains(res, "All tasks completed") {
		t.Errorf("Expected 'All tasks completed' in output, got: %q", res)
	}
}

func TestTodoWriteInvalidStatus(t *testing.T) {
	ctx := context.Background()
	b := []byte(`{"todos":[{"content":"Task","status":"bogus"}]}`)
	_, err := TodoWriteTool.Execute(ctx, b)
	if err == nil {
		t.Fatal("expected error for invalid status")
	}
}

func TestTodoRead(t *testing.T) {
	ctx := context.Background()
	// Write first
	todos := []todoItem{{Content: "Read test", Status: "pending"}}
	b, _ := json.Marshal(map[string]interface{}{"todos": todos})
	_, _ = TodoWriteTool.Execute(ctx, b)

	res, err := TodoReadTool.Execute(ctx, nil)
	if err != nil {
		t.Fatalf("TodoRead: %v", err)
	}
	if !strings.Contains(res, "Read test") {
		t.Errorf("TodoRead missing item: %q", res)
	}
}

func TestTodoPersistence(t *testing.T) {
	ctx := context.Background()
	tmp := t.TempDir()
	todoPath := tmp + "/todos.json"
	t.Setenv("OHC_TODO_FILE", todoPath)

	todos := []todoItem{{Content: "Persist me", Status: "pending"}}
	b, _ := json.Marshal(map[string]interface{}{"todos": todos})
	_, err := TodoWriteTool.Execute(ctx, b)
	if err != nil {
		t.Fatalf("TodoWrite: %v", err)
	}
	data, err := os.ReadFile(todoPath)
	if err != nil {
		t.Fatalf("ReadFile: %v", err)
	}
	if !strings.Contains(string(data), "Persist me") {
		t.Errorf("Todo file missing content: %q", string(data))
	}
}

// ─── Task store tests ─────────────────────────────────────────────────────────

func TestTaskStoreCreateGet(t *testing.T) {
	ctx := context.Background()
	createArgs, _ := json.Marshal(map[string]interface{}{
		"subject":     "my test task",
		"description": "do the thing",
	})
	res, err := TaskCreateTool.Execute(ctx, createArgs)
	if err != nil {
		t.Fatalf("TaskCreate: %v", err)
	}
	if !strings.Contains(res, "my test task") {
		t.Errorf("TaskCreate missing subject: %q", res)
	}
}

func TestTaskStoreList(t *testing.T) {
	ctx := context.Background()
	createArgs, _ := json.Marshal(map[string]interface{}{
		"subject":     "list test task",
		"description": "list check",
	})
	_, _ = TaskCreateTool.Execute(ctx, createArgs)

	res, err := TaskListTool.Execute(ctx, nil)
	if err != nil {
		t.Fatalf("TaskList: %v", err)
	}
	if res == "" {
		t.Fatal("TaskList returned empty")
	}
}

func TestTaskStoreUpdate(t *testing.T) {
	ctx := context.Background()
	// Create
	cArgs, _ := json.Marshal(map[string]interface{}{
		"subject":     "update test",
		"description": "needs update",
	})
	_, _ = TaskCreateTool.Execute(ctx, cArgs)

	// Get a fresh task ID
	tasks := globalTaskStore.list()
	if len(tasks) == 0 {
		t.Skip("no tasks to update")
	}
	id := tasks[len(tasks)-1].ID

	uArgs, _ := json.Marshal(map[string]interface{}{
		"taskId": id,
		"status": "in_progress",
		"owner":  "alice",
	})
	res, err := TaskUpdateTool.Execute(ctx, uArgs)
	if err != nil {
		t.Fatalf("TaskUpdate: %v", err)
	}
	if !strings.Contains(res, "success") && !strings.Contains(res, "true") {
		t.Errorf("TaskUpdate unexpected result: %q", res)
	}

	// Verify
	gArgs, _ := json.Marshal(map[string]interface{}{"taskId": id})
	res, err = TaskGetTool.Execute(ctx, gArgs)
	if err != nil {
		t.Fatalf("TaskGet: %v", err)
	}
	if !strings.Contains(res, "in_progress") {
		t.Errorf("Expected in_progress status in TaskGet result: %q", res)
	}
}

func TestTaskStoreDelete(t *testing.T) {
	ctx := context.Background()
	cArgs, _ := json.Marshal(map[string]interface{}{
		"subject":     "delete test",
		"description": "to be deleted",
	})
	_, _ = TaskCreateTool.Execute(ctx, cArgs)
	tasks := globalTaskStore.list()
	if len(tasks) == 0 {
		t.Skip("no tasks to delete")
	}
	id := tasks[len(tasks)-1].ID

	dArgs, _ := json.Marshal(map[string]interface{}{
		"taskId": id,
		"status": "deleted",
	})
	res, err := TaskUpdateTool.Execute(ctx, dArgs)
	if err != nil {
		t.Fatalf("TaskUpdate/delete: %v", err)
	}
	if !strings.Contains(res, "deleted") {
		t.Errorf("Expected 'deleted' in result: %q", res)
	}
}

// ─── SleepTool tests ──────────────────────────────────────────────────────────

func TestSleepTool(t *testing.T) {
	ctx := context.Background()
	start := time.Now()
	res, err := SleepTool.Execute(ctx, []byte(`{"seconds":0.05}`))
	elapsed := time.Since(start)
	if err != nil {
		t.Fatalf("Sleep: %v", err)
	}
	if elapsed < 40*time.Millisecond {
		t.Errorf("Sleep too short: %v", elapsed)
	}
	if !strings.Contains(res, "Slept") {
		t.Errorf("Unexpected result: %q", res)
	}
}

func TestSleepToolCancellation(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	go func() {
		time.Sleep(50 * time.Millisecond)
		cancel()
	}()
	_, err := SleepTool.Execute(ctx, []byte(`{"seconds":10}`))
	if err == nil {
		t.Fatal("expected cancellation error")
	}
}

func TestSleepToolClamp(t *testing.T) {
	// Clamp to 0 for negative seconds.
	ctx := context.Background()
	res, err := SleepTool.Execute(ctx, []byte(`{"seconds":-1}`))
	if err != nil {
		t.Fatalf("Sleep negative: %v", err)
	}
	if !strings.Contains(res, "0s") {
		t.Errorf("Expected '0s' in result for negative input: %q", res)
	}
}

// ─── SendMessage tests ────────────────────────────────────────────────────────

func TestSendMessageToUser(t *testing.T) {
	ctx := context.Background()
	res, err := SendMessageTool.Execute(ctx, []byte(`{"message":"hello user"}`))
	if err != nil {
		t.Fatalf("SendMessage: %v", err)
	}
	if !strings.Contains(res, "Message sent") {
		t.Errorf("Unexpected result: %q", res)
	}
}

func TestSendMessageToAgent(t *testing.T) {
	ctx := context.Background()
	res, err := SendMessageTool.Execute(ctx, []byte(`{"message":"hello agent","to":"agent-42"}`))
	if err != nil {
		t.Fatalf("SendMessage to agent: %v", err)
	}
	if !strings.Contains(res, "agent-42") {
		t.Errorf("Unexpected result: %q", res)
	}
	// Verify it's in the mailbox.
	msgs := agentMailbox.Drain("agent-42")
	if len(msgs) == 0 {
		t.Fatal("mailbox should have 1 message")
	}
	if msgs[0] != "hello agent" {
		t.Errorf("unexpected message: %q", msgs[0])
	}
}

// ─── AgentTool tests ──────────────────────────────────────────────────────────

func TestAgentToolMissingPrompt(t *testing.T) {
	ctx := context.Background()
	_, err := AgentTool.Execute(ctx, []byte(`{"description":"test"}`))
	if err == nil {
		t.Fatal("expected error for missing prompt")
	}
}

func TestAgentToolSpawn(t *testing.T) {
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	args, _ := json.Marshal(map[string]interface{}{
		"description": "noop task",
		"prompt":      "Do nothing, just respond 'done'.",
	})
	res, err := AgentTool.Execute(ctx, args)
	if err != nil {
		t.Fatalf("AgentTool: %v", err)
	}
	var out map[string]interface{}
	if err := json.Unmarshal([]byte(res), &out); err != nil {
		t.Fatalf("AgentTool result not JSON: %v — got %q", err, res)
	}
	if out["status"] != "async_launched" {
		t.Errorf("expected async_launched, got %q", out["status"])
	}
	taskID, _ := out["agentId"].(string)
	if taskID == "" {
		t.Fatal("expected agentId in result")
	}
}

// ─── TaskStop / TaskStatus tests ─────────────────────────────────────────────

func TestTaskStopNotFound(t *testing.T) {
	ctx := context.Background()
	res, err := TaskStopTool.Execute(ctx, []byte(`{"task_id":"no-such-task"}`))
	if err != nil {
		t.Fatalf("TaskStop: %v", err)
	}
	if !strings.Contains(res, "not found") {
		t.Errorf("Unexpected result: %q", res)
	}
}

func TestTaskStatusNotFound(t *testing.T) {
	ctx := context.Background()
	res, err := TaskStatusTool.Execute(ctx, []byte(`{"task_id":"no-such-task"}`))
	if err != nil {
		t.Fatalf("TaskStatus: %v", err)
	}
	if !strings.Contains(res, "not found") {
		t.Errorf("Unexpected result: %q", res)
	}
}

// ─── WebFetch tests ───────────────────────────────────────────────────────────

func TestWebFetchMissingURL(t *testing.T) {
	ctx := context.Background()
	_, err := WebFetchTool.Execute(ctx, []byte(`{}`))
	if err == nil {
		t.Fatal("expected error for missing URL")
	}
}

func TestExtractTextFromHTML(t *testing.T) {
	html := `<html><head><title>Test</title><script>alert('x')</script><style>body{}</style></head>
<body><h1>Hello</h1><p>World &amp; friends</p></body></html>`
	out := extractTextFromHTML(html)
	if !strings.Contains(out, "Hello") {
		t.Errorf("missing heading: %q", out)
	}
	if !strings.Contains(out, "World & friends") {
		t.Errorf("missing text: %q", out)
	}
	if strings.Contains(out, "alert") {
		t.Errorf("script not stripped: %q", out)
	}
}

// ─── ToolSearch tests ─────────────────────────────────────────────────────────

func TestToolSearchAll(t *testing.T) {
	ctx := context.Background()
	res, err := ToolSearchTool.Execute(ctx, []byte(`{}`))
	if err != nil {
		t.Fatalf("ToolSearch: %v", err)
	}
	if !strings.Contains(res, "Bash") {
		t.Errorf("expected Bash in results: %q", res)
	}
}

func TestToolSearchFiltered(t *testing.T) {
	ctx := context.Background()
	res, err := ToolSearchTool.Execute(ctx, []byte(`{"query":"file"}`))
	if err != nil {
		t.Fatalf("ToolSearch: %v", err)
	}
	if !strings.Contains(res, "Read") && !strings.Contains(res, "Write") {
		t.Errorf("expected file tools in results: %q", res)
	}
	// Should not return unrelated tools
	if strings.Contains(res, "Sleep") {
		t.Errorf("Sleep should not match 'file' query: %q", res)
	}
}

// ─── ForkChildMessage tests ───────────────────────────────────────────────────

func TestForkChildMessage(t *testing.T) {
	msg := ForkChildMessage("analyze main.go", "/repo")
	if msg.Role != RoleUser {
		t.Errorf("expected user role, got %s", msg.Role)
	}
	if !strings.Contains(msg.Content, "analyze main.go") {
		t.Errorf("directive missing from fork message: %q", msg.Content)
	}
	if !strings.Contains(msg.Content, "fork-boilerplate") {
		t.Errorf("boilerplate missing from fork message: %q", msg.Content)
	}
	if !strings.Contains(msg.Content, "Scope:") {
		t.Errorf("Scope instruction missing from fork message: %q", msg.Content)
	}
}

// ─── CoordinatorTools tests ───────────────────────────────────────────────────

func TestCoordinatorToolsContents(t *testing.T) {
	tools := CoordinatorTools()
	byName := map[string]bool{}
	for _, t := range tools {
		byName[t.Name] = true
	}
	required := []string{"Agent", "TaskStop", "TaskStatus", "SendMessage"}
	for _, name := range required {
		if !byName[name] {
			t.Errorf("CoordinatorTools missing %q", name)
		}
	}
}

func TestAgentToolsWithSubagentSupport(t *testing.T) {
	tools := AgentToolsWithSubagentSupport()
	byName := map[string]bool{}
	for _, tool := range tools {
		byName[tool.Name] = true
	}
	// Should include all normal tools plus subagent tools
	checkTools := []string{"Bash", "Read", "Write", "Agent", "TaskStop", "TaskStatus"}
	for _, name := range checkTools {
		if !byName[name] {
			t.Errorf("AgentToolsWithSubagentSupport missing %q", name)
		}
	}
}

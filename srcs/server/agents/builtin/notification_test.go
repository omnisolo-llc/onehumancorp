package builtin

import (
	"encoding/json"
	"strings"
	"testing"
	"time"
)

func TestBuildTaskNotification(t *testing.T) {
	notif := BuildTaskNotification(
		"task-abc",
		"tool-use-123",
		"/tmp/output.txt",
		"completed",
		"Task finished successfully",
		"The answer is 42",
		1500,
		7,
		3*time.Second,
	)

	// Should be valid JSON (not XML)
	var msg TaskNotification
	if err := json.Unmarshal([]byte(notif), &msg); err != nil {
		t.Fatalf("BuildTaskNotification should return valid JSON: %v — got %q", err, notif)
	}

	if msg.TaskID != "task-abc" {
		t.Errorf("TaskID: got %q, want %q", msg.TaskID, "task-abc")
	}
	if msg.ToolUseID != "tool-use-123" {
		t.Errorf("ToolUseID: got %q, want %q", msg.ToolUseID, "tool-use-123")
	}
	if msg.OutputFile != "/tmp/output.txt" {
		t.Errorf("OutputFile: got %q", msg.OutputFile)
	}
	if msg.Status != "completed" {
		t.Errorf("Status: got %q", msg.Status)
	}
	if msg.Result != "The answer is 42" {
		t.Errorf("Result: got %q", msg.Result)
	}
	if msg.TokenCount != 1500 {
		t.Errorf("TokenCount: got %d, want 1500", msg.TokenCount)
	}
	if msg.ToolUses != 7 {
		t.Errorf("ToolUses: got %d, want 7", msg.ToolUses)
	}
	if msg.DurationMs != 3000 {
		t.Errorf("DurationMs: got %d, want 3000", msg.DurationMs)
	}

	// Must NOT contain XML tags
	if strings.Contains(notif, "<task-notification>") || strings.Contains(notif, "<task-id>") {
		t.Errorf("notification should NOT contain XML tags: %q", notif)
	}
}

func TestBuildTaskNotificationMsg(t *testing.T) {
	msg := BuildTaskNotificationMsg("t1", "u1", "", "failed", "err", "", 0, 0, 0)
	if msg.TaskID != "t1" {
		t.Errorf("TaskID: %q", msg.TaskID)
	}
	if msg.Status != "failed" {
		t.Errorf("Status: %q", msg.Status)
	}
	if msg.OutputFile != "" {
		t.Errorf("OutputFile should be empty: %q", msg.OutputFile)
	}
}

func TestBuildTaskNotificationXMLFree(t *testing.T) {
	notif := BuildTaskNotification(
		"task-1", "", "", "completed",
		"Result: x < y && z > 0", `value="42"`, 0, 0, 0,
	)
	// No XML escaping in JSON output
	var msg TaskNotification
	if err := json.Unmarshal([]byte(notif), &msg); err != nil {
		t.Fatalf("must be valid JSON: %v", err)
	}
	if !strings.Contains(msg.Summary, "x < y && z > 0") {
		t.Errorf("Special chars should be unescaped in struct: %q", msg.Summary)
	}
}

func TestBuildTaskNotificationTruncation(t *testing.T) {
	longResult := strings.Repeat("x", 3000)
	notif := BuildTaskNotification("task-1", "", "", "completed", "ok", longResult, 0, 0, 0)
	var msg TaskNotification
	if err := json.Unmarshal([]byte(notif), &msg); err != nil {
		t.Fatalf("must be valid JSON: %v", err)
	}
	if len(msg.Result) > 2100 { // 2000 + ellipsis
		t.Errorf("Result should be truncated, len=%d", len(msg.Result))
	}
	if !strings.HasSuffix(msg.Result, "…") {
		t.Errorf("Expected truncation ellipsis in long result: %q", msg.Result[len(msg.Result)-10:])
	}
}

func TestParseSubagentStatus(t *testing.T) {
	cases := map[string]string{
		"completed": "✓ completed",
		"failed":    "✗ failed",
		"killed":    "⊘ killed",
		"running":   "⟳ running",
		"other":     "other",
	}
	for input, expected := range cases {
		got := ParseSubagentStatus(input)
		if got != expected {
			t.Errorf("ParseSubagentStatus(%q) = %q, want %q", input, got, expected)
		}
	}
}

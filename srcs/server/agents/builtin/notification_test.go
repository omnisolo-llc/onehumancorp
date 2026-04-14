package builtin

import (
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

	if !strings.Contains(notif, "<task-notification>") {
		t.Error("missing task-notification tag")
	}
	if !strings.Contains(notif, "<task-id>task-abc</task-id>") {
		t.Errorf("missing task-id: %q", notif)
	}
	if !strings.Contains(notif, "<tool_use_id>tool-use-123</tool_use_id>") {
		t.Errorf("missing tool_use_id: %q", notif)
	}
	if !strings.Contains(notif, "<status>completed</status>") {
		t.Errorf("missing status: %q", notif)
	}
	if !strings.Contains(notif, "The answer is 42") {
		t.Errorf("missing result: %q", notif)
	}
	if !strings.Contains(notif, "<total_tokens>1500</total_tokens>") {
		t.Errorf("missing total_tokens: %q", notif)
	}
	if !strings.Contains(notif, "<tool_uses>7</tool_uses>") {
		t.Errorf("missing tool_uses: %q", notif)
	}
	if !strings.Contains(notif, "<duration_ms>3000</duration_ms>") {
		t.Errorf("missing duration_ms: %q", notif)
	}
}

func TestBuildTaskNotificationNoToolUseID(t *testing.T) {
	notif := BuildTaskNotification("task-x", "", "", "failed", "Error occurred", "", 0, 0, 0)
	if strings.Contains(notif, "<tool_use_id>") {
		t.Error("should not emit empty tool_use_id")
	}
	if !strings.Contains(notif, "<status>failed</status>") {
		t.Errorf("expected failed status: %q", notif)
	}
}

func TestBuildTaskNotificationXMLEscape(t *testing.T) {
	notif := BuildTaskNotification(
		"task-1", "", "", "completed",
		"Result: x < y && z > 0", "value=\"42\"", 0, 0, 0,
	)
	if strings.Contains(notif, "<y") {
		t.Error("unescaped < in notification")
	}
	if strings.Contains(notif, "&&") {
		t.Error("unescaped & in notification")
	}
}

func TestBuildTaskNotificationTruncation(t *testing.T) {
	longResult := strings.Repeat("x", 3000)
	notif := BuildTaskNotification("task-1", "", "", "completed", "ok", longResult, 0, 0, 0)
	if len(notif) > 10000 {
		t.Errorf("notification too long: %d", len(notif))
	}
	if !strings.Contains(notif, "…") {
		t.Errorf("expected truncation ellipsis in long result")
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

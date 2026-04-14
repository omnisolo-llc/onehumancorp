package builtin

import (
	"encoding/json"
	"fmt"
	"time"
)

// TaskNotification is the typed notification sent when a sub-agent task completes.
//
// This struct mirrors the ohc.agent.service.TaskNotification protobuf message
// defined in agent_service.proto.  It is currently serialized to JSON; it is
// ready to be proto.Marshal'd once protoc code-generation is integrated into
// the Bazel build.
//
// Field numbers are documented as comments so that the mapping to the proto
// definition is explicit and migration is mechanical.
type TaskNotification struct {
	TaskID     string `json:"task_id"`      // field 1
	ToolUseID  string `json:"tool_use_id"`  // field 2
	OutputFile string `json:"output_file"`  // field 3
	Status     string `json:"status"`       // field 4
	Summary    string `json:"summary"`      // field 5
	Result     string `json:"result"`       // field 6
	TokenCount int64  `json:"token_count"`  // field 7
	ToolUses   int64  `json:"tool_uses"`    // field 8
	DurationMs int64  `json:"duration_ms"`  // field 9
}

// BuildTaskNotificationMsg creates a typed TaskNotification.
// This replaces the deprecated XML-based BuildTaskNotification helper.
func BuildTaskNotificationMsg(
	taskID string,
	toolUseID string,
	outputFile string,
	status string,
	summary string,
	result string,
	tokenCount int64,
	toolUses int64,
	elapsed time.Duration,
) *TaskNotification {
	return &TaskNotification{
		TaskID:     taskID,
		ToolUseID:  toolUseID,
		OutputFile: outputFile,
		Status:     status,
		Summary:    truncateNotif(summary, 1000),
		Result:     truncateNotif(result, 2000),
		TokenCount: tokenCount,
		ToolUses:   toolUses,
		DurationMs: elapsed.Milliseconds(),
	}
}

// BuildTaskNotification creates a JSON-encoded TaskNotification.
// The returned string replaces the old XML <task-notification> format.
// When protoc integration is available, callers should switch to
// proto.Marshal(BuildTaskNotificationMsg(...)) instead.
func BuildTaskNotification(
	taskID string,
	toolUseID string,
	outputFile string,
	status string,
	summary string,
	result string,
	tokenCount int64,
	toolUses int64,
	elapsed time.Duration,
) string {
	msg := BuildTaskNotificationMsg(
		taskID, toolUseID, outputFile, status, summary, result,
		tokenCount, toolUses, elapsed,
	)
	b, err := json.Marshal(msg)
	if err != nil {
		return fmt.Sprintf(`{"task_id":%q,"status":%q,"error":"marshal failed"}`, taskID, status)
	}
	return string(b)
}

// truncateNotif truncates a string for notification payloads.
func truncateNotif(s string, maxLen int) string {
	if len(s) <= maxLen {
		return s
	}
	return s[:maxLen] + "…"
}

// ParseSubagentStatus maps a TaskStatus string to a human-readable label.
// Mirrors CC-Source's task status display logic.
func ParseSubagentStatus(status string) string {
	switch status {
	case "completed":
		return "✓ completed"
	case "failed":
		return "✗ failed"
	case "killed":
		return "⊘ killed"
	case "running":
		return "⟳ running"
	default:
		return status
	}
}

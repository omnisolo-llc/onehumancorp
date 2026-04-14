package builtin

import (
	"fmt"
	"strings"
	"time"
)

// BuildTaskNotification creates the XML-tagged <task-notification> message
// that is sent back to the parent agent when a sub-agent task completes.
//
// Mirrors the CC-Source LocalAgentTask notification format used by both the
// local/runner.go buildNotification and the grpc service's task harness.
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
	var sb strings.Builder
	sb.WriteString("<task-notification>\n")
	sb.WriteString(fmt.Sprintf("  <task-id>%s</task-id>\n", xmlEscape(taskID)))

	if toolUseID != "" {
		sb.WriteString(fmt.Sprintf("  <tool_use_id>%s</tool_use_id>\n", xmlEscape(toolUseID)))
	}
	if outputFile != "" {
		sb.WriteString(fmt.Sprintf("  <output-file>%s</output-file>\n", xmlEscape(outputFile)))
	}

	sb.WriteString(fmt.Sprintf("  <status>%s</status>\n", xmlEscape(status)))
	sb.WriteString(fmt.Sprintf("  <summary>%s</summary>\n", xmlEscape(truncateNotif(summary, 1000))))

	if result != "" {
		sb.WriteString(fmt.Sprintf("  <result>%s</result>\n", xmlEscape(truncateNotif(result, 2000))))
	}

	sb.WriteString(fmt.Sprintf(
		"  <usage><total_tokens>%d</total_tokens><tool_uses>%d</tool_uses><duration_ms>%d</duration_ms></usage>\n",
		tokenCount, toolUses, elapsed.Milliseconds(),
	))
	sb.WriteString("</task-notification>")
	return sb.String()
}

// xmlEscape escapes XML special characters.
func xmlEscape(s string) string {
	s = strings.ReplaceAll(s, "&", "&amp;")
	s = strings.ReplaceAll(s, "<", "&lt;")
	s = strings.ReplaceAll(s, ">", "&gt;")
	s = strings.ReplaceAll(s, `"`, "&quot;")
	return s
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

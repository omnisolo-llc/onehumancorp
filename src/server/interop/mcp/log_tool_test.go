package mcp

import (
	"context"
	"os"
	"strings"
	"testing"
	"time"
)

func TestLogAnalyzerTool_Execute(t *testing.T) {
	// Create a temporary mock log file
	tmpFile, err := os.CreateTemp("", "agent_harness_*.log")
	if err != nil {
		t.Fatalf("failed to create temp file: %v", err)
	}
	defer os.Remove(tmpFile.Name())

	now := time.Now()
	recentTime1 := now.Add(-5 * time.Minute).Format(time.RFC3339)
	recentTime2 := now.Add(-10 * time.Minute).Format(time.RFC3339)
	oldTime := now.Add(-120 * time.Minute).Format(time.RFC3339)

	mockLogs := []string{
		oldTime + " INFO Server started",
		recentTime2 + " WARN High memory usage",
		recentTime1 + " ERROR Connection failed",
		now.Format(time.RFC3339) + " ERROR Database timeout",
	}

	for _, log := range mockLogs {
		tmpFile.WriteString(log + "\n")
	}
	tmpFile.Close()

	tool := NewLogAnalyzerTool(tmpFile.Name())
	ctx := context.Background()

	// Test matching ERROR level
	res, err := tool.Execute(ctx, "ERROR", 60)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}

	if !strings.Contains(res, "Connection failed") || !strings.Contains(res, "Database timeout") {
		t.Errorf("Expected ERROR logs in response, got: %v", res)
	}
	if strings.Contains(res, "INFO") || strings.Contains(res, "WARN") {
		t.Errorf("Did not expect INFO or WARN logs in ERROR response, got: %v", res)
	}

	// Test no matching logs
	res, err = tool.Execute(ctx, "FATAL", 60)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}
	if !strings.Contains(res, "No logs found") {
		t.Errorf("Expected 'No logs found' message, got: %v", res)
	}

	// Test time filter
	res, err = tool.Execute(ctx, "INFO", 60)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}
	if !strings.Contains(res, "No logs found") {
		t.Errorf("Expected 'No logs found' message because INFO log is old, got: %v", res)
	}
}

package mcp

import (
	"context"
	"os"
	"testing"
	"time"
	"fmt"
    "strings"
)

func TestLogAnalyzerTool_Execute(t *testing.T) {
	ctx := context.Background()

	// Create a temp file
	tmpFile, err := os.CreateTemp("", "agent_harness_test_*.log")
	if err != nil {
		t.Fatalf("failed to create temp file: %v", err)
	}
	defer os.Remove(tmpFile.Name())

	now := time.Now()
	oldTime := now.Add(-60 * time.Minute).Format(time.RFC3339Nano)
	recentTime := now.Add(-5 * time.Minute).Format(time.RFC3339Nano)

	lines := []string{
		fmt.Sprintf(`{"time":"%s","level":"INFO","msg":"old info"}`, oldTime),
		fmt.Sprintf(`{"time":"%s","level":"ERROR","msg":"old error"}`, oldTime),
		fmt.Sprintf(`{"time":"%s","level":"INFO","msg":"recent info"}`, recentTime),
		fmt.Sprintf(`{"time":"%s","level":"ERROR","msg":"recent error"}`, recentTime),
        "PLAIN TEXT ERROR",
		`{"level":"ERROR", "msg":"JSON without time, falling back to plaintext ERROR"}`,
	}

    // Add 60 dummy lines to test truncation
    // But since the level filter is ERROR, we need them to be INFO to not mess up Test 1
    for i := 0; i < 60; i++ {
        lines = append(lines, fmt.Sprintf("DUMMY INFO %d", i))
    }

	for _, line := range lines {
		tmpFile.WriteString(line + "\n")
	}
	tmpFile.Close()

	tool := NewLogAnalyzerTool(tmpFile.Name())

	// Test 1: Recent errors (last 10 minutes)
	res, err := tool.Execute(ctx, "ERROR", 10)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}
	if strings.Contains(res, "old error") {
		t.Errorf("Result should not contain old error: %s", res)
	}
	if !strings.Contains(res, "recent error") {
		t.Errorf("Result should contain recent error: %s", res)
	}
    if !strings.Contains(res, "PLAIN TEXT ERROR") {
		t.Errorf("Result should contain plain text error: %s", res)
	}
    if !strings.Contains(res, "JSON without time, falling back to plaintext ERROR") {
        t.Errorf("Result should contain invalid JSON fallback: %s", res)
    }

	// Test 2: All logs (last 120 minutes), verify truncation (only last 50 lines are kept)
	res2, err := tool.Execute(ctx, "", 120)
	if err != nil {
		t.Fatalf("Execute failed: %v", err)
	}
    lineCount := len(strings.Split(res2, "\n"))
    if lineCount != 50 {
        t.Errorf("Expected 50 lines, got %d", lineCount)
    }
	if strings.Contains(res2, "old info") {
		t.Errorf("Result should NOT contain old logs because they were truncated: %s", res2)
	}

    // Test 3: No file found
    noFileTool := NewLogAnalyzerTool("/tmp/this_file_does_not_exist.log")
    res3, err := noFileTool.Execute(ctx, "", 10)
    if err != nil {
        t.Fatalf("Expected no error for missing file, got %v", err)
    }
    if res3 != "No logs found." {
        t.Errorf("Expected 'No logs found.', got %s", res3)
    }

    // Test 4: Empty results
    res4, err := tool.Execute(ctx, "NONEXISTENT_LEVEL", 10)
    if err != nil {
        t.Fatalf("Expected no error for no matches, got %v", err)
    }
    if res4 != "No logs found matching criteria." {
        t.Errorf("Expected 'No logs found matching criteria.', got %s", res4)
    }
}

package mcp

import (
	"context"
	"testing"
)

// Add test to hit the generic open error (e.g., trying to read a directory as a file)
func TestLogAnalyzerTool_Execute_OpenError(t *testing.T) {
	ctx := context.Background()
	tool := NewLogAnalyzerTool("/tmp")
	_, err := tool.Execute(ctx, "", 10)
	if err == nil {
		t.Fatalf("Expected error opening directory as file, got nil")
	}
}

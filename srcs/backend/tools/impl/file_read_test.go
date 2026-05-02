package impl

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
)

func TestFileReadTool(t *testing.T) {
	tool := NewFileReadTool()

	if tool.Name() != "file_read" {
		t.Errorf("expected name 'file_read', got %s", tool.Name())
	}
	if tool.Description() == "" {
		t.Errorf("expected non-empty description")
	}
	if !json.Valid(tool.InputSchema()) {
		t.Errorf("expected valid JSON schema")
	}

	// Setup a temporary file
	dir := t.TempDir()
	filePath := filepath.Join(dir, "test.txt")
	err := os.WriteFile(filePath, []byte("test content"), 0644)
	if err != nil {
		t.Fatalf("failed to create temp file: %v", err)
	}

	ctx := context.Background()

	// Test successful read
	input := []byte(`{"path": "` + filePath + `"}`)
	output, err := tool.Execute(ctx, input)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var out struct {
		Content string `json:"content"`
		Error   string `json:"error,omitempty"`
	}
	if err := json.Unmarshal(output, &out); err != nil {
		t.Fatalf("failed to unmarshal output: %v", err)
	}
	if out.Content != "test content" {
		t.Errorf("expected 'test content', got '%s'", out.Content)
	}
	if out.Error != "" {
		t.Errorf("expected no error, got '%s'", out.Error)
	}

	// Test invalid JSON
	_, err = tool.Execute(ctx, []byte(`{invalid`))
	if err == nil {
		t.Errorf("expected error for invalid JSON")
	}

	// Test file not found
	inputNotFound := []byte(`{"path": "` + filepath.Join(dir, "nonexistent.txt") + `"}`)
	outputNotFound, err := tool.Execute(ctx, inputNotFound)
	if err != nil {
		t.Fatalf("did not expect Execute to return an error on file not found, got %v", err)
	}

	var outNotFound struct {
		Error string `json:"error,omitempty"`
	}
	json.Unmarshal(outputNotFound, &outNotFound)
	if outNotFound.Error == "" {
		t.Errorf("expected file not found error in output")
	}
}

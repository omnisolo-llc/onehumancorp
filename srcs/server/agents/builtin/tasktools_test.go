package builtin

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestTaskTools(t *testing.T) {
	// Setup test environment
	missionsDir := ".agent-task/missions"
	os.MkdirAll(missionsDir, 0755)
	defer os.RemoveAll(".agent-task") // cleanup

	ctx := context.Background()

	// 1. TaskCreate
	createArgs := []byte(`{"filename":"2026-04-07T08-02-24Z.md","content":"Initial mission"}`)
	_, err := TaskCreateTool.Execute(ctx, createArgs)
	if err != nil {
		t.Fatalf("TaskCreateTool error: %v", err)
	}

	// Verify file exists
	filePath := filepath.Join(missionsDir, "2026-04-07T08-02-24Z.md")
	if _, err := os.Stat(filePath); os.IsNotExist(err) {
		t.Fatalf("Expected file %s to exist", filePath)
	}

	// 2. TaskGet
	getArgs := []byte(`{"filename":"2026-04-07T08-02-24Z.md"}`)
	res, err := TaskGetTool.Execute(ctx, getArgs)
	if err != nil {
		t.Fatalf("TaskGetTool error: %v", err)
	}
	if res != "Initial mission" {
		t.Fatalf("Expected 'Initial mission', got %q", res)
	}

	// 3. TaskUpdate
	updateArgs := []byte(`{"filename":"2026-04-07T08-02-24Z.md","content":"Updated mission"}`)
	_, err = TaskUpdateTool.Execute(ctx, updateArgs)
	if err != nil {
		t.Fatalf("TaskUpdateTool error: %v", err)
	}

	res, err = TaskGetTool.Execute(ctx, getArgs)
	if err != nil {
		t.Fatalf("TaskGetTool error: %v", err)
	}
	if res != "Updated mission" {
		t.Fatalf("Expected 'Updated mission', got %q", res)
	}

	// 4. TaskList
	listArgs := []byte(`{}`)
	res, err = TaskListTool.Execute(ctx, listArgs)
	if err != nil {
		t.Fatalf("TaskListTool error: %v", err)
	}
	if !strings.Contains(res, "2026-04-07T08-02-24Z.md") {
		t.Fatalf("Expected list to contain filename, got %q", res)
	}
}

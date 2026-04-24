package interop

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	pb "github.com/onehumancorp/mono/srcs/proto/agentservice"
)

func TestStateHandoff_SyncAndLoad(t *testing.T) {
	os.Setenv("OHC_DATA_DIR", os.TempDir())
	defer os.Unsetenv("OHC_DATA_DIR")
	handoff, err := NewStateHandoff()
	if err != nil {
		t.Fatalf("Failed to create handoff: %v", err)
	}

	// Clean up before test
	os.RemoveAll(handoff.baseDir)
	os.MkdirAll(handoff.baseDir, 0755)

	ctx := context.Background()

	notification1 := &pb.TaskNotification{
		TaskId: "task-123",
		Status: "completed",
	}

	err = handoff.SyncToStandalone(ctx, notification1)
	if err != nil {
		t.Fatalf("SyncToStandalone failed: %v", err)
	}

	notification2 := &pb.TaskNotification{
		TaskId: "task-456",
		Status: "running",
	}

	err = handoff.SyncToCloud(ctx, notification2)
	if err != nil {
		t.Fatalf("SyncToCloud failed: %v", err)
	}

	pending, err := handoff.LoadPendingHandoffs()
	if err != nil {
		t.Fatalf("LoadPendingHandoffs failed: %v", err)
	}

	if len(pending) != 2 {
		t.Fatalf("Expected 2 pending handoffs, got %d", len(pending))
	}

	// Mark one as complete
	err = handoff.MarkHandoffComplete("task-123")
	if err != nil {
		t.Fatalf("MarkHandoffComplete failed: %v", err)
	}

	pendingAfter, err := handoff.LoadPendingHandoffs()
	if err != nil {
		t.Fatalf("LoadPendingHandoffs failed: %v", err)
	}

	if len(pendingAfter) != 1 {
		t.Fatalf("Expected 1 pending handoff after mark, got %d", len(pendingAfter))
	}
	if pendingAfter[0].TaskId != "task-456" {
		t.Errorf("Expected remaining task to be task-456, got %s", pendingAfter[0].TaskId)
	}

	// Cleanup after test
	os.RemoveAll(handoff.baseDir)
}

func TestStateHandoff_LoadCorrupt(t *testing.T) {
	os.Setenv("OHC_DATA_DIR", os.TempDir())
	defer os.Unsetenv("OHC_DATA_DIR")

	// Ensure we start with a fresh directory for THIS test
	tempHandoffDir := filepath.Join(os.TempDir(), "corrupt-test")
	os.Setenv("OHC_DATA_DIR", tempHandoffDir)
	os.RemoveAll(tempHandoffDir)

	handoff, err := NewStateHandoff()
	if err != nil {
		t.Fatalf("Failed to create handoff: %v", err)
	}

	// Write a corrupt .pb file
	corruptFile := filepath.Join(handoff.baseDir, "corrupt.pb")
	os.WriteFile(corruptFile, []byte("not a protobuf"), 0644)

	pending, err := handoff.LoadPendingHandoffs()
	if err != nil {
		t.Fatalf("LoadPendingHandoffs failed: %v", err)
	}

	if len(pending) != 0 {
		t.Errorf("Expected 0 valid handoffs, got %d. Found tasks: %v", len(pending), pending)
	}

	os.RemoveAll(handoff.baseDir)
}

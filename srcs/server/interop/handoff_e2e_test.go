package interop

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	pb "github.com/onehumancorp/mono/srcs/proto/agentservice"
)

func TestStateHandoff_E2EWorkflow(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	tempDir := filepath.Join(os.TempDir(), "ohc_e2e_handoff")
	os.MkdirAll(tempDir, 0755)
	os.Setenv("OHC_DATA_DIR", tempDir)
	defer os.Unsetenv("OHC_DATA_DIR")
	defer os.RemoveAll(tempDir)

	handoff, err := NewStateHandoff()
	if err != nil {
		t.Fatalf("Failed to initialize handoff: %v", err)
	}

	// 1. Simulate a cloud state saving a task before switching to standalone
	ctx := context.Background()
	notification := &pb.TaskNotification{
		TaskId: "task-e2e-789",
		Status: "running",
	}

	err = handoff.SyncToStandalone(ctx, notification)
	if err != nil {
		t.Fatalf("SyncToStandalone failed: %v", err)
	}

	// 2. Simulate Standalone app startup
	pending, err := handoff.LoadPendingHandoffs()
	if err != nil {
		t.Fatalf("LoadPendingHandoffs failed: %v", err)
	}

	found := false
	for _, p := range pending {
		if p.TaskId == "task-e2e-789" {
			found = true
			// 3. Simulate orchestrator picking it up and marking it done
			err = handoff.MarkHandoffComplete(p.TaskId)
			if err != nil {
				t.Fatalf("Failed to mark handoff complete: %v", err)
			}
		}
	}

	if !found {
		t.Fatalf("Failed to find task-e2e-789 in pending handoffs")
	}

	// 4. Verify it's actually removed
	pendingAfter, err := handoff.LoadPendingHandoffs()
	if err != nil {
		t.Fatalf("LoadPendingHandoffs failed: %v", err)
	}

	for _, p := range pendingAfter {
		if p.TaskId == "task-e2e-789" {
			t.Fatalf("Task was not properly removed: task-e2e-789 still found")
		}
	}
}

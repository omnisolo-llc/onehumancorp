package interop

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	agentservicepb "github.com/onehumancorp/mono/src/proto/agentservice"
)

func TestFileHandoffStore(t *testing.T) {
	tempDir := t.TempDir()
	store, err := NewFileHandoffStore(tempDir)
	if err != nil {
		t.Fatalf("failed to create handoff store: %v", err)
	}

	ctx := context.Background()
	taskID := "task-123"
	payload := &agentservicepb.TaskNotification{
		TaskId:  taskID,
		Status:  "COMPLETED",
		Summary: "Task completed",
		Result:  "Result string",
	}

	// Test WriteHandoff
	if err := store.WriteHandoff(ctx, taskID, payload); err != nil {
		t.Fatalf("WriteHandoff failed: %v", err)
	}

	// Test ReadHandoff
	readPayload, err := store.ReadHandoff(ctx, taskID)
	if err != nil {
		t.Fatalf("ReadHandoff failed: %v", err)
	}
	if readPayload == nil {
		t.Fatalf("ReadHandoff returned nil payload")
	}
	if readPayload.TaskId != payload.TaskId || readPayload.Status != payload.Status || readPayload.Summary != payload.Summary || readPayload.Result != payload.Result {
		t.Fatalf("ReadHandoff returned mismatched payload. Expected: %+v, Got: %+v", payload, readPayload)
	}

	// Test ListHandoffs
	taskIDs, err := store.ListHandoffs(ctx)
	if err != nil {
		t.Fatalf("ListHandoffs failed: %v", err)
	}
	if len(taskIDs) != 1 || taskIDs[0] != taskID {
		t.Fatalf("ListHandoffs returned mismatched task IDs. Expected: [%s], Got: %v", taskID, taskIDs)
	}

	// Test DeleteHandoff
	if err := store.DeleteHandoff(ctx, taskID); err != nil {
		t.Fatalf("DeleteHandoff failed: %v", err)
	}

	// Ensure file is deleted
	_, err = os.Stat(filepath.Join(tempDir, taskID+".pb"))
	if !os.IsNotExist(err) {
		t.Fatalf("handoff file still exists after deletion")
	}
}

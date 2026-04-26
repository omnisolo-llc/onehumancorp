package restic

import (
	"context"
	"testing"
)

func TestResticTool_CloudModeRejection(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "false")
	tool := NewResticTool()

	payload := map[string]interface{}{
		"action":     "status",
		"repository": "/tmp/backup",
		"password":   "secret",
	}

	_, err := tool.Execute(context.Background(), payload)
	if err == nil {
		t.Fatal("expected error in cloud mode")
	}
	if err.Error() != "restic integration is only supported in standalone mode" {
		t.Errorf("unexpected error message: %v", err)
	}
}

func TestResticTool_MissingAction(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "true")
	tool := NewResticTool()

	payload := map[string]interface{}{
		"repository": "/tmp/backup",
		"password":   "secret",
	}

	_, err := tool.Execute(context.Background(), payload)
	if err == nil {
		t.Fatal("expected error for missing action")
	}
	if err.Error() != "action is required" {
		t.Errorf("unexpected error message: %v", err)
	}
}

func TestResticTool_MissingRepository(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "true")
	tool := NewResticTool()

	payload := map[string]interface{}{
		"action":   "status",
		"password": "secret",
	}

	_, err := tool.Execute(context.Background(), payload)
	if err == nil {
		t.Fatal("expected error for missing repository")
	}
	if err.Error() != "repository is required" {
		t.Errorf("unexpected error message: %v", err)
	}
}

func TestResticTool_MissingPassword(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "true")
	tool := NewResticTool()

	payload := map[string]interface{}{
		"action":     "status",
		"repository": "/tmp/backup",
	}

	_, err := tool.Execute(context.Background(), payload)
	if err == nil {
		t.Fatal("expected error for missing password")
	}
	if err.Error() != "password is required" {
		t.Errorf("unexpected error message: %v", err)
	}
}

func TestResticTool_UnknownAction(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "true")
	tool := NewResticTool()

	payload := map[string]interface{}{
		"action":     "unknown",
		"repository": "/tmp/backup",
		"password":   "secret",
	}

	_, err := tool.Execute(context.Background(), payload)
	if err == nil {
		t.Fatal("expected error for unknown action")
	}
	if err.Error() != "unknown action: unknown" {
		t.Errorf("unexpected error message: %v", err)
	}
}

func TestResticTool_MissingSnapshotParams(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "true")
	tool := NewResticTool()

	payload := map[string]interface{}{
		"action":     "snapshot",
		"repository": "/tmp/backup",
		"password":   "secret",
	}

	_, err := tool.Execute(context.Background(), payload)
	if err == nil {
		t.Fatal("expected error for missing target_dir")
	}
	if err.Error() != "target_dir is required for snapshot action" {
		t.Errorf("unexpected error message: %v", err)
	}
}

func TestResticTool_MissingRestoreParams(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "true")
	tool := NewResticTool()

	// Missing snapshot_id
	payload1 := map[string]interface{}{
		"action":     "restore",
		"repository": "/tmp/backup",
		"password":   "secret",
		"target_dir": "/tmp/restore",
	}
	_, err := tool.Execute(context.Background(), payload1)
	if err == nil {
		t.Fatal("expected error for missing snapshot_id")
	}
	if err.Error() != "snapshot_id is required for restore action" {
		t.Errorf("unexpected error message: %v", err)
	}

	// Missing target_dir
	payload2 := map[string]interface{}{
		"action":      "restore",
		"repository":  "/tmp/backup",
		"password":    "secret",
		"snapshot_id": "latest",
	}
	_, err = tool.Execute(context.Background(), payload2)
	if err == nil {
		t.Fatal("expected error for missing target_dir")
	}
	if err.Error() != "target_dir is required for restore action" {
		t.Errorf("unexpected error message: %v", err)
	}
}

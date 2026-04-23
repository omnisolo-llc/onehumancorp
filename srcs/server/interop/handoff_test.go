package interop

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/proto/interop"
)

func TestStateHandoffManager(t *testing.T) {
	manager := NewStateHandoffManager()
	ctx := context.Background()

	data := &interoppb.StateHandoffData{
		TenantId:   "tenant-1",
		LastSynced: time.Now().Unix(),
	}

	err := manager.SyncState(ctx, data)
	if err != nil {
		t.Fatalf("failed to sync state: %v", err)
	}

	retrieved, err := manager.GetState(ctx, "tenant-1")
	if err != nil {
		t.Fatalf("failed to get state: %v", err)
	}

	if retrieved.TenantId != "tenant-1" {
		t.Fatalf("expected tenant-1, got %s", retrieved.TenantId)
	}

	// Idempotency test: Older timestamp should be ignored
	olderData := &interoppb.StateHandoffData{
		TenantId:   "tenant-1",
		LastSynced: data.LastSynced - 100,
		MissionState: []byte("older"),
	}
	_ = manager.SyncState(ctx, olderData)

	retrieved2, _ := manager.GetState(ctx, "tenant-1")
	if string(retrieved2.MissionState) == "older" {
		t.Fatalf("expected older state to be ignored")
	}

	// Error cases
	if err := manager.SyncState(ctx, nil); err == nil {
		t.Fatal("expected error on nil data")
	}
	if err := manager.SyncState(ctx, &interoppb.StateHandoffData{}); err == nil {
		t.Fatal("expected error on empty tenant")
	}
	if _, err := manager.GetState(ctx, ""); err == nil {
		t.Fatal("expected error on empty tenant")
	}
	if _, err := manager.GetState(ctx, "nonexistent"); err == nil {
		t.Fatal("expected error on nonexistent tenant")
	}
}

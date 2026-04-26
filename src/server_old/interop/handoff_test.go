package interop

import (
	"context"
	"testing"
	"time"
)

func TestHandoffManager_ExportImport(t *testing.T) {
	mesh, _ := NewTeammateMesh()
	lock, _ := NewDistributedLock()

	hm := NewHandoffManager(mesh, lock)
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	tenantID := "tenant-123"
	testState := &State{
		ID:    "state-xyz",
		Owner: "agent-007",
		Data: map[string]interface{}{
			"mission": "sync_test",
		},
	}

	received := make(chan *State, 1)

	err := hm.ImportState(ctx, tenantID, func(s *State) error {
		received <- s
		return nil
	})
	if err != nil {
		t.Fatalf("failed to import state: %v", err)
	}

	// Allow subscriber to initialize
	time.Sleep(50 * time.Millisecond)

	err = hm.ExportState(ctx, tenantID, testState)
	if err != nil {
		t.Fatalf("failed to export state: %v", err)
	}

	select {
	case s := <-received:
		if s.ID != testState.ID {
			t.Errorf("expected state ID %s, got %s", testState.ID, s.ID)
		}
		if s.Owner != testState.Owner {
			t.Errorf("expected owner %s, got %s", testState.Owner, s.Owner)
		}
	case <-time.After(2 * time.Second):
		t.Fatal("timed out waiting for state handoff")
	}

	// Test idempotency: Export same state again
	err = hm.ExportState(ctx, tenantID, testState)
	if err != nil {
		t.Fatalf("failed to export state second time: %v", err)
	}

	select {
	case <-received:
		t.Fatal("expected idempotent execution to drop duplicate state")
	case <-time.After(50 * time.Millisecond):
		// Success: no message received
	}
}

func TestHandoffManager_ExportLockConflict(t *testing.T) {
	mesh, _ := NewTeammateMesh()
	lock, _ := NewDistributedLock()

	hm := NewHandoffManager(mesh, lock)
	ctx := context.Background()
	tenantID := "tenant-locked"

	// Simulate external lock acquisition
	lockKey := "ohc:lock:handoff:" + tenantID
	acquired, err := lock.Lock(ctx, lockKey, 5*time.Second)
	if err != nil || !acquired {
		t.Fatalf("failed to setup pre-lock")
	}

	testState := &State{ID: "state-locked"}
	err = hm.ExportState(ctx, tenantID, testState)
	if err == nil {
		t.Fatal("expected error due to active lock, got nil")
	}
}

package sync_adapter

import (
	"errors"
	"testing"
	"time"

	syncpkg "github.com/onehumancorp/mono/src/server/lib/sync"
)

func TestPowerSyncAdapter(t *testing.T) {
	adapter := NewPowerSyncAdapter()

	// Initial state
	if adapter.GetSyncStatus() != syncpkg.SyncStatusIdle {
		t.Errorf("Expected initial status %s, got %s", syncpkg.SyncStatusIdle, adapter.GetSyncStatus())
	}

	// Stop sync when idle (should be a no-op)
	err := adapter.StopSync()
	if err != nil {
		t.Errorf("Expected no error from StopSync when idle, got %v", err)
	}

	// Start sync
	err = adapter.StartSync()
	if err != nil {
		t.Errorf("Expected no error from StartSync, got %v", err)
	}
	if adapter.GetSyncStatus() != syncpkg.SyncStatusSyncing {
		t.Errorf("Expected status %s, got %s", syncpkg.SyncStatusSyncing, adapter.GetSyncStatus())
	}

	// Start sync again (should return error)
	err = adapter.StartSync()
	if err == nil {
		t.Errorf("Expected error from StartSync when already syncing, got nil")
	}

	// Stop sync
	err = adapter.StopSync()
	if err != nil {
		t.Errorf("Expected no error from StopSync, got %v", err)
	}
	if adapter.GetSyncStatus() != syncpkg.SyncStatusIdle {
		t.Errorf("Expected status %s, got %s", syncpkg.SyncStatusIdle, adapter.GetSyncStatus())
	}

	// Wait for background process to finish simulation to test "UpToDate"
	adapter.StartSync()
	time.Sleep(150 * time.Millisecond) // Wait longer than the 100ms in syncLoop
	if adapter.GetSyncStatus() != syncpkg.SyncStatusUpToDate {
		t.Errorf("Expected status %s, got %s", syncpkg.SyncStatusUpToDate, adapter.GetSyncStatus())
	}

	// Start sync with error
	adapter = NewPowerSyncAdapter()
	expectedErr := errors.New("sync error")
	adapter.SetError(expectedErr)
	err = adapter.StartSync()
	if err == nil || err.Error() != expectedErr.Error() {
		t.Errorf("Expected error %v, got %v", expectedErr, err)
	}
	if adapter.GetSyncStatus() != syncpkg.SyncStatusError {
		t.Errorf("Expected status %s, got %s", syncpkg.SyncStatusError, adapter.GetSyncStatus())
	}
}

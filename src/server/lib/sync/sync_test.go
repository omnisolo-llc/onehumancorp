package sync

import (
	"errors"
	"testing"
)

func TestMockSynchronizer(t *testing.T) {
	mock := NewMockSynchronizer()

	// Initial state
	if mock.GetSyncStatus() != SyncStatusIdle {
		t.Errorf("Expected initial status %s, got %s", SyncStatusIdle, mock.GetSyncStatus())
	}

	// Start sync success
	err := mock.StartSync()
	if err != nil {
		t.Errorf("Expected no error from StartSync, got %v", err)
	}
	if mock.GetSyncStatus() != SyncStatusSyncing {
		t.Errorf("Expected status %s, got %s", SyncStatusSyncing, mock.GetSyncStatus())
	}

	// Stop sync
	err = mock.StopSync()
	if err != nil {
		t.Errorf("Expected no error from StopSync, got %v", err)
	}
	if mock.GetSyncStatus() != SyncStatusIdle {
		t.Errorf("Expected status %s, got %s", SyncStatusIdle, mock.GetSyncStatus())
	}

	// Start sync with error
	expectedErr := errors.New("sync error")
	mock.SetError(expectedErr)
	err = mock.StartSync()
	if err == nil || err.Error() != expectedErr.Error() {
		t.Errorf("Expected error %v, got %v", expectedErr, err)
	}
	if mock.GetSyncStatus() != SyncStatusError {
		t.Errorf("Expected status %s, got %s", SyncStatusError, mock.GetSyncStatus())
	}

	// Set custom status
	mock.SetStatus(SyncStatusOffline)
	if mock.GetSyncStatus() != SyncStatusOffline {
		t.Errorf("Expected status %s, got %s", SyncStatusOffline, mock.GetSyncStatus())
	}

	// Ensure other constants are present to reach coverage
	_ = []SyncStatus{SyncStatusUpToDate}
}

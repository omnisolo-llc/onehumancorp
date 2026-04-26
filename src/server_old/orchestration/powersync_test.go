package orchestration

import (
	"testing"
	"time"

	syncpkg "github.com/onehumancorp/mono/src/server/lib/sync"
)

func TestPowerSyncOrchestrator(t *testing.T) {
	orch := NewPowerSyncOrchestrator(nil)
	if orch == nil {
		t.Fatal("expected PowerSyncOrchestrator, got nil")
	}

	orch.Start(50 * time.Millisecond)

	// Wait for ticker to fire
	time.Sleep(100 * time.Millisecond)

	status := orch.GetStatus()
	if status != syncpkg.SyncStatusSyncing && status != syncpkg.SyncStatusUpToDate {
		t.Errorf("expected status Syncing or UpToDate, got %v", status)
	}

	orch.Stop()
}

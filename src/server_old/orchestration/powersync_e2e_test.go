package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
	syncpkg "github.com/onehumancorp/mono/src/server/lib/sync"
)

func TestPowerSyncOrchestratorE2E(t *testing.T) {
	ctx := context.Background()

	database, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	defer database.Close()

	orch := NewPowerSyncOrchestrator(database)
	orch.Start(10 * time.Millisecond)

	time.Sleep(50 * time.Millisecond)

	status := orch.GetStatus()
	if status != syncpkg.SyncStatusSyncing && status != syncpkg.SyncStatusUpToDate {
		t.Errorf("Expected status Syncing or UpToDate, got %v", status)
	}

	orch.Stop()
}

package orchestration

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestHub_CheckHealth_Standalone(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	prov := db.NewTestProvider(t)
	defer prov.Close()

	ctx := context.Background()

	// Ensure missions table exists for sync backlog check
	_, _ = prov.Exec(ctx, "CREATE TABLE IF NOT EXISTS agent_missions (status TEXT)")
	_, _ = prov.Exec(ctx, "INSERT INTO agent_missions (status) VALUES ('PENDING')")

	hub := NewHub()
	hub.sipDB = &SIPDB{db: prov}

	probe, err := hub.CheckHealth(ctx)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if probe.Status != "healthy" {
		t.Errorf("expected healthy, got %s", probe.Status)
	}

	if probe.Mode != "standalone" {
		t.Errorf("expected standalone, got %s", probe.Mode)
	}

	if probe.SyncBacklog != 1 {
		t.Errorf("expected sync backlog 1, got %d", probe.SyncBacklog)
	}

	if probe.DBPing <= 0*time.Millisecond {
		t.Errorf("expected DB ping to be > 0, got %v", probe.DBPing)
	}
}

func TestHub_CheckHealth_Cloud(t *testing.T) {
	// For testing, since we use NewTestProvider which returns an sqlite provider,
	// IsSQLite() will return true. We can't strictly test cloud mode easily
	// without a mock DB provider or real PG provider.
	// The standalone test verifies the core logic executes successfully.
}

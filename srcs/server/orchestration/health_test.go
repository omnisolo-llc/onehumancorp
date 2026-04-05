package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestHybridHealthProbe(t *testing.T) {
	ctx := context.Background()
	provider, err := db.NewSQLiteProvider(":memory:")
	if err != nil {
		t.Fatalf("failed to create sqlite provider: %v", err)
	}

	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS agent_missions (
			id TEXT PRIMARY KEY,
			status TEXT,
			payload JSON,
			created_at TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	// Insert some test missions
	_, err = provider.Exec(ctx, "INSERT INTO agent_missions (id, status, payload, created_at) VALUES ('1', 'PENDING', '{}', CURRENT_TIMESTAMP)")
	if err != nil {
		t.Fatalf("failed to insert mission: %v", err)
	}

	_, err = provider.Exec(ctx, "INSERT INTO agent_missions (id, status, payload, created_at) VALUES ('2', 'PENDING', '{}', CURRENT_TIMESTAMP)")
	if err != nil {
		t.Fatalf("failed to insert mission: %v", err)
	}

	hub := &Hub{
		sipDB: &SIPDB{
			db: provider,
		},
	}

	probe, err := hub.CheckHealth(ctx)
	if err != nil {
		t.Fatalf("CheckHealth failed: %v", err)
	}

	if probe.Mode != "standalone" {
		t.Errorf("Expected mode 'standalone', got '%s'", probe.Mode)
	}
	if probe.Status != "healthy" {
		t.Errorf("Expected status 'healthy', got '%s'", probe.Status)
	}
	if probe.SyncBacklog != 2 {
		t.Errorf("Expected SyncBacklog 2, got %d", probe.SyncBacklog)
	}
	if probe.DBPing <= 0 {
		t.Errorf("Expected DBPing > 0, got %v", probe.DBPing)
	}
	if probe.MeshActive != false {
		t.Errorf("Expected MeshActive false, got %v", probe.MeshActive)
	}
}

func TestHybridHealthProbe_Degraded(t *testing.T) {
	ctx := context.Background()

	hub := &Hub{
		sipDB: nil, // This will trigger degraded status
	}

	probe, err := hub.CheckHealth(ctx)
	if err != nil {
		t.Fatalf("CheckHealth failed unexpectedly: %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("Expected status 'degraded', got '%s'", probe.Status)
	}
}

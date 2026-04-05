package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestHybridHealthProbe_Struct(t *testing.T) {
	probe := HybridHealthProbe{
		Mode:        "cloud",
		Status:      "healthy",
		DBPing:      10 * time.Millisecond,
		SyncBacklog: 5,
		MeshActive:  true,
	}

	if probe.Mode != "cloud" {
		t.Errorf("Expected mode 'cloud', got '%s'", probe.Mode)
	}
	if probe.Status != "healthy" {
		t.Errorf("Expected status 'healthy', got '%s'", probe.Status)
	}
}

func TestHub_CheckHealth_Standalone(t *testing.T) {
	hub := NewHub()
	defer hub.Close()

	// Using SQLite in-memory, typical for testing "standalone"
	dbInstance, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	hub.SetSIPDB(dbInstance)

	ctx := context.Background()
	// Add a test task to verify sync backlog
	_, err = dbInstance.db.Exec(ctx, "INSERT INTO agent_missions (id, agent, status, priority, payload, updated_at) VALUES ('1', 'test_agent', 'PENDING', 1, '{}', CURRENT_TIMESTAMP)")
	if err != nil {
		t.Fatalf("failed to insert mission: %v", err)
	}
	_, err = dbInstance.db.Exec(ctx, "INSERT INTO agent_missions (id, agent, status, priority, payload, updated_at) VALUES ('2', 'test_agent', 'PENDING', 1, '{}', CURRENT_TIMESTAMP)")
	if err != nil {
		t.Fatalf("failed to insert mission: %v", err)
	}

	probe, err := hub.CheckHealth(ctx)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if probe.Status != "healthy" {
		t.Errorf("expected status 'healthy', got %v", probe.Status)
	}
	if probe.Mode != "standalone" {
		t.Errorf("expected mode 'standalone', got %v", probe.Mode)
	}
	if probe.SyncBacklog != 2 {
		t.Errorf("expected sync_backlog 2, got %v", probe.SyncBacklog)
	}
	if probe.MeshActive {
		t.Errorf("expected mesh_active false without centrifuge node")
	}
}

func TestHub_CheckHealth_NoDB(t *testing.T) {
	hub := NewHub()
	defer hub.Close()

	ctx := context.Background()
	probe, err := hub.CheckHealth(ctx)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("expected status 'degraded', got %v", probe.Status)
	}
}

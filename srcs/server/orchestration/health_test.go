package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestHybridHealthProbe(t *testing.T) {
	// A simple test ensuring the struct exists and fields map correctly.
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

func TestCheckHealth_SQLite(t *testing.T) {
	ctx := context.Background()
	hub := NewHub()

	provider := db.NewTestSqliteProvider()
	database := &db.DB{Provider: provider}

	_, err := database.Exec(ctx, `
		CREATE TABLE agent_missions (
			id TEXT PRIMARY KEY,
			status TEXT,
			payload TEXT,
			synced_to_cloud INTEGER
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}
	_, err = database.Exec(ctx, "INSERT INTO agent_missions (id, status, synced_to_cloud) VALUES ('1', 'CLOUD_ESCALATION', 0)")
	if err != nil {
		t.Fatalf("Failed to insert mock mission: %v", err)
	}

	hub.SetSIPDB(&SIPDB{db: database})

	probe, err := hub.CheckHealth(ctx)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if probe.Mode != "standalone" {
		t.Errorf("Expected mode 'standalone', got '%s'", probe.Mode)
	}
	if probe.Status != "healthy" {
		t.Errorf("Expected status 'healthy', got '%s'", probe.Status)
	}
	if probe.SyncBacklog != 1 {
		t.Errorf("Expected SyncBacklog 1, got %d", probe.SyncBacklog)
	}
}

package orchestration

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestHybridHealthProbe_DegradedNoDB(t *testing.T) {
	hub := NewHub()
	ctx := context.Background()

	probe, err := hub.CheckHealth(ctx)
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("Expected status 'degraded' when DB is not set, got '%s'", probe.Status)
	}
	if probe.MeshActive != false {
		t.Errorf("Expected MeshActive false when CentrifugeNode is not set")
	}
}

func TestHybridHealthProbe_HealthyStandalone(t *testing.T) {
	hub := NewHub()
	ctx := context.Background()

	// Setup SQLite in-memory DB to simulate standalone mode
	dbProvider := db.NewSQLiteProvider(":memory:")
	err := dbProvider.Connect(ctx)
	if err != nil {
		t.Fatalf("Failed to connect to SQLite: %v", err)
	}
	defer dbProvider.Close()

	// Initialize the schema to allow querying agent_missions
	_, err = dbProvider.Exec(ctx, `
		CREATE TABLE agent_missions (
			id TEXT PRIMARY KEY,
			status TEXT
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	hub.SetSIPDB(&SIPDB{db: dbProvider})

	probe, err := hub.CheckHealth(ctx)
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}

	if probe.Status != "healthy" {
		t.Errorf("Expected status 'healthy', got '%s'", probe.Status)
	}
	if probe.Mode != "standalone" {
		t.Errorf("Expected mode 'standalone' for SQLite, got '%s'", probe.Mode)
	}
	if probe.SyncBacklog != 0 {
		t.Errorf("Expected SyncBacklog 0, got %d", probe.SyncBacklog)
	}
}

func TestHybridHealthProbe_SyncBacklog(t *testing.T) {
	hub := NewHub()
	ctx := context.Background()

	dbProvider := db.NewSQLiteProvider(":memory:")
	err := dbProvider.Connect(ctx)
	if err != nil {
		t.Fatalf("Failed to connect to SQLite: %v", err)
	}
	defer dbProvider.Close()

	_, err = dbProvider.Exec(ctx, `
		CREATE TABLE agent_missions (
			id TEXT PRIMARY KEY,
			status TEXT
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	// Insert some PENDING missions
	_, err = dbProvider.Exec(ctx, "INSERT INTO agent_missions (id, status) VALUES ('1', 'PENDING'), ('2', 'PENDING'), ('3', 'DONE')")
	if err != nil {
		t.Fatalf("Failed to insert records: %v", err)
	}

	hub.SetSIPDB(&SIPDB{db: dbProvider})

	probe, err := hub.CheckHealth(ctx)
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}

	if probe.Status != "healthy" {
		t.Errorf("Expected status 'healthy', got '%s'", probe.Status)
	}
	if probe.SyncBacklog != 2 {
		t.Errorf("Expected SyncBacklog 2, got %d", probe.SyncBacklog)
	}
}

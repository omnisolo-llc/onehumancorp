package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestHybridHealthProbe_NoDB(t *testing.T) {
	hub := NewHub()
	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("Expected status 'degraded' when DB is nil, got '%s'", probe.Status)
	}
}

func TestHybridHealthProbe_SQLite(t *testing.T) {
	hub := NewHub()
	testDB, err := db.NewSQLiteProvider(":memory:")
	if err != nil {
		t.Fatalf("failed to create sqlite provider: %v", err)
	}
	defer testDB.Close()

	_, err = testDB.Exec(context.Background(), `
		CREATE TABLE agent_missions (
			id TEXT PRIMARY KEY,
			status TEXT,
			synced_to_cloud INTEGER
		)
	`)
	if err != nil {
		t.Fatalf("failed to setup mock db: %v", err)
	}

	// Insert some sync backlog (2 unsynced missions)
	_, _ = testDB.Exec(context.Background(), "INSERT INTO agent_missions (id, status, synced_to_cloud) VALUES ('1', 'PENDING', 0)")
	_, _ = testDB.Exec(context.Background(), "INSERT INTO agent_missions (id, status, synced_to_cloud) VALUES ('2', 'PENDING', 0)")
	_, _ = testDB.Exec(context.Background(), "INSERT INTO agent_missions (id, status, synced_to_cloud) VALUES ('3', 'DONE', 1)")

	sipDB := &SIPDB{db: testDB}
	hub.SetSIPDB(sipDB)

	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if probe.Status != "healthy" {
		t.Errorf("Expected status 'healthy', got '%s'", probe.Status)
	}
	if probe.Mode != "standalone" {
		t.Errorf("Expected mode 'standalone' for SQLite, got '%s'", probe.Mode)
	}
	if probe.SyncBacklog != 2 {
		t.Errorf("Expected SyncBacklog 2, got %d", probe.SyncBacklog)
	}
	if probe.MeshActive != false {
		t.Errorf("Expected MeshActive false since CentrifugeNode is nil, got %v", probe.MeshActive)
	}
}

// Ensure the struct itself maps fields properly (sanity check)
func TestHybridHealthProbe_StructFields(t *testing.T) {
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

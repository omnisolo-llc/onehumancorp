package orchestration

import (
	"context"
	"database/sql"
	"os"
	"testing"
	"time"

	_ "modernc.org/sqlite"
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

func TestCheckHealthDegraded_NoDB(t *testing.T) {
	hub := NewHub()
	// No SIPDB and no CentrifugeNode set
	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("Expected status 'degraded', got '%s'", probe.Status)
	}
	if probe.MeshActive {
		t.Errorf("Expected MeshActive to be false")
	}
}

func TestCheckHealth_SQLite(t *testing.T) {
	// Setup a physical temporary sqlite file
	ctx := context.Background()
	tmpFile, err := os.CreateTemp("", "health_test_*.db")
	if err != nil {
		t.Fatalf("Failed to create temp db file: %v", err)
	}
	tmpFile.Close()
	defer os.Remove(tmpFile.Name())

	sqliteDB, err := sql.Open("sqlite", tmpFile.Name())
	if err != nil {
		t.Fatalf("Failed to open test sqlite db: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)
	defer sqliteDB.Close()

	// Ensure the agent_missions table exists
	_, err = provider.Exec(ctx, "CREATE TABLE agent_missions (status TEXT)")
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	// Insert some pending missions
	_, err = provider.Exec(ctx, "INSERT INTO agent_missions (status) VALUES ('PENDING')")
	if err != nil {
		t.Fatalf("Failed to insert pending mission: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO agent_missions (status) VALUES ('PENDING')")
	if err != nil {
		t.Fatalf("Failed to insert pending mission: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO agent_missions (status) VALUES ('DONE')")
	if err != nil {
		t.Fatalf("Failed to insert done mission: %v", err)
	}

	hub := NewHub()
	sipDB := &SIPDB{db: provider}
	hub.SetSIPDB(sipDB)

	probe, err := hub.CheckHealth(ctx)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if probe.Status != "healthy" {
		t.Errorf("Expected status 'healthy', got '%s'", probe.Status)
	}
	if probe.Mode != "standalone" {
		t.Errorf("Expected mode 'standalone', got '%s'", probe.Mode)
	}
	if probe.SyncBacklog != 2 {
		t.Errorf("Expected SyncBacklog to be 2, got %d", probe.SyncBacklog)
	}
	if probe.MeshActive {
		t.Errorf("Expected MeshActive to be false")
	}
}

func TestCheckHealth_MeshActive(t *testing.T) {
	ctx := context.Background()

	hub := NewHub()

	// Use a mock setup to test MeshActive
	cn, err := NewCentrifugeNode()
	if err != nil {
		t.Skipf("Skipping TestCheckHealth_MeshActive since NewCentrifugeNode failed to initialize: %v", err)
	}

	hub.SetCentrifugeNode(cn)
	probe, err := hub.CheckHealth(ctx)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("Expected status 'degraded', got '%s'", probe.Status)
	}
	if probe.MeshActive != true {
		t.Errorf("Expected MeshActive to be true")
	}
}

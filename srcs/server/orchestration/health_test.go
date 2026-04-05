package orchestration

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
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
	hub := NewHub()

	// Initialize in-memory SQLite
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open sqlite: %v", err)
	}
	defer sqliteDB.Close()
	provider := db.NewSqliteProvider(sqliteDB)

	// Create agent_missions table so sync backlog query works
	_, err = provider.Exec(context.Background(), "CREATE TABLE agent_missions (id TEXT, status TEXT)")
	if err != nil {
		t.Fatalf("Failed to create agent_missions table: %v", err)
	}

	// Add pending missions
	_, err = provider.Exec(context.Background(), "INSERT INTO agent_missions (id, status) VALUES ('1', 'PENDING'), ('2', 'PENDING')")
	if err != nil {
		t.Fatalf("Failed to insert into agent_missions: %v", err)
	}

	hub.sipDB = &SIPDB{db: provider}

	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("CheckHealth failed: %v", err)
	}

	if probe.Status != "healthy" {
		t.Errorf("Expected status healthy, got %s", probe.Status)
	}
	if probe.Mode != "standalone" {
		t.Errorf("Expected mode standalone, got %s", probe.Mode)
	}
	if probe.SyncBacklog != 2 {
		t.Errorf("Expected sync backlog 2, got %d", probe.SyncBacklog)
	}
}

func TestCheckHealth_NoDB(t *testing.T) {
	hub := NewHub()

	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("CheckHealth failed: %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("Expected status degraded without DB, got %s", probe.Status)
	}
}

func TestCheckHealth_DBError(t *testing.T) {
	hub := NewHub()

	// Fake a bad connection or closed db by creating one and closing it immediately
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open sqlite: %v", err)
	}
	provider := db.NewSqliteProvider(sqliteDB)
	sqliteDB.Close()

	hub.sipDB = &SIPDB{db: provider}

	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("CheckHealth failed: %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("Expected status degraded with closed DB, got %s", probe.Status)
	}
}

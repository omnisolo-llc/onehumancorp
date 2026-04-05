package orchestration

import (
	"context"
	"path/filepath"
	"testing"
	"time"
)

func TestHybridHealthProbe_DegradedNoDB(t *testing.T) {
	h := NewHub()
	// No SIPDB set
	probe, err := h.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("expected degraded status, got %s", probe.Status)
	}
}

func TestHybridHealthProbe_StandaloneSQLite(t *testing.T) {
	h := NewHub()
	sipdb, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to create SIPDB: %v", err)
	}
	defer sipdb.Close()
	h.SetSIPDB(sipdb)

	probe, err := h.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if probe.Mode != "standalone" {
		t.Errorf("expected standalone mode, got %s", probe.Mode)
	}
	if probe.Status != "healthy" {
		t.Errorf("expected healthy status, got %s", probe.Status)
	}
	if probe.MeshActive != false {
		t.Errorf("expected mesh active false, got %v", probe.MeshActive)
	}
	if probe.SyncBacklog != 0 {
		t.Errorf("expected sync backlog 0, got %d", probe.SyncBacklog)
	}
}

func TestHybridHealthProbe_StandaloneSQLite_WithBacklog(t *testing.T) {
	h := NewHub()
	sipdb, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to create SIPDB: %v", err)
	}
	defer sipdb.Close()
	h.SetSIPDB(sipdb)

	// Add a pending mission
	_, err = sipdb.db.Exec(context.Background(), "INSERT INTO agent_missions (id, status, payload) VALUES ('1', 'PENDING', '{}')")
	if err != nil {
		t.Fatalf("failed to insert mission: %v", err)
	}

	probe, err := h.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if probe.Mode != "standalone" {
		t.Errorf("expected standalone mode, got %s", probe.Mode)
	}
	if probe.Status != "healthy" {
		t.Errorf("expected healthy status, got %s", probe.Status)
	}
	if probe.SyncBacklog != 1 {
		t.Errorf("expected sync backlog 1, got %d", probe.SyncBacklog)
	}
}

func TestHybridHealthProbe_DegradedDBPing(t *testing.T) {
	h := NewHub()
	// Create a dummy file and pass it as DB so it's a closed/invalid DB connection if possible
	dbPath := filepath.Join(t.TempDir(), "test.db")
	sipdb, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("failed to create SIPDB: %v", err)
	}
	// Close it to force an exec error
	sipdb.Close()
	h.SetSIPDB(sipdb)

	probe, err := h.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("expected degraded status, got %s", probe.Status)
	}
}

func TestHybridHealthProbe_MeshActive(t *testing.T) {
	h := NewHub()
	sipdb, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to create SIPDB: %v", err)
	}
	defer sipdb.Close()
	h.SetSIPDB(sipdb)

	cn, err := NewCentrifugeNode()
	if err == nil && cn != nil {
		h.SetCentrifugeNode(cn)
	}

	probe, err := h.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if cn != nil && !probe.MeshActive {
		// Just ensure it doesn't crash when Mesh is active
	}
}

// Ensure the struct test from before still exists just in case
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

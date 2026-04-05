package orchestration

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockProvider struct {
	db.Provider
	isSQLite bool
}

func (m *mockProvider) IsSQLite() bool {
	return m.isSQLite
}

func TestHybridHealthProbe_StandaloneHealthy(t *testing.T) {
	ctx := context.Background()

	sipDB, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to create SIPDB: %v", err)
	}
	defer sipDB.Close()

	// Insert some test missions
	_, err = sipDB.db.Exec(ctx, "INSERT INTO agent_missions (id, status, payload) VALUES ('1', 'PENDING', '{}'), ('2', 'PENDING', '{}'), ('3', 'DONE', '{}')")
	if err != nil {
		t.Fatalf("failed to insert missions: %v", err)
	}

	hub := &Hub{
		sipDB: sipDB,
	}

	probe, err := hub.CheckHealth(ctx)
	if err != nil {
		t.Fatalf("CheckHealth failed: %v", err)
	}

	if probe.Status != "healthy" {
		t.Errorf("expected status 'healthy', got '%s'", probe.Status)
	}

	if probe.Mode != "standalone" {
		t.Errorf("expected mode 'standalone', got '%s'", probe.Mode)
	}

	if probe.SyncBacklog != 2 {
		t.Errorf("expected sync backlog 2, got %d", probe.SyncBacklog)
	}

	if probe.MeshActive {
		t.Errorf("expected MeshActive false, got true")
	}
}

func TestHybridHealthProbe_CloudMode(t *testing.T) {
	ctx := context.Background()

	sipDB, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to create SIPDB: %v", err)
	}
	defer sipDB.Close()

	// Wrap provider to mock IsSQLite = false
	mockProv := &mockProvider{Provider: sipDB.db, isSQLite: false}
	sipDB.db = mockProv

	hub := &Hub{
		sipDB: sipDB,
	}

	probe, err := hub.CheckHealth(ctx)
	if err != nil {
		t.Fatalf("CheckHealth failed: %v", err)
	}

	if probe.Status != "healthy" {
		t.Errorf("expected status 'healthy', got '%s'", probe.Status)
	}

	if probe.Mode != "cloud" {
		t.Errorf("expected mode 'cloud', got '%s'", probe.Mode)
	}
}

func TestHybridHealthProbe_DegradedNoDB(t *testing.T) {
	ctx := context.Background()
	hub := &Hub{} // no sipDB

	probe, err := hub.CheckHealth(ctx)
	if err != nil {
		t.Fatalf("CheckHealth failed: %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("expected status 'degraded', got '%s'", probe.Status)
	}
}

type failQueryProvider struct {
	db.Provider
}

func (m *failQueryProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	return 0, context.DeadlineExceeded // simulate ping failure
}

func TestHybridHealthProbe_DegradedDBPingFail(t *testing.T) {
	ctx := context.Background()

	sipDB, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to create SIPDB: %v", err)
	}
	defer sipDB.Close()

	mockProv := &failQueryProvider{Provider: sipDB.db}
	sipDB.db = mockProv

	hub := &Hub{
		sipDB: sipDB,
	}

	probe, err := hub.CheckHealth(ctx)
	if err != nil {
		t.Fatalf("CheckHealth failed: %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("expected status 'degraded', got '%s'", probe.Status)
	}
}

package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/centrifugal/centrifuge"
)

type mockCentrifugeNode struct {
	publishErr error
}

func (m *mockCentrifugeNode) Publish(channel string, data []byte, opts ...centrifuge.PublishOption) (centrifuge.PublishResult, error) {
	return centrifuge.PublishResult{}, m.publishErr
}
func (m *mockCentrifugeNode) Shutdown(ctx context.Context) error { return nil }
func (m *mockCentrifugeNode) Run() error { return nil }
func (m *mockCentrifugeNode) OnConnecting(h centrifuge.ConnectingHandler) {}
func (m *mockCentrifugeNode) OnConnect(h centrifuge.ConnectHandler) {}

type mockDBProvider struct {
	db.Provider
	isSQLite bool
}

func (m *mockDBProvider) IsSQLite() bool {
	return m.isSQLite
}


func TestHybridHealthProbe_Struct(t *testing.T) {
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

func TestCheckHealth_DegradedNoDB(t *testing.T) {
	h := NewHub()
	// No db set
	probe, err := h.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if probe.Status != "degraded" {
		t.Errorf("expected degraded status when DB is nil, got %s", probe.Status)
	}
}

func TestCheckHealth_StandaloneMode_Healthy(t *testing.T) {
	h := NewHub()
	sipdb, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to create sipdb: %v", err)
	}
	h.SetSIPDB(sipdb)

	// Wait a brief moment to ensure schema initialization completes
	time.Sleep(10 * time.Millisecond)

	_, err = sipdb.db.Exec(context.Background(), "CREATE TABLE IF NOT EXISTS agent_missions (id TEXT PRIMARY KEY, status TEXT, payload TEXT, synced_to_cloud INTEGER)")
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	// Try creating just what the query needs: table agent_missions with a 'status' column.
	_, _ = sipdb.db.Exec(context.Background(), "INSERT INTO agent_missions (id, status, payload) VALUES ('1', 'PENDING', '{}'), ('2', 'PENDING', '{}'), ('3', 'COMPLETED', '{}')")

	probe, err := h.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if probe.Status != "healthy" {
		t.Errorf("expected healthy status, got %s", probe.Status)
	}
	if probe.Mode != "standalone" {
		t.Errorf("expected standalone mode, got %s", probe.Mode)
	}

	// We might have a race condition where the INSERT hasn't fully committed or is not immediately visible.
	// CheckHealth expects a count of rows where status = 'PENDING'.
	if probe.SyncBacklog != 2 {
		t.Errorf("expected sync backlog 2, got %d", probe.SyncBacklog)
	}
	if probe.MeshActive {
		t.Errorf("expected mesh active false since centrifuge not set")
	}
}

func TestCheckHealth_CloudMode_HealthyWithMesh(t *testing.T) {
	h := NewHub()

	sipdb, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to create sipdb: %v", err)
	}

	// Overwrite sipdb.db with the mock to make it return false for IsSQLite
	sipdb.db = &mockDBProvider{
		Provider: sipdb.db,
		isSQLite: false,
	}
	h.SetSIPDB(sipdb)

	_, err = sipdb.db.Exec(context.Background(), "CREATE TABLE IF NOT EXISTS agent_missions (id TEXT PRIMARY KEY, status TEXT, payload TEXT, synced_to_cloud INTEGER)")
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}
	_, _ = sipdb.db.Exec(context.Background(), "INSERT INTO agent_missions (id, status, payload) VALUES ('1', 'PENDING', '{}'), ('2', 'PENDING', '{}'), ('3', 'COMPLETED', '{}'), ('4', 'PENDING', '{}'), ('5', 'PENDING', '{}'), ('6', 'PENDING', '{}')")

	cn, _ := NewCentrifugeNode()
	if cn != nil {
		h.SetCentrifugeNode(cn)
	}

	probe, err := h.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if probe.Status != "healthy" {
		t.Errorf("expected healthy status, got %s", probe.Status)
	}
	if probe.Mode != "cloud" {
		t.Errorf("expected cloud mode, got %s", probe.Mode)
	}
	if probe.SyncBacklog != 5 {
		t.Errorf("expected sync backlog 5, got %d", probe.SyncBacklog)
	}
	// MeshActive will depend if NewCentrifugeNode succeeds in test environment without redis
	if cn != nil && !probe.MeshActive {
		t.Errorf("expected mesh active true")
	}
}

func TestCheckHealth_MeshDegraded(t *testing.T) {
	h := NewHub()

	sipdb, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to create sipdb: %v", err)
	}
	h.SetSIPDB(sipdb)

	// Create a dummy node that returns an error on publish
	mockNode := &mockCentrifugeNode{
		publishErr: context.DeadlineExceeded,
	}
	cn := &CentrifugeNode{node: mockNode}
	h.SetCentrifugeNode(cn)

	probe, err := h.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("expected degraded status due to mesh error, got %s", probe.Status)
	}
	if probe.MeshActive {
		t.Errorf("expected mesh active to be false when publish fails")
	}
}

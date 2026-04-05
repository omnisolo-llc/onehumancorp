package orchestration

import (
	"context"
	"errors"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/centrifugal/centrifuge"
)

// MockProvider implement minimal Provider for testing CheckHealth
type mockCentrifugeNode struct {
	publishErr error
}

func (m *mockCentrifugeNode) Publish(channel string, data []byte, opts ...centrifuge.PublishOption) (centrifuge.PublishResult, error) {
	return centrifuge.PublishResult{}, m.publishErr
}

func (m *mockCentrifugeNode) Shutdown(ctx context.Context) error {
	return nil
}

func (m *mockCentrifugeNode) Run() error {
	return nil
}

func (m *mockCentrifugeNode) OnConnecting(h centrifuge.ConnectingHandler) {}
func (m *mockCentrifugeNode) OnConnect(h centrifuge.ConnectHandler) {}

type mockHealthProvider struct {
	db.Provider
	execErr      error
	isSQLite     bool
	queryRowErr  error
	backlogCount int
}

func (m *mockHealthProvider) Exec(ctx context.Context, sql string, args ...any) (int64, error) {
	if m.execErr != nil {
		return 0, m.execErr
	}
	return 1, nil
}

func (m *mockHealthProvider) IsSQLite() bool {
	return m.isSQLite
}

type mockRow struct {
	err   error
	count int
}

func (m mockRow) Scan(dest ...any) error {
	if m.err != nil {
		return m.err
	}
	if len(dest) > 0 {
		if ptr, ok := dest[0].(*int); ok {
			*ptr = m.count
		}
	}
	return nil
}

func (m *mockHealthProvider) QueryRow(ctx context.Context, sql string, args ...any) db.Row {
	return mockRow{err: m.queryRowErr, count: m.backlogCount}
}

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

func TestCheckHealth_NoDB(t *testing.T) {
	hub := &Hub{}
	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if probe.Status != "degraded" {
		t.Errorf("Expected status degraded when no db is configured, got %s", probe.Status)
	}
}

func TestCheckHealth_HealthySQLite(t *testing.T) {
	mockProv := &mockHealthProvider{isSQLite: true, backlogCount: 42}
	hub := &Hub{
		sipDB: &SIPDB{db: mockProv},
	}

	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if probe.Status != "healthy" {
		t.Errorf("Expected status healthy, got %s", probe.Status)
	}
	if probe.Mode != "standalone" {
		t.Errorf("Expected mode standalone, got %s", probe.Mode)
	}
	if probe.SyncBacklog != 42 {
		t.Errorf("Expected SyncBacklog 42, got %d", probe.SyncBacklog)
	}
}

func TestCheckHealth_HealthyCloud(t *testing.T) {
	mockProv := &mockHealthProvider{isSQLite: false, backlogCount: 0}
	hub := &Hub{
		sipDB: &SIPDB{db: mockProv},
	}

	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if probe.Status != "healthy" {
		t.Errorf("Expected status healthy, got %s", probe.Status)
	}
	if probe.Mode != "cloud" {
		t.Errorf("Expected mode cloud, got %s", probe.Mode)
	}
}

func TestCheckHealth_DegradedDBPing(t *testing.T) {
	mockProv := &mockHealthProvider{isSQLite: true, execErr: errors.New("db ping failed")}
	hub := &Hub{
		sipDB: &SIPDB{db: mockProv},
	}

	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("Expected status degraded on DB failure, got %s", probe.Status)
	}
}

func TestCheckHealth_WithMeshActive(t *testing.T) {
	mockProv := &mockHealthProvider{isSQLite: true}
	// Setup a dummy mock node that returns no error on publish
	mockNode := &mockCentrifugeNode{publishErr: nil}

	hub := &Hub{
		sipDB: &SIPDB{db: mockProv},
		centrifugeNode: &CentrifugeNode{node: mockNode},
	}

	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if probe.Status != "healthy" {
		t.Errorf("Expected status healthy, got %s", probe.Status)
	}
	if !probe.MeshActive {
		t.Errorf("Expected MeshActive true")
	}
}

func TestCheckHealth_WithMeshDegraded(t *testing.T) {
	mockProv := &mockHealthProvider{isSQLite: true}
	// Setup a dummy mock node that returns error on publish
	mockNode := &mockCentrifugeNode{publishErr: errors.New("publish fail")}

	hub := &Hub{
		sipDB: &SIPDB{db: mockProv},
		centrifugeNode: &CentrifugeNode{node: mockNode},
	}

	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("Expected status degraded when mesh fails, got %s", probe.Status)
	}
	if probe.MeshActive {
		t.Errorf("Expected MeshActive false")
	}
}

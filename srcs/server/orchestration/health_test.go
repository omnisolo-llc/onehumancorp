package orchestration

import (
	"context"
	"errors"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/centrifugal/centrifuge"
)

// mockDBProvider is a basic mock for db.Provider.
type mockDBProvider struct {
	db.Provider
	execErr      error
	isSQLite     bool
	queryRowErr  error
	queryRowScan int
}

func (m *mockDBProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	return 0, m.execErr
}

func (m *mockDBProvider) IsSQLite() bool {
	return m.isSQLite
}

func (m *mockDBProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
	return &mockRow{
		err:  m.queryRowErr,
		scan: m.queryRowScan,
	}
}

type mockRow struct {
	err  error
	scan int
}

func (m *mockRow) Scan(dest ...any) error {
	if m.err != nil {
		return m.err
	}
	if len(dest) > 0 {
		if ptr, ok := dest[0].(*int); ok {
			*ptr = m.scan
		}
	}
	return nil
}

// mockNode is a stub for the Centrifuge node interface
type mockNode struct {
	publishErr error
}

func (m *mockNode) Publish(channel string, data []byte, opts ...centrifuge.PublishOption) (centrifuge.PublishResult, error) {
	return centrifuge.PublishResult{}, m.publishErr
}

func (m *mockNode) Shutdown(ctx context.Context) error { return nil }
func (m *mockNode) Run() error                         { return nil }
func (m *mockNode) OnConnecting(h centrifuge.ConnectingHandler) {}
func (m *mockNode) OnConnect(h centrifuge.ConnectHandler)       {}

func TestCheckHealth_Degraded_NoDB(t *testing.T) {
	hub := &Hub{}
	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("expected status 'degraded', got '%s'", probe.Status)
	}
}

func TestCheckHealth_Degraded_DBPingFails(t *testing.T) {
	mockProv := &mockDBProvider{execErr: errors.New("db error")}
	hub := &Hub{
		sipDB: &SIPDB{db: mockProv},
	}

	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("expected status 'degraded', got '%s'", probe.Status)
	}
}

func TestCheckHealth_Healthy_Standalone(t *testing.T) {
	mockProv := &mockDBProvider{isSQLite: true, queryRowScan: 42}
	hub := &Hub{
		sipDB: &SIPDB{db: mockProv},
	}

	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if probe.Status != "healthy" {
		t.Errorf("expected status 'healthy', got '%s'", probe.Status)
	}
	if probe.Mode != "standalone" {
		t.Errorf("expected mode 'standalone', got '%s'", probe.Mode)
	}
	if probe.SyncBacklog != 42 {
		t.Errorf("expected backlog 42, got %d", probe.SyncBacklog)
	}
}

func TestCheckHealth_Healthy_Cloud(t *testing.T) {
	mockProv := &mockDBProvider{isSQLite: false, queryRowScan: 7}
	hub := &Hub{
		sipDB: &SIPDB{db: mockProv},
	}

	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if probe.Mode != "cloud" {
		t.Errorf("expected mode 'cloud', got '%s'", probe.Mode)
	}
	if probe.SyncBacklog != 7 {
		t.Errorf("expected backlog 7, got %d", probe.SyncBacklog)
	}
}

func TestCheckHealth_Mesh_Healthy(t *testing.T) {
	mockProv := &mockDBProvider{isSQLite: true}
	mockN := &mockNode{}
	hub := &Hub{
		sipDB:          &SIPDB{db: mockProv},
		centrifugeNode: &CentrifugeNode{node: mockN},
	}

	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if probe.Status != "healthy" {
		t.Errorf("expected status 'healthy', got '%s'", probe.Status)
	}
	if !probe.MeshActive {
		t.Errorf("expected mesh active to be true")
	}
}

func TestCheckHealth_Mesh_Degraded(t *testing.T) {
	mockProv := &mockDBProvider{isSQLite: true}
	mockN := &mockNode{publishErr: errors.New("mesh error")}
	hub := &Hub{
		sipDB:          &SIPDB{db: mockProv},
		centrifugeNode: &CentrifugeNode{node: mockN},
	}

	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("expected status 'degraded', got '%s'", probe.Status)
	}
	if probe.MeshActive {
		t.Errorf("expected mesh active to be false")
	}
}

func TestCheckHealth_Healthy_Cloud_QueryErr(t *testing.T) {
	mockProv := &mockDBProvider{isSQLite: false, queryRowErr: errors.New("no backlog")}
	hub := &Hub{
		sipDB: &SIPDB{db: mockProv},
	}

	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if probe.Mode != "cloud" {
		t.Errorf("expected mode 'cloud', got '%s'", probe.Mode)
	}
	if probe.SyncBacklog != 0 {
		t.Errorf("expected backlog 0, got %d", probe.SyncBacklog)
	}
}

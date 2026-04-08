package orchestration

import (
	"context"
	"errors"
	"testing"
	"time"

	"github.com/centrifugal/centrifuge"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// MockProvider is a mock for db.Provider
type MockProvider struct {
	db.Provider
	execErr      error
	queryRowErr  error
	isSQLite     bool
	count        int
}

func (m *MockProvider) Exec(ctx context.Context, query string, args ...any) (db.Result, error) {
	return nil, m.execErr
}

func (m *MockProvider) QueryRow(ctx context.Context, query string, args ...any) db.Row {
	return &MockRow{err: m.queryRowErr, count: m.count}
}

func (m *MockProvider) IsSQLite() bool {
	return m.isSQLite
}

type MockRow struct {
	err   error
	count int
}

func (r *MockRow) Scan(dest ...any) error {
	if r.err != nil {
		return r.err
	}
	if len(dest) > 0 {
		if c, ok := dest[0].(*int); ok {
			*c = r.count
		}
	}
	return nil
}

// MockNode is a mock for Node interface
type MockNode struct {
	publishErr error
}

func (m *MockNode) Publish(channel string, data []byte, opts ...centrifuge.PublishOption) (centrifuge.PublishResult, error) {
	return centrifuge.PublishResult{}, m.publishErr
}

func (m *MockNode) Shutdown(ctx context.Context) error { return nil }
func (m *MockNode) Run() error { return nil }
func (m *MockNode) OnConnecting(h centrifuge.ConnectingHandler) {}
func (m *MockNode) OnConnect(h centrifuge.ConnectHandler) {}

func TestCheckHealth(t *testing.T) {
	t.Run("Healthy Cloud Mode", func(t *testing.T) {
		h := &Hub{
			sipDB: &SIPDB{
				db: &MockProvider{isSQLite: false, count: 5},
			},
			centrifugeNode: &CentrifugeNode{
				node: &MockNode{},
			},
		}

		probe, err := h.CheckHealth(context.Background())
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		if probe.Status != "healthy" {
			t.Errorf("expected status healthy, got %v", probe.Status)
		}
		if probe.Mode != "cloud" {
			t.Errorf("expected mode cloud, got %v", probe.Mode)
		}
		if !probe.MeshActive {
			t.Errorf("expected mesh active")
		}
		if probe.SyncBacklog != 5 {
			t.Errorf("expected backlog 5, got %v", probe.SyncBacklog)
		}
	})

	t.Run("Healthy Standalone Mode", func(t *testing.T) {
		h := &Hub{
			sipDB: &SIPDB{
				db: &MockProvider{isSQLite: true, count: 2},
			},
			centrifugeNode: &CentrifugeNode{
				node: &MockNode{},
			},
		}

		probe, err := h.CheckHealth(context.Background())
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		if probe.Status != "healthy" {
			t.Errorf("expected status healthy, got %v", probe.Status)
		}
		if probe.Mode != "standalone" {
			t.Errorf("expected mode standalone, got %v", probe.Mode)
		}
		if !probe.MeshActive {
			t.Errorf("expected mesh active")
		}
		if probe.SyncBacklog != 2 {
			t.Errorf("expected backlog 2, got %v", probe.SyncBacklog)
		}
	})

	t.Run("Degraded DB Mode", func(t *testing.T) {
		h := &Hub{
			sipDB: &SIPDB{
				db: &MockProvider{isSQLite: true, count: 2, execErr: errors.New("db error")},
			},
			centrifugeNode: &CentrifugeNode{
				node: &MockNode{},
			},
		}

		probe, err := h.CheckHealth(context.Background())
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		if probe.Status != "degraded" {
			t.Errorf("expected status degraded, got %v", probe.Status)
		}
	})

	t.Run("Degraded Mesh Mode", func(t *testing.T) {
		h := &Hub{
			sipDB: &SIPDB{
				db: &MockProvider{isSQLite: true, count: 2},
			},
			centrifugeNode: &CentrifugeNode{
				node: &MockNode{publishErr: errors.New("mesh error")},
			},
		}

		probe, err := h.CheckHealth(context.Background())
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		if probe.Status != "degraded" {
			t.Errorf("expected status degraded, got %v", probe.Status)
		}
		if probe.MeshActive {
			t.Errorf("expected mesh inactive")
		}
	})

	t.Run("Degraded No DB", func(t *testing.T) {
		h := &Hub{}

		probe, err := h.CheckHealth(context.Background())
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}
		if probe.Status != "degraded" {
			t.Errorf("expected status degraded, got %v", probe.Status)
		}
	})
}

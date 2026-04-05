package orchestration

import (
	"context"
	"errors"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestHybridHealthProbe_NoDBAndNoMesh(t *testing.T) {
	h := &Hub{}
	probe, err := h.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("Expected status 'degraded', got '%s'", probe.Status)
	}
	if probe.MeshActive != false {
		t.Errorf("Expected MeshActive false, got true")
	}
}

func TestHybridHealthProbe_DBError(t *testing.T) {
	mockDB := &mockDBProvider{
		execErr: errors.New("db down"),
	}
	sipDB := &SIPDatabase{db: mockDB}
	h := &Hub{sipDB: sipDB}

	probe, err := h.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("Expected status 'degraded', got '%s'", probe.Status)
	}
}

func TestHybridHealthProbe_DBSQLite(t *testing.T) {
	mockDB := &mockDBProvider{
		isSQLite: true,
		queryRowFunc: func(ctx context.Context, query string, args ...any) db.Row {
			return &mockRow{scanErr: nil, val: 5}
		},
	}
	sipDB := &SIPDatabase{db: mockDB}
	h := &Hub{sipDB: sipDB}

	probe, err := h.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if probe.Status != "healthy" {
		t.Errorf("Expected status 'healthy', got '%s'", probe.Status)
	}
	if probe.Mode != "standalone" {
		t.Errorf("Expected mode 'standalone', got '%s'", probe.Mode)
	}
	if probe.SyncBacklog != 5 {
		t.Errorf("Expected SyncBacklog 5, got %d", probe.SyncBacklog)
	}
}

func TestHybridHealthProbe_DBCloud(t *testing.T) {
	mockDB := &mockDBProvider{
		isSQLite: false,
		queryRowFunc: func(ctx context.Context, query string, args ...any) db.Row {
			return &mockRow{scanErr: nil, val: 10}
		},
	}
	sipDB := &SIPDatabase{db: mockDB}
	h := &Hub{sipDB: sipDB}

	probe, err := h.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if probe.Status != "healthy" {
		t.Errorf("Expected status 'healthy', got '%s'", probe.Status)
	}
	if probe.Mode != "cloud" {
		t.Errorf("Expected mode 'cloud', got '%s'", probe.Mode)
	}
	if probe.SyncBacklog != 10 {
		t.Errorf("Expected SyncBacklog 10, got %d", probe.SyncBacklog)
	}
}

func TestHybridHealthProbe_DBQueryRowError(t *testing.T) {
	mockDB := &mockDBProvider{
		isSQLite: false,
		queryRowFunc: func(ctx context.Context, query string, args ...any) db.Row {
			return &mockRow{scanErr: errors.New("row error")}
		},
	}
	sipDB := &SIPDatabase{db: mockDB}
	h := &Hub{sipDB: sipDB}

	probe, err := h.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// even if sync backlog query fails, status should still be healthy since SELECT 1 passed
	if probe.Status != "healthy" {
		t.Errorf("Expected status 'healthy', got '%s'", probe.Status)
	}
	if probe.SyncBacklog != 0 {
		t.Errorf("Expected SyncBacklog 0, got %d", probe.SyncBacklog)
	}
}

func TestHybridHealthProbe_MeshActiveAndDegraded(t *testing.T) {
	mockNode := &mockCentrifugeNode{
		publishErr: nil,
	}
	cNode := &CentrifugeNode{node: mockNode}

	mockDB := &mockDBProvider{isSQLite: true, queryRowFunc: func(ctx context.Context, query string, args ...any) db.Row { return &mockRow{} }}
	sipDB := &SIPDatabase{db: mockDB}

	h := &Hub{centrifugeNode: cNode, sipDB: sipDB}

	probe, err := h.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if probe.MeshActive != true {
		t.Errorf("Expected MeshActive true, got false")
	}

	// now test degraded mesh
	mockNode.publishErr = errors.New("mesh error")
	probe2, err := h.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if probe2.MeshActive != false {
		t.Errorf("Expected MeshActive false, got true")
	}
	if probe2.Status != "degraded" {
		t.Errorf("Expected Status degraded, got %s", probe2.Status)
	}
}

// Ensure CentrifugeNode mock returns context error if it's nil inner node
func TestHybridHealthProbe_MeshNilInner(t *testing.T) {
	cNode := &CentrifugeNode{node: nil}

	mockDB := &mockDBProvider{isSQLite: true, queryRowFunc: func(ctx context.Context, query string, args ...any) db.Row { return &mockRow{} }}
	sipDB := &SIPDatabase{db: mockDB}

	h := &Hub{centrifugeNode: cNode, sipDB: sipDB}

	probe, err := h.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if probe.MeshActive != false {
		t.Errorf("Expected MeshActive false, got true")
	}
	if probe.Status != "degraded" {
		t.Errorf("Expected Status degraded, got %s", probe.Status)
	}
}

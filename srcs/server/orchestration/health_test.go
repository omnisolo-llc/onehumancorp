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
	sqliteProvider := db.NewSQLiteProvider(":memory:")
	h.SetSIPDB(NewSIPDB(sqliteProvider))

	_, err := sqliteProvider.Exec(context.Background(), "CREATE TABLE agent_missions (status TEXT)")
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}
	_, _ = sqliteProvider.Exec(context.Background(), "INSERT INTO agent_missions (status) VALUES ('PENDING'), ('PENDING'), ('COMPLETED')")

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
	if probe.SyncBacklog != 2 {
		t.Errorf("expected sync backlog 2, got %d", probe.SyncBacklog)
	}
	if probe.MeshActive {
		t.Errorf("expected mesh active false since centrifuge not set")
	}
}

func TestCheckHealth_CloudMode_HealthyWithMesh(t *testing.T) {
	h := NewHub()

	// Use test mock provider or create fake pg provider
	// We just need a provider where IsSQLite returns false
	mockProvider := &db.MockProvider{
		ExecFn: func(ctx context.Context, sql string, args ...any) (int64, error) {
			return 1, nil
		},
		QueryRowFn: func(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
			return &db.MockRow{
				ScanFn: func(dest ...any) error {
					if len(dest) > 0 {
						if count, ok := dest[0].(*int); ok {
							*count = 5
						}
					}
					return nil
				},
			}
		},
		IsSQLiteFn: func() bool { return false },
	}

	h.SetSIPDB(NewSIPDB(mockProvider))

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

	mockProvider := &db.MockProvider{
		ExecFn: func(ctx context.Context, sql string, args ...any) (int64, error) {
			return 1, nil
		},
		QueryRowFn: func(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
			return &db.MockRow{
				ScanFn: func(dest ...any) error {
					return nil
				},
			}
		},
		IsSQLiteFn: func() bool { return true },
	}
	h.SetSIPDB(NewSIPDB(mockProvider))

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

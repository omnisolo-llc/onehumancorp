package orchestration

import (
	"context"
	"errors"
	"testing"

	"github.com/centrifugal/centrifuge"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestHybridHealthProbe_NoDB(t *testing.T) {
	hub := NewHub() // Hub without DB

	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("Expected status 'degraded', got '%s'", probe.Status)
	}
	if probe.MeshActive != false {
		t.Errorf("Expected MeshActive to be false")
	}
}

func TestHybridHealthProbe_WithSQLiteDB_Healthy(t *testing.T) {
	sqlitedb, err := db.NewSQLiteProvider(":memory:")
	if err != nil {
		t.Fatalf("failed to create sqlite provider: %v", err)
	}

	// Create dummy table for SyncBacklog check
	_, err = sqlitedb.Exec(context.Background(), "CREATE TABLE agent_missions (id TEXT PRIMARY KEY, status TEXT)")
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}
	_, err = sqlitedb.Exec(context.Background(), "INSERT INTO agent_missions (id, status) VALUES ('1', 'PENDING'), ('2', 'PENDING'), ('3', 'COMPLETED')")
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	hub := NewHub()
	hub.SetSIPDB(&SIPDB{db: sqlitedb})

	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if probe.Mode != "standalone" {
		t.Errorf("Expected mode 'standalone', got '%s'", probe.Mode)
	}
	if probe.Status != "healthy" {
		t.Errorf("Expected status 'healthy', got '%s'", probe.Status)
	}
	if probe.SyncBacklog != 2 {
		t.Errorf("Expected SyncBacklog to be 2, got %d", probe.SyncBacklog)
	}
}

// We simulate a failing database.
type failingDBProvider struct {
	db.Provider
}

func (f *failingDBProvider) Exec(ctx context.Context, query string, args ...interface{}) (int64, error) {
	if query == "SELECT 1" {
		return 0, errors.New("db ping failed")
	}
	return f.Provider.Exec(ctx, query, args...)
}

func TestHybridHealthProbe_WithDB_Degraded(t *testing.T) {
	sqlitedb, err := db.NewSQLiteProvider(":memory:")
	if err != nil {
		t.Fatalf("failed to create sqlite provider: %v", err)
	}

	hub := NewHub()
	hub.SetSIPDB(&SIPDB{db: &failingDBProvider{Provider: sqlitedb}})

	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("Expected status 'degraded', got '%s'", probe.Status)
	}
}

// simulate cloud mode provider
type cloudDBProvider struct {
	db.Provider
}

func (c *cloudDBProvider) IsSQLite() bool {
	return false
}

func TestHybridHealthProbe_CloudMode(t *testing.T) {
	sqlitedb, err := db.NewSQLiteProvider(":memory:")
	if err != nil {
		t.Fatalf("failed to create sqlite provider: %v", err)
	}

	// Create dummy table for SyncBacklog check
	_, err = sqlitedb.Exec(context.Background(), "CREATE TABLE agent_missions (id TEXT PRIMARY KEY, status TEXT)")
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	hub := NewHub()
	hub.SetSIPDB(&SIPDB{db: &cloudDBProvider{Provider: sqlitedb}})

	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if probe.Mode != "cloud" {
		t.Errorf("Expected mode 'cloud', got '%s'", probe.Mode)
	}
}

// mockNode for centrifuge tests
type mockNode struct {
	publishFunc func(channel string, data []byte, opts ...centrifuge.PublishOption) (centrifuge.PublishResult, error)
}

func (m *mockNode) Publish(channel string, data []byte, opts ...centrifuge.PublishOption) (centrifuge.PublishResult, error) {
	if m.publishFunc != nil {
		return m.publishFunc(channel, data, opts...)
	}
	return centrifuge.PublishResult{}, nil
}
func (m *mockNode) Shutdown(ctx context.Context) error          { return nil }
func (m *mockNode) Run() error                                  { return nil }
func (m *mockNode) OnConnecting(h centrifuge.ConnectingHandler) {}
func (m *mockNode) OnConnect(h centrifuge.ConnectHandler)       {}

func TestHybridHealthProbe_MeshActive(t *testing.T) {
	hub := NewHub()

	mn := &mockNode{
		publishFunc: func(channel string, data []byte, opts ...centrifuge.PublishOption) (centrifuge.PublishResult, error) {
			return centrifuge.PublishResult{}, nil
		},
	}

	cn := &CentrifugeNode{node: mn}
	hub.SetCentrifugeNode(cn)

	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if probe.MeshActive != true {
		t.Errorf("Expected MeshActive to be true")
	}
}

func TestHybridHealthProbe_MeshActive_Degraded(t *testing.T) {
	hub := NewHub()

	mn := &mockNode{
		publishFunc: func(channel string, data []byte, opts ...centrifuge.PublishOption) (centrifuge.PublishResult, error) {
			return centrifuge.PublishResult{}, errors.New("centrifuge fail")
		},
	}

	cn := &CentrifugeNode{node: mn}
	hub.SetCentrifugeNode(cn)

	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if probe.MeshActive != false {
		t.Errorf("Expected MeshActive to be false, since publishing failed")
	}
	if probe.Status != "degraded" {
		t.Errorf("Expected Status to be degraded, got %v", probe.Status)
	}
}

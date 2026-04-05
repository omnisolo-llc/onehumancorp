package orchestration

import (
	"context"
	"errors"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/centrifugal/centrifuge"
)

func TestHybridHealthProbeStruct(t *testing.T) {
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

func TestCheckHealth_Degraded_NoDB(t *testing.T) {
	h := NewHub()
	ctx := context.Background()
	probeIface, err := h.CheckHealth(ctx)
	probe := probeIface.(HybridHealthProbe)
	if err != nil {
		t.Fatalf("CheckHealth returned error: %v", err)
	}
	if probe.Status != "degraded" {
		t.Errorf("Expected status 'degraded', got '%s'", probe.Status)
	}
	if probe.Mode != "standalone" {
		t.Errorf("Expected mode 'standalone', got '%s'", probe.Mode)
	}
	if probe.MeshActive {
		t.Errorf("Expected MeshActive to be false")
	}
}

func TestCheckHealth_SQLite_Healthy(t *testing.T) {
	dbProvider, err := db.NewSQLiteProvider(":memory:")
	if err != nil {
		t.Fatalf("Failed to create SQLite provider: %v", err)
	}
	defer dbProvider.Close()

	ctx := context.Background()
	_, err = dbProvider.Exec(ctx, "CREATE TABLE agent_missions (status TEXT)")
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}
	_, err = dbProvider.Exec(ctx, "INSERT INTO agent_missions (status) VALUES ('PENDING')")
	if err != nil {
		t.Fatalf("Failed to insert row: %v", err)
	}
	_, err = dbProvider.Exec(ctx, "INSERT INTO agent_missions (status) VALUES ('PENDING')")
	if err != nil {
		t.Fatalf("Failed to insert row: %v", err)
	}

	h := NewHub()
	sipDB := &SIPDB{db: dbProvider}
	h.SetSIPDB(sipDB)

	probeIface, err := h.CheckHealth(ctx)
	probe := probeIface.(HybridHealthProbe)
	if err != nil {
		t.Fatalf("CheckHealth returned error: %v", err)
	}
	if probe.Status != "healthy" {
		t.Errorf("Expected status 'healthy', got '%s'", probe.Status)
	}
	if probe.Mode != "standalone" {
		t.Errorf("Expected mode 'standalone', got '%s'", probe.Mode)
	}
	if probe.SyncBacklog != 2 {
		t.Errorf("Expected sync backlog 2, got %d", probe.SyncBacklog)
	}
}

func TestCheckHealth_DBPingError(t *testing.T) {
	dbProvider, err := db.NewSQLiteProvider(":memory:")
	if err != nil {
		t.Fatalf("Failed to create SQLite provider: %v", err)
	}
	dbProvider.Close()

	h := NewHub()
	sipDB := &SIPDB{db: dbProvider}
	h.SetSIPDB(sipDB)

	ctx := context.Background()
	probeIface, err := h.CheckHealth(ctx)
	probe := probeIface.(HybridHealthProbe)
	if err != nil {
		t.Fatalf("CheckHealth returned error: %v", err)
	}
	if probe.Status != "degraded" {
		t.Errorf("Expected status 'degraded', got '%s'", probe.Status)
	}
}

type fakeNode struct {
	PublishErr error
}
func (f *fakeNode) Publish(channel string, data []byte, opts ...centrifuge.PublishOption) (centrifuge.PublishResult, error) {
	return centrifuge.PublishResult{}, f.PublishErr
}
func (f *fakeNode) Shutdown(ctx context.Context) error { return nil }
func (f *fakeNode) Run() error { return nil }
func (f *fakeNode) OnConnecting(h centrifuge.ConnectingHandler) {}
func (f *fakeNode) OnConnect(h centrifuge.ConnectHandler) {}

func TestCheckHealth_MeshError(t *testing.T) {
	dbProvider, err := db.NewSQLiteProvider(":memory:")
	if err != nil {
		t.Fatalf("Failed to create SQLite provider: %v", err)
	}
	defer dbProvider.Close()

	h := NewHub()
	sipDB := &SIPDB{db: dbProvider}
	h.SetSIPDB(sipDB)

	h.SetCentrifugeNode(&CentrifugeNode{
		node: &fakeNode{PublishErr: errors.New("mesh error")},
	})

	ctx := context.Background()
	_, err = dbProvider.Exec(ctx, "CREATE TABLE agent_missions (status TEXT)")
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	probeIface, err := h.CheckHealth(ctx)
	probe := probeIface.(HybridHealthProbe)
	if err != nil {
		t.Fatalf("CheckHealth returned error: %v", err)
	}
	if probe.Status != "degraded" {
		t.Errorf("Expected status 'degraded', got '%s'", probe.Status)
	}
	if probe.MeshActive {
		t.Errorf("Expected MeshActive to be false")
	}
}

func TestCheckHealth_MeshHealthy(t *testing.T) {
	dbProvider, err := db.NewSQLiteProvider(":memory:")
	if err != nil {
		t.Fatalf("Failed to create SQLite provider: %v", err)
	}
	defer dbProvider.Close()

	h := NewHub()
	sipDB := &SIPDB{db: dbProvider}
	h.SetSIPDB(sipDB)

	h.SetCentrifugeNode(&CentrifugeNode{
		node: &fakeNode{PublishErr: nil},
	})

	ctx := context.Background()
	_, err = dbProvider.Exec(ctx, "CREATE TABLE agent_missions (status TEXT)")
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	probeIface, err := h.CheckHealth(ctx)
	probe := probeIface.(HybridHealthProbe)
	if err != nil {
		t.Fatalf("CheckHealth returned error: %v", err)
	}
	if probe.Status != "healthy" {
		t.Errorf("Expected status 'healthy', got '%s'", probe.Status)
	}
	if !probe.MeshActive {
		t.Errorf("Expected MeshActive to be true")
	}
}

type fakePgProvider struct {
	*db.SqliteProvider
}
func (p *fakePgProvider) IsSQLite() bool {
	return false
}

func TestCheckHealth_CloudHealthy(t *testing.T) {
	dbProvider, err := db.NewSQLiteProvider(":memory:")
	if err != nil {
		t.Fatalf("Failed to create SQLite provider: %v", err)
	}
	defer dbProvider.Close()

	h := NewHub()
	// Override IsSQLite to return false to mock Postgres
	fakePg := &fakePgProvider{SqliteProvider: dbProvider}
	sipDB := &SIPDB{db: fakePg}
	h.SetSIPDB(sipDB)

	ctx := context.Background()
	_, err = dbProvider.Exec(ctx, "CREATE TABLE agent_missions (status TEXT)")
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	probeIface, err := h.CheckHealth(ctx)
	probe := probeIface.(HybridHealthProbe)
	if err != nil {
		t.Fatalf("CheckHealth returned error: %v", err)
	}
	if probe.Status != "healthy" {
		t.Errorf("Expected status 'healthy', got '%s'", probe.Status)
	}
	if probe.Mode != "cloud" {
		t.Errorf("Expected mode 'cloud', got '%s'", probe.Mode)
	}
}

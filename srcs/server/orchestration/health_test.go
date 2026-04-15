package orchestration

import (
	"context"
	"database/sql"
	"os"
	"testing"
	"time"

	_ "modernc.org/sqlite"
	"github.com/onehumancorp/mono/srcs/server/db"
)

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

func TestCheckHealthDegraded_NoDB(t *testing.T) {
	hub := NewHub()
	// No SIPDB and no CentrifugeNode set
	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("Expected status 'degraded', got '%s'", probe.Status)
	}
	if probe.MeshActive {
		t.Errorf("Expected MeshActive to be false")
	}
}

func TestCheckHealth_SQLite(t *testing.T) {
	// Setup a physical temporary sqlite file
	ctx := context.Background()
	tmpFile, err := os.CreateTemp("", "health_test_*.db")
	if err != nil {
		t.Fatalf("Failed to create temp db file: %v", err)
	}
	tmpFile.Close()
	defer os.Remove(tmpFile.Name())

	sqliteDB, err := sql.Open("sqlite", tmpFile.Name())
	if err != nil {
		t.Fatalf("Failed to open test sqlite db: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)
	defer sqliteDB.Close()

	// Ensure the agent_missions table exists
	_, err = provider.Exec(ctx, "CREATE TABLE agent_missions (status TEXT)")
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	// Insert some pending missions
	_, err = provider.Exec(ctx, "INSERT INTO agent_missions (status) VALUES ('PENDING')")
	if err != nil {
		t.Fatalf("Failed to insert pending mission: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO agent_missions (status) VALUES ('PENDING')")
	if err != nil {
		t.Fatalf("Failed to insert pending mission: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO agent_missions (status) VALUES ('DONE')")
	if err != nil {
		t.Fatalf("Failed to insert done mission: %v", err)
	}

	hub := NewHub()
	sipDB := &SIPDB{db: provider}
	hub.SetSIPDB(sipDB)

	probe, err := hub.CheckHealth(ctx)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if probe.Status != "healthy" {
		t.Errorf("Expected status 'healthy', got '%s'", probe.Status)
	}
	if probe.Mode != "standalone" {
		t.Errorf("Expected mode 'standalone', got '%s'", probe.Mode)
	}
	if probe.SyncBacklog != 2 {
		t.Errorf("Expected SyncBacklog to be 2, got %d", probe.SyncBacklog)
	}
	if probe.MeshActive {
		t.Errorf("Expected MeshActive to be false")
	}
}

func TestCheckHealth_MeshActive(t *testing.T) {
	ctx := context.Background()

	hub := NewHub()

	// Use a mock setup to test MeshActive
	cn, err := NewCentrifugeNode()
	if err != nil {
		t.Skipf("Skipping TestCheckHealth_MeshActive since NewCentrifugeNode failed to initialize: %v", err)
	}

	hub.SetCentrifugeNode(cn)
	probe, err := hub.CheckHealth(ctx)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("Expected status 'degraded', got '%s'", probe.Status)
	}
	if probe.MeshActive != true {
		t.Errorf("Expected MeshActive to be true")
	}
}

type mockProvider struct {
	db.Provider
	execErr   error
	isSqlite  bool
}

func (m *mockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	if m.execErr != nil {
		return 0, m.execErr
	}
	return 1, nil
}


func (m *mockProvider) Ping(ctx context.Context) error {
	if m.execErr != nil {
		return m.execErr
	}
	return nil
}

func (m *mockProvider) IsSQLite() bool {
	return m.isSqlite
}

func (m *mockProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
	return &mockRow{}
}

type mockRow struct {
}

func (r *mockRow) Scan(dest ...any) error {
	for _, d := range dest {
		switch v := d.(type) {
		case *int:
			*v = 5
		case *sql.NullTime:
			v.Time = time.Now()
			v.Valid = true
		}
	}
	return nil
}

func TestCheckHealth_DBPingFails(t *testing.T) {
	hub := NewHub()
	provider := &mockProvider{
		execErr: context.DeadlineExceeded,
	}
	sipDB := &SIPDB{db: provider}
	hub.SetSIPDB(sipDB)

	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}
	if probe.Status != "degraded" {
		t.Errorf("Expected status 'degraded', got '%s'", probe.Status)
	}
}

func TestCheckHealth_Postgres(t *testing.T) {
	hub := NewHub()
	provider := &mockProvider{
		isSqlite: false,
	}
	sipDB := &SIPDB{db: provider}
	hub.SetSIPDB(sipDB)

	probe, err := hub.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}
	if probe.Status != "healthy" {
		t.Errorf("Expected status 'healthy', got '%s'", probe.Status)
	}
	if probe.Mode != "cloud" {
		t.Errorf("Expected mode 'cloud', got '%s'", probe.Mode)
	}
	if probe.SyncBacklog != 5 {
		t.Errorf("Expected SyncBacklog to be 5, got %d", probe.SyncBacklog)
	}
}

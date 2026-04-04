package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestHybridMCPRAGDaemon_ProcessSync(t *testing.T) {
	defer ClearSemaphore()
	// Setup SQLite in-memory db
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	_, err = sqlDB.Exec(`
		CREATE TABLE agent_missions (
			id TEXT PRIMARY KEY,
			status TEXT,
			payload TEXT,
			synced_to_cloud BOOLEAN DEFAULT false
		)
	`)
	if err != nil {
		t.Fatalf("failed to create agent_missions table: %v", err)
	}

	_, err = sqlDB.Exec(`
		INSERT INTO agent_missions (id, status, payload, synced_to_cloud)
		VALUES
			('m1', 'CLOUD_ESCALATION', '{"task":"test-mission", "details":"[PRIVATE:secret] email is a@b.com"}', false),
			('m2', 'COMPLETED', '{"task":"synced-mission"}', true),
			('m3', 'PENDING', '{"task":"ignored"}', false)
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := &db.DB{Provider: sqliteProv}

	// Mock cloud API
	var receivedPayloads []SyncDaemonPayload
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/orchestration/sync" && r.Method == http.MethodPost {
			if err := json.NewDecoder(r.Body).Decode(&receivedPayloads); err != nil {
				w.WriteHeader(http.StatusBadRequest)
				return
			}
			w.WriteHeader(http.StatusOK)
		} else {
			w.WriteHeader(http.StatusNotFound)
		}
	}))
	defer srv.Close()

	daemon := NewHybridMCPRAGDaemon(dbWrapper, 1*time.Minute, srv.URL)

	// Process sync manually for testing
	daemon.ProcessSync(context.Background())

	// Validate received payload
	if len(receivedPayloads) != 1 {
		t.Fatalf("expected 1 mission to be synced, got %d", len(receivedPayloads))
	}
	if receivedPayloads[0].ID != "m1" {
		t.Errorf("expected payload ID m1, got %s", receivedPayloads[0].ID)
	}
	if receivedPayloads[0].Status != "CLOUD_ESCALATION" {
		t.Errorf("expected status CLOUD_ESCALATION, got %s", receivedPayloads[0].Status)
	}

	// Verify sanitization
	expectedPayload := `{"details":" email is [REDACTED_EMAIL]","task":"test-mission"}`
	if receivedPayloads[0].Payload != expectedPayload {
		t.Errorf("expected sanitized payload %q, got %q", expectedPayload, receivedPayloads[0].Payload)
	}

	// Validate db status updated
	var synced bool
	err = sqlDB.QueryRow("SELECT synced_to_cloud FROM agent_missions WHERE id = 'm1'").Scan(&synced)
	if err != nil {
		t.Fatalf("failed to query m1 synced status: %v", err)
	}
	if !synced {
		t.Error("expected m1 to be synced_to_cloud = true")
	}
}

func TestHybridMCPRAGDaemon_StartStop(t *testing.T) {
	defer ClearSemaphore()
	// Setup SQLite in-memory db
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	_, err = sqlDB.Exec(`
		CREATE TABLE agent_missions (
			id TEXT PRIMARY KEY,
			status TEXT,
			payload TEXT,
			synced_to_cloud BOOLEAN DEFAULT false
		)
	`)
	if err != nil {
		t.Fatalf("failed to create agent_missions table: %v", err)
	}

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := &db.DB{Provider: sqliteProv}

	daemon := NewHybridMCPRAGDaemon(dbWrapper, 10*time.Millisecond, "http://dummy")

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	daemon.Start(ctx)

	time.Sleep(50 * time.Millisecond)

	daemon.Stop()
	// No panic implies successful shutdown via stop channel
}

func TestHybridMCPRAGDaemon_ProcessSync_EmptyDB(t *testing.T) {
	defer ClearSemaphore()
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	_, err = sqlDB.Exec(`
		CREATE TABLE agent_missions (
			id TEXT PRIMARY KEY,
			status TEXT,
			payload TEXT,
			synced_to_cloud BOOLEAN DEFAULT false
		)
	`)
	if err != nil {
		t.Fatalf("failed to create agent_missions table: %v", err)
	}

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := &db.DB{Provider: sqliteProv}

	daemon := NewHybridMCPRAGDaemon(dbWrapper, 1*time.Minute, "http://dummy")
	daemon.ProcessSync(context.Background())
}

func TestHybridMCPRAGDaemon_ProcessSync_CloudError(t *testing.T) {
	defer ClearSemaphore()
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	_, err = sqlDB.Exec(`
		CREATE TABLE agent_missions (
			id TEXT PRIMARY KEY,
			status TEXT,
			payload TEXT,
			synced_to_cloud BOOLEAN DEFAULT false
		)
	`)
	if err != nil {
		t.Fatalf("failed to create agent_missions table: %v", err)
	}

	_, err = sqlDB.Exec(`
		INSERT INTO agent_missions (id, status, payload, synced_to_cloud)
		VALUES ('m1', 'CLOUD_ESCALATION', '{"task":"test"}', false)
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := &db.DB{Provider: sqliteProv}

	// Mock cloud API that returns an error
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
	}))
	defer srv.Close()

	daemon := NewHybridMCPRAGDaemon(dbWrapper, 1*time.Minute, srv.URL)
	daemon.ProcessSync(context.Background())

	// Validate db status is NOT updated
	var synced bool
	err = sqlDB.QueryRow("SELECT synced_to_cloud FROM agent_missions WHERE id = 'm1'").Scan(&synced)
	if err != nil {
		t.Fatalf("failed to query m1 synced status: %v", err)
	}
	if synced {
		t.Error("expected m1 to not be synced after cloud error")
	}
}

type mockNotSqliteProvider struct {
	*db.SqliteProvider
}

func (m *mockNotSqliteProvider) IsSQLite() bool {
	return false
}

func TestHybridMCPRAGDaemon_NotSQLite(t *testing.T) {
	defer ClearSemaphore()

	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	sqliteProv := db.NewSqliteProvider(sqlDB)
	mockProv := &mockNotSqliteProvider{SqliteProvider: sqliteProv}
	dbWrapper := &db.DB{Provider: mockProv}

	daemon := NewHybridMCPRAGDaemon(dbWrapper, 1*time.Minute, "http://dummy")

	// Start shouldn't panic
	daemon.Start(context.Background())

	// ProcessSync shouldn't panic
	daemon.ProcessSync(context.Background())
}

func TestHybridMCPRAGDaemon_Stop(t *testing.T) {
	defer ClearSemaphore()
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := &db.DB{Provider: sqliteProv}

	daemon := NewHybridMCPRAGDaemon(dbWrapper, 1*time.Minute, "http://dummy")
	daemon.Stop() // Testing multiple calls or direct calling
}

func TestHybridMCPRAGDaemon_Defaults(t *testing.T) {
	defer ClearSemaphore()
	daemon := NewHybridMCPRAGDaemon(nil, 1*time.Minute, "")
	if daemon.cloudAPIURL == "" {
		t.Error("expected default cloudAPIURL to be set")
	}
}

package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
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
			('m3', 'IGNORED', '{"task":"ignored"}', false)
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := &db.DB{Provider: sqliteProv}

	// Mock cloud API
	var receivedPayloads []SyncDaemonPayload
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/sync/missions" && r.Method == http.MethodPost {
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

func TestHybridMCPRAGDaemon_RAGSync(t *testing.T) {
	defer ClearSemaphore()
	// Setup SQLite in-memory db
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	_, err = sqlDB.Exec(`
		CREATE TABLE swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create swarm_memory_embeddings table: %v", err)
	}

	_, err = sqlDB.Exec(`
		INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status)
		VALUES
			('r1', 'rag content 1', 'pending'),
			('r2', 'rag content 2', 'synced'),
			('r3', 'rag content 3', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := &db.DB{Provider: sqliteProv}

	// Mock cloud API
	var receivedRecords struct {
		Records []RagSyncRecord `json:"records"`
	}
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/mcp/rag/sync" && r.Method == http.MethodPost {
			if err := json.NewDecoder(r.Body).Decode(&receivedRecords); err != nil {
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

	// Validate received records
	if len(receivedRecords.Records) != 2 {
		t.Fatalf("expected 2 RAG records to be synced, got %d", len(receivedRecords.Records))
	}

	// Validate db status updated
	var status string
	err = sqlDB.QueryRow("SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = 'r1'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to query r1 sync status: %v", err)
	}
	if status != "synced" {
		t.Errorf("expected r1 to be sync_status = 'synced', got %s", status)
	}

	var lastSyncAt sql.NullTime
	err = sqlDB.QueryRow("SELECT last_sync_at FROM swarm_memory_embeddings WHERE memory_id = 'r1'").Scan(&lastSyncAt)
	if err != nil {
		t.Fatalf("failed to query r1 last_sync_at: %v", err)
	}
	if !lastSyncAt.Valid {
		t.Error("expected last_sync_at to be set for r1")
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

	// Wait a moment for the goroutine to actually exit before we defer-close the DB
	time.Sleep(10 * time.Millisecond)
	// No panic implies successful shutdown via stop channel
}

package hybrid_sync

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

func TestHybridSyncDaemon_ProcessSync(t *testing.T) {
	// Setup SQLite in-memory db
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	_, err = sqlDB.Exec(`
		CREATE TABLE swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create swarm_memory_embeddings table: %v", err)
	}

	_, err = sqlDB.Exec(`
		INSERT INTO swarm_memory_embeddings (memory_id, context)
		VALUES
			('m1', '{"escalation_required":true, "details":" email is test@example.com"}'),
			('m2', '{"escalation_required":false, "details":"should be ignored"}'),
			('m3', '{"escalation_required":1, "data":"some public data"}')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := &db.DB{Provider: sqliteProv}

	// Mock cloud API
	var receivedPayloads []SyncPayload
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/sync/escalation" && r.Method == http.MethodPost {
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

	daemon := NewHybridSyncDaemon(dbWrapper, 1*time.Minute, srv.URL)

	// Process sync manually for testing
	daemon.ProcessSync(context.Background())

	// Validate received payload
	if len(receivedPayloads) != 2 {
		t.Fatalf("expected 2 memories to be synced, got %d", len(receivedPayloads))
	}

	hasM1 := false
	hasM3 := false
	for _, p := range receivedPayloads {
		if p.MemoryID == "m1" {
			hasM1 = true
			expectedPayload := `{"details":" email is [REDACTED_EMAIL]","escalation_required":true}`
			if p.Context != expectedPayload {
				t.Errorf("expected sanitized context %q, got %q", expectedPayload, p.Context)
			}
		} else if p.MemoryID == "m3" {
			hasM3 = true
		}
	}

	if !hasM1 || !hasM3 {
		t.Errorf("expected to sync m1 and m3")
	}

	// Validate db status updated
	var contextData string
	err = sqlDB.QueryRow("SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'm1'").Scan(&contextData)
	if err != nil {
		t.Fatalf("failed to query m1 context: %v", err)
	}

	var parsedContext map[string]interface{}
	json.Unmarshal([]byte(contextData), &parsedContext)

	if val, ok := parsedContext["escalation_required"]; ok {
		if boolVal, isBool := val.(bool); isBool && boolVal {
			t.Error("expected m1 escalation_required to be false, but was true")
		} else if floatVal, isFloat := val.(float64); isFloat && floatVal == 1 {
			t.Error("expected m1 escalation_required to be false, but was 1")
		}
	}
}

func TestHybridSyncDaemon_ProcessCRDTSync(t *testing.T) {
	// Setup SQLite in-memory db
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	_, err = sqlDB.Exec(`
		CREATE TABLE crdt_deltas (
			id TEXT PRIMARY KEY,
			entity_id TEXT NOT NULL,
			data TEXT NOT NULL,
			updated_at TIMESTAMP NOT NULL,
			synced_to_cloud BOOLEAN DEFAULT FALSE
		)
	`)
	if err != nil {
		t.Fatalf("failed to create crdt_deltas table: %v", err)
	}

	_, err = sqlDB.Exec(`
		INSERT INTO crdt_deltas (id, entity_id, data, updated_at, synced_to_cloud)
		VALUES
			('d1', 'e1', '{"status":"done"}', '2026-04-17 12:00:00', FALSE),
			('d2', 'e1', '{"status":"pending"}', '2026-04-17 13:00:00', TRUE),
			('d3', 'e2', '{"status":"active"}', '2026-04-17 14:00:00', FALSE)
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := &db.DB{Provider: sqliteProv}

	// Mock cloud API
	var receivedPayload map[string][]CRDTDelta
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/v1/sync/mcp-deltas" && r.Method == http.MethodPost {
			if err := json.NewDecoder(r.Body).Decode(&receivedPayload); err != nil {
				w.WriteHeader(http.StatusBadRequest)
				return
			}
			w.WriteHeader(http.StatusOK)
		} else {
			w.WriteHeader(http.StatusNotFound)
		}
	}))
	defer srv.Close()

	daemon := NewHybridSyncDaemon(dbWrapper, 1*time.Minute, srv.URL)

	// Process sync manually for testing
	daemon.ProcessCRDTSync(context.Background())

	// Validate received payload
	deltas := receivedPayload["deltas"]
	if len(deltas) != 2 {
		t.Fatalf("expected 2 deltas to be synced, got %d", len(deltas))
	}

	hasD1 := false
	hasD3 := false
	for _, p := range deltas {
		if p.ID == "d1" {
			hasD1 = true
		} else if p.ID == "d3" {
			hasD3 = true
		}
	}

	if !hasD1 || !hasD3 {
		t.Errorf("expected to sync d1 and d3")
	}

	// Validate db status updated
	var synced bool
	err = sqlDB.QueryRow("SELECT synced_to_cloud FROM crdt_deltas WHERE id = 'd1'").Scan(&synced)
	if err != nil {
		t.Fatalf("failed to query d1 status: %v", err)
	}

	if !synced {
		t.Error("expected d1 synced_to_cloud to be true, but was false")
	}
}

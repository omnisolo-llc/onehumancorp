package statesyncmcp

import (
	"context"
	"database/sql"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Failed to open sqlite: %v", err)
	}
	return db.NewSqliteProvider(sqliteDB)
}

func TestDBStateSyncProvider_SyncUp(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	_, _ = provider.Exec(ctx, "CREATE TABLE agent_missions (id TEXT PRIMARY KEY, status TEXT, payload TEXT, updated_at TIMESTAMP, sync_status TEXT DEFAULT 'pending')")
	_, _ = provider.Exec(ctx, "INSERT INTO agent_missions (id, status, payload, updated_at, sync_status) VALUES ('1', 'DONE', 'result1', '2023-01-01T00:00:00Z', 'pending')")
	_, _ = provider.Exec(ctx, "INSERT INTO agent_missions (id, status, payload, updated_at, sync_status) VALUES ('2', 'IN_PROGRESS', 'result2', '2023-01-02T00:00:00Z', 'synced')")

	var receivedPayload []TaskTransition
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/v1/sync/up" && r.Method == http.MethodPost {
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

	syncProvider := NewDBStateSyncProvider(provider, srv.URL)

	claims := &auth.Claims{OrganizationID: "org1"}
	res, err := syncProvider.SyncUp(ctx, claims)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if res.SyncedCount != 1 {
		t.Fatalf("Expected 1 synced count, got %d", res.SyncedCount)
	}

	if len(receivedPayload) != 1 {
		t.Fatalf("Expected 1 item received by cloud, got %d", len(receivedPayload))
	}
	if receivedPayload[0].ID != "1" {
		t.Fatalf("Expected ID 1, got %s", receivedPayload[0].ID)
	}

	// Verify idempotency (second sync should not send anything)
	receivedPayload = nil
	res2, err := syncProvider.SyncUp(ctx, claims)
	if err != nil {
		t.Fatalf("Expected no error on second sync, got %v", err)
	}

	if res2.SyncedCount != 0 {
		t.Fatalf("Expected 0 synced count on second sync, got %d", res2.SyncedCount)
	}
}

func TestDBStateSyncProvider_SyncDown(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	_, _ = provider.Exec(ctx, "CREATE TABLE agent_missions (id TEXT PRIMARY KEY, status TEXT, payload TEXT, updated_at TIMESTAMP, sync_status TEXT DEFAULT 'pending')")
	_, _ = provider.Exec(ctx, "INSERT INTO agent_missions (id, status, payload, updated_at) VALUES ('1', 'IN_PROGRESS', 'initial', '2023-01-01T00:00:00Z')")

	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/v1/sync/down" && r.Method == http.MethodGet {
			w.Header().Set("Content-Type", "application/json")
			t1 := time.Date(2023, 1, 2, 0, 0, 0, 0, time.UTC)
			payload := "newpayload"
			transitions := []TaskTransition{
				{ID: "1", Status: "DONE", Payload: &payload, UpdatedAt: t1},
				{ID: "2", Status: "DONE", Payload: nil, UpdatedAt: t1}, // ID 2 doesn't exist, should be inserted
			}
			json.NewEncoder(w).Encode(transitions)
		} else {
			w.WriteHeader(http.StatusNotFound)
		}
	}))
	defer srv.Close()

	syncProvider := NewDBStateSyncProvider(provider, srv.URL)

	claims := &auth.Claims{OrganizationID: "org1"}
	res, err := syncProvider.SyncDown(ctx, claims)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if res.SyncedCount != 2 {
		t.Fatalf("Expected 2 rows synced down (1 update, 1 insert), got %d", res.SyncedCount)
	}

	// Verify DB update for ID 1
	rows, _ := provider.Query(ctx, "SELECT status FROM agent_missions WHERE id = '1'")
	defer rows.Close()
	if rows.Next() {
		var status string
		rows.Scan(&status)
		if status != "DONE" {
			t.Fatalf("Expected status DONE for ID 1, got %s", status)
		}
	}

	// Verify DB insert for ID 2
	rows2, _ := provider.Query(ctx, "SELECT status FROM agent_missions WHERE id = '2'")
	defer rows2.Close()
	if rows2.Next() {
		var status string
		rows2.Scan(&status)
		if status != "DONE" {
			t.Fatalf("Expected status DONE for ID 2, got %s", status)
		}
	} else {
		t.Fatalf("Expected ID 2 to be inserted, but was not found")
	}
}

func TestDBStateSyncProvider_GetStatus(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	syncProvider := NewDBStateSyncProvider(provider, "http://localhost")

	claims := &auth.Claims{OrganizationID: "org1"}
	statusRaw, err := syncProvider.GetStatus(ctx, claims)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	statusMap := statusRaw.(map[string]interface{})
	if statusMap["status"] != "standalone_mode" {
		t.Fatalf("Expected standalone_mode, got %s", statusMap["status"])
	}
}

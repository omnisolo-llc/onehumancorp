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

func TestSyncDaemonProcessSyncTick(t *testing.T) {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqlDB.Close()

	prov := db.NewSqliteProvider(sqlDB)
	ctx := context.Background()

	// Create agent_missions table with synced_to_cloud
	_, err = prov.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS agent_missions (
			id TEXT PRIMARY KEY,
			status TEXT,
			payload JSON,
			synced_to_cloud BOOLEAN DEFAULT false,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	// Insert test data
	_, err = prov.Exec(ctx, "INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('m1', 'PENDING', '{\"foo\":\"bar\"}', false)")
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	// Setup mock remote server
	serverCalled := false
	var receivedPayload map[string]interface{}
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		serverCalled = true
		if r.URL.Path != "/api/sync/missions" {
			t.Errorf("expected path /api/sync/missions, got %s", r.URL.Path)
		}
		if err := json.NewDecoder(r.Body).Decode(&receivedPayload); err != nil {
			t.Errorf("failed to decode payload: %v", err)
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	// Setup daemon
	daemon := NewSyncDaemon(prov)
	daemon.remoteURL = ts.URL
	daemon.ticker = time.NewTicker(1 * time.Millisecond) // Don't actually use ticker for manual tick testing

	// Run single tick
	daemon.ProcessSyncTick()

	if !serverCalled {
		t.Fatalf("expected remote server to be called")
	}

	missions, ok := receivedPayload["missions"].([]interface{})
	if !ok || len(missions) != 1 {
		t.Fatalf("expected 1 mission in payload, got %v", receivedPayload["missions"])
	}

	// Check if synced_to_cloud was updated
	var synced bool
	err = prov.QueryRow(ctx, "SELECT synced_to_cloud FROM agent_missions WHERE id = 'm1'").Scan(&synced)
	if err != nil {
		t.Fatalf("failed to select synced_to_cloud: %v", err)
	}
	if !synced {
		t.Errorf("expected synced_to_cloud to be true, got false")
	}
}

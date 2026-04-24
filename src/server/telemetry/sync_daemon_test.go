package telemetry

import (
	"context"
	"database/sql"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	_ "modernc.org/sqlite"
)

func TestSyncDaemon(t *testing.T) {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("open sqlite: %v", err)
	}
	defer db.Close()

	_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS telemetry_buffer (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			metric_type TEXT NOT NULL,
			payload TEXT NOT NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("create table: %v", err)
	}

	_, err = db.Exec("INSERT INTO telemetry_buffer (metric_type, payload) VALUES ('test_metric', '{\"foo\":\"bar\"}')")
	if err != nil {
		t.Fatalf("insert: %v", err)
	}

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/telemetry/sync" {
			t.Errorf("unexpected path: %s", r.URL.Path)
		}
		var payloads []map[string]interface{}
		if err := json.NewDecoder(r.Body).Decode(&payloads); err != nil {
			t.Errorf("decode request body: %v", err)
		}
		if len(payloads) != 1 {
			t.Errorf("expected 1 payload, got %d", len(payloads))
		}
		if payloads[0]["metric_type"] != "test_metric" {
			t.Errorf("expected test_metric, got %v", payloads[0]["metric_type"])
		}
		if payloads[0]["payload"] != "{\"foo\":\"bar\"}" {
			t.Errorf("expected {\"foo\":\"bar\"}, got %v", payloads[0]["payload"])
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	daemon := NewSyncDaemon(db, server.URL, time.Second, 10)
	err = daemon.syncOnce(context.Background())
	if err != nil {
		t.Fatalf("syncOnce: %v", err)
	}

	var count int
	err = db.QueryRow("SELECT COUNT(*) FROM telemetry_buffer").Scan(&count)
	if err != nil {
		t.Fatalf("count: %v", err)
	}
	if count != 0 {
		t.Errorf("expected 0 rows after sync, got %d", count)
	}
}

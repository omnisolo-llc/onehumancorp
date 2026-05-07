package telemetry

import (
	"context"
	"database/sql"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
	"time"

	_ "github.com/mattn/go-sqlite3"
)

func setupTestDB(t *testing.T) *sql.DB {
	// Use file URI to enable shared cache since testing uses multiple connections
	db, err := sql.Open("sqlite3", "file:sync_worker_test?mode=memory&cache=shared")
	if err != nil {
		t.Fatalf("Failed to open database: %v", err)
	}

	_, err = db.Exec(`
		CREATE TABLE telemetry_buffer (
			id TEXT PRIMARY KEY,
			metric_name TEXT NOT NULL,
			metric_value REAL NOT NULL,
			attributes TEXT,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	return db
}

func TestSyncWorker(t *testing.T) {
	ResetForTest()
	db := setupTestDB(t)
	defer db.Close()

	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_TELEMETRY_ENABLED", "true")
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

	InitTelemetry(db)

	_, err := db.Exec(`INSERT INTO telemetry_buffer (id, metric_name, metric_value, attributes) VALUES (?, ?, ?, ?)`, "1", "test_metric", 1.0, "{}")
	if err != nil {
		t.Fatalf("Failed to insert mock data: %v", err)
	}

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Header.Get("X-OHC-Conflict-Resolution") != "force-local" {
			t.Errorf("Expected X-OHC-Conflict-Resolution header to be force-local")
		}

		var records []TelemetryRecord
		if err := json.NewDecoder(r.Body).Decode(&records); err != nil {
			t.Errorf("Failed to decode body: %v", err)
		}

		if len(records) != 1 {
			t.Errorf("Expected 1 record, got %d", len(records))
		}

		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	worker := NewSyncWorker(db, server.URL, 1*time.Millisecond)
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	go worker.Start(ctx)

	time.Sleep(200 * time.Millisecond) // wait for sync to happen

	// Verify the record was deleted
	var count int
	err = db.QueryRow("SELECT COUNT(*) FROM telemetry_buffer").Scan(&count)
	if err != nil {
		t.Fatalf("Failed to count rows: %v", err)
	}

	if count != 0 {
		t.Errorf("Expected 0 rows in telemetry_buffer, got %d", count)
	}
}

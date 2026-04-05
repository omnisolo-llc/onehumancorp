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
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestMetricSyncDaemon(t *testing.T) {
	ctx := context.Background()
	sqlDB, _ := sql.Open("sqlite", ":memory:")
	provider := db.NewSqliteProvider(sqlDB)
	dbWrapper := &db.DB{Provider: provider}

	// Setup telemetry_buffer table
	_, err := dbWrapper.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS telemetry_buffer (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			metric_type TEXT NOT NULL,
			payload TEXT NOT NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create telemetry_buffer table: %v", err)
	}

	// Insert some dummy metrics
	for i := 0; i < 505; i++ { // 505 to test looping
		_, err := dbWrapper.Exec(ctx, "INSERT INTO telemetry_buffer (metric_type, payload) VALUES ($1, $2)", "test_metric", `{"value":1}`)
		if err != nil {
			t.Fatalf("failed to insert metric: %v", err)
		}
	}

	callCount := 0
	// Setup mock server
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		callCount++
		var payloads []map[string]interface{}
		if err := json.NewDecoder(r.Body).Decode(&payloads); err != nil {
			t.Fatalf("failed to decode payloads: %v", err)
		}
		if callCount == 1 && len(payloads) != 500 {
			t.Errorf("expected 500 payloads, got %d", len(payloads))
		}
		if callCount == 2 && len(payloads) != 5 {
			t.Errorf("expected 5 payloads, got %d", len(payloads))
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	daemon := NewMetricSyncDaemon(dbWrapper, 1*time.Minute, ts.URL)
	daemon.ProcessSync(ctx)

	if callCount != 2 {
		t.Errorf("expected 2 server calls, got %d", callCount)
	}

	// Verify buffer is empty
	var count int
	err = dbWrapper.QueryRow(ctx, "SELECT COUNT(*) FROM telemetry_buffer").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 0 {
		t.Errorf("expected 0 remaining metrics, got %d", count)
	}
}

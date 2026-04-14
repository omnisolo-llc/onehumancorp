package telemetry

import (
	"context"
	"database/sql"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

// Helper to create test provider since it's defined in db_test package but not exported as a library function
func newTestProvider(t *testing.T) db.Provider {
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	// Create table for tests
	_, err = dbConn.Exec(`
		CREATE TABLE telemetry_buffer (
			id TEXT PRIMARY KEY,
			metric_name TEXT NOT NULL,
			value REAL NOT NULL,
			labels_json TEXT DEFAULT '{}',
			timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status TEXT DEFAULT 'pending',
			organization_id TEXT DEFAULT 'system'
		)
	`)
	if err != nil {
		t.Fatalf("failed to create test table: %v", err)
	}

	return db.NewSqliteProvider(dbConn)
}

func TestMcpSyncWorker_SyncsMetrics(t *testing.T) {
	provider := newTestProvider(t)
	defer provider.Close()

	ctx := context.Background()

	// Insert some pending metrics
	_, err := provider.Exec(ctx, `
		INSERT INTO telemetry_buffer (id, metric_name, value, labels_json, timestamp, sync_status)
		VALUES
			('uuid-1', 'test_metric_1', 10.5, '{"key":"value"}', CURRENT_TIMESTAMP, 'pending'),
			('uuid-2', 'test_metric_2', 20.0, '{}', CURRENT_TIMESTAMP, 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test metrics: %v", err)
	}

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	worker := NewMcpSyncWorker(provider, ts.URL)

	// Manually invoke the sync function for testing
	worker.syncMetrics(ctx)

	// Verify they are marked as synced
	var count int
	err = provider.QueryRow(ctx, "SELECT count(*) FROM telemetry_buffer WHERE sync_status = 'synced'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to count synced metrics: %v", err)
	}

	if count != 2 {
		t.Errorf("expected 2 metrics to be marked as synced, got %d", count)
	}

	// Verify no pending metrics remain
	err = provider.QueryRow(ctx, "SELECT count(*) FROM telemetry_buffer WHERE sync_status = 'pending'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to count pending metrics: %v", err)
	}

	if count != 0 {
		t.Errorf("expected 0 pending metrics, got %d", count)
	}
}

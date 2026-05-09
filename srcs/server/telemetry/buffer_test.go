package telemetry

import (
	"context"
	"database/sql"
	"testing"

	_ "github.com/mattn/go-sqlite3"
)

func TestTelemetryEngine_BufferMetric(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("failed to open db: %v", err)
	}
	defer db.Close()

	// Initialize schema
	_, err = db.Exec(`
		CREATE TABLE local_telemetry_metrics (
			id TEXT PRIMARY KEY,
			metric_name TEXT NOT NULL,
			value REAL NOT NULL,
			attributes TEXT,
			timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
			synced_to_cloud INTEGER NOT NULL DEFAULT FALSE
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	engine := NewTelemetrySyncEngine(db, "http://localhost:8080/telemetry")

	ctx := context.Background()
	attrs := map[string]interface{}{"service": "ohc"}
	err = engine.BufferMetric(ctx, "test_metric", 1.0, attrs)
	if err != nil {
		t.Fatalf("failed to buffer metric: %v", err)
	}

	var count int
	err = db.QueryRow("SELECT count(*) FROM local_telemetry_metrics").Scan(&count)
	if err != nil {
		t.Fatalf("failed to count metrics: %v", err)
	}

	if count != 1 {
		t.Fatalf("expected 1 metric, got %d", count)
	}
}

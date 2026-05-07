package telemetry

import (
	"database/sql"
	"os"
	"testing"
	"time"

	_ "github.com/mattn/go-sqlite3"
)

func setupTestDBMem(t *testing.T, name string) *sql.DB {
	db, err := sql.Open("sqlite3", "file:"+name+"?mode=memory&cache=shared")
	if err != nil {
		t.Fatalf("Failed to open database: %v", err)
	}

	_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS telemetry_buffer (
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

func TestInterceptMetric_StandaloneEnabled(t *testing.T) {
	ResetForTest()
	db := setupTestDBMem(t, "test1")
	defer db.Close()

	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_TELEMETRY_ENABLED", "true")
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

	InitTelemetry(db)

	intercepted := InterceptMetric("test_metric", 42.0, map[string]string{"foo": "bar"})
	if !intercepted {
		t.Errorf("Expected metric to be intercepted")
	}

	// Wait for the async flush
	time.Sleep(1 * time.Second)

	var count int
	err := db.QueryRow("SELECT COUNT(*) FROM telemetry_buffer").Scan(&count)
	if err != nil {
		t.Fatalf("Failed to count rows: %v", err)
	}

	if count != 1 {
		t.Errorf("Expected 1 row in telemetry_buffer, got %d", count)
	}
}

func TestInterceptMetric_StandaloneDisabled(t *testing.T) {
	ResetForTest()
	db := setupTestDBMem(t, "test2")
	defer db.Close()

	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_TELEMETRY_ENABLED", "false")
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

	InitTelemetry(db)

	intercepted := InterceptMetric("test_metric", 42.0, map[string]string{"foo": "bar"})
	if !intercepted {
		t.Errorf("Expected metric to be intercepted (but dropped due to disabled telemetry)")
	}

	time.Sleep(1 * time.Second)

	var count int
	err := db.QueryRow("SELECT COUNT(*) FROM telemetry_buffer").Scan(&count)
	if err != nil {
		t.Fatalf("Failed to count rows: %v", err)
	}

	if count != 0 {
		t.Errorf("Expected 0 rows in telemetry_buffer, got %d", count)
	}
}

func TestInterceptMetric_Cloud(t *testing.T) {
	ResetForTest()
	db := setupTestDBMem(t, "test3")
	defer db.Close()

	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	InitTelemetry(db)

	intercepted := InterceptMetric("test_metric", 42.0, map[string]string{"foo": "bar"})
	if intercepted {
		t.Errorf("Expected metric not to be intercepted in cloud mode")
	}
}

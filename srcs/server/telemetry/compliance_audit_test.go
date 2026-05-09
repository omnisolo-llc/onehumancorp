package telemetry

import (
	"context"
	"database/sql"
	"testing"
	"encoding/json"

	"github.com/stretchr/testify/assert"
	_ "github.com/mattn/go-sqlite3"
)

func setupTestDBForAudit(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite DB: %v", err)
	}

	_, err = db.Exec(`
		CREATE TABLE local_telemetry_metrics (
			id TEXT PRIMARY KEY,
			metric_name TEXT NOT NULL,
			value REAL NOT NULL,
			attributes JSON NOT NULL,
			timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
			synced_to_cloud BOOLEAN DEFAULT FALSE
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}
	return db
}

func getPendingMetricsCount(db *sql.DB) (int, error) {
    var count int
    err := db.QueryRow("SELECT COUNT(*) FROM local_telemetry_metrics WHERE synced_to_cloud = FALSE").Scan(&count)
    return count, err
}

func getFirstMetricAttributes(db *sql.DB) (map[string]interface{}, error) {
    var attrStr string
    err := db.QueryRow("SELECT attributes FROM local_telemetry_metrics WHERE synced_to_cloud = FALSE LIMIT 1").Scan(&attrStr)
    if err != nil {
        return nil, err
    }
    var attrs map[string]interface{}
    err = json.Unmarshal([]byte(attrStr), &attrs)
    return attrs, err
}


func TestStandaloneLocalSovereigntyTelemetry(t *testing.T) {
    t.Setenv("OHC_STANDALONE", "true")
	t.Setenv("OHC_TELEMETRY_ENABLED", "false")

    db := setupTestDBForAudit(t)
    defer db.Close()

	engine := NewTelemetrySyncEngine(db, "http://dummy")
	InitGlobalSyncEngine(engine)

	ctx := context.Background()
	bufferMetricHelper(ctx, "test_metric", 1.0, map[string]interface{}{"sensitive": "data"})

	count, err := getPendingMetricsCount(db)
	assert.NoError(t, err)
	assert.Equal(t, 0, count, "No metrics should be exfiltrated in standalone mode when telemetry is disabled")
}

func TestStandaloneSovereigntyRedaction(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "true")
	t.Setenv("OHC_TELEMETRY_ENABLED", "true")

    db := setupTestDBForAudit(t)
    defer db.Close()

	engine := NewTelemetrySyncEngine(db, "http://dummy")
	InitGlobalSyncEngine(engine)

	ctx := context.Background()
	attrs := map[string]interface{}{
		"email": "user@test.com",
		"tenant_id": "secret-tenant-1",
		"harmless": "data",
	}

	bufferMetricHelper(ctx, "sovereignty_metric", 1.0, attrs)

	count, err := getPendingMetricsCount(db)
	assert.NoError(t, err)
	assert.Equal(t, 1, count)

	redactedAttrs, err := getFirstMetricAttributes(db)
    assert.NoError(t, err)
	assert.Equal(t, "[REDACTED]", redactedAttrs["email"])
	assert.Equal(t, "[REDACTED]", redactedAttrs["tenant_id"])
	assert.Equal(t, "data", redactedAttrs["harmless"])
}

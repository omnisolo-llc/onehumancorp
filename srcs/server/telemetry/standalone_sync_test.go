package telemetry_test

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"os"
	"strings"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	_ "modernc.org/sqlite"
)

func TestStandaloneSyncFlow(t *testing.T) {
	// 1. Setup local SQLite buffer
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqlDB.Close()

	provider := db.NewSqliteProvider(sqlDB)
	sipdb, err := orchestration.NewSIPDBWithProvider(provider, "test-org")
	if err != nil {
		t.Fatalf("failed to create sipdb: %v", err)
	}

	ctx := context.Background()
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	// Inject the buffer function
	telemetry.BufferMetricFunc = sipdb.BufferMetric

	// 2. Record a metric with PII
	piiPayload := `{"agent_id": "agent-123", "email": "user@example.com", "api_key": "sk-123456789012345678901234567890123456789012345678"}`
	err = telemetry.BufferMetricFunc(ctx, "test_pii_metric", piiPayload)
	if err != nil {
		t.Fatalf("failed to buffer metric: %v", err)
	}

	// 3. Verify PII scrubbing in local buffer
	var payload string
	err = sqlDB.QueryRow("SELECT payload FROM local_telemetry_buffer WHERE metric_type = 'test_pii_metric'").Scan(&payload)
	if err != nil {
		t.Fatalf("failed to query buffer: %v", err)
	}

	if strings.Contains(payload, "user@example.com") {
		t.Errorf("PII not scrubbed from local buffer: %s", payload)
	}
	if !strings.Contains(payload, "[REDACTED_EMAIL]") {
		t.Errorf("Expected [REDACTED_EMAIL] in scrubbed payload: %s", payload)
	}

	// 4. Setup mock cloud endpoint
	var receivedPayloads []interface{}
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/telemetry/sync" {
			t.Errorf("unexpected path: %s", r.URL.Path)
		}
		var batch []interface{}
		if err := json.NewDecoder(r.Body).Decode(&batch); err != nil {
			t.Errorf("failed to decode sync batch: %v", err)
		}
		receivedPayloads = append(receivedPayloads, batch...)
		w.WriteHeader(http.StatusOK)
		fmt.Fprint(w, `{"status":"ok"}`)
	}))
	defer server.Close()

	// 5. Trigger sync worker
	syncedCount, err := sipdb.SyncBufferedMetrics(ctx, server.URL+"/api/telemetry/sync")
	if err != nil {
		t.Fatalf("sync failed: %v", err)
	}
	if syncedCount != 1 {
		t.Errorf("expected 1 synced record, got %d", syncedCount)
	}

	// 6. Verify data on "cloud" side
	if len(receivedPayloads) != 1 {
		t.Fatalf("expected 1 received payload, got %d", len(receivedPayloads))
	}

	cloudData, _ := json.Marshal(receivedPayloads[0])
	if strings.Contains(string(cloudData), "user@example.com") {
		t.Errorf("PII leaked to cloud: %s", string(cloudData))
	}

	// 7. Verify buffer is empty
	var count int
	err = sqlDB.QueryRow("SELECT COUNT(*) FROM local_telemetry_buffer").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 0 {
		t.Errorf("buffer not cleared after sync, count: %d", count)
	}
}

func TestTelemetrySyncWorkerExponentialBackoff(t *testing.T) {
	sqlDB, _ := sql.Open("sqlite", ":memory:")
	defer sqlDB.Close()

	// Create table
	_, _ = sqlDB.Exec(`CREATE TABLE local_telemetry_buffer (id INTEGER PRIMARY KEY, metric_type TEXT, payload TEXT)`)
	_, _ = sqlDB.Exec(`INSERT INTO local_telemetry_buffer (metric_type, payload) VALUES ('test', '{}')`)

	// Mock server that fails
	failCount := 0
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		failCount++
		w.WriteHeader(http.StatusInternalServerError)
	}))
	defer server.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	syncFunc := func(ctx context.Context, remoteEndpoint string) (int, error) {
		resp, err := http.Post(remoteEndpoint, "application/json", strings.NewReader("[]"))
		if err != nil {
			return 0, err
		}
		if resp.StatusCode != http.StatusOK {
			return 0, fmt.Errorf("fail")
		}
		return 1, nil
	}

	telemetry.StartTelemetrySyncWorker(ctx, syncFunc, server.URL, 10*time.Millisecond)

	// Wait for some retries
	time.Sleep(1500 * time.Millisecond)

	// With 1s initial backoff, it should have tried at 0ms, then failed, then wait 1s.
	// So around 2 attempts should happen in 1.5s.
	if failCount < 1 {
		t.Errorf("expected at least 1 fail attempt, got %d", failCount)
	}
	if failCount > 3 {
		t.Errorf("backoff seems not working, too many attempts: %d", failCount)
	}
}

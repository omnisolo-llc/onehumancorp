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
		CREATE TABLE IF NOT EXISTS offline_telemetry_buffer (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			metric_name TEXT NOT NULL,
			payload_bytes BLOB NOT NULL,
			timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("create table: %v", err)
	}

	payload := map[string]interface{}{"foo": "bar"}
	payloadBytes, _ := json.Marshal(payload)
	_, err = db.Exec("INSERT INTO offline_telemetry_buffer (metric_name, payload_bytes) VALUES ('test_metric', ?)", payloadBytes)
	if err != nil {
		t.Fatalf("insert: %v", err)
	}

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/v1/telemetry/sync" {
			t.Errorf("unexpected path: %s", r.URL.Path)
		}
		var batch []struct {
			MetricName string      `json:"metric_name"`
			Payload    interface{} `json:"payload"`
		}
		if err := json.NewDecoder(r.Body).Decode(&batch); err != nil {
			t.Errorf("decode request body: %v", err)
		}
		if len(batch) != 1 {
			t.Errorf("expected 1 item, got %d", len(batch))
		}
		if batch[0].MetricName != "test_metric" {
			t.Errorf("expected test_metric, got %v", batch[0].MetricName)
		}

		payloadMap := batch[0].Payload.(map[string]interface{})
		if payloadMap["foo"] != "bar" {
			t.Errorf("expected bar, got %v", payloadMap["foo"])
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
	err = db.QueryRow("SELECT COUNT(*) FROM offline_telemetry_buffer").Scan(&count)
	if err != nil {
		t.Fatalf("count: %v", err)
	}
	if count != 0 {
		t.Errorf("expected 0 rows after sync, got %d", count)
	}
}

func TestInitStandaloneBuffer(t *testing.T) {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("open sqlite: %v", err)
	}
	defer db.Close()

	_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS offline_telemetry_buffer (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			metric_name TEXT NOT NULL,
			payload_bytes BLOB NOT NULL,
			timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("create table: %v", err)
	}

	InitStandaloneBuffer(db)
	if BufferMetricFunc == nil {
		t.Fatal("BufferMetricFunc was not initialized")
	}

	payload := `{"foo":"bar"}`
	err = BufferMetricFunc(context.Background(), "test_type", payload)
	if err != nil {
		t.Fatalf("BufferMetricFunc failed: %v", err)
	}

	var count int
	err = db.QueryRow("SELECT COUNT(*) FROM offline_telemetry_buffer").Scan(&count)
	if err != nil {
		t.Fatalf("count: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 row, got %d", count)
	}

	var metricName string
	var payloadBytes []byte
	err = db.QueryRow("SELECT metric_name, payload_bytes FROM offline_telemetry_buffer").Scan(&metricName, &payloadBytes)
	if err != nil {
		t.Fatalf("scan: %v", err)
	}
	if metricName != "test_type" {
		t.Errorf("expected test_type, got %s", metricName)
	}
	if string(payloadBytes) != payload {
		t.Errorf("expected %s, got %s", payload, string(payloadBytes))
	}
}

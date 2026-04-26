package telemetry

import (
	"context"
	"database/sql"
	"io"
	"net/http"
	"strings"
	"testing"
	"time"

	"github.com/google/uuid"
	_ "modernc.org/sqlite"
)

type mockProvider struct {
	db *sql.DB
}

func (m *mockProvider) DB() *sql.DB {
	return m.db
}

type mockHTTPClient struct {
	DoFunc func(req *http.Request) (*http.Response, error)
}

func (m *mockHTTPClient) Do(req *http.Request) (*http.Response, error) {
	return m.DoFunc(req)
}

func TestMcpSyncWorker_SyncOnce_Success(t *testing.T) {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open db: %v", err)
	}
	defer db.Close()
	provider := &mockProvider{db: db}

	// Create table if not exists (sqlite memory db might need it for testing)
	_, err = provider.DB().Exec(`
		CREATE TABLE IF NOT EXISTS telemetry_buffer (
			id TEXT PRIMARY KEY,
			metric_name TEXT NOT NULL,
			value REAL NOT NULL,
			labels_json TEXT,
			timestamp DATETIME NOT NULL,
			sync_status TEXT DEFAULT 'pending'
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create test table: %v", err)
	}

	// Insert test data
	id1 := uuid.New().String()
	_, err = provider.DB().Exec(`
		INSERT INTO telemetry_buffer (id, metric_name, value, labels_json, timestamp, sync_status)
		VALUES (?, ?, ?, ?, ?, 'pending')
	`, id1, "test_metric_1", 42.0, "{}", time.Now())
	if err != nil {
		t.Fatalf("Failed to insert test metric: %v", err)
	}

	mockClient := &mockHTTPClient{
		DoFunc: func(req *http.Request) (*http.Response, error) {
			if req.Header.Get("X-OHC-Conflict-Resolution") != "force-local" {
				t.Errorf("Expected header X-OHC-Conflict-Resolution: force-local")
			}
			if req.Header.Get("Content-Type") != "application/json" {
				t.Errorf("Expected header Content-Type: application/json")
			}

			bodyBytes, _ := io.ReadAll(req.Body)
			req.Body.Close()
			if !strings.Contains(string(bodyBytes), `"test_metric_1"`) {
				t.Errorf("Expected body to contain test_metric_1, got: %s", string(bodyBytes))
			}

			return &http.Response{
				StatusCode: 200,
				Body:       io.NopCloser(strings.NewReader(`{"status":"ok"}`)),
			}, nil
		},
	}

	worker := NewMcpSyncWorker(provider, 100*time.Millisecond, "http://mock-endpoint", mockClient)

	ctx := context.Background()
	worker.syncOnce(ctx)

	// Verify sync status
	var status string
	err = provider.DB().QueryRow("SELECT sync_status FROM telemetry_buffer WHERE id = ?", id1).Scan(&status)
	if err != nil {
		t.Fatalf("Failed to query status: %v", err)
	}

	if status != "synced" {
		t.Errorf("Expected status 'synced', got '%s'", status)
	}
}

func TestMcpSyncWorker_SyncOnce_Failure(t *testing.T) {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open db: %v", err)
	}
	defer db.Close()
	provider := &mockProvider{db: db}

	// Create table if not exists
	_, err = provider.DB().Exec(`
		CREATE TABLE IF NOT EXISTS telemetry_buffer (
			id TEXT PRIMARY KEY,
			metric_name TEXT NOT NULL,
			value REAL NOT NULL,
			labels_json TEXT,
			timestamp DATETIME NOT NULL,
			sync_status TEXT DEFAULT 'pending'
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create test table: %v", err)
	}

	// Insert test data
	id1 := uuid.New().String()
	_, err = provider.DB().Exec(`
		INSERT INTO telemetry_buffer (id, metric_name, value, labels_json, timestamp, sync_status)
		VALUES (?, ?, ?, ?, ?, 'pending')
	`, id1, "test_metric_1", 42.0, "{}", time.Now())
	if err != nil {
		t.Fatalf("Failed to insert test metric: %v", err)
	}

	mockClient := &mockHTTPClient{
		DoFunc: func(req *http.Request) (*http.Response, error) {
			bodyBytes, _ := io.ReadAll(req.Body)
			req.Body.Close()
			if !strings.Contains(string(bodyBytes), `"test_metric_1"`) {
				t.Errorf("Expected body to contain test_metric_1, got: %s", string(bodyBytes))
			}

			return &http.Response{
				StatusCode: 500,
				Status:     "500 Internal Server Error",
				Body:       io.NopCloser(strings.NewReader(`{"status":"error"}`)),
			}, nil
		},
	}

	worker := NewMcpSyncWorker(provider, 100*time.Millisecond, "http://mock-endpoint", mockClient)

	ctx := context.Background()
	worker.syncOnce(ctx)

	// Verify sync status remains pending on HTTP failure
	var status string
	err = provider.DB().QueryRow("SELECT sync_status FROM telemetry_buffer WHERE id = ?", id1).Scan(&status)
	if err != nil {
		t.Fatalf("Failed to query status: %v", err)
	}

	if status != "pending" {
		t.Errorf("Expected status 'pending' due to HTTP error, got '%s'", status)
	}
}

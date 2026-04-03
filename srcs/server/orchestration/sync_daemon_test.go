package orchestration

import (
	"context"
	"encoding/json"
	"io"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestHybridSyncDaemon(t *testing.T) {
	ctx := context.Background()

	// Use shared mock DB setup for tests
	dbProvider, err := db.NewSqliteProvider("file::memory:?cache=shared", false)
	if err != nil {
		t.Fatalf("failed to create sqlite provider: %v", err)
	}
	defer dbProvider.Close()

	// Create required schema
	_, err = dbProvider.Exec(ctx, `
		CREATE TABLE agent_missions (
			id TEXT PRIMARY KEY,
			status TEXT,
			payload TEXT,
			created_at DATETIME,
			synced_to_cloud BOOLEAN DEFAULT false
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	// Insert test data
	_, err = dbProvider.Exec(ctx, `
		INSERT INTO agent_missions (id, status, payload, synced_to_cloud)
		VALUES
			('m1', 'PENDING', '{"task":"do work"}', false),
			('m2', 'COMPLETED', '{"task":"done"}', true),
			('m3', 'BLOCKED', '{"task":"blocked"}', false)
	`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	// Setup cloud API mock
	payloadReceived := false
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/sync/missions" {
			t.Errorf("expected path /api/sync/missions, got %s", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("expected POST method, got %s", r.Method)
		}

		body, _ := io.ReadAll(r.Body)
		var payloads []AutoDreamPayload
		err := json.Unmarshal(body, &payloads)
		if err != nil {
			t.Fatalf("failed to decode JSON array: %v", err)
		}

		if len(payloads) != 2 {
			t.Errorf("expected 2 payloads, got %d", len(payloads))
		}

		foundM1 := false
		for _, p := range payloads {
			if p.ID == "m1" {
				foundM1 = true
				if p.Metadata != "PENDING" {
					t.Errorf("expected m1 metadata PENDING, got %s", p.Metadata)
				}
			}
		}
		if !foundM1 {
			t.Errorf("expected m1 payload to be present")
		}

		payloadReceived = true
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"status":"success"}`))
	}))
	defer server.Close()

	// Initialize and run the daemon tick
	daemon := NewHybridSyncDaemon(dbProvider, time.Second, server.URL)
	daemon.ProcessSyncTick(ctx)

	// Verify cloud endpoint was hit
	if !payloadReceived {
		t.Errorf("expected cloud API to be hit but it was not")
	}

	// Verify local DB was updated
	var count int
	err = dbProvider.QueryRow(ctx, "SELECT COUNT(*) FROM agent_missions WHERE synced_to_cloud = false").Scan(&count)
	if err != nil {
		t.Fatalf("failed to count unsynced: %v", err)
	}
	if count != 0 {
		t.Errorf("expected 0 unsynced records, got %d", count)
	}
}

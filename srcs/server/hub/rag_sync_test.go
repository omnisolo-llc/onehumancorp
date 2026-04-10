package hub

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) *db.DB {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	dbWrapper, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to connect to test db: %v", err)
	}

	createTableQuery := `
	CREATE TABLE swarm_memory_embeddings (
		memory_id        TEXT PRIMARY KEY,
		context          TEXT NOT NULL,
		vector_embedding BYTEA,
		sync_status      TEXT DEFAULT 'pending',
		last_sync_at     TIMESTAMP NULL
	);
	`
	_, err = dbWrapper.Exec(context.Background(), createTableQuery)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return dbWrapper
}

func TestRAGSyncService_Flow(t *testing.T) {
	ctx := context.Background()
	dbWrapper := setupTestDB(t)
	defer dbWrapper.Close()

	// Insert pending records
	_, err := dbWrapper.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES ('1', 'test1', 'pending'), ('2', 'test2', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert initial data: %v", err)
	}

	// Setup mock cloud server
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var records []RAGSyncRecord
		if err := json.NewDecoder(r.Body).Decode(&records); err != nil {
			t.Errorf("failed to decode request body: %v", err)
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		if len(records) != 2 {
			t.Errorf("expected 2 records, got %d", len(records))
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	service := NewRAGSyncService(dbWrapper, server.URL)

	// Fetch pending syncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}

	// Push pending syncs
	err = service.PushPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify records are marked as synced
	rows, err := dbWrapper.Query(ctx, "SELECT memory_id, sync_status, last_sync_at FROM swarm_memory_embeddings")
	if err != nil {
		t.Fatalf("failed to query records: %v", err)
	}
	defer rows.Close()

	for rows.Next() {
		var id, status string
		var lastSyncAt time.Time
		if err := rows.Scan(&id, &status, &lastSyncAt); err != nil {
			t.Fatalf("failed to scan record: %v", err)
		}
		if status != "synced" {
			t.Errorf("expected status synced, got %s", status)
		}
		if lastSyncAt.IsZero() {
			t.Errorf("expected last_sync_at to be set")
		}
	}

	// Process incoming sync
	err = service.ProcessIncomingSync(ctx, []RAGSyncRecord{
		{ID: "3", Context: "test3", Vector: []float32{1.0, 2.0}},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify incoming sync was saved
	var count int
	err = dbWrapper.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_memory_embeddings WHERE memory_id = '3'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 record, got %d", count)
	}
}

package hub

import (
	"context"
	"encoding/json"
	"testing"
	"time"

	"database/sql"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func setupTestDB(t *testing.T) db.Provider {
	t.Helper()
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	// Ensure the db is alive
	if err := dbConn.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	// Important: register db cleanup
	t.Cleanup(func() {
		dbConn.Close()
	})

	provider := db.NewSqliteProvider(dbConn)

	ctx := context.Background()
	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
			organization_id  TEXT DEFAULT 'system',
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     DATETIME NULL
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	return provider
}

func TestRAGSyncProvider_FetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()
	ctx := context.Background()

	svc := NewRAGSyncProvider(provider)

	vec := []float32{0.1, 0.2, 0.3}
	vecJSON, _ := json.Marshal(vec)

	// Insert test data
	_, err := provider.Exec(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status)
		VALUES ('m1', 'test context 1', $1, 'pending'),
		       ('m2', 'test context 2', $1, 'synced'),
		       ('m3', 'test context 3', $1, 'pending')
	`, string(vecJSON))
	if err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	if len(records) != 2 {
		t.Errorf("Expected 2 pending records, got %d", len(records))
	}

	// Check content
	found1 := false
	for _, r := range records {
		if r.ID == "m1" {
			found1 = true
			if r.Context != "test context 1" {
				t.Errorf("Expected context 'test context 1', got %s", r.Context)
			}
			if len(r.Vector) != 3 {
				t.Errorf("Expected vector of length 3, got %d", len(r.Vector))
			} else if r.Vector[0] != 0.1 {
				t.Errorf("Expected vector[0] to be 0.1, got %f", r.Vector[0])
			}
		}
	}
	if !found1 {
		t.Errorf("Expected to find record m1")
	}
}

func TestRAGSyncProvider_MarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()
	ctx := context.Background()

	svc := NewRAGSyncProvider(provider)

	// Insert test data
	_, err := provider.Exec(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status)
		VALUES ('m1', 'c1', 'pending'), ('m2', 'c2', 'pending')
	`)
	if err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	err = svc.MarkSynced(ctx, []string{"m1"})
	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	var status string
	var lastSync sql.NullTime
	err = provider.QueryRow(ctx, "SELECT sync_status, last_sync_at FROM swarm_memory_embeddings WHERE memory_id = 'm1'").Scan(&status, &lastSync)
	if err != nil {
		t.Fatalf("Failed to query: %v", err)
	}

	if status != string(SyncStatusSynced) {
		t.Errorf("Expected status synced, got %s", status)
	}
	if !lastSync.Valid {
		t.Errorf("Expected last_sync_at to be set")
	}
}

func TestRAGSyncProvider_ProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()
	ctx := context.Background()

	svc := NewRAGSyncProvider(provider)

	// Insert test data to test ON CONFLICT
	_, err := provider.Exec(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status)
		VALUES ('m1', 'old context', 'pending')
	`)
	if err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	records := []RAGSyncRecord{
		{
			ID:         "m1",
			Context:    "new context",
			Vector:     []float32{0.5, 0.6},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now().UTC(),
		},
		{
			ID:         "m2",
			Context:    "fresh context",
			Vector:     []float32{0.7, 0.8},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now().UTC(),
		},
	}

	err = svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	// Verify m1 was updated
	var ctxStr string
	err = provider.QueryRow(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'm1'").Scan(&ctxStr)
	if err != nil {
		t.Fatalf("Failed to query m1: %v", err)
	}
	if ctxStr != "new context" {
		t.Errorf("Expected context 'new context', got %s", ctxStr)
	}

	// Verify m2 was inserted
	err = provider.QueryRow(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'm2'").Scan(&ctxStr)
	if err != nil {
		t.Fatalf("Failed to query m2: %v", err)
	}
	if ctxStr != "fresh context" {
		t.Errorf("Expected context 'fresh context', got %s", ctxStr)
	}
}

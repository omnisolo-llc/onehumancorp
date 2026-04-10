package hub_test

import (
	"context"
	"database/sql"
	"reflect"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/hub"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) (*sql.DB, db.Provider) {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite DB: %v", err)
	}

	createTableQuery := `
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin    TEXT,
			created_at       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     TIMESTAMPTZ NULL
		);
	`
	if _, err := sqlDB.Exec(createTableQuery); err != nil {
		t.Fatalf("failed to create tables: %v", err)
	}

	provider := db.NewSqliteProvider(sqlDB)
	return sqlDB, provider
}

func TestFetchPendingSyncs(t *testing.T) {
	sqlDB, provider := setupTestDB(t)
	defer sqlDB.Close()

	ctx := context.Background()
	syncProvider := hub.NewRAGSyncProvider(provider)

	// Insert some test data
	vecBytes := hub.FloatsToBytes([]float32{1.0, 2.0, 3.0})
	_, err := sqlDB.ExecContext(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status)
		VALUES ('m1', 'test context 1', ?, 'pending'),
		       ('m2', 'test context 2', ?, 'synced')
	`, vecBytes, vecBytes)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	records, err := syncProvider.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	if records[0].ID != "m1" {
		t.Errorf("expected record ID 'm1', got '%s'", records[0].ID)
	}
	if !reflect.DeepEqual(records[0].Vector, []float32{1.0, 2.0, 3.0}) {
		t.Errorf("expected vector [1.0, 2.0, 3.0], got %v", records[0].Vector)
	}
}

func TestMarkSynced(t *testing.T) {
	sqlDB, provider := setupTestDB(t)
	defer sqlDB.Close()

	ctx := context.Background()
	syncProvider := hub.NewRAGSyncProvider(provider)

	// Insert some test data
	_, err := sqlDB.ExecContext(ctx, `
		INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status)
		VALUES ('m1', 'test', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	if err := syncProvider.MarkSynced(ctx, []string{"m1"}); err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var status string
	var lastSyncAt sql.NullString
	if err := sqlDB.QueryRowContext(ctx, "SELECT sync_status, last_sync_at FROM swarm_memory_embeddings WHERE memory_id = 'm1'").Scan(&status, &lastSyncAt); err != nil {
		t.Fatalf("failed to query record: %v", err)
	}

	if status != "synced" {
		t.Errorf("expected status 'synced', got '%s'", status)
	}
	if !lastSyncAt.Valid {
		t.Error("expected last_sync_at to be valid")
	}
}

func TestProcessIncomingSync(t *testing.T) {
	sqlDB, provider := setupTestDB(t)
	defer sqlDB.Close()

	ctx := context.Background()
	syncProvider := hub.NewRAGSyncProvider(provider)

	now := time.Now()
	records := []hub.RAGSyncRecord{
		{
			ID:         "m1",
			Context:    "new context",
			Vector:     []float32{4.0, 5.0, 6.0},
			SyncStatus: hub.SyncStatusSynced,
			LastSyncAt: now,
		},
	}

	if err := syncProvider.ProcessIncomingSync(ctx, records); err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var contextStr string
	var vecBytes []byte
	var status string
	if err := sqlDB.QueryRowContext(ctx, "SELECT context, vector_embedding, sync_status FROM swarm_memory_embeddings WHERE memory_id = 'm1'").Scan(&contextStr, &vecBytes, &status); err != nil {
		t.Fatalf("failed to query record: %v", err)
	}

	if contextStr != "new context" {
		t.Errorf("expected context 'new context', got '%s'", contextStr)
	}
	if status != "synced" {
		t.Errorf("expected status 'synced', got '%s'", status)
	}

	floats, _ := hub.BytesToFloats(vecBytes)
	if !reflect.DeepEqual(floats, []float32{4.0, 5.0, 6.0}) {
		t.Errorf("expected vector [4.0, 5.0, 6.0], got %v", floats)
	}

	// Test conflict update
	records[0].Context = "updated context"
	if err := syncProvider.ProcessIncomingSync(ctx, records); err != nil {
		t.Fatalf("unexpected error on update: %v", err)
	}

	if err := sqlDB.QueryRowContext(ctx, "SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'm1'").Scan(&contextStr); err != nil {
		t.Fatalf("failed to query record: %v", err)
	}

	if contextStr != "updated context" {
		t.Errorf("expected context 'updated context', got '%s'", contextStr)
	}
}

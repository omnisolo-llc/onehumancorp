package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
	"go.opentelemetry.io/otel/metric/noop"
)

func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}

	// According to rules, we must explicitly create tables for testing sqlite in memory
	// and use exact data types for swarm_memory_embeddings
	createTableSQL := `
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding BYTEA,
			source_plugin    TEXT,
			created_at       TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_timestamp TIMESTAMPTZ NULL
		);
	`
	_, err = sqliteDB.Exec(createTableSQL)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return db.NewSqliteProvider(sqliteDB)
}

func TestDatabaseRAGSyncService_FetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	InitRAGSyncMetrics(noop.NewMeterProvider().Meter("test"))
	service := NewDatabaseRAGSyncService(provider)

	ctx := context.Background()

	// Insert some test data
	_, err := provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES (?, ?, ?)", "m1", "ctx1", "pending")
	if err != nil {
		t.Fatalf("insert failed: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES (?, ?, ?)", "m2", "ctx2", "synced")
	if err != nil {
		t.Fatalf("insert failed: %v", err)
	}

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("fetch failed: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	if records[0].ID != "m1" {
		t.Errorf("expected id m1, got %s", records[0].ID)
	}
}

func TestDatabaseRAGSyncService_MarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	InitRAGSyncMetrics(noop.NewMeterProvider().Meter("test"))
	service := NewDatabaseRAGSyncService(provider)

	ctx := context.Background()

	_, err := provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status) VALUES (?, ?, ?)", "m1", "ctx1", "pending")
	if err != nil {
		t.Fatalf("insert failed: %v", err)
	}

	err = service.MarkSynced(ctx, []string{"m1"})
	if err != nil {
		t.Fatalf("mark synced failed: %v", err)
	}

	var status string
	var lastSync sql.NullString
	err = provider.QueryRow(ctx, "SELECT sync_status, last_sync_timestamp FROM swarm_memory_embeddings WHERE memory_id = ?", "m1").Scan(&status, &lastSync)
	if err != nil {
		t.Fatalf("verify query failed: %v", err)
	}

	if status != string(SyncStatusSynced) {
		t.Errorf("expected status synced, got %s", status)
	}
	if !lastSync.Valid {
		t.Errorf("expected last_sync_timestamp to be valid")
	}
}

func TestDatabaseRAGSyncService_ProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	InitRAGSyncMetrics(noop.NewMeterProvider().Meter("test"))
	service := NewDatabaseRAGSyncService(provider)

	ctx := context.Background()

	records := []RAGSyncRecord{
		{
			ID:         "m2",
			Context:    "cloud ctx",
			Vector:     []float32{1.0, 2.0, 3.0},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}

	err := service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("process incoming sync failed: %v", err)
	}

	var contextData string
	var status string
	err = provider.QueryRow(ctx, "SELECT context, sync_status FROM swarm_memory_embeddings WHERE memory_id = ?", "m2").Scan(&contextData, &status)
	if err != nil {
		t.Fatalf("verify query failed: %v", err)
	}

	if contextData != "cloud ctx" {
		t.Errorf("expected context 'cloud ctx', got '%s'", contextData)
	}
	if status != string(SyncStatusSynced) {
		t.Errorf("expected status synced, got %s", status)
	}
}

package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel"
	sdkmetric "go.opentelemetry.io/otel/sdk/metric"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}

	_, err = sqlDB.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return db.NewSqliteProvider(sqlDB)
}

func TestDefaultRAGSyncService_FetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	_, err := provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES ('1', 'test content 1', 'pending'),
		       ('2', 'test content 2', 'synced')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	service := NewDefaultRAGSyncService(provider)
	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("Expected no error from FetchPendingSyncs, got %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("Expected 1 pending record, got %d", len(records))
	}

	if records[0].ID != "1" {
		t.Errorf("Expected record ID 1, got %s", records[0].ID)
	}
}

func TestDefaultRAGSyncService_MarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	_, err := provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES ('1', 'test content 1', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	// Initialize telemetry with dummy provider so metric vars are non-nil
	metricProvider := sdkmetric.NewMeterProvider()
	meter := metricProvider.Meter("test")
	err = telemetry.InitWithMeter(meter)
	if err != nil {
		t.Fatalf("Failed to init telemetry: %v", err)
	}
	defer metricProvider.Shutdown(ctx)

	service := NewDefaultRAGSyncService(provider)

	ids := []string{"1"}
	err = service.MarkSynced(ctx, ids)
	if err != nil {
		t.Fatalf("Expected no error from MarkSynced, got %v", err)
	}

	// Verify DB state
	row := provider.QueryRow(ctx, `SELECT sync_status FROM autodream_memories WHERE id = '1'`)
	var status string
	err = row.Scan(&status)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}

	if status != "synced" {
		t.Errorf("Expected status synced, got %s", status)
	}
}

func TestDefaultRAGSyncService_ProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()

	// Initialize telemetry with dummy provider so metric vars are non-nil
	metricProvider := sdkmetric.NewMeterProvider()
	otel.SetMeterProvider(metricProvider)
	meter := metricProvider.Meter("test")
	err := telemetry.InitWithMeter(meter)
	if err != nil {
		t.Fatalf("Failed to init telemetry: %v", err)
	}
	defer metricProvider.Shutdown(ctx)

	service := NewDefaultRAGSyncService(provider)

	records := []RAGSyncRecord{
		{
			ID:         "new-1",
			Context:    "incoming context",
			SyncStatus: SyncStatusPending,
			LastSyncAt: time.Now(),
		},
	}
	err = service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("Expected no error from ProcessIncomingSync, got %v", err)
	}

	// Verify DB state
	row := provider.QueryRow(ctx, `SELECT content, sync_status FROM autodream_memories WHERE id = 'new-1'`)
	var content, status string
	err = row.Scan(&content, &status)
	if err != nil {
		t.Fatalf("failed to query new record: %v", err)
	}

	if content != "incoming context" {
		t.Errorf("Expected content 'incoming context', got '%s'", content)
	}
	if status != "synced" {
		t.Errorf("Expected status 'synced', got '%s'", status)
	}
}

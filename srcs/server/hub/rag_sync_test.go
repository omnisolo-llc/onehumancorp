package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel/metric/noop"
	_ "modernc.org/sqlite"
)

func TestDefaultRAGSyncService(t *testing.T) {
	ctx := context.Background()

	// Set up telemetry mock
	m := noop.NewMeterProvider().Meter("test")
	telemetry.RAGRecordsSyncedTotal, _ = m.Int64Counter("rag_records_synced_total")
	telemetry.RAGSyncErrorsTotal, _ = m.Int64Counter("rag_sync_errors_total")

	// Set up SQLite memory db
	dbConn, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Failed to open sqlite: %v", err)
	}
	defer dbConn.Close()

	provider := db.NewSqliteProvider(dbConn)

	// Create tables
	_, err = provider.Exec(ctx, `
		CREATE TABLE consolidated_memory (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			agent_id TEXT,
			content TEXT NOT NULL,
			embedding BLOB,
			source_type TEXT NOT NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	// Insert test data
	_, err = provider.Exec(ctx, `
		INSERT INTO consolidated_memory (id, organization_id, content, source_type, sync_status)
		VALUES ('1', 'org1', 'test content 1', 'test', 'pending');
		INSERT INTO consolidated_memory (id, organization_id, content, source_type, sync_status)
		VALUES ('2', 'org1', 'test content 2', 'test', 'synced');
	`)
	if err != nil {
		t.Fatalf("Failed to insert data: %v", err)
	}

	service := NewDefaultRAGSyncService(provider)

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 1 {
		t.Fatalf("Expected 1 pending record, got %d", len(pending))
	}
	if pending[0].ID != "1" {
		t.Fatalf("Expected record ID '1', got '%s'", pending[0].ID)
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify MarkSynced
	var status string
	var lastSyncAt sql.NullTime
	row := provider.QueryRow(ctx, "SELECT sync_status, last_sync_at FROM consolidated_memory WHERE id = '1'")
	if err := row.Scan(&status, &lastSyncAt); err != nil {
		t.Fatalf("Failed to query updated record: %v", err)
	}
	if status != "synced" {
		t.Fatalf("Expected status 'synced', got '%s'", status)
	}
	if !lastSyncAt.Valid {
		t.Fatalf("Expected last_sync_at to be set")
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{
			ID: "3",
			Context: "incoming context 3",
		},
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify ProcessIncomingSync
	row = provider.QueryRow(ctx, "SELECT content, sync_status FROM consolidated_memory WHERE id = '3'")
	var content string
	if err := row.Scan(&content, &status); err != nil {
		t.Fatalf("Failed to query incoming record: %v", err)
	}
	if content != "incoming context 3" {
		t.Fatalf("Expected content 'incoming context 3', got '%s'", content)
	}
	if status != "synced" {
		t.Fatalf("Expected status 'synced', got '%s'", status)
	}
}

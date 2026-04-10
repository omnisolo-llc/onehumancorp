package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}

	_, err = sqliteDB.Exec(`
		CREATE TABLE consolidated_memory (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			agent_id TEXT,
			content TEXT NOT NULL,
			embedding TEXT,
			source_type TEXT NOT NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return db.NewSqliteProvider(sqliteDB)
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	service := NewRAGSyncService(provider)

	_, err := provider.Exec(ctx, `
		INSERT INTO consolidated_memory (id, organization_id, content, embedding, source_type, sync_status)
		VALUES
		('1', 'org1', 'test context 1', '[0.1, 0.2]', 'test', 'pending'),
		('2', 'org1', 'test context 2', null, 'test', 'synced')
	`)
	if err != nil {
		t.Fatalf("insert failed: %v", err)
	}

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	rec := records[0]
	if rec.ID != "1" {
		t.Errorf("expected ID '1', got %s", rec.ID)
	}
	if rec.Context != "test context 1" {
		t.Errorf("expected context 'test context 1', got %s", rec.Context)
	}
	if len(rec.Vector) != 2 || rec.Vector[0] != 0.1 || rec.Vector[1] != 0.2 {
		t.Errorf("unexpected vector: %v", rec.Vector)
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	service := NewRAGSyncService(provider)

	_, err := provider.Exec(ctx, `
		INSERT INTO consolidated_memory (id, organization_id, content, source_type, sync_status)
		VALUES ('1', 'org1', 'test context 1', 'test', 'pending')
	`)
	if err != nil {
		t.Fatalf("insert failed: %v", err)
	}

	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	row := provider.QueryRow(ctx, "SELECT sync_status FROM consolidated_memory WHERE id = '1'")
	var status string
	if err := row.Scan(&status); err != nil {
		t.Fatalf("query failed: %v", err)
	}

	if status != "synced" {
		t.Errorf("expected status 'synced', got %s", status)
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	service := NewRAGSyncService(provider)

	records := []RAGSyncRecord{
		{
			ID:         "new-1",
			Context:    "cloud context",
			Vector:     []float32{0.5, 0.6},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}

	err := service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	row := provider.QueryRow(ctx, "SELECT content, sync_status FROM consolidated_memory WHERE id = 'new-1'")
	var content, status string
	if err := row.Scan(&content, &status); err != nil {
		t.Fatalf("query failed: %v", err)
	}

	if content != "cloud context" {
		t.Errorf("expected content 'cloud context', got %s", content)
	}
	if status != "synced" {
		t.Errorf("expected status 'synced', got %s", status)
	}
}

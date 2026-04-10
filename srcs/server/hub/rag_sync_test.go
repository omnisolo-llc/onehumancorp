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
	dbConn, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite memory db: %v", err)
	}

	prov := db.NewSqliteProvider(dbConn)

	// Create table schema
	query := `CREATE TABLE autodream_memories (
		id TEXT PRIMARY KEY,
		content TEXT NOT NULL,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	)`
	_, err = prov.Exec(context.Background(), query)
	if err != nil {
		t.Fatalf("failed to create autodream_memories table: %v", err)
	}
	return prov
}

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	prov := setupTestDB(t)
	svc := NewRAGSyncService(prov)
	ctx := context.Background()

	// Insert test data
	_, err := prov.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES (?, ?, ?)", "1", "data1", "pending")
	if err != nil {
		t.Fatalf("insert failed: %v", err)
	}
	_, err = prov.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES (?, ?, ?)", "2", "data2", "synced")
	if err != nil {
		t.Fatalf("insert failed: %v", err)
	}
	_, err = prov.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES (?, ?, ?)", "3", "data3", "pending")
	if err != nil {
		t.Fatalf("insert failed: %v", err)
	}

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(records))
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	prov := setupTestDB(t)
	svc := NewRAGSyncService(prov)
	ctx := context.Background()

	// Insert test data
	_, err := prov.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES (?, ?, ?)", "1", "data1", "pending")
	if err != nil {
		t.Fatalf("insert failed: %v", err)
	}

	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify update
	var status string
	var lastSync *time.Time
	err = prov.QueryRow(ctx, "SELECT sync_status, last_sync_at FROM autodream_memories WHERE id = '1'").Scan(&status, &lastSync)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}

	if status != "synced" {
		t.Errorf("expected status 'synced', got %s", status)
	}
	if lastSync == nil {
		t.Errorf("expected last_sync_at to be set, got nil")
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	prov := setupTestDB(t)
	svc := NewRAGSyncService(prov)
	ctx := context.Background()

	records := []RAGSyncRecord{
		{ID: "100", Context: "new cloud data"},
	}

	err := svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var content, status string
	err = prov.QueryRow(ctx, "SELECT content, sync_status FROM autodream_memories WHERE id = '100'").Scan(&content, &status)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}

	if content != "new cloud data" {
		t.Errorf("expected 'new cloud data', got '%s'", content)
	}
	if status != "synced" {
		t.Errorf("expected status 'synced', got '%s'", status)
	}
}

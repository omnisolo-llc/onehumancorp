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
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Failed to open sqlite memory db: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)

	ctx := context.Background()

	// Create tables explicitly for tests since migrations aren't run automatically
	schema := `
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		);
	`
	_, err = provider.Exec(ctx, schema)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	return provider
}

func TestFetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	svc, err := NewRAGSyncService(provider)
	if err != nil {
		t.Fatalf("Failed to create service: %v", err)
	}

	// Insert some test data
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES ('1', 'ctx1', '[1.0, 2.0]', 'pending')")
	if err != nil {
		t.Fatalf("Failed to insert data: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES ('2', 'ctx2', '[3.0, 4.0]', 'synced')")
	if err != nil {
		t.Fatalf("Failed to insert data: %v", err)
	}

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("Expected 1 record, got %d", len(records))
	}

	if records[0].ID != "1" {
		t.Errorf("Expected ID '1', got '%s'", records[0].ID)
	}
	if len(records[0].Vector) != 2 || records[0].Vector[0] != 1.0 {
		t.Errorf("Unexpected vector parsing result: %v", records[0].Vector)
	}
}

func TestMarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	svc, err := NewRAGSyncService(provider)
	if err != nil {
		t.Fatalf("Failed to create service: %v", err)
	}

	// Insert test data
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'ctx1', 'pending')")
	if err != nil {
		t.Fatalf("Failed to insert data: %v", err)
	}

	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify update
	rows, err := provider.Query(ctx, "SELECT sync_status FROM autodream_memories WHERE id = '1'")
	if err != nil {
		t.Fatalf("Failed to query data: %v", err)
	}
	defer rows.Close()

	if !rows.Next() {
		t.Fatal("Expected record to exist")
	}

	var status string
	if err := rows.Scan(&status); err != nil {
		t.Fatal(err)
	}

	if status != string(SyncStatusSynced) {
		t.Errorf("Expected status 'synced', got '%s'", status)
	}
}

func TestProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	svc, err := NewRAGSyncService(provider)
	if err != nil {
		t.Fatalf("Failed to create service: %v", err)
	}

	records := []RAGSyncRecord{
		{
			ID:         "1",
			Context:    "test context",
			Vector:     []float32{1.5, 2.5},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}

	err = svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify insertion
	rows, err := provider.Query(ctx, "SELECT content, sync_status, embedding FROM autodream_memories WHERE id = '1'")
	if err != nil {
		t.Fatalf("Failed to query data: %v", err)
	}
	defer rows.Close()

	if !rows.Next() {
		t.Fatal("Expected record to exist")
	}

	var content, status, embedding string
	if err := rows.Scan(&content, &status, &embedding); err != nil {
		t.Fatal(err)
	}

	if content != "test context" {
		t.Errorf("Expected 'test context', got '%s'", content)
	}
	if status != string(SyncStatusSynced) {
		t.Errorf("Expected status 'synced', got '%s'", status)
	}
	if embedding != "[1.5,2.5]" {
		t.Errorf("Expected '[1.5,2.5]', got '%s'", embedding)
	}
}

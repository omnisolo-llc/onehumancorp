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
		t.Fatalf("failed to open sqlite: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)

	// Clean up for shared cache
	_, err = provider.Exec(context.Background(), "DROP TABLE IF EXISTS autodream_memories")
	if err != nil {
		t.Fatalf("failed to drop table: %v", err)
	}

	createTableQuery := `
	CREATE TABLE autodream_memories (
		id TEXT PRIMARY KEY,
		content TEXT NOT NULL,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	);`

	_, err = provider.Exec(context.Background(), createTableQuery)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return provider
}

func TestFetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	service := NewDBRAGSyncService(provider)
	ctx := context.Background()

	// Insert test data
	_, err := provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES (?, ?, ?)", "1", "test context 1", "pending")
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES (?, ?, ?)", "2", "test context 2", "synced")
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES (?, ?, ?)", "3", "test context 3", "pending")
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 2 {
		t.Fatalf("expected 2 records, got %d", len(records))
	}

	// Verify order doesn't matter, just IDs
	ids := map[string]bool{}
	for _, r := range records {
		ids[r.ID] = true
		if r.SyncStatus != SyncStatusPending {
			t.Errorf("expected status pending, got %v", r.SyncStatus)
		}
	}

	if !ids["1"] || !ids["3"] {
		t.Errorf("expected IDs 1 and 3, got %v", ids)
	}
}

func TestMarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	service := NewDBRAGSyncService(provider)
	ctx := context.Background()

	// Insert test data
	_, err := provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES (?, ?, ?)", "1", "test context 1", "pending")
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES (?, ?, ?)", "2", "test context 2", "pending")
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	err = service.MarkSynced(ctx, []string{"1", "2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	rows, err := provider.Query(ctx, "SELECT id, sync_status FROM autodream_memories WHERE sync_status = 'synced'")
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	defer rows.Close()

	count := 0
	for rows.Next() {
		var id string
		var status string
		if err := rows.Scan(&id, &status); err != nil {
			t.Fatalf("scan failed: %v", err)
		}
		count++
		if status != "synced" {
			t.Errorf("expected status synced, got %s", status)
		}
	}

	if count != 2 {
		t.Errorf("expected 2 synced records, got %d", count)
	}
}

func TestProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	service := NewDBRAGSyncService(provider)
	ctx := context.Background()

	// Existing record
	_, err := provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES (?, ?, ?)", "1", "old content", "pending")
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	records := []RAGSyncRecord{
		{
			ID:         "1",
			Context:    "updated context",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
		{
			ID:         "2",
			Context:    "new context",
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}

	err = service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	rows, err := provider.Query(ctx, "SELECT id, content, sync_status FROM autodream_memories ORDER BY id")
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}
	defer rows.Close()

	var count int
	for rows.Next() {
		var id, content, status string
		if err := rows.Scan(&id, &content, &status); err != nil {
			t.Fatalf("scan failed: %v", err)
		}

		if id == "1" {
			if content != "updated context" || status != "synced" {
				t.Errorf("record 1 not updated correctly: %s, %s", content, status)
			}
		} else if id == "2" {
			if content != "new context" || status != "synced" {
				t.Errorf("record 2 not inserted correctly: %s, %s", content, status)
			}
		} else {
			t.Errorf("unexpected record id: %s", id)
		}
		count++
	}

	if count != 2 {
		t.Errorf("expected 2 records, got %d", count)
	}
}

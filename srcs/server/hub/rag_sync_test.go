package hub

import (
	"context"
	"database/sql"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	d, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}

	_, err = d.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status TEXT DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return db.NewSqliteProvider(d)
}

func TestFetchPendingSyncs(t *testing.T) {
	ctx := context.Background()
	provider := setupTestDB(t)
	defer provider.Close()

	_, err := provider.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'context 1', 'pending'), ('2', 'context 2', 'synced'), ('3', 'context 3', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert records: %v", err)
	}

	manager := NewRAGSyncManager(provider, "http://localhost")
	pending, err := manager.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}
}

func TestMarkSynced(t *testing.T) {
	ctx := context.Background()
	provider := setupTestDB(t)
	defer provider.Close()

	_, err := provider.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'context 1', 'pending'), ('2', 'context 2', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert records: %v", err)
	}

	manager := NewRAGSyncManager(provider, "http://localhost")
	err = manager.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	rows, err := provider.Query(ctx, "SELECT id, sync_status FROM autodream_memories ORDER BY id")
	if err != nil {
		t.Fatalf("failed to query records: %v", err)
	}
	defer rows.Close()

	var statuses []string
	for rows.Next() {
		var id, status string
		if err := rows.Scan(&id, &status); err != nil {
			t.Fatalf("failed to scan: %v", err)
		}
		statuses = append(statuses, status)
	}

	if statuses[0] != "synced" {
		t.Errorf("expected record 1 to be synced")
	}
	if statuses[1] != "pending" {
		t.Errorf("expected record 2 to be pending")
	}
}

func TestProcessIncomingSync(t *testing.T) {
	ctx := context.Background()
	provider := setupTestDB(t)
	defer provider.Close()

	manager := NewRAGSyncManager(provider, "http://localhost")

	now := time.Now()
	newRecords := []RAGSyncRecord{
		{ID: "1", Context: "cloud context 1", SyncStatus: SyncStatusSynced, LastSyncAt: now},
	}

	err := manager.ProcessIncomingSync(ctx, newRecords)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	row := provider.QueryRow(ctx, "SELECT content FROM autodream_memories WHERE id = '1'")
	var content string
	if err := row.Scan(&content); err != nil {
		t.Fatalf("expected record to exist: %v", err)
	}

	if content != "cloud context 1" {
		t.Fatalf("expected 'cloud context 1', got %s", content)
	}
}

func TestSyncToCloud(t *testing.T) {
	ctx := context.Background()
	provider := setupTestDB(t)
	defer provider.Close()

	_, err := provider.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'context 1', 'pending')`)
	if err != nil {
		t.Fatalf("failed to insert records: %v", err)
	}

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	manager := NewRAGSyncManager(provider, ts.URL)
	err = manager.SyncToCloud(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	row := provider.QueryRow(ctx, "SELECT sync_status FROM autodream_memories WHERE id = '1'")
	var status string
	if err := row.Scan(&status); err != nil {
		t.Fatalf("expected record to exist: %v", err)
	}

	if status != "synced" {
		t.Fatalf("expected status 'synced', got %s", status)
	}
}

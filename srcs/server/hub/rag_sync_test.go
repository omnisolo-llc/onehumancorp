package hub

import (
	"context"
	"database/sql"
	"encoding/json"
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

	_, err = sqliteDB.Exec(`
		DROP TABLE IF EXISTS autodream_memories;
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return db.NewSqliteProvider(sqliteDB)
}

func TestFetchPendingSyncs(t *testing.T) {
	provider := setupTestDB(t)
	svc := NewDefaultRAGSyncService(provider)

	ctx := context.Background()
	tx, err := provider.Begin(ctx)
	if err != nil {
		t.Fatalf("failed to begin tx: %v", err)
	}
	vec, _ := json.Marshal([]float32{1.1, 2.2, 3.3})
	_, err = tx.Exec(ctx, `INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES ('1', 'test content 1', $1, 'pending')`, string(vec))
	if err != nil {
		t.Fatalf("insert failed: %v", err)
	}
	_, err = tx.Exec(ctx, `INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES ('2', 'test content 2', null, 'synced')`)
	if err != nil {
		t.Fatalf("insert failed: %v", err)
	}
	tx.Commit(ctx)

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	rec := records[0]
	if rec.ID != "1" || rec.Context != "test content 1" || rec.SyncStatus != SyncStatusPending {
		t.Errorf("unexpected record data: %+v", rec)
	}

	if len(rec.Vector) != 3 || rec.Vector[0] != 1.1 {
		t.Errorf("unexpected vector data: %v", rec.Vector)
	}
}

func TestMarkSynced(t *testing.T) {
	provider := setupTestDB(t)
	svc := NewDefaultRAGSyncService(provider)

	ctx := context.Background()
	tx, err := provider.Begin(ctx)
	if err != nil {
		t.Fatalf("failed to begin tx: %v", err)
	}
	_, err = tx.Exec(ctx, `INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test content 1', 'pending')`)
	if err != nil {
		t.Fatalf("insert failed: %v", err)
	}
	tx.Commit(ctx)

	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	tx, _ = provider.Begin(ctx)
	defer tx.Rollback(ctx)
	var syncStatus string
	var lastSyncAt *time.Time
	err = tx.QueryRow(ctx, "SELECT sync_status, last_sync_at FROM autodream_memories WHERE id = '1'").Scan(&syncStatus, &lastSyncAt)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}

	if syncStatus != "synced" {
		t.Errorf("expected synced status, got %s", syncStatus)
	}
	if lastSyncAt == nil {
		t.Errorf("expected last_sync_at to be set")
	}
}

func TestProcessIncomingSync(t *testing.T) {
	provider := setupTestDB(t)
	svc := NewDefaultRAGSyncService(provider)

	ctx := context.Background()
	records := []RAGSyncRecord{
		{
			ID:      "100",
			Context: "cloud context 100",
			Vector:  []float32{4.4, 5.5},
		},
	}

	err := svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	tx, _ := provider.Begin(ctx)
	defer tx.Rollback(ctx)

	var content, syncStatus string
	var vecStr *string
	err = tx.QueryRow(ctx, "SELECT content, embedding, sync_status FROM autodream_memories WHERE id = '100'").Scan(&content, &vecStr, &syncStatus)
	if err != nil {
		t.Fatalf("query failed: %v", err)
	}

	if content != "cloud context 100" {
		t.Errorf("unexpected content: %s", content)
	}
	if syncStatus != "synced" {
		t.Errorf("unexpected sync_status: %s", syncStatus)
	}
	if vecStr == nil {
		t.Errorf("expected embedding to be set")
	} else {
		var vec []float32
		json.Unmarshal([]byte(*vecStr), &vec)
		if len(vec) != 2 || vec[0] != 4.4 {
			t.Errorf("unexpected vector: %v", vec)
		}
	}
}

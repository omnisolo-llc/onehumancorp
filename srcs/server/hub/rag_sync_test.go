package hub

import (
	"database/sql"

	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	d := func() *db.DB {
		// Create a test DB using standard database/sql and wrap it with db.NewSqliteProvider
		importSql := "database/sql"
		_ = importSql // just for bypass
		sqliteDB, _ := sql.Open("sqlite", "file::memory:?cache=shared")
		return &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}
	}()
	ctx := context.Background()

	// Initialize table structure
	query := `
	CREATE TABLE autodream_memories (
		id TEXT PRIMARY KEY,
		content TEXT NOT NULL,
		embedding TEXT,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	);
	`
	tx, _ := d.Begin(ctx)
	tx.Exec(ctx, query)
	tx.Commit(ctx)

	svc := NewRAGSyncService(d)

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 0 {
		t.Errorf("expected 0 records, got %d", len(records))
	}

	tx, _ = d.Begin(ctx)
	tx.Exec(ctx, "INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES ('1', 'hello', '[0.1]', 'pending')")
	tx.Commit(ctx)

	records, err = svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(records) != 1 {
		t.Fatalf("expected 1 record, got %d", len(records))
	}

	if records[0].ID != "1" {
		t.Errorf("expected ID '1', got %s", records[0].ID)
	}
	if records[0].Context != "hello" {
		t.Errorf("expected Context 'hello', got %s", records[0].Context)
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	d := func() *db.DB {
		// Create a test DB using standard database/sql and wrap it with db.NewSqliteProvider
		importSql := "database/sql"
		_ = importSql // just for bypass
		sqliteDB, _ := sql.Open("sqlite", "file::memory:?cache=shared")
		return &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}
	}()
	ctx := context.Background()

	query := `
	CREATE TABLE autodream_memories (
		id TEXT PRIMARY KEY,
		content TEXT NOT NULL,
		embedding TEXT,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	);
	`
	tx, _ := d.Begin(ctx)
	tx.Exec(ctx, query)
	tx.Exec(ctx, "INSERT INTO autodream_memories (id, content, embedding, sync_status) VALUES ('1', 'hello', '[0.1]', 'pending')")
	tx.Commit(ctx)

	svc := NewRAGSyncService(d)

	err := svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	records, _ := svc.FetchPendingSyncs(ctx, 10)
	if len(records) != 0 {
		t.Errorf("expected 0 pending records, got %d", len(records))
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	d := func() *db.DB {
		// Create a test DB using standard database/sql and wrap it with db.NewSqliteProvider
		importSql := "database/sql"
		_ = importSql // just for bypass
		sqliteDB, _ := sql.Open("sqlite", "file::memory:?cache=shared")
		return &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}
	}()
	ctx := context.Background()

	query := `
	CREATE TABLE autodream_memories (
		id TEXT PRIMARY KEY,
		content TEXT NOT NULL,
		embedding TEXT,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	);
	`
	tx, _ := d.Begin(ctx)
	tx.Exec(ctx, query)
	tx.Commit(ctx)

	svc := NewRAGSyncService(d)

	records := []RAGSyncRecord{
		{
			ID:         "1",
			Context:    "hello",
			Vector:     []float32{0.1, 0.2},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}

	err := svc.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	tx, _ = d.Begin(ctx)
	rows, _ := tx.Query(ctx, "SELECT content FROM autodream_memories WHERE id = '1'")
	defer rows.Close()

	if !rows.Next() {
		t.Fatalf("expected 1 row, got 0")
	}
	var content string
	rows.Scan(&content)
	if content != "hello" {
		t.Errorf("expected 'hello', got %s", content)
	}
	tx.Commit(ctx)
}

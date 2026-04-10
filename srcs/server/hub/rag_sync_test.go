package hub

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService(t *testing.T) {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite in memory: %v", err)
	}
	defer sqliteDB.Close()

	_, err = sqliteDB.Exec(`
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at DATETIME NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	_, err = sqliteDB.Exec(`
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES
			('1', 'test content 1', 'pending'),
			('2', 'test content 2', 'synced'),
			('3', 'test content 3', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert initial data: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)
	service := NewRAGSyncService(provider)
	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending records, got %d", len(pending))
	}

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced error: %v", err)
	}

	pending2, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs error: %v", err)
	}
	if len(pending2) != 1 {
		t.Fatalf("expected 1 pending record after MarkSynced, got %d", len(pending2))
	}
	if pending2[0].ID != "3" {
		t.Fatalf("expected pending record to be ID '3', got %s", pending2[0].ID)
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "4", Context: "test content 4"},
		{ID: "3", Context: "updated test content 3"}, // Upsert
	}
	err = service.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync error: %v", err)
	}

	// Verify the final state
	rows, err := sqliteDB.Query(`SELECT id, content, sync_status FROM autodream_memories ORDER BY id`)
	if err != nil {
		t.Fatalf("failed to query final state: %v", err)
	}
	defer rows.Close()

	type Rec struct {
		id, content, status string
	}
	var finalRecs []Rec
	for rows.Next() {
		var r Rec
		if err := rows.Scan(&r.id, &r.content, &r.status); err != nil {
			t.Fatalf("failed to scan row: %v", err)
		}
		finalRecs = append(finalRecs, r)
	}

	if len(finalRecs) != 4 {
		t.Fatalf("expected 4 total records, got %d", len(finalRecs))
	}

	expectedRecs := map[string]Rec{
		"1": {"1", "test content 1", "synced"},
		"2": {"2", "test content 2", "synced"},
		"3": {"3", "updated test content 3", "synced"},
		"4": {"4", "test content 4", "synced"},
	}

	for _, rec := range finalRecs {
		exp, ok := expectedRecs[rec.id]
		if !ok {
			t.Fatalf("unexpected record found: %v", rec)
		}
		if exp.content != rec.content || exp.status != rec.status {
			t.Fatalf("record mismatch for ID %s: expected %v, got %v", rec.id, exp, rec)
		}
	}
}

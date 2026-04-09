package hub

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open in-memory db: %v", err)
	}

	_, err = db.Exec(`
		CREATE TABLE swarm_memory_embeddings (
			memory_id        TEXT PRIMARY KEY,
			context          TEXT NOT NULL,
			vector_embedding TEXT,
			source_plugin    TEXT,
			created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
			sync_status      VARCHAR(50) DEFAULT 'pending',
			last_sync_at     DATETIME NULL
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return db
}

func TestFetchPendingSyncs(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	svc := NewDefaultRAGSyncService(db)

	_, err := db.Exec(`
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status)
		VALUES
			('m1', 'ctx1', '[0.1, 0.2]', 'pending'),
			('m2', 'ctx2', '[0.3, 0.4]', 'synced'),
			('m3', 'ctx3', NULL, 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	records, err := svc.FetchPendingSyncs(context.Background(), 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(records))
	}

	for _, r := range records {
		if r.SyncStatus != SyncStatusPending {
			t.Errorf("expected pending status, got %s", r.SyncStatus)
		}
		if r.ID == "m1" && (len(r.Vector) != 2 || r.Vector[0] != 0.1) {
			t.Errorf("expected vector [0.1, 0.2] for m1, got %v", r.Vector)
		}
		if r.ID == "m3" && len(r.Vector) != 0 {
			t.Errorf("expected nil vector for m3, got %v", r.Vector)
		}
	}
}

func TestMarkSynced(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	svc := NewDefaultRAGSyncService(db)

	_, err := db.Exec(`
		INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status)
		VALUES ('m1', 'ctx1', 'pending'), ('m2', 'ctx2', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	err = svc.MarkSynced(context.Background(), []string{"m1", "m2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	var status1, status2 string
	err = db.QueryRow(`SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = 'm1'`).Scan(&status1)
	if err != nil {
		t.Fatal(err)
	}
	err = db.QueryRow(`SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = 'm2'`).Scan(&status2)
	if err != nil {
		t.Fatal(err)
	}

	if status1 != "synced" || status2 != "synced" {
		t.Errorf("expected status 'synced', got %s and %s", status1, status2)
	}
}

func TestProcessIncomingSync(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	svc := NewDefaultRAGSyncService(db)

	records := []RAGSyncRecord{
		{
			ID:         "m1",
			Context:    "new context",
			Vector:     []float32{0.5, 0.6},
			LastSyncAt: time.Now().UTC(),
		},
	}

	err := svc.ProcessIncomingSync(context.Background(), records)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	var ctxStr string
	var status string
	err = db.QueryRow(`SELECT context, sync_status FROM swarm_memory_embeddings WHERE memory_id = 'm1'`).Scan(&ctxStr, &status)
	if err != nil {
		t.Fatalf("failed to query after process: %v", err)
	}

	if ctxStr != "new context" {
		t.Errorf("expected context 'new context', got '%s'", ctxStr)
	}
	if status != "synced" {
		t.Errorf("expected status 'synced', got '%s'", status)
	}
}

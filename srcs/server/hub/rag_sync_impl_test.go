package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"testing"

	_ "modernc.org/sqlite"
)

func TestRAGSyncServiceImpl(t *testing.T) {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open db: %v", err)
	}
	defer db.Close()

	_, err = db.Exec(`
		CREATE TABLE swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMPTZ NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	v1, _ := json.Marshal([]float32{1.0, 2.0})
	v2, _ := json.Marshal([]float32{3.0, 4.0})

	_, err = db.Exec(`
		INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status)
		VALUES ('1', 'ctx1', ?, 'pending'), ('2', 'ctx2', ?, 'synced'), ('3', 'ctx3', ?, 'pending')
	`, string(v1), string(v2), string(v1))
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	svc := NewRAGSyncService(db)
	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("Expected 2 pending records, got %d", len(pending))
	}
	if pending[0].ID != "1" || pending[1].ID != "3" {
		t.Fatalf("Unexpected records returned")
	}
	if len(pending[0].Vector) != 2 || pending[0].Vector[0] != 1.0 {
		t.Fatalf("Vector not scanned correctly")
	}

	// Test MarkSynced
	err = svc.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	pendingAfter, _ := svc.FetchPendingSyncs(ctx, 10)
	if len(pendingAfter) != 1 || pendingAfter[0].ID != "3" {
		t.Fatalf("Expected 1 pending record after MarkSynced, got %d", len(pendingAfter))
	}

	// Test ProcessIncomingSync
	incoming := []RAGSyncRecord{
		{ID: "3", Context: "ctx3_updated", Vector: []float32{5.0, 6.0}, SyncStatus: SyncStatusSynced}, // Should update
		{ID: "4", Context: "ctx4", Vector: []float32{7.0, 8.0}, SyncStatus: SyncStatusSynced},         // Should insert
	}
	err = svc.ProcessIncomingSync(ctx, incoming)
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	var ctx3 string
	var vec3Bytes []byte
	err = db.QueryRow("SELECT context, vector_embedding FROM swarm_memory_embeddings WHERE memory_id = '3'").Scan(&ctx3, &vec3Bytes)
	if err != nil || ctx3 != "ctx3_updated" {
		t.Fatalf("Expected context 'ctx3_updated', got %v (err: %v)", ctx3, err)
	}

	var vec3 []float32
	json.Unmarshal(vec3Bytes, &vec3)
	if len(vec3) != 2 || vec3[0] != 5.0 {
		t.Fatalf("Expected vector [5.0, 6.0], got %v", vec3)
	}

	var ctx4 string
	err = db.QueryRow("SELECT context FROM swarm_memory_embeddings WHERE memory_id = '4'").Scan(&ctx4)
	if err != nil || ctx4 != "ctx4" {
		t.Fatalf("Expected context 'ctx4', got %v (err: %v)", ctx4, err)
	}
}

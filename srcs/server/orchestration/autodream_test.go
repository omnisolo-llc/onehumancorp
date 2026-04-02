package orchestration

import (
	"context"
	"testing"
	"time"
)

func TestAutoDreamWorker_PruneStaleSessions(t *testing.T) {
	ctx := context.Background()
	db, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	defer db.Close()

	worker := NewAutoDreamWorker(db)

	// Insert an old memory and a new memory
	oldTime := time.Now().Add(-60 * 24 * time.Hour).UTC().Format("2006-01-02 15:04:05")
	newTime := time.Now().UTC().Format("2006-01-02 15:04:05")

	_, err = db.db.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, source_plugin, created_at) VALUES ('old-1', 'old context', 'vec', 'plugin', ?)", oldTime)
	if err != nil {
		t.Fatalf("failed to insert old memory: %v", err)
	}

	_, err = db.db.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, source_plugin, created_at) VALUES ('new-1', 'new context', 'vec', 'plugin', ?)", newTime)
	if err != nil {
		t.Fatalf("failed to insert new memory: %v", err)
	}

	err = worker.PruneStaleSessions(ctx, 30*24*time.Hour)
	if err != nil {
		t.Fatalf("PruneStaleSessions failed: %v", err)
	}

	var count int
	err = db.db.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_memory_embeddings").Scan(&count)
	if err != nil {
		t.Fatalf("failed to count memories: %v", err)
	}

	if count != 1 {
		t.Fatalf("expected 1 memory after pruning, got %d", count)
	}
}

func TestAutoDreamWorker_ConsolidateMemories(t *testing.T) {
	ctx := context.Background()
	db, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	defer db.Close()

	worker := NewAutoDreamWorker(db)

	// Insert duplicate memories
	_, err = db.db.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, source_plugin, created_at) VALUES ('dup-1', 'same context', 'vec1', 'plugin', '2023-01-01 00:00:00')")
	if err != nil {
		t.Fatalf("failed to insert dup-1: %v", err)
	}

	_, err = db.db.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, source_plugin, created_at) VALUES ('dup-2', 'same context', 'vec2', 'plugin', '2023-01-01 00:00:00')")
	if err != nil {
		t.Fatalf("failed to insert dup-2: %v", err)
	}

	err = worker.ConsolidateMemories(ctx)
	if err != nil {
		t.Fatalf("ConsolidateMemories failed: %v", err)
	}

	var count int
	err = db.db.QueryRow(ctx, "SELECT COUNT(*) FROM swarm_memory_embeddings").Scan(&count)
	if err != nil {
		t.Fatalf("failed to count memories: %v", err)
	}

	if count != 1 {
		t.Fatalf("expected 1 memory after consolidation, got %d", count)
	}
}

func TestAutoDreamWorker_Start(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	db, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	defer db.Close()

	worker := NewAutoDreamWorker(db)

	// Run Start in background for a brief moment to test the loop
	go worker.Start(ctx, 10*time.Millisecond)

	time.Sleep(50 * time.Millisecond)
	cancel() // Stop the loop
}

package orchestration

import (
	"context"
	"os"
	"testing"
	"time"
)

func TestAutoDreamSystem(t *testing.T) {
	// Initialize an in-memory SIPDB directly
	sipdb, err := NewSIPDB("file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to create sipdb: %v", err)
	}
	defer sipdb.Close()

	autodream := NewAutoDreamSystem(sipdb)

	// Insert transient memory
	err = withRetry(context.Background(), func() error {
		_, err := sipdb.db.Exec(context.Background(), "INSERT INTO swarm_memory (key, value) VALUES ('test-key', 'conflict-trigger-value')")
		return err
	})
	if err != nil {
		t.Fatalf("failed to insert transient memory: %v", err)
	}

	// Insert existing episodic memory to trigger conflict
	memory := EpisodicMemory{
		MemoryID:        "existing-mem",
		Context:         "existing-conflict-trigger-context",
		VectorEmbedding: []byte("mock"),
		SourcePlugin:    "test",
	}
	err = sipdb.StoreEpisodicMemory(context.Background(), memory)
	if err != nil {
		t.Fatalf("failed to insert existing memory: %v", err)
	}

	// Run consolidation pipeline directly
	autodream.consolidateMemories(context.Background())

	// Verify transient memory was pruned
	var count int
	err = withRetry(context.Background(), func() error {
		return sipdb.db.QueryRow(context.Background(), "SELECT COUNT(*) FROM swarm_memory WHERE key = 'test-key'").Scan(&count)
	})
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}
	if count != 0 {
		t.Errorf("expected transient memory to be pruned, got %d", count)
	}

	// Verify consolidated memory was stored
	var contextValue string
	err = withRetry(context.Background(), func() error {
		return sipdb.db.QueryRow(context.Background(), "SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'consolidated-test-key'").Scan(&contextValue)
	})
	if err != nil {
		t.Fatalf("failed to query consolidated memory: %v", err)
	}
	if contextValue == "" {
		t.Errorf("expected consolidated memory to be stored")
	}

	// Start the AutoDream system briefly
	ctx, cancel := context.WithCancel(context.Background())
	autodream.Start(ctx)

	// Check if environment variables for standalone vs cloud parity are respected
	os.Setenv("OHC_STANDALONE", "true")
	autodream.pruneTransientContext(ctx)

	// Cancel background workers
	cancel()
	time.Sleep(10 * time.Millisecond) // Give workers time to exit
}

package orchestration

import (
	"context"
	"fmt"
	"net/http"
	"net/http/httptest"
	"path/filepath"
	"sync"
	"testing"
	"time"
)

// TestChaosHybridRAGSync verifies that the ContextSync daemon safely
// tolerates concurrent executions, lock contention, and deletes successfully.
func TestChaosHybridRAGSync(t *testing.T) {
	tempDir := t.TempDir()
	dbPath := filepath.Join(tempDir, "swarm.db")

	sip, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to init SIPDB: %v", err)
	}
	defer sip.Close()

	// 1. Start Mock Server to simulate Cloud Postgres webhook endpoint
	mockServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		time.Sleep(10 * time.Millisecond) // Simulate network lag
		w.WriteHeader(http.StatusOK)
	}))
	defer mockServer.Close()

	// 2. Seed memories
	ctx := context.Background()
	totalMemories := 150
	for i := 0; i < totalMemories; i++ {
		memory := EpisodicMemory{
			MemoryID:        fmt.Sprintf("mem-%d", i),
			Context:         fmt.Sprintf("chaos test context %d", i),
			VectorEmbedding: []byte("vector"),
			SourcePlugin:    "test",
		}
		if err := sip.StoreEpisodicMemory(ctx, memory); err != nil {
			t.Fatalf("Failed to insert: %v", err)
		}
	}

	// 3. Chaos Execution - Run sync concurrently to induce SQLite locks
	var wg sync.WaitGroup
	totalSyncs := 0
	var syncMu sync.Mutex

	start := time.Now()
	for i := 0; i < 5; i++ {
		wg.Add(1)
		go func(workerID int) {
			defer wg.Done()
			for {
				count, err := sip.SyncContextSync(ctx, mockServer.URL)
				if err != nil {
					t.Errorf("Worker %d failed to sync: %v", workerID, err)
					return
				}
				if count == 0 {
					return
				}
				syncMu.Lock()
				totalSyncs += count
				syncMu.Unlock()
			}
		}(i)
	}
	wg.Wait()
	duration := time.Since(start)

	// 4. Assert all memories are successfully synced and local state is clean
	if totalSyncs != totalMemories {
		t.Errorf("Expected %d synced records, got %d", totalMemories, totalSyncs)
	}

	memories, err := sip.GetEpisodicMemoriesByPlugin(ctx, "test")
	if err != nil {
		t.Fatalf("Failed to query remaining memories: %v", err)
	}
	if len(memories) != 0 {
		t.Errorf("Expected local RAG buffer to be empty, found %d records", len(memories))
	}

	t.Logf("Chaos test passed: %d records synced successfully in %s under contention", totalSyncs, duration)
}

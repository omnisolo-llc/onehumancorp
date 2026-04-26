package sync

import (
	"context"
	"fmt"
	"sync"
	"testing"
	"time"
)

// sqliteDeltaStore simulates a local SQLite database acting as a DeltaStore.
type sqliteDeltaStore struct {
	mu     sync.Mutex
	deltas map[string]SyncDelta
}

func newSQLiteDeltaStore() *sqliteDeltaStore {
	return &sqliteDeltaStore{
		deltas: make(map[string]SyncDelta),
	}
}

func (s *sqliteDeltaStore) InsertDelta(d SyncDelta) {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.deltas[d.ID] = d
}

func (s *sqliteDeltaStore) GetUnsyncedDeltas(ctx context.Context, limit int) ([]SyncDelta, error) {
	s.mu.Lock()
	defer s.mu.Unlock()

	var unsynced []SyncDelta
	for _, d := range s.deltas {
		unsynced = append(unsynced, d)
		if len(unsynced) >= limit {
			break
		}
	}
	return unsynced, nil
}

func (s *sqliteDeltaStore) MarkDeltasSynced(ctx context.Context, ids []string) error {
	s.mu.Lock()
	defer s.mu.Unlock()

	for _, id := range ids {
		delete(s.deltas, id)
	}
	return nil
}

// cloudPostgresGateway simulates a cloud environment receiving deltas.
type cloudPostgresGateway struct {
	mu       sync.Mutex
	received []SyncDelta
}

func newCloudPostgresGateway() *cloudPostgresGateway {
	return &cloudPostgresGateway{}
}

func (g *cloudPostgresGateway) PushDeltas(ctx context.Context, deltas []SyncDelta) error {
	g.mu.Lock()
	defer g.mu.Unlock()
	g.received = append(g.received, deltas...)
	return nil
}

func (g *cloudPostgresGateway) getReceivedCount() int {
	g.mu.Lock()
	defer g.mu.Unlock()
	return len(g.received)
}

// TestHybridStateSynchronizer_E2E verifies the full integration path of the daemon
// processing a large backlog of deltas, simulating the local-to-cloud sync mechanism.
func TestHybridStateSynchronizer_E2E(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	localStore := newSQLiteDeltaStore()
	cloudGateway := newCloudPostgresGateway()

	// Insert 120 deltas to test pagination (batches of 50)
	for i := 0; i < 120; i++ {
		localStore.InsertDelta(SyncDelta{
			ID:        fmt.Sprintf("e2e-delta-%d", i),
			EntityID:  "e2e-entity",
			Data:      `{"status":"offline_change"}`,
			UpdatedAt: time.Now(),
		})
	}

	// Create and start synchronizer with very short interval for test
	syncDaemon := NewHybridStateSynchronizer(localStore, cloudGateway, 5*time.Millisecond)
	syncDaemon.Start(ctx)

	// Wait for the daemon to exhaust the queue.
	// 120 items should be processed in 3 batches (50, 50, 20).
	// Because sync() exhausts the queue in a loop, it should do it in one tick.
	time.Sleep(50 * time.Millisecond)

	syncDaemon.Stop()

	// Verify all items were processed
	unsynced, _ := localStore.GetUnsyncedDeltas(ctx, 10)
	if len(unsynced) != 0 {
		t.Errorf("Expected 0 unsynced deltas, got %d", len(unsynced))
	}

	if cloudGateway.getReceivedCount() != 120 {
		t.Errorf("Expected 120 received deltas in cloud gateway, got %d", cloudGateway.getReceivedCount())
	}
}

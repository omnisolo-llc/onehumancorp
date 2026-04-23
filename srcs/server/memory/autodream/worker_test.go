package autodream

import (
	"context"
	"sync"
	"testing"
	"time"
)

type mockMemoryConsolidator struct {
	mu         sync.Mutex
	pruneCalls int
	prunedOrg  string
	prunedDur  time.Duration
	callChan   chan struct{}
}

func (m *mockMemoryConsolidator) Consolidate(ctx context.Context, taskID string, logs []string) error {
	return nil
}

func (m *mockMemoryConsolidator) ResolveConflicts(ctx context.Context, organizationID string, topic string) error {
	return nil
}

func (m *mockMemoryConsolidator) PruneStaleContext(ctx context.Context, organizationID string, threshold time.Duration) (int64, error) {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.pruneCalls++
	m.prunedOrg = organizationID
	m.prunedDur = threshold

	if m.callChan != nil {
		select {
		case m.callChan <- struct{}{}:
		default:
		}
	}
	return 1, nil
}

func (m *mockMemoryConsolidator) GetSharedContext(ctx context.Context, query string) (string, error) {
	return "", nil
}

func TestPruneWorker(t *testing.T) {
	callChan := make(chan struct{}, 1)
	mockService := &mockMemoryConsolidator{
		callChan: callChan,
	}
	orgID := "test-org"
	threshold := 24 * time.Hour
	interval := 10 * time.Millisecond

	worker := NewPruneWorker(mockService, orgID, threshold, interval)

	ctx := context.Background()
	worker.Start(ctx)

	// Wait for the worker to run at least once
	select {
	case <-callChan:
	case <-time.After(1 * time.Second):
		t.Fatalf("Worker did not call prune within expected time")
	}

	worker.Stop()

	// Wait a bit to ensure it actually stops and we can verify no more calls happen
	time.Sleep(30 * time.Millisecond)

	mockService.mu.Lock()
	callsAfterStop := mockService.pruneCalls
	mockService.mu.Unlock()

	time.Sleep(30 * time.Millisecond)

	mockService.mu.Lock()
	callsLater := mockService.pruneCalls

	if mockService.pruneCalls == 0 {
		t.Errorf("Expected prune to be called at least once, but it was not")
	}

	if mockService.prunedOrg != orgID {
		t.Errorf("Expected organizationID %s, got %s", orgID, mockService.prunedOrg)
	}

	if mockService.prunedDur != threshold {
		t.Errorf("Expected threshold %v, got %v", threshold, mockService.prunedDur)
	}
	mockService.mu.Unlock()

	if callsLater > callsAfterStop {
		t.Errorf("Worker did not stop, prune was called after Stop()")
	}
}

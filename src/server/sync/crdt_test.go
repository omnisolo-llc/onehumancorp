package sync

import (
	"context"
	"errors"
	"testing"
	"time"
)

type mockDeltaStore struct {
	deltas   []SyncDelta
	getErr   error
	markErr  error
	marked   []string
}

func (m *mockDeltaStore) GetUnsyncedDeltas(ctx context.Context, limit int) ([]SyncDelta, error) {
	if m.getErr != nil {
		return nil, m.getErr
	}

	// Filter out marked deltas
	var remaining []SyncDelta
	for _, d := range m.deltas {
		isMarked := false
		for _, markedID := range m.marked {
			if d.ID == markedID {
				isMarked = true
				break
			}
		}
		if !isMarked {
			remaining = append(remaining, d)
		}
	}

	if len(remaining) > limit {
		return remaining[:limit], nil
	}
	return remaining, nil
}

func (m *mockDeltaStore) MarkDeltasSynced(ctx context.Context, ids []string) error {
	if m.markErr != nil {
		return m.markErr
	}
	m.marked = append(m.marked, ids...)
	return nil
}

type mockCloudGateway struct {
	pushErr error
	pushed  []SyncDelta
}

func (m *mockCloudGateway) PushDeltas(ctx context.Context, deltas []SyncDelta) error {
	if m.pushErr != nil {
		return m.pushErr
	}
	m.pushed = append(m.pushed, deltas...)
	return nil
}

func TestHybridStateSynchronizer_sync(t *testing.T) {
	ctx := context.Background()

	t.Run("successful sync", func(t *testing.T) {
		store := &mockDeltaStore{
			deltas: []SyncDelta{
				{ID: "1", EntityID: "e1", Data: "{}", UpdatedAt: time.Now()},
				{ID: "2", EntityID: "e2", Data: "{}", UpdatedAt: time.Now()},
			},
		}
		gateway := &mockCloudGateway{}
		sync := NewHybridStateSynchronizer(store, gateway, time.Minute)

		sync.sync(ctx)

		if len(gateway.pushed) != 2 {
			t.Errorf("expected 2 pushed deltas, got %d", len(gateway.pushed))
		}
		if len(store.marked) != 2 {
			t.Errorf("expected 2 marked deltas, got %d", len(store.marked))
		}
	})

	t.Run("no deltas to sync", func(t *testing.T) {
		store := &mockDeltaStore{
			deltas: []SyncDelta{},
		}
		gateway := &mockCloudGateway{}
		sync := NewHybridStateSynchronizer(store, gateway, time.Minute)

		sync.sync(ctx)

		if len(gateway.pushed) != 0 {
			t.Errorf("expected 0 pushed deltas, got %d", len(gateway.pushed))
		}
		if len(store.marked) != 0 {
			t.Errorf("expected 0 marked deltas, got %d", len(store.marked))
		}
	})

	t.Run("store get error", func(t *testing.T) {
		store := &mockDeltaStore{
			getErr: errors.New("db error"),
		}
		gateway := &mockCloudGateway{}
		sync := NewHybridStateSynchronizer(store, gateway, time.Minute)

		sync.sync(ctx)

		if len(gateway.pushed) != 0 {
			t.Errorf("expected 0 pushed deltas, got %d", len(gateway.pushed))
		}
		if len(store.marked) != 0 {
			t.Errorf("expected 0 marked deltas, got %d", len(store.marked))
		}
	})

	t.Run("gateway push error", func(t *testing.T) {
		store := &mockDeltaStore{
			deltas: []SyncDelta{
				{ID: "1", EntityID: "e1", Data: "{}", UpdatedAt: time.Now()},
			},
		}
		gateway := &mockCloudGateway{
			pushErr: errors.New("network error"),
		}
		sync := NewHybridStateSynchronizer(store, gateway, time.Minute)

		sync.sync(ctx)

		if len(gateway.pushed) != 0 {
			t.Errorf("expected 0 pushed deltas, got %d", len(gateway.pushed))
		}
		if len(store.marked) != 0 {
			t.Errorf("expected 0 marked deltas, got %d", len(store.marked))
		}
	})

	t.Run("store mark error", func(t *testing.T) {
		store := &mockDeltaStore{
			deltas: []SyncDelta{
				{ID: "1", EntityID: "e1", Data: "{}", UpdatedAt: time.Now()},
			},
			markErr: errors.New("db error"),
		}
		gateway := &mockCloudGateway{}
		sync := NewHybridStateSynchronizer(store, gateway, time.Minute)

		sync.sync(ctx)

		if len(gateway.pushed) != 1 {
			t.Errorf("expected 1 pushed deltas, got %d", len(gateway.pushed))
		}
		if len(store.marked) != 0 {
			t.Errorf("expected 0 marked deltas, got %d", len(store.marked))
		}
	})
}

func TestHybridStateSynchronizer_StartStop(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	store := &mockDeltaStore{}
	gateway := &mockCloudGateway{}
	sync := NewHybridStateSynchronizer(store, gateway, 10*time.Millisecond)

	sync.Start(ctx)

	// Wait enough time for at least one tick
	time.Sleep(25 * time.Millisecond)

	sync.Stop()

	// Try stopping again to ensure it doesn't panic
	// sync.Stop() might panic if we double close the channel, but our implementation uses sync.Once
	sync.Stop()
}

func TestHybridStateSynchronizer_StartContextDone(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())

	store := &mockDeltaStore{}
	gateway := &mockCloudGateway{}
	sync := NewHybridStateSynchronizer(store, gateway, 10*time.Millisecond)

	sync.Start(ctx)

	// Cancel context to trigger the <-ctx.Done() case
	cancel()

	// Wait a moment for goroutine to exit
	time.Sleep(20 * time.Millisecond)
}

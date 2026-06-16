package telemetry

import (
	"context"
	"testing"
	"time"
)

type MockProvider struct {
    pending []string
    synced  []string
}

func (m *MockProvider) GetPendingMetrics() []string {
    return m.pending
}

func (m *MockProvider) MarkSynced(id string) error {
    m.synced = append(m.synced, id)
    return nil
}

func TestMcpSyncWorker(t *testing.T) {
    mockDB := &MockProvider{
        pending: []string{"metric1", "metric2"},
    }

	worker := &McpSyncWorker{
        DB: mockDB,
    }
	ctx, cancel := context.WithCancel(context.Background())
	worker.Start(ctx)

	time.Sleep(1 * time.Second) // allow a sync cycle
	cancel()
}

func TestMcpSyncWorker_SyncsMetrics(t *testing.T) {
    mockDB := &MockProvider{
        pending: []string{"metric1", "metric2"},
    }

	worker := &McpSyncWorker{
        DB: mockDB,
    }

    worker.syncMetrics()

    if len(mockDB.synced) != 2 {
        t.Errorf("Expected 2 metrics to be synced, got %d", len(mockDB.synced))
    }
}

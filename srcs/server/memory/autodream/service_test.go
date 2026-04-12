package autodream

import (
    "context"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/memory"
)

type mockRepo struct {
    upserted *memory.OHCMemoryEmbedding
}

func (m *mockRepo) UpsertEmbedding(ctx context.Context, mem *memory.OHCMemoryEmbedding) error {
    m.upserted = mem
    return nil
}

func (m *mockRepo) SemanticSearch(ctx context.Context, tenantID string, queryEmbedding []float32, limit int) ([]*memory.OHCMemoryEmbedding, error) {
    return nil, nil
}

type mockLLM struct{}

func (m *mockLLM) Summarize(ctx context.Context, text string) (string, error) {
    return "Summary", nil
}

type mockEmbedding struct{}

func (m *mockEmbedding) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
    return []float32{1.0, 2.0}, nil
}

func TestConsolidateTaskMemory(t *testing.T) {
    repo := &mockRepo{}
    svc := NewAutoDreamService(repo, &mockLLM{}, &mockEmbedding{})

    err := svc.ConsolidateTaskMemory(context.Background(), "tenant-1", "task-1", "raw logs")
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }

    if repo.upserted == nil {
        t.Fatal("expected an embedding to be upserted")
    }
    if repo.upserted.Content != "Summary" {
        t.Errorf("expected Content 'Summary', got '%s'", repo.upserted.Content)
    }
    if repo.upserted.SourceTaskID != "task-1" {
        t.Errorf("expected SourceTaskID 'task-1', got '%s'", repo.upserted.SourceTaskID)
    }
}

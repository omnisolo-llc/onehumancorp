package autodream

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/memory"
)

type MockRepo struct {
	UpsertCalled bool
	LastMem      memory.Memory
}

func (m *MockRepo) Upsert(ctx context.Context, mem memory.Memory) error {
	m.UpsertCalled = true
	m.LastMem = mem
	return nil
}

type MockMinimaxClient struct {
	GenerateEmbeddingFunc func(ctx context.Context, text string) ([]float32, error)
}

func (m *MockMinimaxClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	if m.GenerateEmbeddingFunc != nil {
		return m.GenerateEmbeddingFunc(ctx, text)
	}
	return make([]float32, 1536), nil
}

func TestAutoDreamService(t *testing.T) {
	repo := &MockRepo{}
	client := &MockMinimaxClient{}

	service := NewAutoDreamService(repo, client)

	err := service.Consolidate(context.Background(), "task-123", "tenant-1", "Test logs")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if !repo.UpsertCalled {
		t.Errorf("expected Upsert to be called")
	}

	if repo.LastMem.SourceTaskID != "task-123" {
		t.Errorf("expected SourceTaskID task-123, got %s", repo.LastMem.SourceTaskID)
	}
}

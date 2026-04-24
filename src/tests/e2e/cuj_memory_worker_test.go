package e2e

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/memory"
	"github.com/onehumancorp/mono/src/server/memory/autodream"
	"github.com/onehumancorp/mono/src/server/workers/memory"
)

type mockLLMForWorker struct{}

func (m *mockLLMForWorker) Reason(ctx context.Context, prompt string) (string, error) {
	return "Mock merged memory e2e", nil
}

func (m *mockLLMForWorker) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return []float32{0.5, 0.5, 0.5}, nil
}

type mockDBForWorker struct {
	db.Provider
}
func (m *mockDBForWorker) Query(ctx context.Context, sql string, args ...any) (db.Rows, error) {
	return &mockRowsForWorker{}, nil
}
func (m *mockDBForWorker) Exec(ctx context.Context, sql string, args ...any) (int64, error) { return 0, nil }
func (m *mockDBForWorker) IsSQLite() bool { return true }
type mockRowsForWorker struct { db.Rows }
func (m *mockRowsForWorker) Next() bool { return false }
func (m *mockRowsForWorker) Scan(dest ...any) error { return nil }
func (m *mockRowsForWorker) Close() {}
func (m *mockRowsForWorker) Err() error { return nil }


func TestCUJ_MemoryWorker_E2E(t *testing.T) {
	provider := &mockDBForWorker{}
	repo := memory.NewVectorRepository(provider)
	llm := &mockLLMForWorker{}
	service := autodream.NewService(repo, llm)
	worker := worker_memory.NewWorker(provider, service)
	ctx := context.Background()

	err := worker.ProcessOrganization(ctx, "tenant-1")
	if err != nil {
		t.Fatalf("expected no error from worker e2e, got %v", err)
	}
}

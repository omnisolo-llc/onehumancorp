package memory

import (
	"context"
	"testing"
	"fmt"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/memory"
	"github.com/onehumancorp/mono/src/server/memory/autodream"
)

type mockLLM struct{}

func (m *mockLLM) Reason(ctx context.Context, prompt string) (string, error) {
	return "Mock merged memory", nil
}

func (m *mockLLM) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return []float32{0.5, 0.5}, nil
}

type mockDBProvider struct {
	db.Provider
}
func (m *mockDBProvider) Query(ctx context.Context, sql string, args ...any) (db.Rows, error) {
	return &mockRows{}, nil
}
func (m *mockDBProvider) Exec(ctx context.Context, sql string, args ...any) (int64, error) { return 0, nil }
func (m *mockDBProvider) IsSQLite() bool { return true }
type mockRows struct { db.Rows }
func (m *mockRows) Next() bool { return false }
func (m *mockRows) Scan(dest ...any) error {
	// To simulate empty rows or valid rows
	return nil
}
func (m *mockRows) Close() {}
func (m *mockRows) Err() error { return nil }

type mockDBProviderFail struct {
	db.Provider
}
func (m *mockDBProviderFail) Query(ctx context.Context, sql string, args ...any) (db.Rows, error) {
	return nil, fmt.Errorf("simulated db error")
}
func (m *mockDBProviderFail) Exec(ctx context.Context, sql string, args ...any) (int64, error) { return 0, nil }
func (m *mockDBProviderFail) IsSQLite() bool { return true }


func TestWorker_ProcessOrganization(t *testing.T) {
	provider := &mockDBProvider{}
	repo := memory.NewVectorRepository(provider)
	llm := &mockLLM{}
	service := autodream.NewService(repo, llm)

	worker := NewWorker(provider, service)

	ctx := context.Background()

	err := worker.ProcessOrganization(ctx, "tenant-1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
}

func TestWorker_ProcessOrganization_Fail(t *testing.T) {
	provider := &mockDBProviderFail{}
	repo := memory.NewVectorRepository(provider)
	llm := &mockLLM{}
	service := autodream.NewService(repo, llm)
	worker := NewWorker(provider, service)
	ctx := context.Background()
	err := worker.ProcessOrganization(ctx, "tenant-1")
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

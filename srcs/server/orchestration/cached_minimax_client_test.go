package orchestration

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type MockMinimaxClient struct {
	callCount int
	response  []float32
}

func (m *MockMinimaxClient) Reason(ctx context.Context, prompt string) (string, error) {
	return "mock", nil
}

func (m *MockMinimaxClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	m.callCount++
	return m.response, nil
}

func TestCachedMinimaxClient_DBFallback(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://:memory:")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer pool.Close()

	if err := pool.RunMigrations(context.Background()); err != nil {
		t.Fatalf("failed migrations: %v", err)
	}

	baseClient := &MockMinimaxClient{response: []float32{1.0, 2.0, 3.0}}

	// Create cache wrapper without redis for simple test
	client := NewCachedMinimaxClient(baseClient, pool.Provider, nil)

	ctx := context.Background()

	// First call should hit the base client
	embedding1, err := client.GenerateEmbedding(ctx, "test text")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if baseClient.callCount != 1 {
		t.Errorf("expected 1 call to base client, got %d", baseClient.callCount)
	}

	// Second call should hit the cache
	embedding2, err := client.GenerateEmbedding(ctx, "test text")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if baseClient.callCount != 1 {
		t.Errorf("expected 1 call to base client, got %d", baseClient.callCount)
	}

	if len(embedding1) != len(embedding2) {
		t.Errorf("expected embeddings to match, got %v and %v", embedding1, embedding2)
	}
	for i := range embedding1 {
		if embedding1[i] != embedding2[i] {
			t.Errorf("expected embeddings to match, got %v and %v", embedding1, embedding2)
		}
	}
}

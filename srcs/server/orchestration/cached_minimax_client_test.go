package orchestration

import (
	"context"
	"errors"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

// mockMinimax is a mock client for testing.
type mockMinimax struct {
	calls int
	err   error
}

func (m *mockMinimax) Reason(ctx context.Context, prompt string) (string, error) {
	return "mock response", nil
}

func (m *mockMinimax) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	m.calls++
	if m.err != nil {
		return nil, m.err
	}
	return []float32{0.1, 0.2, 0.3}, nil
}

// setupDB creates an in-memory SQLite database and initializes the embedding_cache table.
func setupDB(t *testing.T) db.Provider {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	dbWrapper, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to create db provider: %v", err)
	}
	prov := dbWrapper.Provider

	_, err = prov.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS embedding_cache (
			content_hash TEXT PRIMARY KEY,
			embedding TEXT NOT NULL,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create embedding_cache table: %v", err)
	}
	return prov
}

func TestCachedMinimaxClient_GenerateEmbedding(t *testing.T) {
	ctx := context.Background()
	prov := setupDB(t)
	defer prov.Close()

	mockClient := &mockMinimax{}

	// Create cached client with DB but no Redis
	var redisClient rueidis.Client
	cachedClient := NewCachedMinimaxClient(mockClient, prov, redisClient)

	text := "hello world"

	// 1. First call should hit the mock client and cache the result
	emb1, err := cachedClient.GenerateEmbedding(ctx, text)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(emb1) != 3 {
		t.Fatalf("expected 3 floats, got %d", len(emb1))
	}
	if mockClient.calls != 1 {
		t.Fatalf("expected 1 call to mock client, got %d", mockClient.calls)
	}

	// 2. Second call with same text should hit the DB cache, not the mock client
	emb2, err := cachedClient.GenerateEmbedding(ctx, text)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(emb2) != 3 {
		t.Fatalf("expected 3 floats, got %d", len(emb2))
	}
	if mockClient.calls != 1 {
		t.Fatalf("expected 1 call to mock client, got %d", mockClient.calls)
	}

	// 3. Call with different text should hit the mock client
	_, err = cachedClient.GenerateEmbedding(ctx, "hello universe")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if mockClient.calls != 2 {
		t.Fatalf("expected 2 calls to mock client, got %d", mockClient.calls)
	}

	// 4. Test error propagation
	mockClient.err = errors.New("API error")
	_, err = cachedClient.GenerateEmbedding(ctx, "error text")
	if err == nil || err.Error() != "API error" {
		t.Fatalf("expected 'API error', got %v", err)
	}
	if mockClient.calls != 3 {
		t.Fatalf("expected 3 calls to mock client, got %d", mockClient.calls)
	}
}

package orchestration

import (
	"context"
	"errors"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

func TestDataCompression(t *testing.T) {
	originalData := []byte("This is a test string to be compressed and decompressed.")
	compressed, err := compressData(originalData)
	if err != nil {
		t.Fatalf("compression failed: %v", err)
	}
	if len(compressed) == 0 {
		t.Fatalf("compressed data is empty")
	}

	decompressed, err := decompressData(compressed)
	if err != nil {
		t.Fatalf("decompression failed: %v", err)
	}

	if string(decompressed) != string(originalData) {
		t.Fatalf("expected '%s', got '%s'", originalData, decompressed)
	}

	// Test backward compatibility (uncompressed data)
	uncompressedData := []byte("Uncompressed data should pass through.")
	decompressedUncompressed, err := decompressData(uncompressedData)
	if err != nil {
		t.Fatalf("decompression of uncompressed data failed: %v", err)
	}
	if string(decompressedUncompressed) != string(uncompressedData) {
		t.Fatalf("expected '%s', got '%s'", uncompressedData, decompressedUncompressed)
	}
}

// mockMinimax is a mock client for testing.
type mockMinimax struct {
	calls       int
	reasonCalls int
	err         error
}

func (m *mockMinimax) Reason(ctx context.Context, prompt string) (string, error) {
	m.reasonCalls++
	if m.err != nil {
		return "", m.err
	}
	return "mock response for " + prompt, nil
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
	prov, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to create db provider: %v", err)
	}

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

	_, err = prov.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS llm_reason_cache (
			prompt_hash TEXT PRIMARY KEY,
			response TEXT NOT NULL,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create llm_reason_cache table: %v", err)
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

func TestCachedMinimaxClient_EvictOldCaches(t *testing.T) {
	ctx := context.Background()
	prov := setupDB(t)
	defer prov.Close()

	// Insert old and new records
	oldTime := time.Now().Add(-40 * 24 * time.Hour)
	newTime := time.Now().Add(-10 * 24 * time.Hour)

	_, err := prov.Exec(ctx, "INSERT INTO embedding_cache (content_hash, embedding, created_at) VALUES ('hash1', 'embed1', ?)", oldTime)
	if err != nil {
		t.Fatalf("failed to insert old embedding: %v", err)
	}
	_, err = prov.Exec(ctx, "INSERT INTO embedding_cache (content_hash, embedding, created_at) VALUES ('hash2', 'embed2', ?)", newTime)
	if err != nil {
		t.Fatalf("failed to insert new embedding: %v", err)
	}

	_, err = prov.Exec(ctx, "INSERT INTO llm_reason_cache (prompt_hash, response, created_at) VALUES ('hash1', 'resp1', ?)", oldTime)
	if err != nil {
		t.Fatalf("failed to insert old reason: %v", err)
	}
	_, err = prov.Exec(ctx, "INSERT INTO llm_reason_cache (prompt_hash, response, created_at) VALUES ('hash2', 'resp2', ?)", newTime)
	if err != nil {
		t.Fatalf("failed to insert new reason: %v", err)
	}

	client := NewCachedMinimaxClient(&mockMinimax{}, prov, nil).(*CachedMinimaxClient)

	// Evict caches older than 30 days
	err = client.EvictOldCaches(ctx, 30*24*time.Hour)
	if err != nil {
		t.Fatalf("EvictOldCaches failed: %v", err)
	}

	// Verify embedding_cache
	var count int
	err = prov.QueryRow(ctx, "SELECT COUNT(*) FROM embedding_cache").Scan(&count)
	if err != nil || count != 1 {
		t.Fatalf("expected 1 embedding, got %d, err: %v", count, err)
	}

	// Verify llm_reason_cache
	err = prov.QueryRow(ctx, "SELECT COUNT(*) FROM llm_reason_cache").Scan(&count)
	if err != nil || count != 1 {
		t.Fatalf("expected 1 reason, got %d, err: %v", count, err)
	}
}

func TestCachedMinimaxClient_Reason(t *testing.T) {
	ctx := context.Background()
	prov := setupDB(t)
	defer prov.Close()

	mockClient := &mockMinimax{}

	// Create cached client with DB but no Redis
	var redisClient rueidis.Client
	cachedClient := NewCachedMinimaxClient(mockClient, prov, redisClient)

	prompt := "what is the meaning of life?"

	// 1. First call should hit the mock client and cache the result
	resp1, err := cachedClient.Reason(ctx, prompt)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if resp1 != "mock response for what is the meaning of life?" {
		t.Fatalf("unexpected response: %s", resp1)
	}
	if mockClient.reasonCalls != 1 {
		t.Fatalf("expected 1 call to mock client, got %d", mockClient.reasonCalls)
	}

	// 2. Second call with same prompt should hit the DB cache, not the mock client
	resp2, err := cachedClient.Reason(ctx, prompt)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if resp2 != resp1 {
		t.Fatalf("expected response %s, got %s", resp1, resp2)
	}
	if mockClient.reasonCalls != 1 {
		t.Fatalf("expected 1 call to mock client, got %d", mockClient.reasonCalls)
	}

	// 3. Call with different prompt should hit the mock client
	_, err = cachedClient.Reason(ctx, "what is 2+2?")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if mockClient.reasonCalls != 2 {
		t.Fatalf("expected 2 calls to mock client, got %d", mockClient.reasonCalls)
	}

	// 4. Test error propagation
	mockClient.err = errors.New("API error")
	_, err = cachedClient.Reason(ctx, "error prompt")
	if err == nil || err.Error() != "API error" {
		t.Fatalf("expected 'API error', got %v", err)
	}
	if mockClient.reasonCalls != 3 {
		t.Fatalf("expected 3 calls to mock client, got %d", mockClient.reasonCalls)
	}
}

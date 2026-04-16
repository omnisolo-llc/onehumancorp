package local_test

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/agents/local"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockLLMClient struct {
	callCount int
	resp      *local.AssistantMessage
	err       error
}

func (m *mockLLMClient) Complete(ctx context.Context, req local.CompletionRequest) (*local.AssistantMessage, error) {
	m.callCount++
	return m.resp, m.err
}

func TestCachedLLMClient_DB(t *testing.T) {
	// Setup in-memory SQLite
	sqlDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Failed to open sqlite memory db: %v", err)
	}
	defer sqlDB.Close()

	provider := db.NewSqliteProvider(sqlDB)

	// Create table
	_, err = provider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS llm_completion_cache (
			request_hash VARCHAR(64) PRIMARY KEY,
			response_payload BLOB NOT NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	mockClient := &mockLLMClient{
		resp: &local.AssistantMessage{
			Text: "Hello cached world",
		},
	}

	cachedClient := local.NewCachedLLMClient(mockClient, provider, nil)

	req := local.CompletionRequest{
		SystemPrompt: "Test system prompt",
		MaxTokens:    100,
	}

	// First call - should miss cache
	resp1, err := cachedClient.Complete(context.Background(), req)
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}
	if mockClient.callCount != 1 {
		t.Errorf("Expected 1 call to underlying client, got %d", mockClient.callCount)
	}
	if resp1.Text != "Hello cached world" {
		t.Errorf("Expected 'Hello cached world', got %q", resp1.Text)
	}

	// Second call - should hit DB cache
	resp2, err := cachedClient.Complete(context.Background(), req)
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}
	if mockClient.callCount != 1 {
		t.Errorf("Expected still 1 call to underlying client (cached), got %d", mockClient.callCount)
	}
	if resp2.Text != "Hello cached world" {
		t.Errorf("Expected 'Hello cached world', got %q", resp2.Text)
	}
}

// Removing Redis mock test as org_uber_go_mock isn't available in main repository

func TestCachedLLMClient_PruneCache(t *testing.T) {
	// Setup in-memory SQLite
	sqlDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqlDB.Close()

	provider := db.NewSqliteProvider(sqlDB)

	// Create table
	_, err = provider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS llm_completion_cache (
			request_hash VARCHAR(64) PRIMARY KEY,
			response_payload BLOB NOT NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	// Insert an old record (older than 1 day)
	_, err = provider.Exec(context.Background(), `
		INSERT INTO llm_completion_cache (request_hash, response_payload, created_at)
		VALUES ('old_hash', 'dummy_payload', datetime('now', '-2 day'))
	`)
	if err != nil {
		t.Fatalf("failed to insert old record: %v", err)
	}

	// Insert a new record
	_, err = provider.Exec(context.Background(), `
		INSERT INTO llm_completion_cache (request_hash, response_payload, created_at)
		VALUES ('new_hash', 'dummy_payload', datetime('now'))
	`)
	if err != nil {
		t.Fatalf("failed to insert new record: %v", err)
	}

	mockClient := &mockLLMClient{}
	cachedClient := local.NewCachedLLMClient(mockClient, provider, nil)

	client, ok := cachedClient.(*local.CachedLLMClient)
	if !ok {
		t.Fatalf("expected *local.CachedLLMClient")
	}

	// Run PruneCache
	client.PruneCache(context.Background())

	// Verify old record is deleted and new record remains
	var count int
	err = provider.QueryRow(context.Background(), "SELECT COUNT(*) FROM llm_completion_cache").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query count: %v", err)
	}

	if count != 1 {
		t.Fatalf("expected 1 record after pruning, got %d", count)
	}

	var remainingHash string
	err = provider.QueryRow(context.Background(), "SELECT request_hash FROM llm_completion_cache").Scan(&remainingHash)
	if err != nil {
		t.Fatalf("failed to query remaining record: %v", err)
	}

	if remainingHash != "new_hash" {
		t.Fatalf("expected remaining record to be 'new_hash', got '%s'", remainingHash)
	}
}

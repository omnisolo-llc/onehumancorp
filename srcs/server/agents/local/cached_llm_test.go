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

func TestCachedLLMClient_InMemoryFallback(t *testing.T) {
	mockClient := &mockLLMClient{
		resp: &local.AssistantMessage{
			Text: "Hello in-memory cached world",
		},
	}

	// db and redis are nil
	cachedClient := local.NewCachedLLMClient(mockClient, nil, nil)

	req := local.CompletionRequest{
		SystemPrompt: "Test memory system prompt",
		MaxTokens:    200,
	}

	// First call - should miss cache
	resp1, err := cachedClient.Complete(context.Background(), req)
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}
	if mockClient.callCount != 1 {
		t.Errorf("Expected 1 call to underlying client, got %d", mockClient.callCount)
	}
	if resp1.Text != "Hello in-memory cached world" {
		t.Errorf("Expected 'Hello in-memory cached world', got %q", resp1.Text)
	}

	// Second call - should hit in-memory cache
	resp2, err := cachedClient.Complete(context.Background(), req)
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}
	if mockClient.callCount != 1 {
		t.Errorf("Expected still 1 call to underlying client (cached), got %d", mockClient.callCount)
	}
	if resp2.Text != "Hello in-memory cached world" {
		t.Errorf("Expected 'Hello in-memory cached world', got %q", resp2.Text)
	}
}

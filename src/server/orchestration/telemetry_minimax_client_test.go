package orchestration

import (
	"context"
	"errors"
	"testing"
)

func TestTelemetryMinimaxClient_Reason(t *testing.T) {
	ctx := context.Background()
	mockClient := &mockMinimax{}
	telemetryClient := NewTelemetryMinimaxClient(mockClient)

	// Test success path
	resp, err := telemetryClient.Reason(ctx, "test prompt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if resp != "mock response for test prompt" {
		t.Fatalf("unexpected response: %s", resp)
	}
	if mockClient.reasonCalls != 1 {
		t.Fatalf("expected 1 call, got %d", mockClient.reasonCalls)
	}

	// Test error path
	mockClient.err = errors.New("mock error")
	_, err = telemetryClient.Reason(ctx, "test error prompt")
	if err == nil || err.Error() != "mock error" {
		t.Fatalf("expected 'mock error', got %v", err)
	}
	if mockClient.reasonCalls != 2 {
		t.Fatalf("expected 2 calls, got %d", mockClient.reasonCalls)
	}
}

func TestTelemetryMinimaxClient_GenerateEmbedding(t *testing.T) {
	ctx := context.Background()
	mockClient := &mockMinimax{}
	telemetryClient := NewTelemetryMinimaxClient(mockClient)

	// Test success path
	emb, err := telemetryClient.GenerateEmbedding(ctx, "test text")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(emb) != 3 {
		t.Fatalf("expected 3 floats, got %d", len(emb))
	}
	if mockClient.calls != 1 {
		t.Fatalf("expected 1 call, got %d", mockClient.calls)
	}

	// Test error path
	mockClient.err = errors.New("mock error")
	_, err = telemetryClient.GenerateEmbedding(ctx, "test error text")
	if err == nil || err.Error() != "mock error" {
		t.Fatalf("expected 'mock error', got %v", err)
	}
	if mockClient.calls != 2 {
		t.Fatalf("expected 2 calls, got %d", mockClient.calls)
	}
}

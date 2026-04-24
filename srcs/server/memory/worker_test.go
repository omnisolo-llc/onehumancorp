package memory

import (
	"context"
	"testing"
	"time"
)

type MockLLMClient struct{}

func (m *MockLLMClient) Reason(ctx context.Context, prompt string) (string, error) {
	return "YES", nil
}

func TestWorker(t *testing.T) {
	mockProvider := &MockPgProvider{}
	realRepo := NewVectorRepository(mockProvider)
	mockLLM := &MockLLMClient{}

	// Fast interval for testing
	worker := NewWorker(realRepo, mockLLM, 10*time.Millisecond)

	ctx, cancel := context.WithCancel(context.Background())
	worker.Start(ctx)

	time.Sleep(50 * time.Millisecond)
	cancel() // Stop via context cancellation

	// Start again and stop via Stop() method
	ctx2 := context.Background()
	worker2 := NewWorker(realRepo, mockLLM, 10*time.Millisecond)
	worker2.Start(ctx2)
	time.Sleep(50 * time.Millisecond)
	worker2.Stop()

	// Also test manual Run
	worker2.Run(ctx2)

	// Test default interval
	worker3 := NewWorker(realRepo, mockLLM, 0)
	if worker3.Interval != 24*time.Hour {
		t.Fatalf("expected 24h, got %v", worker3.Interval)
	}
}

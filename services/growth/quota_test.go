package growth

import (
	"context"
	"sync"
	"testing"
	"time"

	"github.com/onehumancorp/mono/lib/analytics"
)

type MockQuotaProvider struct {
	mu     sync.Mutex
	counts map[string]int
}

func NewMockQuotaProvider() *MockQuotaProvider {
	return &MockQuotaProvider{
		counts: make(map[string]int),
	}
}

func (m *MockQuotaProvider) IncrementAndGet(ctx context.Context, userID string, window time.Time) (int, error) {
	m.mu.Lock()
	defer m.mu.Unlock()
	key := userID + "_" + window.Format(time.RFC3339)
	m.counts[key]++
	return m.counts[key], nil
}

func TestQuotaService(t *testing.T) {
	tracker := analytics.NewTracker()
	provider := NewMockQuotaProvider()
	service := NewQuotaService(tracker, 2, provider)
	ctx := context.Background()

	err := service.CheckAndIncrement(ctx, "user1")
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	err = service.CheckAndIncrement(ctx, "user1")
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	err = service.CheckAndIncrement(ctx, "user1")
	if err == nil || err.Error() != "quota exceeded" {
		t.Errorf("expected quota exceeded error, got %v", err)
	}

	err = service.CheckAndIncrement(ctx, "")
	if err == nil || err.Error() != "invalid user ID" {
		t.Errorf("expected invalid user ID error, got %v", err)
	}
}

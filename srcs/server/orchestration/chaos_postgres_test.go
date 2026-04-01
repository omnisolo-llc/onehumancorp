package orchestration

import (
	"context"
	"fmt"
	"sync"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// mockPgProvider implements db.Provider for testing pgWithRetry
type mockPgProvider struct {
	attempts int
	mu       sync.Mutex
}

func (m *mockPgProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	return nil, nil
}

func (m *mockPgProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
	return nil
}

func (m *mockPgProvider) Begin(ctx context.Context) (db.Tx, error) {
	return nil, nil
}

func (m *mockPgProvider) Close() {}

func (m *mockPgProvider) IsSQLite() bool {
	return false
}

func (m *mockPgProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.attempts++
	if m.attempts < 3 {
		return 0, fmt.Errorf("database is locked") // Simulate Postgres deadlock by using an error pgWithRetry catches
	}
	return 1, nil
}

func TestPgWithRetry_Chaos(t *testing.T) {
	mockPool := &mockPgProvider{}
	repo := NewPgHubRepository(mockPool)

	ctx := context.Background()
	start := time.Now()

	err := repo.UpdateAgentStatus(ctx, "agent-1", StatusIdle)
	if err != nil {
		t.Fatalf("Expected successful update after retries, got %v", err)
	}

	if mockPool.attempts != 3 {
		t.Errorf("Expected 3 attempts, got %d", mockPool.attempts)
	}

	t.Logf("Successfully recovered from simulated Postgres serialization failure in %v", time.Since(start))
}

package orchestration

import (
	"context"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/stretchr/testify/assert"
)

type MockDBProvider struct {
	db.Provider
	lastQuery string
	isSQLite  bool
	tx        *MockTx
}

func (m *MockDBProvider) IsSQLite() bool {
	return m.isSQLite
}

func (m *MockDBProvider) Begin(ctx context.Context) (db.Tx, error) {
	return m.tx, nil
}

type MockTx struct {
	db.Tx
	lastQuery string
	queryRowFunc func(query string)
}

func (m *MockTx) QueryRow(ctx context.Context, query string, args ...interface{}) db.Row {
	m.lastQuery = query
	if m.queryRowFunc != nil {
		m.queryRowFunc(query)
	}
	return &MockRow{}
}

func (m *MockTx) Exec(ctx context.Context, query string, args ...interface{}) (db.Result, error) {
	return &MockResult{}, nil
}

func (m *MockTx) Rollback(ctx context.Context) error { return nil }
func (m *MockTx) Commit(ctx context.Context) error { return nil }

type MockRow struct{}
func (m *MockRow) Scan(dest ...interface{}) error { return nil }

type MockResult struct{}
func (m *MockResult) LastInsertId() (int64, error) { return 1, nil }
func (m *MockResult) RowsAffected() (int64, error) { return 1, nil }


func TestDynamicTaskRouter_ClaimTask_PostgresLock(t *testing.T) {
	ctx := context.Background()

	mockTx := &MockTx{
		queryRowFunc: func(query string) {
			if !strings.Contains(query, "FOR UPDATE SKIP LOCKED") {
				t.Errorf("Expected FOR UPDATE SKIP LOCKED in query, got: %s", query)
			}
		},
	}

	provider := &MockDBProvider{
		isSQLite: false,
		tx:       mockTx,
	}

	router := NewDynamicTaskRouter(provider, nil)

	success, err := router.ClaimTask(ctx, "task-123", "agent-456")
	assert.NoError(t, err)
	assert.True(t, success)
}

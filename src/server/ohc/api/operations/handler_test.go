package operations

import (
	"context"
	"testing"

	domain "mono/src/server/ohc/domain/operations"
)

type MockResult struct {
    rows int64
}

func (m *MockResult) RowsAffected() (int64, error) {
    return m.rows, nil
}

type MockTx struct {
    ExecContextCount int
}

func (m *MockTx) ExecContext(ctx context.Context, query string, args ...interface{}) (domain.Result, error) {
    m.ExecContextCount++
    return &MockResult{rows: 1}, nil
}
func (m *MockTx) Commit() error { return nil }
func (m *MockTx) Rollback() error { return nil }


type MockDB struct {
    ExecContextCount int
    LastQuery string
    LastArgs []interface{}
    ResultRows int64
}

func (m *MockDB) ExecContext(ctx context.Context, query string, args ...interface{}) (domain.Result, error) {
    m.ExecContextCount++
    m.LastQuery = query
    m.LastArgs = args
    return &MockResult{rows: m.ResultRows}, nil
}

func (m *MockDB) BeginTx(ctx context.Context, opts interface{}) (domain.Tx, error) {
    return &MockTx{}, nil
}

func TestApproveActionCard(t *testing.T) {
    db := &MockDB{ResultRows: 1}
	manager := domain.NewOperationsManager(db)
	handler := NewHandler(manager)

	err := handler.ApproveActionCard(context.Background(), "tenant-1", "BOOKING_REQUEST", map[string]interface{}{
        "date": "Friday",
    })
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
}

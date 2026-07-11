package operations

import (
	"context"
	"testing"
)

type MockResult struct {
    rows int64
}

func (m *MockResult) RowsAffected() (int64, error) {
    return m.rows, nil
}

type MockTx struct {
    ExecContextCount int
    LastQuery string
    LastArgs []interface{}
    ResultRows int64
    Committed bool
    Rollbacked bool
}

func (m *MockTx) ExecContext(ctx context.Context, query string, args ...interface{}) (Result, error) {
    m.ExecContextCount++
    m.LastQuery = query
    m.LastArgs = args
    return &MockResult{rows: m.ResultRows}, nil
}

func (m *MockTx) Commit() error {
    m.Committed = true
    return nil
}

func (m *MockTx) Rollback() error {
    m.Rollbacked = true
    return nil
}

type MockDB struct {
    ExecContextCount int
    LastQuery string
    LastArgs []interface{}
    ResultRows int64
    LastTx *MockTx
}

func (m *MockDB) ExecContext(ctx context.Context, query string, args ...interface{}) (Result, error) {
    m.ExecContextCount++
    m.LastQuery = query
    m.LastArgs = args
    return &MockResult{rows: m.ResultRows}, nil
}

func (m *MockDB) BeginTx(ctx context.Context, opts interface{}) (Tx, error) {
    m.LastTx = &MockTx{ResultRows: m.ResultRows}
    return m.LastTx, nil
}

func TestExecuteAction_BookingRequest(t *testing.T) {
    db := &MockDB{ResultRows: 1}
	om := NewOperationsManager(db)
	intent := ActionIntent{
		TenantID:   "tenant-maya-123",
		ActionType: "BOOKING_REQUEST",
		Payload:    map[string]interface{}{"date": "Friday"},
	}

	err := om.ExecuteAction(context.Background(), intent)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

    if db.LastTx.ExecContextCount != 2 {
        t.Fatalf("expected 2 DB executions on tx, got %d", db.LastTx.ExecContextCount)
    }
    if !db.LastTx.Committed {
        t.Fatalf("expected transaction to be committed")
    }
}

func TestExecuteAction_InventoryDeduction(t *testing.T) {
    db := &MockDB{ResultRows: 1}
	om := NewOperationsManager(db)
	intent := ActionIntent{
		TenantID:   "tenant-maya-123",
		ActionType: "INVENTORY_DEDUCTION",
		Payload:    map[string]interface{}{"item_id": "cake-1", "quantity": 2.0},
	}

	err := om.ExecuteAction(context.Background(), intent)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

    if db.LastTx.ExecContextCount != 2 {
        t.Fatalf("expected 2 DB executions on tx, got %d", db.LastTx.ExecContextCount)
    }
    if !db.LastTx.Committed {
        t.Fatalf("expected transaction to be committed")
    }
}

func TestExecuteAction_UnsupportedType(t *testing.T) {
    db := &MockDB{}
	om := NewOperationsManager(db)
	intent := ActionIntent{
		TenantID:   "tenant-maya-123",
		ActionType: "UNKNOWN",
		Payload:    map[string]interface{}{},
	}

	err := om.ExecuteAction(context.Background(), intent)
	if err == nil {
		t.Fatalf("expected error for unsupported type")
	}
}

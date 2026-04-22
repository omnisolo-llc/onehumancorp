package tasks

import (
	"context"
	"database/sql"
	"errors"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type MockProvider struct {
	db.Provider
	IsSQLiteFunc func() bool
	ExecFunc     func(ctx context.Context, sql string, args ...any) (int64, error)
	QueryRowFunc func(ctx context.Context, sql string, args ...any) db.Row
	BeginFunc    func(ctx context.Context) (db.Tx, error)
}

func (m *MockProvider) IsSQLite() bool {
	if m.IsSQLiteFunc != nil {
		return m.IsSQLiteFunc()
	}
	return false
}

func (m *MockProvider) Exec(ctx context.Context, sql string, args ...any) (int64, error) {
	if m.ExecFunc != nil {
		return m.ExecFunc(ctx, sql, args...)
	}
	return 1, nil
}

func (m *MockProvider) QueryRow(ctx context.Context, sql string, args ...any) db.Row {
	if m.QueryRowFunc != nil {
		return m.QueryRowFunc(ctx, sql, args...)
	}
	return &MockRow{}
}

func (m *MockProvider) Begin(ctx context.Context) (db.Tx, error) {
	if m.BeginFunc != nil {
		return m.BeginFunc(ctx)
	}
	return &MockTx{}, nil
}

type MockRow struct {
	ScanFunc func(dest ...any) error
}

func (m *MockRow) Scan(dest ...any) error {
	if m.ScanFunc != nil {
		return m.ScanFunc(dest...)
	}
	return nil
}

type MockTx struct {
	db.Tx
	ExecFunc     func(ctx context.Context, sql string, args ...any) (int64, error)
	QueryRowFunc func(ctx context.Context, sql string, args ...any) db.Row
	CommitFunc   func(ctx context.Context) error
	RollbackFunc func(ctx context.Context) error
}

func (m *MockTx) Exec(ctx context.Context, sql string, args ...any) (int64, error) {
	if m.ExecFunc != nil {
		return m.ExecFunc(ctx, sql, args...)
	}
	return 1, nil
}

func (m *MockTx) QueryRow(ctx context.Context, sql string, args ...any) db.Row {
	if m.QueryRowFunc != nil {
		return m.QueryRowFunc(ctx, sql, args...)
	}
	return &MockRow{}
}

func (m *MockTx) Commit(ctx context.Context) error {
	if m.CommitFunc != nil {
		return m.CommitFunc(ctx)
	}
	return nil
}

func (m *MockTx) Rollback(ctx context.Context) error {
	if m.RollbackFunc != nil {
		return m.RollbackFunc(ctx)
	}
	return nil
}

func TestCreate_WithID(t *testing.T) {
	mockProvider := &MockProvider{
		ExecFunc: func(ctx context.Context, sqlQuery string, args ...any) (int64, error) {
			if args[0] != "test-id" {
				t.Errorf("expected ID 'test-id', got %v", args[0])
			}
			if args[3] != "[]" {
				t.Errorf("expected dependencies '[]', got %v", args[3])
			}
			if args[5] != "PENDING" {
				t.Errorf("expected status 'PENDING', got %v", args[5])
			}
			return 1, nil
		},
	}
	svc := NewTaskDecompositionService(mockProvider)
	id, err := svc.Create(context.Background(), TaskDecomposition{ID: "test-id"})
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
	if id != "test-id" {
		t.Errorf("expected ID 'test-id', got %s", id)
	}
}

func TestCreate_WithoutID(t *testing.T) {
	mockProvider := &MockProvider{
		ExecFunc: func(ctx context.Context, sqlQuery string, args ...any) (int64, error) {
			if args[0] == "" {
				t.Error("expected generated ID")
			}
			return 1, nil
		},
	}
	svc := NewTaskDecompositionService(mockProvider)
	id, err := svc.Create(context.Background(), TaskDecomposition{})
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
	if id == "" {
		t.Error("expected generated ID")
	}
}

func TestCreate_Error(t *testing.T) {
	expectedErr := errors.New("db error")
	mockProvider := &MockProvider{
		ExecFunc: func(ctx context.Context, sqlQuery string, args ...any) (int64, error) {
			return 0, expectedErr
		},
	}
	svc := NewTaskDecompositionService(mockProvider)
	_, err := svc.Create(context.Background(), TaskDecomposition{})
	if err != expectedErr {
		t.Errorf("expected error %v, got %v", expectedErr, err)
	}
}

func TestGet_Success(t *testing.T) {
	mockProvider := &MockProvider{
		QueryRowFunc: func(ctx context.Context, sqlQuery string, args ...any) db.Row {
			return &MockRow{
				ScanFunc: func(dest ...any) error {
					*dest[0].(*string) = "test-id"
					return nil
				},
			}
		},
	}
	svc := NewTaskDecompositionService(mockProvider)
	task, err := svc.Get(context.Background(), "test-id")
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
	if task.ID != "test-id" {
		t.Errorf("expected task ID 'test-id', got %s", task.ID)
	}
}

func TestGet_NoRows(t *testing.T) {
	mockProvider := &MockProvider{
		QueryRowFunc: func(ctx context.Context, sqlQuery string, args ...any) db.Row {
			return &MockRow{
				ScanFunc: func(dest ...any) error {
					return sql.ErrNoRows
				},
			}
		},
	}
	svc := NewTaskDecompositionService(mockProvider)
	task, err := svc.Get(context.Background(), "test-id")
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
	if task != nil {
		t.Errorf("expected nil task, got %v", task)
	}
}

func TestGet_Error(t *testing.T) {
	expectedErr := errors.New("db error")
	mockProvider := &MockProvider{
		QueryRowFunc: func(ctx context.Context, sqlQuery string, args ...any) db.Row {
			return &MockRow{
				ScanFunc: func(dest ...any) error {
					return expectedErr
				},
			}
		},
	}
	svc := NewTaskDecompositionService(mockProvider)
	_, err := svc.Get(context.Background(), "test-id")
	if err != expectedErr {
		t.Errorf("expected error %v, got %v", expectedErr, err)
	}
}

func TestUpdateState_Success(t *testing.T) {
	mockTx := &MockTx{
		QueryRowFunc: func(ctx context.Context, sqlQuery string, args ...any) db.Row {
			return &MockRow{
				ScanFunc: func(dest ...any) error {
					*dest[0].(*string) = "PENDING"
					return nil
				},
			}
		},
		ExecFunc: func(ctx context.Context, sqlQuery string, args ...any) (int64, error) {
			return 1, nil
		},
	}
	mockProvider := &MockProvider{
		BeginFunc: func(ctx context.Context) (db.Tx, error) {
			return mockTx, nil
		},
	}
	svc := NewTaskDecompositionService(mockProvider)
	agentID := "agent-1"
	reason := "done"
	err := svc.UpdateState(context.Background(), "test-id", "COMPLETED", &agentID, &reason)
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
}

func TestUpdateState_BeginError(t *testing.T) {
	expectedErr := errors.New("begin error")
	mockProvider := &MockProvider{
		BeginFunc: func(ctx context.Context) (db.Tx, error) {
			return nil, expectedErr
		},
	}
	svc := NewTaskDecompositionService(mockProvider)
	err := svc.UpdateState(context.Background(), "test-id", "COMPLETED", nil, nil)
	if err != expectedErr {
		t.Errorf("expected error %v, got %v", expectedErr, err)
	}
}

func TestUpdateState_QueryRowError(t *testing.T) {
	expectedErr := errors.New("query error")
	mockTx := &MockTx{
		QueryRowFunc: func(ctx context.Context, sqlQuery string, args ...any) db.Row {
			return &MockRow{
				ScanFunc: func(dest ...any) error {
					return expectedErr
				},
			}
		},
	}
	mockProvider := &MockProvider{
		BeginFunc: func(ctx context.Context) (db.Tx, error) {
			return mockTx, nil
		},
	}
	svc := NewTaskDecompositionService(mockProvider)
	err := svc.UpdateState(context.Background(), "test-id", "COMPLETED", nil, nil)
	if err != expectedErr {
		t.Errorf("expected error %v, got %v", expectedErr, err)
	}
}

func TestUpdateState_ExecError(t *testing.T) {
	expectedErr := errors.New("exec error")
	mockTx := &MockTx{
		QueryRowFunc: func(ctx context.Context, sqlQuery string, args ...any) db.Row {
			return &MockRow{
				ScanFunc: func(dest ...any) error {
					*dest[0].(*string) = "PENDING"
					return nil
				},
			}
		},
		ExecFunc: func(ctx context.Context, sqlQuery string, args ...any) (int64, error) {
			return 0, expectedErr
		},
	}
	mockProvider := &MockProvider{
		BeginFunc: func(ctx context.Context) (db.Tx, error) {
			return mockTx, nil
		},
	}
	svc := NewTaskDecompositionService(mockProvider)
	err := svc.UpdateState(context.Background(), "test-id", "COMPLETED", nil, nil)
	if err != expectedErr {
		t.Errorf("expected error %v, got %v", expectedErr, err)
	}
}

func TestUpdateState_TransitionExecError(t *testing.T) {
	expectedErr := errors.New("exec transition error")
	execCount := 0
	mockTx := &MockTx{
		QueryRowFunc: func(ctx context.Context, sqlQuery string, args ...any) db.Row {
			return &MockRow{
				ScanFunc: func(dest ...any) error {
					*dest[0].(*string) = "PENDING"
					return nil
				},
			}
		},
		ExecFunc: func(ctx context.Context, sqlQuery string, args ...any) (int64, error) {
			if execCount == 1 {
				return 0, expectedErr
			}
			execCount++
			return 1, nil
		},
	}
	mockProvider := &MockProvider{
		BeginFunc: func(ctx context.Context) (db.Tx, error) {
			return mockTx, nil
		},
	}
	svc := NewTaskDecompositionService(mockProvider)
	err := svc.UpdateState(context.Background(), "test-id", "COMPLETED", nil, nil)
	if err != expectedErr {
		t.Errorf("expected error %v, got %v", expectedErr, err)
	}
}

func TestClaim_PG_Success(t *testing.T) {
	mockTx := &MockTx{
		QueryRowFunc: func(ctx context.Context, sqlQuery string, args ...any) db.Row {
			return &MockRow{
				ScanFunc: func(dest ...any) error {
					*dest[0].(*string) = "test-id"
					return nil
				},
			}
		},
	}
	mockProvider := &MockProvider{
		IsSQLiteFunc: func() bool { return false },
		BeginFunc: func(ctx context.Context) (db.Tx, error) {
			return mockTx, nil
		},
	}
	svc := NewTaskDecompositionService(mockProvider)
	task, err := svc.Claim(context.Background(), "mission-1", "agent-1")
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
	if task.ID != "test-id" {
		t.Errorf("expected task ID 'test-id', got %s", task.ID)
	}
}

func TestClaim_PG_NoRows(t *testing.T) {
	mockTx := &MockTx{
		QueryRowFunc: func(ctx context.Context, sqlQuery string, args ...any) db.Row {
			return &MockRow{
				ScanFunc: func(dest ...any) error {
					return sql.ErrNoRows
				},
			}
		},
	}
	mockProvider := &MockProvider{
		IsSQLiteFunc: func() bool { return false },
		BeginFunc: func(ctx context.Context) (db.Tx, error) {
			return mockTx, nil
		},
	}
	svc := NewTaskDecompositionService(mockProvider)
	task, err := svc.Claim(context.Background(), "mission-1", "agent-1")
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
	if task != nil {
		t.Errorf("expected nil task, got %v", task)
	}
}

func TestClaim_PG_QueryError(t *testing.T) {
	expectedErr := errors.New("query error")
	mockTx := &MockTx{
		QueryRowFunc: func(ctx context.Context, sqlQuery string, args ...any) db.Row {
			return &MockRow{
				ScanFunc: func(dest ...any) error {
					return expectedErr
				},
			}
		},
	}
	mockProvider := &MockProvider{
		IsSQLiteFunc: func() bool { return false },
		BeginFunc: func(ctx context.Context) (db.Tx, error) {
			return mockTx, nil
		},
	}
	svc := NewTaskDecompositionService(mockProvider)
	_, err := svc.Claim(context.Background(), "mission-1", "agent-1")
	if err != expectedErr {
		t.Errorf("expected error %v, got %v", expectedErr, err)
	}
}

func TestClaim_PG_UpdateError(t *testing.T) {
	expectedErr := errors.New("update error")
	mockTx := &MockTx{
		QueryRowFunc: func(ctx context.Context, sqlQuery string, args ...any) db.Row {
			return &MockRow{
				ScanFunc: func(dest ...any) error {
					*dest[0].(*string) = "test-id"
					return nil
				},
			}
		},
		ExecFunc: func(ctx context.Context, sqlQuery string, args ...any) (int64, error) {
			return 0, expectedErr
		},
	}
	mockProvider := &MockProvider{
		IsSQLiteFunc: func() bool { return false },
		BeginFunc: func(ctx context.Context) (db.Tx, error) {
			return mockTx, nil
		},
	}
	svc := NewTaskDecompositionService(mockProvider)
	_, err := svc.Claim(context.Background(), "mission-1", "agent-1")
	if err != expectedErr {
		t.Errorf("expected error %v, got %v", expectedErr, err)
	}
}

func TestClaim_PG_TransitionError(t *testing.T) {
	expectedErr := errors.New("transition error")
	execCount := 0
	mockTx := &MockTx{
		QueryRowFunc: func(ctx context.Context, sqlQuery string, args ...any) db.Row {
			return &MockRow{
				ScanFunc: func(dest ...any) error {
					*dest[0].(*string) = "test-id"
					return nil
				},
			}
		},
		ExecFunc: func(ctx context.Context, sqlQuery string, args ...any) (int64, error) {
			if execCount == 1 {
				return 0, expectedErr
			}
			execCount++
			return 1, nil
		},
	}
	mockProvider := &MockProvider{
		IsSQLiteFunc: func() bool { return false },
		BeginFunc: func(ctx context.Context) (db.Tx, error) {
			return mockTx, nil
		},
	}
	svc := NewTaskDecompositionService(mockProvider)
	_, err := svc.Claim(context.Background(), "mission-1", "agent-1")
	if err != expectedErr {
		t.Errorf("expected error %v, got %v", expectedErr, err)
	}
}

func TestClaim_SQLite_Success(t *testing.T) {
	mockTx := &MockTx{
		QueryRowFunc: func(ctx context.Context, sqlQuery string, args ...any) db.Row {
			return &MockRow{
				ScanFunc: func(dest ...any) error {
					*dest[0].(*string) = "test-id"
					return nil
				},
			}
		},
	}
	mockProvider := &MockProvider{
		IsSQLiteFunc: func() bool { return true },
		BeginFunc: func(ctx context.Context) (db.Tx, error) {
			return mockTx, nil
		},
	}
	svc := NewTaskDecompositionService(mockProvider)
	task, err := svc.Claim(context.Background(), "mission-1", "agent-1")
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
	if task.ID != "test-id" {
		t.Errorf("expected task ID 'test-id', got %s", task.ID)
	}
}

func TestClaim_SQLite_NoRows(t *testing.T) {
	mockTx := &MockTx{
		QueryRowFunc: func(ctx context.Context, sqlQuery string, args ...any) db.Row {
			return &MockRow{
				ScanFunc: func(dest ...any) error {
					return sql.ErrNoRows
				},
			}
		},
	}
	mockProvider := &MockProvider{
		IsSQLiteFunc: func() bool { return true },
		BeginFunc: func(ctx context.Context) (db.Tx, error) {
			return mockTx, nil
		},
	}
	svc := NewTaskDecompositionService(mockProvider)
	task, err := svc.Claim(context.Background(), "mission-1", "agent-1")
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
	if task != nil {
		t.Errorf("expected nil task, got %v", task)
	}
}

func TestClaim_SQLite_QueryError(t *testing.T) {
	expectedErr := errors.New("query error")
	mockTx := &MockTx{
		QueryRowFunc: func(ctx context.Context, sqlQuery string, args ...any) db.Row {
			return &MockRow{
				ScanFunc: func(dest ...any) error {
					return expectedErr
				},
			}
		},
	}
	mockProvider := &MockProvider{
		IsSQLiteFunc: func() bool { return true },
		BeginFunc: func(ctx context.Context) (db.Tx, error) {
			return mockTx, nil
		},
	}
	svc := NewTaskDecompositionService(mockProvider)
	_, err := svc.Claim(context.Background(), "mission-1", "agent-1")
	if err != expectedErr {
		t.Errorf("expected error %v, got %v", expectedErr, err)
	}
}

func TestClaim_SQLite_UpdateError(t *testing.T) {
	expectedErr := errors.New("update error")
	mockTx := &MockTx{
		QueryRowFunc: func(ctx context.Context, sqlQuery string, args ...any) db.Row {
			return &MockRow{
				ScanFunc: func(dest ...any) error {
					*dest[0].(*string) = "test-id"
					return nil
				},
			}
		},
		ExecFunc: func(ctx context.Context, sqlQuery string, args ...any) (int64, error) {
			return 0, expectedErr
		},
	}
	mockProvider := &MockProvider{
		IsSQLiteFunc: func() bool { return true },
		BeginFunc: func(ctx context.Context) (db.Tx, error) {
			return mockTx, nil
		},
	}
	svc := NewTaskDecompositionService(mockProvider)
	_, err := svc.Claim(context.Background(), "mission-1", "agent-1")
	if err != expectedErr {
		t.Errorf("expected error %v, got %v", expectedErr, err)
	}
}

func TestClaim_BeginError(t *testing.T) {
	expectedErr := errors.New("begin error")
	mockProvider := &MockProvider{
		BeginFunc: func(ctx context.Context) (db.Tx, error) {
			return nil, expectedErr
		},
	}
	svc := NewTaskDecompositionService(mockProvider)
	_, err := svc.Claim(context.Background(), "mission-1", "agent-1")
	if err != expectedErr {
		t.Errorf("expected error %v, got %v", expectedErr, err)
	}
}

func TestClaim_CommitError(t *testing.T) {
	expectedErr := errors.New("commit error")
	mockTx := &MockTx{
		QueryRowFunc: func(ctx context.Context, sqlQuery string, args ...any) db.Row {
			return &MockRow{
				ScanFunc: func(dest ...any) error {
					*dest[0].(*string) = "test-id"
					return nil
				},
			}
		},
		CommitFunc: func(ctx context.Context) error {
			return expectedErr
		},
	}
	mockProvider := &MockProvider{
		BeginFunc: func(ctx context.Context) (db.Tx, error) {
			return mockTx, nil
		},
	}
	svc := NewTaskDecompositionService(mockProvider)
	_, err := svc.Claim(context.Background(), "mission-1", "agent-1")
	if err != expectedErr {
		t.Errorf("expected error %v, got %v", expectedErr, err)
	}
}

package tasks

import (
    "context"
    "database/sql"
    "errors"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/db"
)

type mockRow struct {
    err error
    id  string
}

func (m *mockRow) Scan(dest ...any) error {
    if m.err != nil {
        return m.err
    }
    if len(dest) > 0 {
        if idPtr, ok := dest[0].(*string); ok {
            *idPtr = m.id
        }
    }
    return nil
}

type mockRows struct{}

func (m *mockRows) Next() bool { return false }
func (m *mockRows) Scan(dest ...any) error { return nil }
func (m *mockRows) Err() error { return nil }
func (m *mockRows) Close() {}

type mockProvider struct {
    isSqlite bool
    rowErr   error
    execErr  error
    txErr    error
    db.Provider
}

func (m *mockProvider) IsSQLite() bool {
    return m.isSqlite
}

func (m *mockProvider) Exec(ctx context.Context, query string, args ...any) (int64, error) {
    if m.execErr != nil {
        return 0, m.execErr
    }
    return 1, nil
}

func (m *mockProvider) QueryRow(ctx context.Context, query string, args ...any) db.Row {
    if m.rowErr != nil {
        return &mockRow{err: m.rowErr}
    }
    return &mockRow{id: "task-1"}
}

type mockTx struct {
    rowErr  error
    execErr error
    db.Tx
}

func (m *mockTx) Exec(ctx context.Context, query string, args ...any) (int64, error) {
    if m.execErr != nil {
        return 0, m.execErr
    }
    return 1, nil
}

func (m *mockTx) QueryRow(ctx context.Context, query string, args ...any) db.Row {
    if m.rowErr != nil {
        return &mockRow{err: m.rowErr}
    }
    return &mockRow{id: "task-1"}
}

func (m *mockTx) Commit(ctx context.Context) error {
    return nil
}

func (m *mockTx) Rollback(ctx context.Context) error {
    return nil
}

func (m *mockProvider) Begin(ctx context.Context) (db.Tx, error) {
    if m.txErr != nil {
        return nil, m.txErr
    }
    return &mockTx{rowErr: m.rowErr, execErr: m.execErr}, nil
}

func TestTaskDecompositionService_CreateTask(t *testing.T) {
    p := &mockProvider{}
    s := NewTaskDecompositionService(p)
    err := s.CreateTask(context.Background(), &SwarmTask{MissionID: "m1", Title: "t1"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
}

func TestTaskDecompositionService_ClaimTask_SQLite(t *testing.T) {
    p := &mockProvider{isSqlite: true}
    s := NewTaskDecompositionService(p)
    task, err := s.ClaimTask(context.Background(), "agent-1")
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if task.ID != "task-1" {
        t.Fatalf("expected task-1, got %s", task.ID)
    }
}

func TestTaskDecompositionService_ClaimTask_SQLite_NoRows(t *testing.T) {
    p := &mockProvider{isSqlite: true, rowErr: sql.ErrNoRows}
    s := NewTaskDecompositionService(p)
    _, err := s.ClaimTask(context.Background(), "agent-1")
    if err == nil || err.Error() != "no tasks available" {
        t.Fatalf("expected no tasks available error, got %v", err)
    }
}

func TestTaskDecompositionService_ClaimTask_SQLite_RowErr(t *testing.T) {
    p := &mockProvider{isSqlite: true, rowErr: errors.New("db error")}
    s := NewTaskDecompositionService(p)
    _, err := s.ClaimTask(context.Background(), "agent-1")
    if err == nil || err.Error() != "db error" {
        t.Fatalf("expected db error, got %v", err)
    }
}

func TestTaskDecompositionService_ClaimTask_SQLite_ExecErr(t *testing.T) {
    p := &mockProvider{isSqlite: true, execErr: errors.New("update err")}
    s := NewTaskDecompositionService(p)
    _, err := s.ClaimTask(context.Background(), "agent-1")
    if err == nil || err.Error() != "update err" {
        t.Fatalf("expected update err, got %v", err)
    }
}

func TestTaskDecompositionService_ClaimTask_Postgres(t *testing.T) {
    p := &mockProvider{isSqlite: false}
    s := NewTaskDecompositionService(p)
    task, err := s.ClaimTask(context.Background(), "agent-1")
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if task.ID != "task-1" {
        t.Fatalf("expected task-1, got %s", task.ID)
    }
}

func TestTaskDecompositionService_ClaimTask_Postgres_NoRows(t *testing.T) {
    p := &mockProvider{isSqlite: false, rowErr: sql.ErrNoRows}
    s := NewTaskDecompositionService(p)
    _, err := s.ClaimTask(context.Background(), "agent-1")
    if err == nil || err.Error() != "no tasks available" {
        t.Fatalf("expected no tasks available error, got %v", err)
    }
}

func TestTaskDecompositionService_ClaimTask_Postgres_RowErr(t *testing.T) {
    p := &mockProvider{isSqlite: false, rowErr: errors.New("db err")}
    s := NewTaskDecompositionService(p)
    _, err := s.ClaimTask(context.Background(), "agent-1")
    if err == nil || err.Error() != "db err" {
        t.Fatalf("expected db err, got %v", err)
    }
}

func TestTaskDecompositionService_ClaimTask_Postgres_BeginErr(t *testing.T) {
    p := &mockProvider{isSqlite: false, txErr: errors.New("begin err")}
    s := NewTaskDecompositionService(p)
    _, err := s.ClaimTask(context.Background(), "agent-1")
    if err == nil || err.Error() != "begin err" {
        t.Fatalf("expected begin err, got %v", err)
    }
}

func TestTaskDecompositionService_ClaimTask_Postgres_ExecErr(t *testing.T) {
    p := &mockProvider{isSqlite: false, execErr: errors.New("update err")}
    s := NewTaskDecompositionService(p)
    _, err := s.ClaimTask(context.Background(), "agent-1")
    if err == nil || err.Error() != "update err" {
        t.Fatalf("expected update err, got %v", err)
    }
}

func TestTaskDecompositionService_UpdateTaskStatus(t *testing.T) {
    p := &mockProvider{}
    s := NewTaskDecompositionService(p)
    err := s.UpdateTaskStatus(context.Background(), "task-1", "PENDING", "IN_PROGRESS", "agent-1", "started")
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
}

func TestTaskDecompositionService_UpdateTaskStatus_BeginErr(t *testing.T) {
    p := &mockProvider{txErr: errors.New("begin err")}
    s := NewTaskDecompositionService(p)
    err := s.UpdateTaskStatus(context.Background(), "task-1", "PENDING", "IN_PROGRESS", "agent-1", "started")
    if err == nil || err.Error() != "begin err" {
        t.Fatalf("expected begin err, got %v", err)
    }
}

type mockTxNoRows struct {
    mockTx
}
func (m *mockTxNoRows) Exec(ctx context.Context, query string, args ...any) (int64, error) {
    return 0, nil
}
func (m *mockProviderNoRows) Begin(ctx context.Context) (db.Tx, error) {
    return &mockTxNoRows{}, nil
}
type mockProviderNoRows struct { mockProvider }

func TestTaskDecompositionService_UpdateTaskStatus_NoUpdate(t *testing.T) {
    p := &mockProviderNoRows{}
    s := NewTaskDecompositionService(p)
    err := s.UpdateTaskStatus(context.Background(), "task-1", "PENDING", "IN_PROGRESS", "agent-1", "started")
    if err == nil || err.Error() != "task not found or state mismatch" {
        t.Fatalf("expected task not found, got %v", err)
    }
}

type mockTxExecErr struct { mockTx; err1, err2 error; calls int }
func (m *mockTxExecErr) Exec(ctx context.Context, query string, args ...any) (int64, error) {
    m.calls++
    if m.calls == 1 && m.err1 != nil { return 0, m.err1 }
    if m.calls == 2 && m.err2 != nil { return 0, m.err2 }
    return 1, nil
}
type mockProviderExecErr struct { mockProvider; err1, err2 error }
func (m *mockProviderExecErr) Begin(ctx context.Context) (db.Tx, error) {
    return &mockTxExecErr{err1: m.err1, err2: m.err2}, nil
}

func TestTaskDecompositionService_UpdateTaskStatus_UpdateErr(t *testing.T) {
    p := &mockProviderExecErr{err1: errors.New("update err")}
    s := NewTaskDecompositionService(p)
    err := s.UpdateTaskStatus(context.Background(), "task-1", "PENDING", "IN_PROGRESS", "agent-1", "started")
    if err == nil || err.Error() != "update err" {
        t.Fatalf("expected update err, got %v", err)
    }
}

func TestTaskDecompositionService_UpdateTaskStatus_InsertErr(t *testing.T) {
    p := &mockProviderExecErr{err2: errors.New("insert err")}
    s := NewTaskDecompositionService(p)
    err := s.UpdateTaskStatus(context.Background(), "task-1", "PENDING", "IN_PROGRESS", "agent-1", "started")
    if err == nil || err.Error() != "insert err" {
        t.Fatalf("expected insert err, got %v", err)
    }
}

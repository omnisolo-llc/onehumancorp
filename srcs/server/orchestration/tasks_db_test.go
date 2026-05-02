package orchestration

import (
	"context"
	"database/sql"
	"database/sql/driver"
	"errors"
	"testing"
	"time"
)

type mockDriver struct{}
type mockConn struct{}
type mockStmt struct {
	query string
}
type mockTx struct{}

var (
	ErrSimulated = errors.New("simulated error")
	SimulateNoRows = false
	SimulateQueryErr = false
	SimulateExecErr = false
	SimulateTxErr = false
)

func (d mockDriver) Open(name string) (driver.Conn, error) { return mockConn{}, nil }

func (c mockConn) Prepare(query string) (driver.Stmt, error) {
	return mockStmt{query: query}, nil
}
func (c mockConn) Close() error { return nil }
func (c mockConn) Begin() (driver.Tx, error) {
	if SimulateTxErr {
		return nil, ErrSimulated
	}
	return mockTx{}, nil
}

func (s mockStmt) Close() error { return nil }
func (s mockStmt) NumInput() int { return -1 }
func (s mockStmt) Exec(args []driver.Value) (driver.Result, error) {
	if SimulateExecErr {
		return nil, ErrSimulated
	}
	return mockResult{}, nil
}
func (s mockStmt) Query(args []driver.Value) (driver.Rows, error) {
	if SimulateQueryErr {
		return nil, ErrSimulated
	}
	if SimulateNoRows {
		return emptyRows{}, nil
	}
	return &mockRowsData{query: s.query}, nil
}

type mockResult struct{}
func (m mockResult) LastInsertId() (int64, error) { return 1, nil }
func (m mockResult) RowsAffected() (int64, error) { return 1, nil }

type emptyRows struct{}
func (e emptyRows) Columns() []string { return []string{"id", "organization_id", "parent_plan_id", "title", "description", "status", "assigned_agent_id", "dependencies", "created_at", "updated_at"} }
func (e emptyRows) Close() error { return nil }
func (e emptyRows) Next(dest []driver.Value) error { return sql.ErrNoRows }

type mockRowsData struct {
	query string
	read  bool
}
func (r *mockRowsData) Columns() []string {
	return []string{"id", "organization_id", "parent_plan_id", "title", "description", "status", "assigned_agent_id", "dependencies", "created_at", "updated_at"}
}
func (r *mockRowsData) Close() error { return nil }
func (r *mockRowsData) Next(dest []driver.Value) error {
	if r.read {
		return errors.New("EOF")
	}
	r.read = true
	dest[0] = "task-1"
	dest[1] = "org-1"
	dest[2] = "plan-1"
	dest[3] = "Task Title"
	dest[4] = "Task Desc"
	dest[5] = "PENDING"
	dest[6] = "agent-old"
	dest[7] = []byte(`["dep-1"]`)
	dest[8] = time.Now()
	dest[9] = time.Now()
	return nil
}

func (t mockTx) Commit() error   { return nil }
func (t mockTx) Rollback() error { return nil }

func init() {
	sql.Register("custom_mock", mockDriver{})
}

func resetMockState() {
	SimulateNoRows = false
	SimulateQueryErr = false
	SimulateExecErr = false
	SimulateTxErr = false
}

func TestClaimTask_Postgres_Success(t *testing.T) {
	resetMockState()
	db, err := sql.Open("custom_mock", "test")
	if err != nil {
		t.Fatal(err)
	}
	defer db.Close()

	taskDB := NewTaskDB(db, false)
	task, err := taskDB.ClaimTask(context.Background(), "agent-new")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if task == nil {
		t.Fatalf("expected task, got nil")
	}
	if task.ID != "task-1" {
		t.Errorf("expected task-1, got %v", task.ID)
	}
	if task.Status != "ASSIGNED" {
		t.Errorf("expected ASSIGNED, got %v", task.Status)
	}
	if *task.AssignedAgentID != "agent-new" {
		t.Errorf("expected agent-new, got %v", *task.AssignedAgentID)
	}
}

func TestClaimTask_SQLite_Success(t *testing.T) {
	resetMockState()
	db, err := sql.Open("custom_mock", "test")
	if err != nil {
		t.Fatal(err)
	}
	defer db.Close()

	taskDB := NewTaskDB(db, true)
	task, err := taskDB.ClaimTask(context.Background(), "agent-new")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if task == nil {
		t.Fatalf("expected task, got nil")
	}
	if task.ID != "task-1" {
		t.Errorf("expected task-1, got %v", task.ID)
	}
	if task.Status != "ASSIGNED" {
		t.Errorf("expected ASSIGNED, got %v", task.Status)
	}
	if *task.AssignedAgentID != "agent-new" {
		t.Errorf("expected agent-new, got %v", *task.AssignedAgentID)
	}
}

func TestClaimTask_BeginTxErr(t *testing.T) {
	resetMockState()
	SimulateTxErr = true
	db, _ := sql.Open("custom_mock", "test")
	taskDB := NewTaskDB(db, false)
	_, err := taskDB.ClaimTask(context.Background(), "agent-new")
	if err == nil {
		t.Errorf("expected error on BeginTx")
	}

	taskDB = NewTaskDB(db, true)
	_, err = taskDB.ClaimTask(context.Background(), "agent-new")
	if err == nil {
		t.Errorf("expected error on BeginTx (SQLite)")
	}
}

func TestClaimTask_QueryErr(t *testing.T) {
	resetMockState()
	SimulateQueryErr = true
	db, _ := sql.Open("custom_mock", "test")

	taskDB := NewTaskDB(db, false)
	_, err := taskDB.ClaimTask(context.Background(), "agent-new")
	if err == nil {
		t.Errorf("expected query error")
	}

	taskDB = NewTaskDB(db, true)
	_, err = taskDB.ClaimTask(context.Background(), "agent-new")
	if err == nil {
		t.Errorf("expected query error (SQLite)")
	}
}

func TestClaimTask_ExecErr(t *testing.T) {
	resetMockState()
	SimulateExecErr = true
	db, _ := sql.Open("custom_mock", "test")

	taskDB := NewTaskDB(db, false)
	_, err := taskDB.ClaimTask(context.Background(), "agent-new")
	if err == nil {
		t.Errorf("expected exec error")
	}

	taskDB = NewTaskDB(db, true)
	_, err = taskDB.ClaimTask(context.Background(), "agent-new")
	if err == nil {
		t.Errorf("expected exec error (SQLite)")
	}
}

func TestClaimTask_NoRows(t *testing.T) {
	resetMockState()
	SimulateNoRows = true
	db, _ := sql.Open("custom_mock", "test")

	taskDB := NewTaskDB(db, false)
	task, err := taskDB.ClaimTask(context.Background(), "agent-new")
	if err != nil {
		t.Errorf("expected no error for sql.ErrNoRows, got %v", err)
	}
	if task != nil {
		t.Errorf("expected nil task when no rows available")
	}

	taskDB = NewTaskDB(db, true)
	task, err = taskDB.ClaimTask(context.Background(), "agent-new")
	if err != nil {
		t.Errorf("expected no error for sql.ErrNoRows (SQLite), got %v", err)
	}
	if task != nil {
		t.Errorf("expected nil task when no rows available (SQLite)")
	}
}

package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"testing"
	"time"
)

type MockDB struct {
	isSQLite bool
	err      error
	tasks    map[string]*SharedTask
}

func (m *MockDB) IsSQLite() bool {
	return m.isSQLite
}

type MockResult struct{}
func (m MockResult) LastInsertId() (int64, error) { return 0, nil }
func (m MockResult) RowsAffected() (int64, error) { return 1, nil }

func (m *MockDB) ExecContext(ctx context.Context, query string, args ...any) (sql.Result, error) {
	if m.err != nil {
		return nil, m.err
	}
	if len(args) == 2 {
		agentID := args[0].(string)
		taskID := args[1].(string)
		if task, ok := m.tasks[taskID]; ok {
			task.Status = "ASSIGNED"
			task.AssignedAgentID = &agentID
		}
	}
	return MockResult{}, nil
}

type mockRow struct {
	task *SharedTask
	err  error
}

func (r *mockRow) Scan(dest ...any) error {
	if r.err != nil {
		return r.err
	}
	if r.task == nil {
		return sql.ErrNoRows
	}

	*dest[0].(*string) = r.task.ID
	*dest[1].(*string) = r.task.OrganizationID
	*dest[2].(**string) = r.task.ParentPlanID
	*dest[3].(*string) = r.task.Title
	*dest[4].(**string) = r.task.Description
	*dest[5].(*string) = r.task.Status
	*dest[6].(**string) = r.task.AssignedAgentID
	*dest[7].(*json.RawMessage) = r.task.Dependencies
	*dest[8].(*time.Time) = r.task.CreatedAt
	*dest[9].(*time.Time) = r.task.UpdatedAt

	return nil
}

func (m *MockDB) QueryRowContext(ctx context.Context, query string, args ...any) RowScanner {
	if m.err != nil {
		return &mockRow{err: m.err}
	}
	orgID := args[0].(string)
	for _, task := range m.tasks {
		if task.OrganizationID == orgID && task.Status == "PENDING" {
			return &mockRow{task: task}
		}
	}
	return &mockRow{err: sql.ErrNoRows}
}

func TestClaimTask_Postgres(t *testing.T) {
	task := &SharedTask{
		ID:             "1",
		OrganizationID: "org-1",
		Title:          "Task 1",
		Status:         "PENDING",
		Dependencies:   json.RawMessage("[]"),
		CreatedAt:      time.Now(),
		UpdatedAt:      time.Now(),
	}

	mockDB := &MockDB{
		isSQLite: false,
		tasks: map[string]*SharedTask{
			"1": task,
		},
	}

	tasksDB := NewTasksDB(mockDB)

	ctx := context.Background()
	claimedTask, err := tasksDB.ClaimTask(ctx, "org-1", "agent-1")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if claimedTask == nil {
		t.Fatalf("expected task, got nil")
	}
	if claimedTask.Status != "ASSIGNED" {
		t.Errorf("expected status ASSIGNED, got %s", claimedTask.Status)
	}
	if *claimedTask.AssignedAgentID != "agent-1" {
		t.Errorf("expected assigned agent agent-1, got %v", claimedTask.AssignedAgentID)
	}
}

func TestClaimTask_SQLite(t *testing.T) {
	task := &SharedTask{
		ID:             "2",
		OrganizationID: "org-2",
		Title:          "Task 2",
		Status:         "PENDING",
		Dependencies:   json.RawMessage("[]"),
		CreatedAt:      time.Now(),
		UpdatedAt:      time.Now(),
	}

	mockDB := &MockDB{
		isSQLite: true,
		tasks: map[string]*SharedTask{
			"2": task,
		},
	}

	tasksDB := NewTasksDB(mockDB)

	ctx := context.Background()
	claimedTask, err := tasksDB.ClaimTask(ctx, "org-2", "agent-2")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if claimedTask == nil {
		t.Fatalf("expected task, got nil")
	}
	if claimedTask.Status != "ASSIGNED" {
		t.Errorf("expected status ASSIGNED, got %s", claimedTask.Status)
	}
	if *claimedTask.AssignedAgentID != "agent-2" {
		t.Errorf("expected assigned agent agent-2, got %v", claimedTask.AssignedAgentID)
	}
}

func TestClaimTask_NoTask(t *testing.T) {
	mockDB := &MockDB{
		isSQLite: false,
		tasks:    map[string]*SharedTask{},
	}

	tasksDB := NewTasksDB(mockDB)

	ctx := context.Background()
	claimedTask, err := tasksDB.ClaimTask(ctx, "org-3", "agent-3")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if claimedTask != nil {
		t.Fatalf("expected nil task, got %v", claimedTask)
	}
}

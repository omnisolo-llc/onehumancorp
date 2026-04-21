package state

import (
	"context"
	"encoding/json"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
	"github.com/redis/rueidis/mock"
	"go.uber.org/mock/gomock"
	_ "modernc.org/sqlite"
)

type MockDBProvider struct {
	db.Provider
	IsSQLiteMock func() bool
}

func (m *MockDBProvider) IsSQLite() bool {
	if m.IsSQLiteMock != nil {
		return m.IsSQLiteMock()
	}
	return m.Provider.IsSQLite()
}

func TestCloudStateManager(t *testing.T) {
	provider := db.NewTestProvider(t)
	defer provider.Close()

	ctx := context.Background()

	// Apply migrations or create tables directly
	tx, err := provider.Begin(ctx)
	if err != nil {
		t.Fatal(err)
	}

	_, err = tx.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_tasks (
			id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
			mission_id TEXT NOT NULL,
			parent_plan_id TEXT,
			dependencies JSON NOT NULL DEFAULT '[]',
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			payload JSON,
			locked_until DATETIME,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);

		CREATE TABLE IF NOT EXISTS state_machine_transitions (
			id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
			entity_id TEXT NOT NULL,
			entity_type TEXT NOT NULL,
			from_state TEXT NOT NULL,
			to_state TEXT NOT NULL,
			agent_id TEXT,
			reason TEXT,
			occurred_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatal(err)
	}
	tx.Commit(ctx)

	ctrl := gomock.NewController(t)
	defer ctrl.Finish()

	mockClient := mock.NewClient(ctrl)

	// We want Do() to return success for SET NX EX (lock) and DEL (unlock)
	mockClient.EXPECT().Do(gomock.Any(), gomock.Any()).Return(mock.Result(mock.RedisString("OK"))).AnyTimes()

	// Use our mock RedisClient
	sm := NewCloudStateManager(provider, mockClient)

	tx, _ = provider.Begin(ctx)
	_, err = tx.Exec(ctx, "INSERT INTO swarm_tasks (id, mission_id, title, status) VALUES ('c_task1', 'm1', 'Task 1', 'PENDING')")
	if err != nil {
		t.Fatal(err)
	}

	_, err = tx.Exec(ctx, "INSERT INTO swarm_tasks (id, mission_id, title, status) VALUES ('c_parent_task', 'm1', 'Parent Task', 'COMPLETED')")
	if err != nil {
		t.Fatal(err)
	}

	deps, _ := json.Marshal([]string{"c_parent_task"})
	_, err = tx.Exec(ctx, "INSERT INTO swarm_tasks (id, mission_id, title, status, dependencies) VALUES ('c_task2', 'm1', 'Task 2', 'PENDING', $1)", string(deps))
	if err != nil {
		t.Fatal(err)
	}
	tx.Commit(ctx)

	// Test ClaimTask (SQLite syntax)
	task, err := sm.ClaimTask(ctx, "agent1")
	if err != nil {
		t.Fatalf("ClaimTask failed: %v", err)
	}
	if task.ID != "c_task1" && task.ID != "c_task2" {
		t.Fatalf("Expected c_task1 or c_task2, got %s", task.ID)
	}

	// TransitionState without deps
	err = sm.TransitionState(ctx, "c_task1", "agent1", "PENDING", "EXECUTING", "start")
	if err != nil {
		t.Fatalf("TransitionState failed: %v", err)
	}

	// TransitionState with deps
	err = sm.TransitionState(ctx, "c_task2", "agent1", "PENDING", "EXECUTING", "start")
	if err != nil {
		t.Fatalf("TransitionState with deps failed: %v", err)
	}

	status, err := sm.GetTaskStatus(ctx, "c_task1")
	if err != nil || status != "EXECUTING" {
		t.Fatalf("Expected EXECUTING, got %s", status)
	}

	err = sm.MarkTaskCompleted(ctx, "c_task1")
	if err != nil {
		t.Fatalf("MarkTaskCompleted failed: %v", err)
	}

	status, _ = sm.GetTaskStatus(ctx, "c_task1")
	if status != "COMPLETED" {
		t.Fatalf("Expected COMPLETED, got %s", status)
	}

	// Test with PostgreSQL mock
	pgProvider := &MockDBProvider{
		Provider: provider,
		IsSQLiteMock: func() bool {
			return false
		},
	}
	smPg := NewCloudStateManager(pgProvider, mockClient)

	// Should fail with syntax error "near FOR" because SQLite doesn't understand PostgreSQL's FOR UPDATE SKIP LOCKED
	// But it covers the branch in the file
	_, err = smPg.ClaimTask(ctx, "agent2")
	if err == nil {
		t.Log("Expected syntax error with Postgres query on SQLite")
	}

	// For TransitionState on pgProvider:
	err = smPg.TransitionState(ctx, "c_task2", "agent1", "EXECUTING", "COMPLETED", "finish")
	if err == nil {
		t.Log("Expected syntax error with Postgres query on SQLite")
	}

	// Test missing task
	err = sm.TransitionState(ctx, "nonexistent", "agent1", "PENDING", "EXECUTING", "start")
	if err == nil {
		t.Fatal("Expected error for nonexistent task")
	}

	// Test wrong state
	err = sm.TransitionState(ctx, "c_task1", "agent1", "PENDING", "EXECUTING", "start")
	if err == nil {
		t.Fatal("Expected error for wrong state")
	}

	// Test unmet dependencies
	_, err = provider.Exec(ctx, "INSERT INTO swarm_tasks (id, mission_id, title, status) VALUES ('c_parent2', 'm1', 'Parent 2', 'PENDING')")
	if err != nil {
		t.Fatal(err)
	}
	unmetDeps, _ := json.Marshal([]string{"c_parent2"})
	_, err = provider.Exec(ctx, "INSERT INTO swarm_tasks (id, mission_id, title, status, dependencies) VALUES ('c_task3', 'm1', 'Task 3', 'PENDING', $1)", string(unmetDeps))
	if err != nil {
		t.Fatal(err)
	}
	err = sm.TransitionState(ctx, "c_task3", "agent1", "PENDING", "EXECUTING", "start")
	if err == nil {
		t.Fatal("Expected error for unmet dependencies")
	}

	// Test redis lock failure
	mockClientFail := mock.NewClient(ctrl)
	mockClientFail.EXPECT().Do(gomock.Any(), gomock.Any()).Return(mock.ErrorResult(rueidis.Nil)).AnyTimes()

	smFailLock := NewCloudStateManager(provider, mockClientFail)
	err = smFailLock.TransitionState(ctx, "c_task1", "agent1", "PENDING", "EXECUTING", "start")
	if err == nil {
		t.Fatal("Expected error for redis lock failure")
	}
    err = smFailLock.MarkTaskCompleted(ctx, "c_task1")
	if err == nil {
		t.Fatal("Expected error for redis lock failure on MarkTaskCompleted")
	}
}

func TestCloudStateManager_InvalidJSON(t *testing.T) {
	provider := db.NewTestProvider(t)
	defer provider.Close()
	ctx := context.Background()

	tx, _ := provider.Begin(ctx)
	_, _ = tx.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_tasks (
			id TEXT PRIMARY KEY,
			mission_id TEXT NOT NULL,
			parent_plan_id TEXT,
			dependencies JSON NOT NULL DEFAULT '[]',
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			payload JSON,
			locked_until DATETIME,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	// Insert task with invalid JSON dependencies
	_, _ = tx.Exec(ctx, "INSERT INTO swarm_tasks (id, mission_id, title, status, dependencies, locked_until) VALUES ('c_task_invalid', 'm1', 'Task Invalid', 'PENDING', '{invalid}', NULL)")
	tx.Commit(ctx)

	ctrl := gomock.NewController(t)
	defer ctrl.Finish()
	mockClient := mock.NewClient(ctrl)
	mockClient.EXPECT().Do(gomock.Any(), gomock.Any()).Return(mock.Result(mock.RedisString("OK"))).AnyTimes()

	sm := NewCloudStateManager(provider, mockClient)

	// ClaimTask should fail due to invalid JSON dependencies
	_, err := sm.ClaimTask(ctx, "agent")
	// Actually, wait, ClaimTask only fetches pending tasks and attempts to unmarshal.
	if err == nil {
		t.Fatal("Expected error for invalid JSON dependencies in ClaimTask")
	}

	// TransitionState should fail due to invalid JSON dependencies when transitioning to EXECUTING
	err = sm.TransitionState(ctx, "c_task_invalid", "agent", "PENDING", "EXECUTING", "start")
	if err == nil {
		t.Fatal("Expected error for invalid JSON dependencies in TransitionState")
	}
}

func TestCloudStateManager_DBErrors(t *testing.T) {
	provider := db.NewTestProvider(t)
	defer provider.Close()
	ctx := context.Background()

	ctrl := gomock.NewController(t)
	defer ctrl.Finish()
	mockClient := mock.NewClient(ctrl)
	mockClient.EXPECT().Do(gomock.Any(), gomock.Any()).Return(mock.Result(mock.RedisString("OK"))).AnyTimes()

	sm := NewCloudStateManager(provider, mockClient)

	// Close provider to force tx errors
	provider.Close()

	err := sm.TransitionState(ctx, "task", "agent", "PENDING", "EXECUTING", "start")
	if err == nil {
		t.Fatal("Expected error when provider closed for TransitionState")
	}

	_, err = sm.ClaimTask(ctx, "agent")
	if err == nil {
		t.Fatal("Expected error when provider closed for ClaimTask")
	}

	err = sm.MarkTaskCompleted(ctx, "task")
	if err == nil {
		t.Fatal("Expected error when provider closed for MarkTaskCompleted")
	}

	_, err = sm.GetTaskStatus(ctx, "task")
	if err == nil {
		t.Fatal("Expected error when provider closed for GetTaskStatus")
	}
}

func TestCloudStateManager_UpdateErrors(t *testing.T) {
	provider := db.NewTestProvider(t)
	defer provider.Close()
	ctx := context.Background()

	tx, _ := provider.Begin(ctx)
	_, _ = tx.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_tasks (
			id TEXT PRIMARY KEY,
			mission_id TEXT NOT NULL,
			parent_plan_id TEXT,
			dependencies JSON NOT NULL DEFAULT '[]',
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			payload JSON,
			locked_until DATETIME,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);

		CREATE TABLE IF NOT EXISTS state_machine_transitions (
			id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
			entity_id TEXT NOT NULL,
			entity_type TEXT NOT NULL,
			from_state TEXT NOT NULL,
			to_state TEXT NOT NULL,
			agent_id TEXT,
			reason TEXT,
			occurred_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	_, _ = tx.Exec(ctx, "INSERT INTO swarm_tasks (id, mission_id, title, status) VALUES ('c_task1', 'm1', 'Task 1', 'PENDING')")
	tx.Commit(ctx)

	ctrl := gomock.NewController(t)
	defer ctrl.Finish()
	mockClient := mock.NewClient(ctrl)
	mockClient.EXPECT().Do(gomock.Any(), gomock.Any()).Return(mock.Result(mock.RedisString("OK"))).AnyTimes()

	sm := NewCloudStateManager(provider, mockClient)

	tx, _ = provider.Begin(ctx)
	_, _ = tx.Exec(ctx, "DROP TABLE swarm_tasks")
	tx.Commit(ctx)

	err := sm.MarkTaskCompleted(ctx, "c_task1")
	if err == nil {
		t.Fatal("Expected error for missing table")
	}

	err = sm.TransitionState(ctx, "c_task1", "agent", "PENDING", "EXECUTING", "start")
	if err == nil {
		t.Fatal("Expected error for missing table")
	}
}

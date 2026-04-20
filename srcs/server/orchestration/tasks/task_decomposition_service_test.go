package tasks

import (
	"context"
	"errors"
	"sync"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func ptrString(s string) *string {
	return &s
}

func TestTaskDecompositionService_CreateAndGet(t *testing.T) {
	provider := db.NewTestProvider(t)
	setupTestSchema(t, provider)

	svc := NewTaskDecompositionService(provider)
	ctx := context.Background()

	task := &TaskDecomposition{
		ID:             "test-id-1",
		OrganizationID: "org-1",
		Title:          "First Task",
		Description:    ptrString("Description 1"),
		Status:         "PENDING",
		Priority:       "P1",
		Payload:        ptrString(`{"key": "value"}`),
		Dependencies:   "[]",
	}

	err := svc.CreateTask(ctx, task)
	if err != nil {
		t.Fatalf("failed to create task: %v", err)
	}

	fetched, err := svc.GetTask(ctx, "test-id-1")
	if err != nil {
		t.Fatalf("failed to get task: %v", err)
	}

	if fetched.Title != "First Task" || fetched.OrganizationID != "org-1" {
		t.Errorf("unexpected task fields: %+v", fetched)
	}
}

func TestTaskDecompositionService_GetNotFound(t *testing.T) {
	provider := db.NewTestProvider(t)
	setupTestSchema(t, provider)
	svc := NewTaskDecompositionService(provider)
	ctx := context.Background()

	_, err := svc.GetTask(ctx, "nonexistent")
	if !errors.Is(err, ErrTaskNotFound) {
		t.Errorf("expected ErrTaskNotFound, got %v", err)
	}
}

func TestTaskDecompositionService_UpdateTask(t *testing.T) {
	provider := db.NewTestProvider(t)
	setupTestSchema(t, provider)
	svc := NewTaskDecompositionService(provider)
	ctx := context.Background()

	task := &TaskDecomposition{
		ID:             "test-id-2",
		OrganizationID: "org-2",
		Title:          "Old Title",
		Status:         "PENDING",
		Priority:       "P2",
	}
	err := svc.CreateTask(ctx, task)
	if err != nil {
		t.Fatalf("failed to create: %v", err)
	}

	task.Title = "New Title"
	task.Status = "CLAIMED"
	task.AssignedAgentID = ptrString("agent-x")

	err = svc.UpdateTask(ctx, task)
	if err != nil {
		t.Fatalf("failed to update: %v", err)
	}

	fetched, err := svc.GetTask(ctx, "test-id-2")
	if err != nil {
		t.Fatalf("failed to get: %v", err)
	}

	if fetched.Title != "New Title" || fetched.Status != "CLAIMED" || *fetched.AssignedAgentID != "agent-x" {
		t.Errorf("update did not persist correctly: %+v", fetched)
	}
}

func TestTaskDecompositionService_ClaimTask(t *testing.T) {
	provider := db.NewTestProvider(t)
	setupTestSchema(t, provider)
	svc := NewTaskDecompositionService(provider)
	ctx := context.Background()

	task := &TaskDecomposition{
		ID:             "test-id-3",
		OrganizationID: "org-3",
		Title:          "Claimable",
		Status:         "PENDING",
		Priority:       "P0",
	}
	_ = svc.CreateTask(ctx, task)

	claimed, err := svc.ClaimTask(ctx, "org-3", "agent-claim")
	if err != nil {
		t.Fatalf("failed to claim task: %v", err)
	}

	if claimed.Status != "CLAIMED" || *claimed.AssignedAgentID != "agent-claim" {
		t.Errorf("task not claimed correctly: %+v", claimed)
	}

	// Try claiming again, should return ErrTaskClaimed
	_, err = svc.ClaimTask(ctx, "org-3", "agent-claim2")
	if !errors.Is(err, ErrTaskClaimed) {
		t.Errorf("expected ErrTaskClaimed, got %v", err)
	}
}

func TestTaskDecompositionService_ClaimTaskConcurrency(t *testing.T) {
	provider := db.NewTestProvider(t)
	setupTestSchema(t, provider)
	svc := NewTaskDecompositionService(provider)
	ctx := context.Background()

	// Create 1 task
	task := &TaskDecomposition{
		ID:             "test-id-4",
		OrganizationID: "org-4",
		Title:          "Concurrent Claimable",
		Status:         "PENDING",
		Priority:       "P0",
	}
	_ = svc.CreateTask(ctx, task)

	var wg sync.WaitGroup
	successes := 0
	failures := 0
	var mu sync.Mutex

	for i := 0; i < 10; i++ {
		wg.Add(1)
		go func(agentID string) {
			defer wg.Done()
			_, err := svc.ClaimTask(context.Background(), "org-4", agentID)
			mu.Lock()
			if err == nil {
				successes++
			} else if errors.Is(err, ErrTaskClaimed) {
				failures++
			}
			mu.Unlock()
		}("agent-" + string(rune(i)))
	}
	wg.Wait()

	if successes != 1 {
		t.Errorf("expected exactly 1 successful claim, got %d", successes)
	}
	if failures != 9 {
		t.Errorf("expected exactly 9 failed claims, got %d", failures)
	}
}

func setupTestSchema(t *testing.T, provider db.Provider) {
	t.Helper()
	schema := `
	CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
		id VARCHAR PRIMARY KEY,
		organization_id VARCHAR NOT NULL,
		title VARCHAR NOT NULL,
		description TEXT,
		status VARCHAR NOT NULL DEFAULT 'PENDING',
		assigned_agent_id VARCHAR,
		priority VARCHAR NOT NULL DEFAULT 'P2',
		payload TEXT,
		parent_plan_id TEXT,
		dependencies TEXT NOT NULL DEFAULT '[]',
		locked_until TIMESTAMP,
		created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
		updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
	);`
	_, err := provider.Exec(context.Background(), schema)
	if err != nil {
		t.Fatalf("failed to setup schema: %v", err)
	}
}

// MockProvider forces IsSQLite() to return false to test the Postgres codepath.
type MockProvider struct {
	db.Provider
}

func (m *MockProvider) IsSQLite() bool {
	return false
}

// MockRow implements db.Row
type MockRow struct {
	scanErr error
	task    *TaskDecomposition
}

func (m *MockRow) Scan(dest ...any) error {
	if m.scanErr != nil {
		return m.scanErr
	}
	*dest[0].(*string) = m.task.ID
	*dest[1].(*string) = m.task.OrganizationID
	*dest[2].(*string) = m.task.Title
	*dest[3].(**string) = m.task.Description
	*dest[4].(*string) = m.task.Status
	*dest[5].(**string) = m.task.AssignedAgentID
	*dest[6].(*string) = m.task.Priority
	*dest[7].(**string) = m.task.Payload
	*dest[8].(**string) = m.task.ParentPlanID
	*dest[9].(*string) = m.task.Dependencies
	// ignore timestamps for mock
	return nil
}

func (m *MockProvider) QueryRow(ctx context.Context, sql string, args ...any) db.Row {
	// For testing claimTaskPostgres
	task := &TaskDecomposition{
		ID:             "pg-task-1",
		OrganizationID: args[2].(string),
		Title:          "PG Task",
		Status:         "CLAIMED",
		AssignedAgentID: ptrString(args[0].(string)),
		Priority:       "P1",
	}
	return &MockRow{task: task}
}

func TestTaskDecompositionService_ClaimTaskPostgres(t *testing.T) {
	provider := &MockProvider{}
	svc := NewTaskDecompositionService(provider)
	ctx := context.Background()

	claimed, err := svc.ClaimTask(ctx, "org-pg", "agent-pg")
	if err != nil {
		t.Fatalf("failed to claim task via postgres mock: %v", err)
	}

	if claimed.Status != "CLAIMED" || *claimed.AssignedAgentID != "agent-pg" || claimed.ID != "pg-task-1" {
		t.Errorf("postgres mock task not claimed correctly: %+v", claimed)
	}
}

func TestTaskDecompositionService_Transitions(t *testing.T) {
	provider := db.NewTestProvider(t)
	setupTestSchema(t, provider)
	svc := NewTaskDecompositionService(provider)
	ctx := context.Background()

	task := &TaskDecomposition{
		ID:             "test-id-5",
		OrganizationID: "org-5",
		Title:          "Transitionable",
		Status:         "CLAIMED",
		Priority:       "P0",
	}
	_ = svc.CreateTask(ctx, task)

	err := svc.MarkTaskDone(ctx, "test-id-5")
	if err != nil {
		t.Fatalf("failed to mark done: %v", err)
	}
	fetched, _ := svc.GetTask(ctx, "test-id-5")
	if fetched.Status != "DONE" {
		t.Errorf("expected DONE, got %v", fetched.Status)
	}

	err = svc.MarkTaskFailed(ctx, "test-id-5")
	if err != nil {
		t.Fatalf("failed to mark failed: %v", err)
	}
	fetched, _ = svc.GetTask(ctx, "test-id-5")
	if fetched.Status != "FAILED" {
		t.Errorf("expected FAILED, got %v", fetched.Status)
	}
}

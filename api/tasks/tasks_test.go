package tasks

import (
	"database/sql"
	"fmt"
	"testing"

	_ "github.com/mattn/go-sqlite3" // Use sqlite for testing
)

func setupDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open sqlite db: %v", err)
	}

	createTableQuery := `
	CREATE TABLE shared_task_list (
		id TEXT PRIMARY KEY,
		tenant_id TEXT NOT NULL,
		epic_id TEXT,
		title TEXT NOT NULL,
		description TEXT,
		priority TEXT NOT NULL DEFAULT 'P2',
		status TEXT NOT NULL DEFAULT 'PENDING',
		assigned_agent_id TEXT,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
		updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);`

	_, err = db.Exec(createTableQuery)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	return db
}

func TestTaskDAO(t *testing.T) {
	db := setupDB(t)
	defer db.Close()

	dao := NewTaskDAO(db, false)

	// Test Rebind
	pgDao := NewTaskDAO(db, true)
	if impl, ok := pgDao.(*taskDAOImpl); ok {
		q := impl.rebind("SELECT * FROM t WHERE a = ? AND b = ?")
		if q != "SELECT * FROM t WHERE a = $1 AND b = $2" {
			t.Errorf("Unexpected rebind result: %s", q)
		}
		q2 := impl.rebind("SELECT * FROM t")
		if q2 != "SELECT * FROM t" {
			t.Errorf("Unexpected rebind result: %s", q2)
		}
	} else {
		t.Errorf("Could not cast to impl")
	}

	// Test Create
	tenant1 := "tenant1"
	task1 := &SharedTask{
		ID:       "t1",
		TenantID: tenant1,
		Title:    "First Task",
		Priority: "P1",
		Status:   "PENDING",
	}

	err := dao.CreateTask(task1)
	if err != nil {
		t.Fatalf("Failed to create task: %v", err)
	}

	// Test cross-tenant leakage for Get
	tenant2 := "tenant2"
	_, err = dao.GetTask(tenant2, "t1")
	if err == nil {
		t.Errorf("Expected not found error across tenants")
	}

	// Test Get
	gotTask, err := dao.GetTask(tenant1, "t1")
	if err != nil {
		t.Fatalf("Failed to get task: %v", err)
	}
	if gotTask.Title != task1.Title {
		t.Errorf("Expected title %s, got %s", task1.Title, gotTask.Title)
	}

	// Test Get NotFound
	_, err = dao.GetTask(tenant1, "t99")
	if err == nil {
		t.Errorf("Expected not found error")
	}

	// Test Update
	gotTask.Status = "IN_PROGRESS"
	err = dao.UpdateTask(gotTask)
	if err != nil {
		t.Fatalf("Failed to update task: %v", err)
	}

	updatedTask, _ := dao.GetTask(tenant1, "t1")
	if updatedTask.Status != "IN_PROGRESS" {
		t.Errorf("Expected IN_PROGRESS, got %s", updatedTask.Status)
	}

	// Create a task for tenant2 to check isolation
	dao.CreateTask(&SharedTask{
		ID:       "t_tenant2",
		TenantID: tenant2,
		Title:    "Tenant 2 Task",
		Priority: "P2",
		Status:   "PENDING",
	})

	// Test Create more for listing
	for i := 2; i <= 25; i++ {
		id := fmt.Sprintf("t%02d", i)
		dao.CreateTask(&SharedTask{
			ID:       id,
			TenantID: tenant1,
			Title:    fmt.Sprintf("Task %d", i),
			Priority: "P2",
			Status:   "PENDING",
		})
	}

	// Test List (Pagination) - default limit is 20
	tasksPage1, nextCursor, err := dao.ListTasks(tenant1, "", 0) // Should default to 20
	if err != nil {
		t.Fatalf("Failed to list tasks: %v", err)
	}
	if len(tasksPage1) != 20 {
		t.Errorf("Expected 20 tasks, got %d", len(tasksPage1))
	}
	if nextCursor != "t20" {
		t.Errorf("Expected nextCursor t20, got %s", nextCursor)
	}

	// Verify tenant isolation in list
	for _, tsk := range tasksPage1 {
		if tsk.TenantID != tenant1 {
			t.Errorf("Tenant leakage detected, got task from %s", tsk.TenantID)
		}
	}

	// Page 2
	tasksPage2, nextCursor2, err := dao.ListTasks(tenant1, nextCursor, 20)
	if err != nil {
		t.Fatalf("Failed to list tasks page 2: %v", err)
	}
	if len(tasksPage2) != 5 {
		t.Errorf("Expected 5 tasks on page 2, got %d", len(tasksPage2))
	}
	if nextCursor2 != "" {
		t.Errorf("Expected empty nextCursor on last page, got %s", nextCursor2)
	}

	// List for tenant2
	tasksT2, _, _ := dao.ListTasks(tenant2, "", 20)
	if len(tasksT2) != 1 {
		t.Errorf("Expected 1 task for tenant2, got %d", len(tasksT2))
	}

	// Test Delete cross-tenant
	err = dao.DeleteTask(tenant2, "t1") // shouldn't delete t1 from tenant1
	if err != nil {
		t.Fatalf("Failed to execute delete: %v", err)
	}
	_, err = dao.GetTask(tenant1, "t1")
	if err != nil {
		t.Errorf("Task should not have been deleted by another tenant")
	}

	// Test Delete
	err = dao.DeleteTask(tenant1, "t1")
	if err != nil {
		t.Fatalf("Failed to delete task: %v", err)
	}

	_, err = dao.GetTask(tenant1, "t1")
	if err == nil {
		t.Errorf("Expected task to be deleted")
	}
}

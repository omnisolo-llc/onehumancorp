package orchestration

import (
	"context"
	"testing"
	"strings"
	"bytes"
	"net/http"
	"net/http/httptest"
	"encoding/json"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestCircularDependencyCheck(t *testing.T) {
	ctx := context.Background()
	provider, _ := db.NewSQLiteProvider(":memory:")
	provider.Exec(ctx, "CREATE TABLE shared_tasks (id TEXT, organization_id TEXT, title TEXT, description TEXT, status TEXT, priority TEXT, payload TEXT)")
	provider.Exec(ctx, "CREATE TABLE task_dependencies (task_id TEXT, depends_on_task_id TEXT)")

	tm := NewTaskManager(provider, nil, nil, nil, nil)

	ctx = context.WithValue(ctx, auth.ContextKeyClaims, &auth.Claims{OrganizationID: "org1"})

	// Setup task A -> B
	provider.Exec(ctx, "INSERT INTO shared_tasks (id, organization_id) VALUES ('A', 'org1'), ('B', 'org1')")
	provider.Exec(ctx, "INSERT INTO task_dependencies (task_id, depends_on_task_id) VALUES ('A', 'B')")

	// Check if B -> A is allowed (should fail)
	err := tm.CheckCircularDependency(ctx, "B", []string{"A"})
	if err == nil {
		t.Errorf("Expected circular dependency error, got nil")
	}

	// Check if B -> C is allowed (should pass)
	err = tm.CheckCircularDependency(ctx, "B", []string{"C"})
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}

func TestDecomposeTaskAPI(t *testing.T) {
	ctx := context.Background()
	provider, _ := db.NewSQLiteProvider(":memory:")
	provider.Exec(ctx, `
		CREATE TABLE shared_tasks (
			id TEXT, organization_id TEXT, parent_plan_id TEXT, title TEXT, description TEXT, payload TEXT, status TEXT, priority TEXT, locked_until DATETIME, created_at DATETIME, updated_at DATETIME, ultraplan_phase TEXT, deliberation_log TEXT, depth INTEGER
		)
	`)
	provider.Exec(ctx, "CREATE TABLE task_dependencies (task_id TEXT, depends_on_task_id TEXT)")

	tm := NewTaskManager(provider, nil, nil, nil, nil)

	payload := `{
		"parent_task_id": "parent-1",
		"sub_tasks": [
			{"title": "Sub 1", "description": "Desc 1", "priority": "P1", "dependencies": []}
		]
	}`

	req, _ := http.NewRequest("POST", "/api/orchestration/tasks/decompose", bytes.NewBufferString(payload))
	req = req.WithContext(context.WithValue(req.Context(), auth.ContextKeyClaims, &auth.Claims{OrganizationID: "org1"}))

	rr := httptest.NewRecorder()
	handleDecomposeTask(rr, req, tm)

	if rr.Code != http.StatusCreated {
		t.Errorf("Expected status 201, got %d. Body: %s", rr.Code, rr.Body.String())
	}

	var tasks []*SharedTask
	json.Unmarshal(rr.Body.Bytes(), &tasks)

	if len(tasks) != 1 {
		t.Errorf("Expected 1 task, got %d", len(tasks))
	} else if tasks[0].ParentPlanID != "parent-1" {
		t.Errorf("Expected ParentPlanID to be 'parent-1', got '%s'", tasks[0].ParentPlanID)
	}
}

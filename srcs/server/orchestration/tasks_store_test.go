package orchestration

import (
	"testing"
	"github.com/onehumancorp/mono/srcs/server/db"
	"context"
	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestTaskStoreInterface(t *testing.T) {
	dbProv, _ := db.NewSqliteProvider("file::memory:?cache=shared")
	store := NewTaskStore(dbProv)
	if store == nil {
		t.Fatal("store is nil")
	}
}


func TestDecompositionTaskStore(t *testing.T) {
    dbProv, _ := db.NewSqliteProvider("file::memory:?cache=shared")
    store := NewDecompositionTaskStore(dbProv)

    ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-1"})

    // Setup
    dbProv.Exec(ctx, "CREATE TABLE shared_tasks_decomposition (id TEXT PRIMARY KEY, organization_id TEXT, title TEXT, description TEXT, status TEXT, assigned_agent_id TEXT, priority TEXT, payload TEXT, parent_plan_id TEXT, dependencies TEXT, locked_until DATETIME, created_at DATETIME, updated_at DATETIME)")
    dbProv.Exec(ctx, "INSERT INTO shared_tasks_decomposition (id, organization_id, title, status, priority, dependencies, created_at, updated_at) VALUES ('1', 'org-1', 'Test', 'PENDING', 'P2', '[]', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")

    task, err := store.ClaimTask(ctx, "agent-1")
    if err != nil { t.Fatalf("ClaimTask failed: %v", err) }
    if task == nil || task.Status != "IN_PROGRESS" || *task.AssignedAgentID != "agent-1" {
        t.Errorf("Unexpected task state: %+v", task)
    }

    err = store.TransitionTask(ctx, "1", "agent-1", "IN_PROGRESS", "DONE", "finished")
    if err != nil { t.Fatalf("TransitionTask failed: %v", err) }
}

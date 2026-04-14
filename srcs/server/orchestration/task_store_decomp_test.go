package orchestration

import (
    "database/sql"

    "context"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/auth"
    "github.com/onehumancorp/mono/srcs/server/db"
)

func TestClaimDecompositionTask(t *testing.T) {
    d, _ := sql.Open("sqlite", "file::memory:?cache=shared")
    dbProv := db.NewSqliteProvider(d)
    ctx := context.Background()
    db := db.DB{Provider: dbProv}
db.RunMigrations(ctx)

    ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, &auth.Claims{
        OrganizationID: "org1",
    })

    tx, _ := dbProv.Begin(ctx)
    tx.Exec(ctx, `CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
         id TEXT PRIMARY KEY,
         organization_id TEXT NOT NULL,
         title TEXT NOT NULL,
         description TEXT,
         status TEXT NOT NULL DEFAULT 'PENDING',
         assigned_agent_id TEXT,
         priority TEXT NOT NULL DEFAULT 'P2',
         payload TEXT,
         parent_plan_id TEXT,
         dependencies TEXT NOT NULL DEFAULT '[]',
         locked_until DATETIME,
         created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
         updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
     )`)
    tx.Exec(ctx, "INSERT INTO shared_tasks_decomposition (id, organization_id, title) VALUES ('t1', 'org1', 'title')")
    tx.Commit(ctx)

    store := NewSharedTaskOrchestrator(dbProv)
    task, err := store.ClaimDecompositionTask(ctx, "agent1")
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if task == nil {
        t.Fatalf("expected task but got nil")
    }
    if task.ID != "t1" {
        t.Errorf("expected task ID t1, got %s", task.ID)
    }
    if task.Status != "IN_PROGRESS" {
        t.Errorf("expected IN_PROGRESS, got %s", task.Status)
    }

    task2, err := store.ClaimDecompositionTask(ctx, "agent2")
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if task2 != nil {
        t.Fatalf("expected nil task, got %v", task2)
    }
}

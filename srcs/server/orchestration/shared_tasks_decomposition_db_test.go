package orchestration

import (
    "database/sql"
    _ "modernc.org/sqlite"
    "context"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/auth"
    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestSharedTasksDecompositionRepository_ClaimTask(t *testing.T) {
    telemetry.InitTelemetry()
    import_sql_db, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("failed to open test sqlite db: %v", err)
    }
    defer import_sql_db.Close()
    dbProvider := db.NewSqliteProvider(import_sql_db)

    ctx := context.Background()
    _, err = dbProvider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
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
        )
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }
    _, _ = dbProvider.Exec(ctx, "INSERT INTO shared_tasks_decomposition (id, organization_id, title, status, dependencies) VALUES ('t1', 'org1', 't1', 'DONE', '[]')")
    _, _ = dbProvider.Exec(ctx, "INSERT INTO shared_tasks_decomposition (id, organization_id, title, status, dependencies) VALUES ('t2', 'org1', 't2', 'PENDING', '[\"t1\"]')")

    repo := NewSharedTasksDecompositionRepository(dbProvider)
    claims := &auth.Claims{OrganizationID: "org1"}
    ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

    task, err := repo.ClaimTask(ctxWithClaims, "agent1")
    if err != nil {
        t.Fatalf("ClaimTask failed: %v", err)
    }
    if task == nil || task.ID != "t2" {
        t.Fatalf("expected t2 to be claimed")
    }
}

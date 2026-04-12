package orchestration

import (
    "bytes"
    "context"
    "database/sql"
    "encoding/json"
    "net/http"
    "net/http/httptest"
    "testing"

    "github.com/onehumancorp/mono/srcs/server/auth"
    "github.com/onehumancorp/mono/srcs/server/db"
    _ "modernc.org/sqlite"
)

func TestHandlers_CreateSharedTask(t *testing.T) {
    sqliteDB, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("Failed to open sqlite: %v", err)
    }
    provider := &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}

    // Ensure schema exists matches the schema verified in traces
    _, err = provider.Exec(context.Background(), `
        CREATE TABLE IF NOT EXISTS shared_tasks (
            id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
            organization_id VARCHAR NOT NULL,
            title VARCHAR NOT NULL,
            description TEXT,
            status VARCHAR NOT NULL DEFAULT 'PENDING',
            agent_id VARCHAR,
            priority VARCHAR NOT NULL DEFAULT 'P2',
            payload JSONB,
            locked_until TIMESTAMP,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        ALTER TABLE shared_tasks ADD COLUMN parent_plan_id TEXT;
        ALTER TABLE shared_tasks ADD COLUMN dependencies JSONB NOT NULL DEFAULT '[]';
    `)
    if err != nil {
        t.Fatalf("Failed to create schema: %v", err)
    }

    h := NewHandlers(provider)

    payload := SharedTask{
        Title:       "Test Task",
        Description: "Test Description",
        Priority:    "P1",
    }
    body, _ := json.Marshal(payload)
    req := httptest.NewRequest("POST", "/tasks", bytes.NewBuffer(body))

    // Unauthorized
    rr := httptest.NewRecorder()
    h.CreateSharedTask(rr, req)
    if rr.Code != http.StatusUnauthorized {
        t.Errorf("Expected 401, got %d", rr.Code)
    }

    // Authorized
    claims := &auth.Claims{OrganizationID: "tenant-1"}
    ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, claims)
    req = req.WithContext(ctx)

    rr = httptest.NewRecorder()
    h.CreateSharedTask(rr, req)
    if rr.Code != http.StatusOK {
        t.Errorf("Expected 200, got %d: %s", rr.Code, rr.Body.String())
    }

    var resp SharedTask
    json.Unmarshal(rr.Body.Bytes(), &resp)
    if resp.OrganizationID != "tenant-1" {
        t.Errorf("Expected tenant-1, got %s", resp.OrganizationID)
    }
}

func TestHandlers_ListSharedTasks(t *testing.T) {
    sqliteDB, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("Failed to open sqlite: %v", err)
    }
    provider := &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}

    // Ensure schema exists matches the schema verified in traces
    _, err = provider.Exec(context.Background(), `
        CREATE TABLE IF NOT EXISTS shared_tasks (
            id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
            organization_id VARCHAR NOT NULL,
            title VARCHAR NOT NULL,
            description TEXT,
            status VARCHAR NOT NULL DEFAULT 'PENDING',
            agent_id VARCHAR,
            priority VARCHAR NOT NULL DEFAULT 'P2',
            payload JSONB,
            locked_until TIMESTAMP,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        ALTER TABLE shared_tasks ADD COLUMN parent_plan_id TEXT;
        ALTER TABLE shared_tasks ADD COLUMN dependencies JSONB NOT NULL DEFAULT '[]';
    `)
    if err != nil {
        t.Fatalf("Failed to create schema: %v", err)
    }

    h := NewHandlers(provider)

    claims := &auth.Claims{OrganizationID: "tenant-1"}
    ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

    // Create dummy tasks
    provider.Exec(ctx, `INSERT INTO shared_tasks (organization_id, title, description, status, priority, payload, parent_plan_id, dependencies) VALUES ('tenant-1', 'Test Title', 'Test Description', 'PENDING', 'P1', '{}', 'plan-1', '[]')`)
    provider.Exec(ctx, `INSERT INTO shared_tasks (organization_id, title, description, status, priority, payload, parent_plan_id, dependencies) VALUES ('tenant-2', 'Test Title 2', 'Test Description 2', 'PENDING', 'P2', '{}', 'plan-2', '[]')`)

    req := httptest.NewRequest("GET", "/tasks", nil)
    req = req.WithContext(ctx)

    rr := httptest.NewRecorder()
    h.ListSharedTasks(rr, req)

    if rr.Code != http.StatusOK {
        t.Errorf("Expected 200, got %d", rr.Code)
    }

    var tasks []SharedTask
    json.Unmarshal(rr.Body.Bytes(), &tasks)
    if len(tasks) != 1 {
        t.Fatalf("Expected 1 task for tenant-1, got %d", len(tasks))
    }
    if tasks[0].OrganizationID != "tenant-1" {
        t.Errorf("Expected tenant-1 task, got %s", tasks[0].OrganizationID)
    }
}

func TestHandlers_UpdateSharedTask(t *testing.T) {
    sqliteDB, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("Failed to open sqlite: %v", err)
    }
    provider := &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}

    // Ensure schema exists matches the schema verified in traces
    _, err = provider.Exec(context.Background(), `
        CREATE TABLE IF NOT EXISTS shared_tasks (
            id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
            organization_id VARCHAR NOT NULL,
            title VARCHAR NOT NULL,
            description TEXT,
            status VARCHAR NOT NULL DEFAULT 'PENDING',
            agent_id VARCHAR,
            priority VARCHAR NOT NULL DEFAULT 'P2',
            payload JSONB,
            locked_until TIMESTAMP,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        ALTER TABLE shared_tasks ADD COLUMN parent_plan_id TEXT;
        ALTER TABLE shared_tasks ADD COLUMN dependencies JSONB NOT NULL DEFAULT '[]';
    `)
    if err != nil {
        t.Fatalf("Failed to create schema: %v", err)
    }

    h := NewHandlers(provider)
    claims := &auth.Claims{OrganizationID: "tenant-1"}
    ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

    // Create a task
    res, _ := provider.Exec(ctx, `INSERT INTO shared_tasks (id, organization_id, title, status, priority, dependencies) VALUES ('task-1', 'tenant-1', 'Test Title', 'PENDING', 'P1', '[]')`)
    _ = res

    updatePayload := map[string]string{
        "status": "COMPLETED",
        "agent_id": "agent-xyz",
    }
    body, _ := json.Marshal(updatePayload)
    req := httptest.NewRequest("PUT", "/tasks?id=task-1", bytes.NewBuffer(body))
    req = req.WithContext(ctx)

    rr := httptest.NewRecorder()
    h.UpdateSharedTask(rr, req)

    if rr.Code != http.StatusOK {
        t.Errorf("Expected 200, got %d", rr.Code)
    }

    // Verify update
    var status, agent string
    err = provider.QueryRow(ctx, "SELECT status, agent_id FROM shared_tasks WHERE id = 'task-1'").Scan(&status, &agent)
    if err != nil {
        t.Fatalf("Failed to query updated task: %v", err)
    }
    if status != "COMPLETED" || agent != "agent-xyz" {
        t.Errorf("Expected COMPLETED and agent-xyz, got %s and %s", status, agent)
    }
}

func TestHandlers_LockSharedTask(t *testing.T) {
    sqliteDB, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        t.Fatalf("Failed to open sqlite: %v", err)
    }
    provider := &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}

    // Ensure schema exists matches the schema verified in traces
    _, err = provider.Exec(context.Background(), `
        CREATE TABLE IF NOT EXISTS shared_tasks (
            id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
            organization_id VARCHAR NOT NULL,
            title VARCHAR NOT NULL,
            description TEXT,
            status VARCHAR NOT NULL DEFAULT 'PENDING',
            agent_id VARCHAR,
            priority VARCHAR NOT NULL DEFAULT 'P2',
            payload JSONB,
            locked_until TIMESTAMP,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        ALTER TABLE shared_tasks ADD COLUMN parent_plan_id TEXT;
        ALTER TABLE shared_tasks ADD COLUMN dependencies JSONB NOT NULL DEFAULT '[]';
    `)
    if err != nil {
        t.Fatalf("Failed to create schema: %v", err)
    }

    h := NewHandlers(provider)
    claims := &auth.Claims{OrganizationID: "tenant-1"}
    ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

    // Create a task
    provider.Exec(ctx, `INSERT INTO shared_tasks (id, organization_id, title, status, priority, dependencies) VALUES ('task-2', 'tenant-1', 'Test Title', 'PENDING', 'P1', '[]')`)

    req := httptest.NewRequest("POST", "/tasks/lock?id=task-2&agent_id=agent-abc", nil)
    req = req.WithContext(ctx)

    rr := httptest.NewRecorder()
    h.LockSharedTask(rr, req)

    if rr.Code != http.StatusOK {
        t.Errorf("Expected 200, got %d", rr.Code)
    }

    // Verify lock
    var status, agent string
    provider.QueryRow(ctx, "SELECT status, agent_id FROM shared_tasks WHERE id = 'task-2'").Scan(&status, &agent)
    if status != "IN_PROGRESS" || agent != "agent-abc" {
        t.Errorf("Expected IN_PROGRESS and agent-abc, got %s and %s", status, agent)
    }
}

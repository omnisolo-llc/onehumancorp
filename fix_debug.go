package main

import (
    "fmt"
    "database/sql"
    _ "modernc.org/sqlite"
)

func main() {
    sqliteDB, err := sql.Open("sqlite", ":memory:")
    if err != nil {
        panic(err)
    }

    _, err = sqliteDB.Exec(`
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
        panic(err)
    }

    _, err = sqliteDB.Exec(`INSERT INTO shared_tasks (organization_id, title, description, status, priority, payload, parent_plan_id, dependencies) VALUES ('tenant-1', 'Test Title', 'Test Description', 'PENDING', 'P1', '{}', 'plan-1', '[]')`)
    if err != nil {
        panic(err)
    }

    rows, err := sqliteDB.Query(`SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at FROM shared_tasks WHERE organization_id = 'tenant-1'`)
    if err != nil {
        panic(err)
    }
    defer rows.Close()

    count := 0
    for rows.Next() {
        var id, org, title, desc, status, priority, payload, parentPlanID string
        var agentID sql.NullString
        var deps []byte
        var created, updated sql.NullTime
        err := rows.Scan(&id, &org, &title, &desc, &status, &agentID, &priority, &payload, &parentPlanID, &deps, &created, &updated)
        if err != nil {
            fmt.Println("Scan error:", err)
        } else {
            count++
        }
    }
    fmt.Println("Count:", count)
}

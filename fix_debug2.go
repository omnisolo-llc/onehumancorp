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

    for rows.Next() {
        var id, org, title, desc, status, agent, priority, payload, parentPlanID string
        var depsJSON string // Using string for scan due to possible modernc.org/sqlite issue with []byte
        var created_at, updated_at string

        err := rows.Scan(&id, &org, &title, &desc, &status, &agent, &priority, &payload, &parentPlanID, &depsJSON, &created_at, &updated_at)
        if err != nil {
            fmt.Println("Scan error string:", err)
        } else {
            fmt.Println("Success string")
        }
    }
}

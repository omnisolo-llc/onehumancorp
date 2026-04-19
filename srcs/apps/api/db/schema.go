package db

import (
	"database/sql"
	"fmt"
)

func CreateSchema(db *sql.DB, isSQLite bool) error {
	var sharedTasksQuery, agentMeshMessagesQuery string

	if isSQLite {
		sharedTasksQuery = `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			parent_id TEXT REFERENCES shared_tasks(id),
			epic_id TEXT,
			title TEXT NOT NULL,
			status TEXT DEFAULT 'PENDING',
			assigned_agent TEXT,
			payload TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);`
		agentMeshMessagesQuery = `
		CREATE TABLE IF NOT EXISTS agent_mesh_messages (
			id TEXT PRIMARY KEY,
			sender TEXT NOT NULL,
			recipient TEXT,
			channel TEXT NOT NULL,
			content TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);`
	} else {
		sharedTasksQuery = `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id UUID PRIMARY KEY,
			parent_id UUID REFERENCES shared_tasks(id),
			epic_id VARCHAR(255),
			title VARCHAR(255) NOT NULL,
			status VARCHAR(50) DEFAULT 'PENDING',
			assigned_agent VARCHAR(255),
			payload JSONB,
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);`
		agentMeshMessagesQuery = `
		CREATE TABLE IF NOT EXISTS agent_mesh_messages (
			id UUID PRIMARY KEY,
			sender VARCHAR(255) NOT NULL,
			recipient VARCHAR(255),
			channel VARCHAR(100) NOT NULL,
			content JSONB NOT NULL,
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);`
	}

	if _, err := db.Exec(sharedTasksQuery); err != nil {
		return fmt.Errorf("failed to create shared_tasks table: %w", err)
	}

	if _, err := db.Exec(agentMeshMessagesQuery); err != nil {
		return fmt.Errorf("failed to create agent_mesh_messages table: %w", err)
	}

	return nil
}

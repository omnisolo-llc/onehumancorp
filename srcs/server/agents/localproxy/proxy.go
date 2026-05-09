package localproxy

import (
	"context"
	"database/sql"
	"fmt"
	"log"
	"time"

	"onehumancorp/srcs/server/agents/sandbox"
	"github.com/google/uuid"
)

type LocalExecutionProxy struct {
	db      *sql.DB
	sandbox *sandbox.SandboxManager
}

func NewLocalExecutionProxy(db *sql.DB, sm *sandbox.SandboxManager) (*LocalExecutionProxy, error) {
	_, err := db.Exec(`CREATE TABLE IF NOT EXISTS execution_logs (
		id TEXT PRIMARY KEY,
		command TEXT,
		stdout TEXT,
		stderr TEXT,
		executed_at DATETIME
	)`)
	if err != nil {
		return nil, fmt.Errorf("failed to create execution_logs table: %w", err)
	}

	return &LocalExecutionProxy{
		db:      db,
		sandbox: sm,
	}, nil
}

func (p *LocalExecutionProxy) ExecuteTerminal(ctx context.Context, command string) (string, string, error) {
	stdout, stderr, err := p.sandbox.Execute(ctx, command)

	// Sync to local database
	id := uuid.New().String()
	_, dbErr := p.db.ExecContext(ctx,
		"INSERT INTO execution_logs (id, command, stdout, stderr, executed_at) VALUES (?, ?, ?, ?, ?)",
		id, command, stdout, stderr, time.Now(),
	)
	if dbErr != nil {
		log.Printf("failed to sync execution to local database: %v", dbErr)
		// We still return the shell output and original err since execution happened
	}

	return stdout, stderr, err
}

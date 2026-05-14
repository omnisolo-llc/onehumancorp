package db

import (
	"context"
	"database/sql"
	"errors"
	"os"
	"time"

	"github.com/redis/go-redis/v9"
)

// TenantContextKey is a custom type to prevent context key collisions.
type TenantContextKey string

const TenantKey TenantContextKey = "tenant_id"

type Provider struct {
	DB          *sql.DB
	RedisClient *redis.Client
}

func (p *Provider) IsSQLite() bool {
	return os.Getenv("OHC_STANDALONE") == "true"
}

func (p *Provider) CreateTask(ctx context.Context, task *Task) error {
	if p.DB == nil {
		return errors.New("db connection is nil")
	}

	tenantID, ok := ctx.Value(TenantKey).(string)
	if !ok || tenantID == "" {
		return errors.New("missing tenant_id in context")
	}

	// Handle SQLite differences
	if p.IsSQLite() {
		query := `
			INSERT INTO tasks (id, tenant_id, status, created_at, updated_at)
			VALUES (?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
		`
		_, err := p.DB.ExecContext(ctx, query, task.ID, tenantID, task.Status)
		return err
	}

	// Postgres mode: Requires Tx to set RLS variable
	tx, err := p.DB.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	_, err = tx.ExecContext(ctx, "SELECT set_config('app.current_tenant', $1, true)", tenantID)
	if err != nil {
		return err
	}

	query := `
		INSERT INTO tasks (id, tenant_id, status, created_at, updated_at)
		VALUES ($1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
		RETURNING created_at, updated_at
	`
	err = tx.QueryRowContext(ctx, query, task.ID, tenantID, task.Status).Scan(&task.CreatedAt, &task.UpdatedAt)
	if err != nil {
		return err
	}

	return tx.Commit()
}

func (p *Provider) ClaimTask(ctx context.Context, taskID string) error {
	if p.DB == nil {
		return errors.New("db connection is nil")
	}

	tenantID, ok := ctx.Value(TenantKey).(string)
	if !ok || tenantID == "" {
		return errors.New("missing tenant_id in context")
	}

	if p.IsSQLite() {
		// Standalone mode: optimistic concurrency with simple SELECT + UPDATE
		// Check if it's pending
		var status string

		queryCheck := "SELECT status FROM tasks WHERE id = ? AND tenant_id = ?"
		err := p.DB.QueryRowContext(ctx, queryCheck, taskID, tenantID).Scan(&status)
		if err != nil {
			return err
		}
		if status != "PENDING" {
			return errors.New("task already claimed or completed")
		}

		queryUpdate := "UPDATE tasks SET status = 'IN_PROGRESS', updated_at = CURRENT_TIMESTAMP WHERE id = ? AND status = 'PENDING' AND tenant_id = ?"
		res, err := p.DB.ExecContext(ctx, queryUpdate, taskID, tenantID)
		if err != nil {
			return err
		}
		rowsAffected, err := res.RowsAffected()
		if err != nil {
			return err
		}
		if rowsAffected == 0 {
			return errors.New("failed to claim task: concurrent modification")
		}
		return nil
	}

	// Cloud mode: distributed lock with Redis
	if p.RedisClient != nil {
		lockKey := "task_lock:" + taskID
		// Try to acquire lock for 30 seconds
		acquired, err := p.RedisClient.SetNX(ctx, lockKey, "locked", 30*time.Second).Result()
		if err != nil {
			return err
		}
		if !acquired {
			return errors.New("could not acquire distributed lock for task")
		}

		// Ensure we release the lock when done
		defer p.RedisClient.Del(ctx, lockKey)
	}

	// Double check the status using FOR UPDATE SKIP LOCKED
	tx, err := p.DB.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	_, err = tx.ExecContext(ctx, "SELECT set_config('app.current_tenant', $1, true)", tenantID)
	if err != nil {
		return err
	}

	var status string
	queryCheckCloud := "SELECT status FROM tasks WHERE id = $1 AND tenant_id = $2 FOR UPDATE SKIP LOCKED"

	err = tx.QueryRowContext(ctx, queryCheckCloud, taskID, tenantID).Scan(&status)
	if err != nil {
		if err == sql.ErrNoRows {
			return errors.New("task not found or locked by another transaction")
		}
		return err
	}

	if status != "PENDING" {
		return errors.New("task already claimed or completed")
	}

	queryUpdateCloud := "UPDATE tasks SET status = 'IN_PROGRESS', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2"
	_, err = tx.ExecContext(ctx, queryUpdateCloud, taskID, tenantID)
	if err != nil {
		return err
	}

	return tx.Commit()
}

var GlobalProvider = &Provider{}

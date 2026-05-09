package db

import (
	"context"
	"database/sql"
	"errors"
	"os"
	"time"

	"github.com/redis/go-redis/v9"
)

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

	query := `
		INSERT INTO tasks (id, status, created_at, updated_at)
		VALUES ($1, $2, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
		RETURNING created_at, updated_at
	`
	// Handle SQLite differences
	if p.IsSQLite() {
		query = `
			INSERT INTO tasks (id, status, created_at, updated_at)
			VALUES (?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
		`
		_, err := p.DB.ExecContext(ctx, query, task.ID, task.Status)
		return err
	}

	return p.DB.QueryRowContext(ctx, query, task.ID, task.Status).Scan(&task.CreatedAt, &task.UpdatedAt)
}

func (p *Provider) ClaimTask(ctx context.Context, taskID string) error {
	if p.DB == nil {
		return errors.New("db connection is nil")
	}

	if p.IsSQLite() {
		// Standalone mode: optimistic concurrency with simple SELECT + UPDATE
		// Check if it's pending
		var status string
		err := p.DB.QueryRowContext(ctx, "SELECT status FROM tasks WHERE id = ?", taskID).Scan(&status)
		if err != nil {
			return err
		}
		if status != "PENDING" {
			return errors.New("task already claimed or completed")
		}

		var updatedID string
		err = p.DB.QueryRowContext(ctx, "UPDATE tasks SET status = 'IN_PROGRESS', updated_at = CURRENT_TIMESTAMP WHERE id = ? AND status = 'PENDING' RETURNING id", taskID).Scan(&updatedID)
		if err != nil {
			if err == sql.ErrNoRows {
				return errors.New("failed to claim task: concurrent modification")
			}
			return err
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

	var status string
	err = tx.QueryRowContext(ctx, "SELECT status FROM tasks WHERE id = $1 FOR UPDATE SKIP LOCKED", taskID).Scan(&status)
	if err != nil {
		if err == sql.ErrNoRows {
			return errors.New("task not found or locked by another transaction")
		}
		return err
	}

	if status != "PENDING" {
		return errors.New("task already claimed or completed")
	}

	_, err = tx.ExecContext(ctx, "UPDATE tasks SET status = 'IN_PROGRESS', updated_at = CURRENT_TIMESTAMP WHERE id = $1", taskID)
	if err != nil {
		return err
	}

	return tx.Commit()
}

var GlobalProvider = &Provider{}

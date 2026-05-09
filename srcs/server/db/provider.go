package db

import (
	"context"
	"database/sql"
	"errors"
	"os"
	"time"

	"github.com/redis/go-redis/v9"
)

type TaskProvider interface {
	IsSQLite() bool
	CreateTask(ctx context.Context, task *Task) error
	ClaimTask(ctx context.Context, taskID string) error
}

type Provider struct {
	DB          *sql.DB
	RedisClient *redis.Client
}

func (p *Provider) getImpl() TaskProvider {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return &SqliteProvider{DB: p.DB}
	}
	return &PostgresProvider{DB: p.DB, RedisClient: p.RedisClient}
}

func (p *Provider) IsSQLite() bool {
	return os.Getenv("OHC_STANDALONE") == "true"
}

func (p *Provider) CreateTask(ctx context.Context, task *Task) error {
	return p.getImpl().CreateTask(ctx, task)
}

func (p *Provider) ClaimTask(ctx context.Context, taskID string) error {
	return p.getImpl().ClaimTask(ctx, taskID)
}

var GlobalProvider = &Provider{}

type SqliteProvider struct {
	DB *sql.DB
}

func (p *SqliteProvider) IsSQLite() bool {
	return true
}

func (p *SqliteProvider) CreateTask(ctx context.Context, task *Task) error {
	if p.DB == nil {
		return errors.New("db connection is nil")
	}

	query := `
		INSERT INTO tasks (id, status, created_at, updated_at)
		VALUES (?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
	`
	_, err := p.DB.ExecContext(ctx, query, task.ID, task.Status)
	return err
}

func (p *SqliteProvider) ClaimTask(ctx context.Context, taskID string) error {
	if p.DB == nil {
		return errors.New("db connection is nil")
	}

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

	res, err := p.DB.ExecContext(ctx, "UPDATE tasks SET status = 'IN_PROGRESS', updated_at = CURRENT_TIMESTAMP WHERE id = ? AND status = 'PENDING'", taskID)
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

type PostgresProvider struct {
	DB          *sql.DB
	RedisClient *redis.Client
}

func (p *PostgresProvider) IsSQLite() bool {
	return false
}

func (p *PostgresProvider) CreateTask(ctx context.Context, task *Task) error {
	if p.DB == nil {
		return errors.New("db connection is nil")
	}

	query := `
		INSERT INTO tasks (id, status, created_at, updated_at)
		VALUES ($1, $2, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
		RETURNING created_at, updated_at
	`
	return p.DB.QueryRowContext(ctx, query, task.ID, task.Status).Scan(&task.CreatedAt, &task.UpdatedAt)
}

func (p *PostgresProvider) ClaimTask(ctx context.Context, taskID string) error {
	if p.DB == nil {
		return errors.New("db connection is nil")
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

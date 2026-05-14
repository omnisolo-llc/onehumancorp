package db

import (
	"context"
	"database/sql"
	"errors"
	"os"
	"time"

	"github.com/redis/go-redis/v9"
	_ "github.com/mattn/go-sqlite3"
)

type StandaloneProvider struct {
	DB *sql.DB
}

func (p *StandaloneProvider) IsSQLite() bool {
	return true
}

func (p *StandaloneProvider) CreateTask(ctx context.Context, task *Task) error {
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

func (p *StandaloneProvider) AutoDreamInsertQuery() string {
	return `
		INSERT INTO autodream_memories (id, organization_id, task_id, content, embedding, source_type, agent_id)
		VALUES (?, ?, ?, ?, ?, ?, ?)
	`
}

func (p *StandaloneProvider) SearchSimilarMemoriesQuery(orgID string, query string, embeddingBytes []byte, topK int) (string, []interface{}) {
	// Fallback to text-based recency logic in SQLite Standalone mode
	queryStr := `
		SELECT id, organization_id, task_id, content
		FROM consolidated_memory
		WHERE organization_id = ? AND content LIKE ?
		ORDER BY created_at DESC
		LIMIT ?
	`
	args := []interface{}{orgID, "%" + query + "%", topK}
	return queryStr, args
}

func (p *StandaloneProvider) ClaimTask(ctx context.Context, taskID string) error {
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

type CloudProvider struct {
	DB          *sql.DB
	RedisClient *redis.Client
}

func (p *CloudProvider) IsSQLite() bool {
	return false
}

func (p *CloudProvider) CreateTask(ctx context.Context, task *Task) error {
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

func (p *CloudProvider) AutoDreamInsertQuery() string {
	return `
		INSERT INTO autodream_memories (id, organization_id, task_id, content, embedding, source_type, agent_id)
		VALUES ($1, $2, $3, $4, $5, $6, $7)
	`
}

func (p *CloudProvider) SearchSimilarMemoriesQuery(orgID string, query string, embeddingBytes []byte, topK int) (string, []interface{}) {
	// Exact Nearest Neighbor search in Postgres Cloud mode
	queryStr := `
		SELECT id, organization_id, task_id, content
		FROM consolidated_memory
		WHERE organization_id = $1
		ORDER BY embedding <-> $2
		LIMIT $3
	`
	args := []interface{}{orgID, string(embeddingBytes), topK}
	return queryStr, args
}

func (p *CloudProvider) ClaimTask(ctx context.Context, taskID string) error {
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

var GlobalProvider TaskProvider = &CloudProvider{}

func NewSqliteProvider(db *sql.DB) TaskProvider {
	os.Setenv("OHC_STANDALONE", "true")
	p := &StandaloneProvider{
		DB: db,
	}
	GlobalProvider = p
	return p
}

func NewTestProvider(t interface{}) TaskProvider {
	os.Setenv("OHC_STANDALONE", "true")
	db, err := sql.Open("sqlite3", "file::memory:?cache=shared")
	if err != nil {
		panic(err)
	}
	// For testing, we also might need to create tables, but let the tests handle that.
	p := &StandaloneProvider{
		DB: db,
	}
	GlobalProvider = p
	return p
}

func InitCloudProvider(db *sql.DB, redisClient *redis.Client) TaskProvider {
	p := &CloudProvider{
		DB:          db,
		RedisClient: redisClient,
	}
	GlobalProvider = p
	return p
}

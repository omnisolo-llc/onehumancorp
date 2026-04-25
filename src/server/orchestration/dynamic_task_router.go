package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"log/slog"
	"sync"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/redis/go-redis/v9"
)

type DynamicTaskRouter struct {
	dbProvider  db.Provider
	redisClient *redis.Client
	mu          sync.Mutex
}

func NewDynamicTaskRouter(dbProvider db.Provider, redisClient *redis.Client) *DynamicTaskRouter {
	return &DynamicTaskRouter{
		dbProvider:  dbProvider,
		redisClient: redisClient,
	}
}

// BroadcastTaskAvailable emits a task.available event over the Teammate Mesh (Redis Pub/Sub).
func (r *DynamicTaskRouter) BroadcastTaskAvailable(ctx context.Context, taskID string, requirements string) error {
	if r.redisClient == nil {
		slog.Warn("DynamicTaskRouter: redisClient is nil, falling back to no-op broadcast")
		return nil
	}

	payload := map[string]interface{}{
		"event":        "task.available",
		"task_id":      taskID,
		"requirements": requirements,
		"timestamp":    time.Now().UTC().Format(time.RFC3339),
	}

	data, err := json.Marshal(payload)
	if err != nil {
		return fmt.Errorf("failed to marshal task.available event: %w", err)
	}

	// Broadcasting over the Teammate Mesh using the Redis implementation
	topic := "teammate_mesh"
	err = r.redisClient.Publish(ctx, topic, data).Err()
	if err != nil {
		return fmt.Errorf("failed to publish to %s: %w", topic, err)
	}

	slog.Info("DynamicTaskRouter: broadcast task.available", "task_id", taskID)
	return nil
}

// ClaimTask allows an agent to claim a task.
// Uses FOR UPDATE SKIP LOCKED in Postgres, or mu.Lock() in SQLite.
func (r *DynamicTaskRouter) ClaimTask(ctx context.Context, taskID string, agentID string) (bool, error) {
	if r.dbProvider.IsSQLite() {
		r.mu.Lock()
		defer r.mu.Unlock()
	}

	tx, err := r.dbProvider.Begin(ctx)
	if err != nil {
		return false, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	// We'll target shared_tasks.
	query := `SELECT id FROM shared_tasks WHERE id = $1 AND status = 'PENDING'`

	if !r.dbProvider.IsSQLite() {
		query += ` FOR UPDATE SKIP LOCKED`
	}

	var lockedTaskID string
	err = tx.QueryRow(ctx, query, taskID).Scan(&lockedTaskID)
	if err != nil {
		if err == sql.ErrNoRows {
			// Task might already be claimed or doesn't exist
			return false, nil
		}
		return false, fmt.Errorf("failed to acquire lock on task: %w", err)
	}

	updateQuery := `UPDATE shared_tasks SET status = 'IN_PROGRESS', assigned_agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
	res, err := tx.Exec(ctx, updateQuery, agentID, taskID)
	if err != nil {
		return false, fmt.Errorf("failed to update task status: %w", err)
	}

	rowsAffected, err := res.RowsAffected()
	if err != nil || rowsAffected == 0 {
		return false, fmt.Errorf("failed to update task status or no rows affected")
	}

	if err := tx.Commit(ctx); err != nil {
		return false, fmt.Errorf("failed to commit transaction: %w", err)
	}

	slog.Info("DynamicTaskRouter: successfully claimed task", "task_id", taskID, "agent_id", agentID)
	return true, nil
}

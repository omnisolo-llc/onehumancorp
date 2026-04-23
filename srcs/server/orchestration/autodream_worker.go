package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

// AutoDreamConsolidator is a background worker daemon that batches unprocessed
// episodic memories from autodream_memories and embeds them.
type AutoDreamConsolidator struct {
	db     db.Provider
	redis  rueidis.Client
	client MinimaxClient
	done   chan struct{}
}

// NewAutoDreamConsolidator creates a new daemon instance.
func NewAutoDreamConsolidator(provider db.Provider, redisClient rueidis.Client, client MinimaxClient) *AutoDreamConsolidator {
	return &AutoDreamConsolidator{
		db:     provider,
		redis:  redisClient,
		client: client,
		done:   make(chan struct{}),
	}
}

// Start begins the worker polling loop.
func (c *AutoDreamConsolidator) Start(ctx context.Context) {
	slog.Info("AutoDreamConsolidator: starting worker")
	ticker := time.NewTicker(5 * time.Minute)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-c.done:
			return
		case <-ticker.C:
			if err := c.Consolidate(context.Background()); err != nil {
				slog.Error("AutoDreamConsolidator: consolidation failed", "error", err)
			}
		}
	}
}

// Stop gracefully stops the daemon.
func (c *AutoDreamConsolidator) Stop() {
	close(c.done)
}

// Consolidate processes up to 100 unprocessed memories.
func (c *AutoDreamConsolidator) Consolidate(ctx context.Context) error {
	lockKey := "ohc:lock:system:autodream_consolidator:batch"
	lockValue := "locked"

	if c.redis != nil {
		// Acquire distributed lock
		acquireCmd := c.redis.B().Set().Key(lockKey).Value(lockValue).Nx().Ex(1 * time.Minute).Build()
		resp := c.redis.Do(ctx, acquireCmd)
		if err := resp.Error(); err != nil {
			if rueidis.IsRedisNil(err) {
				// Lock already held by another worker
				return nil
			}
			return fmt.Errorf("failed to acquire redis lock: %w", err)
		}

		defer func() {
			delCmd := c.redis.B().Del().Key(lockKey).Build()
			_ = c.redis.Do(ctx, delCmd)
		}()
	}

	tx, err := c.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	var query string
	if c.db.IsSQLite() {
		query = "SELECT id, content FROM autodream_memories WHERE processed_at IS NULL LIMIT 100"
	} else {
		query = "SELECT id, content FROM autodream_memories WHERE processed_at IS NULL LIMIT 100 FOR UPDATE SKIP LOCKED"
	}

	rows, err := tx.Query(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to fetch memories: %w", err)
	}

	type memEntry struct {
		id      string
		content string
	}
	var entries []memEntry

	for rows.Next() {
		var e memEntry
		if err := rows.Scan(&e.id, &e.content); err == nil {
			entries = append(entries, e)
		}
	}
	rows.Close()

	if len(entries) == 0 {
		return nil // Nothing to do
	}

	slog.Info("AutoDreamConsolidator: found unprocessed memories", "count", len(entries))

	for _, e := range entries {
		var embedding []float32
		if c.client != nil {
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, embedErr := c.client.GenerateEmbedding(ctxTimeout, e.content)
			cancel()
			if embedErr == nil && len(resp) == 1536 {
				embedding = resp
			} else {
				slog.Warn("AutoDreamConsolidator: failed to embed memory", "id", e.id, "error", embedErr)
				embedding = make([]float32, 1536)
			}
		} else {
			embedding = make([]float32, 1536)
		}

		var embStr string
		if c.db.IsSQLite() {
			embBytes, _ := json.Marshal(embedding)
			embStr = string(embBytes)
		} else {
			embStr = formatFloat32SliceForVector(embedding)
		}

		var updateQuery string
		if c.db.IsSQLite() {
			updateQuery = "UPDATE autodream_memories SET embedding = $1, processed_at = CURRENT_TIMESTAMP WHERE id = $2"
		} else {
			updateQuery = "UPDATE autodream_memories SET embedding = $1::vector, processed_at = CURRENT_TIMESTAMP WHERE id = $2"
		}

		if _, err := tx.Exec(ctx, updateQuery, embStr, e.id); err != nil {
			slog.Error("AutoDreamConsolidator: failed to update memory", "id", e.id, "error", err)
		} else {
			slog.Debug("AutoDreamConsolidator: successfully consolidated memory", "id", e.id)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	return nil
}

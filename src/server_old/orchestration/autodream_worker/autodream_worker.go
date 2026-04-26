package autodream_worker

import (
	"context"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/redis/rueidis"
)

type EmbeddingClient interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

type AutoDreamConsolidator struct {
	db          db.Provider
	redisClient rueidis.Client
	llmClient   EmbeddingClient
	batchSize   int
}

func NewAutoDreamConsolidator(dbProvider db.Provider, redisClient rueidis.Client, llmClient EmbeddingClient) *AutoDreamConsolidator {
	return &AutoDreamConsolidator{
		db:          dbProvider,
		redisClient: redisClient,
		llmClient:   llmClient,
		batchSize:   500,
	}
}

func (c *AutoDreamConsolidator) ProcessBacklog(ctx context.Context) error {
	slog.Info("AutoDreamConsolidator: waking up to process backlog")

	query := "SELECT id, content FROM autodream_memories WHERE processed_at IS NULL LIMIT $1 FOR UPDATE SKIP LOCKED"
	args := []interface{}{c.batchSize}

	if c.db.IsSQLite() {
		query = "SELECT id, content FROM autodream_memories WHERE processed_at IS NULL LIMIT ?"
	}

	rows, err := c.db.Query(ctx, query, args...)
	if err != nil {
		return fmt.Errorf("failed to query autodream_memories: %w", err)
	}
	defer rows.Close()

	type Memory struct {
		ID      string
		Content string
	}
	var memories []Memory
	for rows.Next() {
		var mem Memory
		if err := rows.Scan(&mem.ID, &mem.Content); err != nil {
			slog.Warn("AutoDreamConsolidator: failed to scan memory row", "error", err)
			continue
		}
		memories = append(memories, mem)
	}
	rows.Close() // close early

	if len(memories) == 0 {
		slog.Debug("AutoDreamConsolidator: no pending memories found")
		return nil
	}

	slog.Info("AutoDreamConsolidator: found pending memories", "count", len(memories))

	for _, mem := range memories {
		// Acquire distributed lock for this specific memory
		lockKey := fmt.Sprintf("ohc:lock:autodream_memory:%s", mem.ID)

		// Setup lock acquisition
		// Try to acquire lock via redis
		if c.redisClient != nil {
			acquireCmd := c.redisClient.B().Set().Key(lockKey).Value("locked").Nx().Ex(2 * time.Minute).Build()
			res := c.redisClient.Do(ctx, acquireCmd)
			if res.Error() != nil {
				// Lock not acquired or error, skip
				slog.Debug("AutoDreamConsolidator: skipping memory, lock not acquired", "id", mem.ID)
				continue
			}
		}

		func() {
			defer func() {
				if c.redisClient != nil {
					delCmd := c.redisClient.B().Del().Key(lockKey).Build()
					c.redisClient.Do(context.Background(), delCmd)
				}
			}()

			// Process embedding
			embedding, err := c.llmClient.GenerateEmbedding(ctx, mem.Content)
			if err != nil {
				slog.Error("AutoDreamConsolidator: failed to generate embedding", "id", mem.ID, "error", err)
				return
			}

			// Prepare embedding string
			// We format it simply as JSON array for PGVector if it's PG, or string for SQLite
			embeddingStr := ""
			if len(embedding) > 0 {
				embBytes := make([]byte, 0, len(embedding)*10)
				embBytes = append(embBytes, '[')
				for i, v := range embedding {
					if i > 0 {
						embBytes = append(embBytes, ',')
					}
					embBytes = append(embBytes, []byte(fmt.Sprintf("%f", v))...)
				}
				embBytes = append(embBytes, ']')
				embeddingStr = string(embBytes)
			}

			updateQuery := "UPDATE autodream_memories SET embedding = $1::vector, processed_at = NOW() WHERE id = $2"
			if c.db.IsSQLite() {
				updateQuery = "UPDATE autodream_memories SET embedding = ?, processed_at = CURRENT_TIMESTAMP WHERE id = ?"
			}

			if c.db.IsSQLite() {
				_, err = c.db.Exec(ctx, updateQuery, embeddingStr, mem.ID)
			} else {
				_, err = c.db.Exec(ctx, updateQuery, embeddingStr, mem.ID)
			}

			if err != nil {
				slog.Error("AutoDreamConsolidator: failed to update memory", "id", mem.ID, "error", err)
			} else {
				slog.Debug("AutoDreamConsolidator: successfully processed memory", "id", mem.ID)
			}
		}()
	}

	return nil
}

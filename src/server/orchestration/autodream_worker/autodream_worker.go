package autodream_worker

import (
	"context"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/lib/llm"
	"github.com/redis/rueidis"
)

type AutoDreamConsolidator struct {
	db          db.Provider
	redisClient rueidis.Client
	llmClient   llm.EmbeddingClient
	batchSize   int
}

func NewAutoDreamConsolidator(dbProvider db.Provider, redisClient rueidis.Client, llmClient llm.EmbeddingClient) *AutoDreamConsolidator {
	return &AutoDreamConsolidator{
		db:          dbProvider,
		redisClient: redisClient,
		llmClient:   llmClient,
		batchSize:   100,
	}
}

func (c *AutoDreamConsolidator) ProcessBacklog(ctx context.Context) error {
	slog.Info("AutoDreamConsolidator: waking up to process backlog")

	query := "SELECT id, content FROM autodream_memories WHERE processed_at IS NULL LIMIT $1"
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

func (c *AutoDreamConsolidator) ProcessCompletedTasks(ctx context.Context) error {
	slog.Info("AutoDreamConsolidator: waking up to process completed tasks")

	// 1. Fetch completed tasks that haven't been dreamed yet
	query := `
		SELECT t.id, t.organization_id, t.title, COALESCE(t.description, ''), COALESCE(t.payload, '{}')
		FROM shared_tasks_master t
		LEFT JOIN autodream_memories_master m ON t.id = m.source_task_id
		WHERE t.status = 'COMPLETED' AND m.id IS NULL
		LIMIT $1
	`
	args := []interface{}{c.batchSize}

	if c.db.IsSQLite() {
		query = `
			SELECT t.id, t.organization_id, t.title, COALESCE(t.description, ''), COALESCE(t.payload, '{}')
			FROM shared_tasks_master t
			LEFT JOIN autodream_memories_master m ON t.id = m.source_task_id
			WHERE t.status = 'COMPLETED' AND m.id IS NULL
			LIMIT ?
		`
	}

	rows, err := c.db.Query(ctx, query, args...)
	if err != nil {
		return fmt.Errorf("failed to query shared_tasks_master: %w", err)
	}
	defer rows.Close()

	type CompletedTask struct {
		ID             string
		OrganizationID string
		Title          string
		Description    string
		Payload        string
	}
	var tasks []CompletedTask
	for rows.Next() {
		var t CompletedTask
		if err := rows.Scan(&t.ID, &t.OrganizationID, &t.Title, &t.Description, &t.Payload); err != nil {
			slog.Warn("AutoDreamConsolidator: failed to scan task row", "error", err)
			continue
		}
		tasks = append(tasks, t)
	}
	rows.Close()

	if len(tasks) == 0 {
		slog.Debug("AutoDreamConsolidator: no pending completed tasks found")
		return nil
	}

	slog.Info("AutoDreamConsolidator: found pending completed tasks", "count", len(tasks))

	for _, task := range tasks {
		// Acquire distributed lock
		lockKey := fmt.Sprintf("ohc:lock:autodream_task:%s", task.ID)
		if c.redisClient != nil {
			acquireCmd := c.redisClient.B().Set().Key(lockKey).Value("locked").Nx().Ex(2 * time.Minute).Build()
			res := c.redisClient.Do(ctx, acquireCmd)
			if res.Error() != nil {
				slog.Debug("AutoDreamConsolidator: skipping task, lock not acquired", "id", task.ID)
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

			content := fmt.Sprintf("Task: %s\nDescription: %s\nPayload: %s", task.Title, task.Description, task.Payload)

			// Generate embedding
			embedding, err := c.llmClient.GenerateEmbedding(ctx, content)
			if err != nil {
				slog.Error("AutoDreamConsolidator: failed to generate embedding for task", "id", task.ID, "error", err)
				return
			}

			// Format embedding string
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

			memoryID := "mem_" + task.ID

			insertQuery := `
				INSERT INTO autodream_memories_master (id, organization_id, memory_type, content, embedding, source_task_id)
				VALUES ($1, $2, 'task', $3, $4::vector, $5)
			`
			if c.db.IsSQLite() {
				insertQuery = `
					INSERT INTO autodream_memories_master (id, organization_id, memory_type, content, embedding, source_task_id)
					VALUES (?, ?, 'task', ?, ?, ?)
				`
			}

			_, err = c.db.Exec(ctx, insertQuery, memoryID, task.OrganizationID, content, embeddingStr, task.ID)
			if err != nil {
				slog.Error("AutoDreamConsolidator: failed to insert completed task memory", "id", task.ID, "error", err)
			} else {
				slog.Debug("AutoDreamConsolidator: successfully processed completed task", "id", task.ID)
			}
		}()
	}

	return nil
}
// Re-trigger CI due to BuildBuddy timeout
// Retrying CI
// Triggering CI again for remote cache timeout
// Retrying CI
// Another retry
// Still retrying CI

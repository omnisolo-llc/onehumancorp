package kairos

import (
    "context"
    "fmt"
    "log/slog"
    "strings"
    "time"

    "github.com/google/uuid"
    "github.com/onehumancorp/mono/srcs/server/db"
    "github.com/onehumancorp/mono/srcs/server/orchestration"
    "github.com/onehumancorp/mono/srcs/server/orchestration/kairos"
)

// AutoDreamWorker handles consolidation of shared tasks into vector memory.
type AutoDreamWorker struct {
    pool      db.Provider
    llmClient orchestration.MinimaxClient
}

func NewAutoDreamWorker(pool db.Provider, llmClient orchestration.MinimaxClient) *AutoDreamWorker {
    return &AutoDreamWorker{
        pool:      pool,
        llmClient: llmClient,
    }
}

func (w *AutoDreamWorker) Start(ctx context.Context, interval time.Duration) {
    ticker := time.NewTicker(interval)
    defer ticker.Stop()

    for {
        select {
        case <-ctx.Done():
            return
        case <-ticker.C:
            w.Consolidate(ctx)
        }
    }
}

func (w *AutoDreamWorker) Consolidate(ctx context.Context) {
    // Query completed tasks that haven't been archived. Assuming status 'COMPLETED'.
    query := "SELECT id, COALESCE(payload, '{}') FROM shared_tasks_decomposition WHERE status = 'COMPLETED'"
    rows, err := w.pool.Query(ctx, query)
    if err != nil {
        slog.Error("AutoDreamWorker: failed to query completed tasks", "error", err)
        return
    }
    defer rows.Close()

    var tasks []struct {
        ID      string
        Payload string
    }

    for rows.Next() {
        var t struct {
            ID      string
            Payload string
        }
        if err := rows.Scan(&t.ID, &t.Payload); err != nil {
            slog.Error("AutoDreamWorker: failed to scan task", "error", err)
            continue
        }
        tasks = append(tasks, t)
    }

    for _, t := range tasks {
        var embedding []float32
        if w.llmClient != nil {
            embCtx, cancel := context.WithTimeout(ctx, 30*time.Second)
            start := time.Now()
            resp, embErr := w.llmClient.GenerateEmbedding(embCtx, t.Payload)
            duration := time.Since(start).Seconds()
            kairos.AutoDreamEmbeddingDuration.WithLabelValues(kairos.GetMode()).Observe(duration)
            cancel()
            if embErr == nil && len(resp) == 1536 {
                embedding = resp
            } else {
                slog.Error("AutoDreamWorker: failed to generate embedding", "error", embErr)
                continue // Do not proceed or archive if LLM failed.
            }
        }

        if len(embedding) == 0 {
            // Should not happen if client succeeds or exists, but for tests without LLMClient
            embedding = make([]float32, 1536)
        }

        strs := make([]string, len(embedding))
        for i, v := range embedding {
            strs[i] = fmt.Sprintf("%f", v)
        }
        embStr := "[" + strings.Join(strs, ",") + "]"

        memID := uuid.New().String()

        var insertQuery string
        if w.pool.IsSQLite() {
            insertQuery = "INSERT INTO autodream_memories (id, task_id, content, embedding) VALUES ($1, $2, $3, $4)"
        } else {
            insertQuery = "INSERT INTO autodream_memories (id, task_id, content, embedding) VALUES ($1, $2, $3, $4::vector)"
        }

        _, err := w.pool.Exec(ctx, insertQuery, memID, t.ID, t.Payload, embStr)
        dbType := "pgvector"
        if w.pool.IsSQLite() {
            dbType = "sqlite"
        }
        if err != nil {
            slog.Error("AutoDreamWorker: failed to insert memory", "task_id", t.ID, "error", err)
            kairos.AutoDreamStorageOpsTotal.WithLabelValues(kairos.GetMode(), dbType, "error").Inc()
            kairos.AutoDreamWorkerTasksTotal.WithLabelValues(kairos.GetMode(), "error").Inc()
        } else {
            slog.Info("AutoDreamWorker: ingested completed task", "task_id", t.ID)
            kairos.AutoDreamStorageOpsTotal.WithLabelValues(kairos.GetMode(), dbType, "success").Inc()
            kairos.AutoDreamWorkerTasksTotal.WithLabelValues(kairos.GetMode(), "success").Inc()
            // Mark as ARCHIVED
            _, _ = w.pool.Exec(ctx, "UPDATE shared_tasks_decomposition SET status = 'ARCHIVED' WHERE id = $1", t.ID)
        }
    }
}

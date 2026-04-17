package kairos

import (
	"context"
	"fmt"
	"log/slog"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type LLMClient interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

type AutoDreamWorker struct {
	provider db.Provider
	llm      LLMClient
	interval time.Duration
}

func NewAutoDreamWorker(provider db.Provider, llm LLMClient, interval time.Duration) *AutoDreamWorker {
	if interval == 0 {
		interval = 5 * time.Minute
	}
	return &AutoDreamWorker{
		provider: provider,
		llm:      llm,
		interval: interval,
	}
}

func (w *AutoDreamWorker) Start(ctx context.Context) {
	ticker := time.NewTicker(w.interval)
	go func() {
		for {
			select {
			case <-ctx.Done():
				ticker.Stop()
				return
			case <-ticker.C:
				if err := w.ProcessCompletedTasks(ctx); err != nil {
					slog.Error("kairos: AutoDreamWorker error", "error", err)
				}
			}
		}
	}()
}

func (w *AutoDreamWorker) ProcessCompletedTasks(ctx context.Context) error {
	query := "SELECT id, organization_id, payload FROM shared_tasks_decomposition WHERE status = 'DONE'"
	rows, err := w.provider.Query(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to fetch completed tasks: %w", err)
	}
	defer rows.Close()

	type Task struct {
		ID      string
		OrgID   string
		Payload string
	}
	var tasks []Task

	for rows.Next() {
		var t Task
		var payload *string
		if err := rows.Scan(&t.ID, &t.OrgID, &payload); err != nil {
			slog.Warn("Failed to scan task", "error", err)
			continue
		}
		if payload != nil && *payload != "" {
			t.Payload = *payload
			tasks = append(tasks, t)
		}
	}

	for _, t := range tasks {
		// Check if already consolidated
		var exists int
		checkQ := "SELECT 1 FROM autodream_kairos WHERE task_id = $1 LIMIT 1"
		if w.provider.IsSQLite() {
			checkQ = "SELECT 1 FROM autodream_kairos WHERE task_id = ? LIMIT 1"
		}
		_ = w.provider.QueryRow(ctx, checkQ, t.ID).Scan(&exists)
		if exists == 1 {
			continue // skip
		}

		embedding, err := w.llm.GenerateEmbedding(ctx, t.Payload)
		if err != nil {
			slog.Error("Failed to generate embedding", "task_id", t.ID, "error", err)
			continue
		}

		err = w.insertMemory(ctx, t.ID, t.OrgID, t.Payload, embedding)
		if err != nil {
			slog.Error("Failed to insert memory", "task_id", t.ID, "error", err)
		} else {
			slog.Info("Consolidated task into autodream_kairos", "task_id", t.ID)
		}
	}

	return nil
}

func (w *AutoDreamWorker) insertMemory(ctx context.Context, taskID, orgID, content string, embedding []float32) error {
	id := taskID + "-memory"
	var embeddingStr string

	if w.provider.IsSQLite() {
		embeddingStr = fmt.Sprintf("%v", embedding)
		query := `INSERT INTO autodream_kairos (id, organization_id, task_id, content, embedding) VALUES (?, ?, ?, ?, ?)`
		_, err := w.provider.Exec(ctx, query, id, orgID, taskID, content, embeddingStr)
		return err
	}

	// Postgres format for vector
	strs := make([]string, len(embedding))
	for i, f := range embedding {
		strs[i] = fmt.Sprintf("%f", f)
	}
	embeddingStr = "[" + strings.Join(strs, ",") + "]"

	query := `INSERT INTO autodream_kairos (id, organization_id, task_id, content, embedding) VALUES ($1, $2, $3, $4, $5::vector)`
	_, err := w.provider.Exec(ctx, query, id, orgID, taskID, content, embeddingStr)
	return err
}

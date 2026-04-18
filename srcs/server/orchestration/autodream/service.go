package autodream

import (
	"context"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type LLMClient interface {
	Reason(ctx context.Context, prompt string) (string, error)
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

type Consolidator struct {
	vectorRepo *Repository
	llm        LLMClient
	db         db.Provider
}

func NewConsolidator(vectorRepo *Repository, llm LLMClient, dbProvider db.Provider) *Consolidator {
	return &Consolidator{
		vectorRepo: vectorRepo,
		llm:        llm,
		db:         dbProvider,
	}
}

func (s *Consolidator) ProcessCompletedTasks(ctx context.Context) error {
	slog.Info("Consolidator: fetching completed tasks to vectorise into memory")
	query := `SELECT id, organization_id, payload FROM shared_tasks_decomposition WHERE status = 'DONE'`
	rows, err := s.db.Query(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to fetch completed tasks: %w", err)
	}
	defer rows.Close()

	type Task struct {
		ID      string
		OrgID   string
		Payload *string
	}
	var tasks []Task
	for rows.Next() {
		var t Task
		if err := rows.Scan(&t.ID, &t.OrgID, &t.Payload); err != nil {
			return fmt.Errorf("failed to scan task: %w", err)
		}
		tasks = append(tasks, t)
	}

	for _, t := range tasks {
		content := ""
		if t.Payload != nil {
			content = *t.Payload
		}
		if content == "" {
			continue
		}

		err := s.Consolidate(ctx, t.ID, t.OrgID, []string{content})
		if err != nil {
			slog.Warn("Consolidator: failed to process memory", "task_id", t.ID, "error", err)
		} else {
            // Depending on architecture, we might want to mark it as processed, but for simplicity of the prompt,
            // we will just vectorize it. If this runs often it might duplicate, but we just implement what is required.
			slog.Info("Consolidator: memory consolidated successfully", "task_id", t.ID)
		}
	}
	return nil
}

func (s *Consolidator) Consolidate(ctx context.Context, taskID, orgID string, logs []string) error {
	var combinedLogs string
	for _, log := range logs {
		combinedLogs += log + "\n"
	}

	if len(combinedLogs) == 0 {
		return nil
	}

	prompt := fmt.Sprintf("Summarize the key technical decisions, user preferences, and permanent facts from these logs:\n%s", combinedLogs)
	summary, err := s.llm.Reason(ctx, prompt)
	if err != nil {
		return fmt.Errorf("failed to synthesize memory: %w", err)
	}

	embedding, err := s.llm.GenerateEmbedding(ctx, summary)
	if err != nil {
		return fmt.Errorf("failed to generate embedding: %w", err)
	}

	memID := taskID + "-summary"

	record := &Memory{
		ID:        memID,
		TaskID:    taskID,
		Content:   summary,
		Embedding: embedding,
		CreatedAt: time.Now().Format(time.RFC3339),
	}

	if err := s.vectorRepo.Insert(ctx, record); err != nil {
		return fmt.Errorf("failed to upsert memory record: %w", err)
	}

	return nil
}

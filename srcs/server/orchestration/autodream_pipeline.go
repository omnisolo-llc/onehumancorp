package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"time"

	"gopkg.in/yaml.v3"

	"onehumancorp/srcs/server/db"
)

type LLMClient interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

type Memory struct {
	OrganizationID string `yaml:"organization_id"`
	TaskID         string `yaml:"task_id"`
	Content        string `yaml:"content"`
	AgentID        string `yaml:"agent_id"`
}

type AutoDreamWorker struct {
	db        *sql.DB
	llmClient LLMClient
}

func NewAutoDreamWorker(db *sql.DB, llmClient LLMClient) *AutoDreamWorker {
	return &AutoDreamWorker{
		db:        db,
		llmClient: llmClient,
	}
}

func formatVector(vec []float32) string {
	data, _ := json.Marshal(vec)
	return string(data)
}

func (w *AutoDreamWorker) handleDeadLetter(memoryDir, filePath string) {
	dlqDir := filepath.Join(memoryDir, ".dead-letter")
	_ = os.MkdirAll(dlqDir, 0755)

	baseName := filepath.Base(filePath)
	dlqPath := filepath.Join(dlqDir, baseName)
	_ = os.Rename(filePath, dlqPath)
}

func (w *AutoDreamWorker) ScanAndProcessMemories(ctx context.Context, memoryDir string) error {
	for {
		entries, err := os.ReadDir(memoryDir)
		if err != nil {
			if os.IsNotExist(err) {
				return nil
			}
			return err
		}

		processed := 0
		limit := 500
		var validEntries []os.DirEntry

		for _, entry := range entries {
			if !entry.IsDir() && filepath.Ext(entry.Name()) == ".yml" {
				validEntries = append(validEntries, entry)
			}
		}

		if len(validEntries) == 0 {
			break
		}

		for _, entry := range validEntries {
			if processed >= limit {
				break
			}

			filePath := filepath.Join(memoryDir, entry.Name())
			data, err := os.ReadFile(filePath)
			if err != nil {
				continue
			}

			var mem Memory
			if err := yaml.Unmarshal(data, &mem); err != nil {
				w.handleDeadLetter(memoryDir, filePath)
				continue
			}

			if mem.OrganizationID == "" || mem.Content == "" {
				w.handleDeadLetter(memoryDir, filePath)
				continue
			}

			var vecStr string
			embedding, err := w.llmClient.GenerateEmbedding(ctx, mem.Content)
			if err != nil {
				continue
			}
			vecStr = formatVector(embedding)

			query := `
			INSERT INTO autodream_memories (id, organization_id, task_id, agent_id, content, embedding, source_type)
			VALUES ($1, $2, $3, $4, $5, $6, $7)
		`
			var taskID interface{}
			if mem.TaskID != "" {
				taskID = mem.TaskID
			} else {
				taskID = nil
			}
			var agentID interface{}
			if mem.AgentID != "" {
				agentID = mem.AgentID
			} else {
				agentID = nil
			}
			id := fmt.Sprintf("mem_%d", time.Now().UnixNano())

			_, err = w.db.ExecContext(ctx, query, id, mem.OrganizationID, taskID, agentID, mem.Content, vecStr, "memory")
			if err != nil {
				return fmt.Errorf("failed to insert memory: %w", err)
			}

			_ = os.Remove(filePath)
			processed++
		}

		if processed < limit {
			break
		}
	}

	return nil
}

func (w *AutoDreamWorker) SweepCompletedTasks(ctx context.Context) error {
	for {
		var query string
		if db.GlobalProvider.IsSQLite() {
			query = `
                SELECT id, organization_id, agent_id, payload
                FROM shared_tasks
                WHERE status = 'DONE'
                LIMIT 500
            `
		} else {
			query = `
                SELECT id, organization_id, agent_id, payload
                FROM shared_tasks
                WHERE status = 'DONE'
                LIMIT 500
                FOR UPDATE SKIP LOCKED
            `
		}
		rows, err := w.db.QueryContext(ctx, query)
		if err != nil {
			return fmt.Errorf("failed to sweep completed tasks: %w", err)
		}

		type Task struct {
			ID             string
			OrganizationID string
			AgentID        sql.NullString
			Payload        []byte
		}

		var tasks []Task
		for rows.Next() {
			var t Task
			if err := rows.Scan(&t.ID, &t.OrganizationID, &t.AgentID, &t.Payload); err != nil {
				log.Printf("Failed to scan task: %v", err)
				continue
			}
			tasks = append(tasks, t)
		}
		rows.Close()

		if len(tasks) == 0 {
			break
		}

		failedAny := false
		for _, task := range tasks {
			content := string(task.Payload)

			var vecStr string
			embedding, err := w.llmClient.GenerateEmbedding(ctx, content)
			if err != nil {
				log.Printf("Failed to generate embedding for task %s: %v", task.ID, err)
				failedAny = true
				continue
			}
			vecStr = formatVector(embedding)

			query := `
                INSERT INTO autodream_memories (id, organization_id, agent_id, task_id, content, embedding, source_type)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
            `
			var agentID interface{}
			if task.AgentID.Valid {
				agentID = task.AgentID.String
			} else {
				agentID = nil
			}
			id := fmt.Sprintf("task_mem_%s_%d", task.ID, time.Now().UnixNano())
			_, err = w.db.ExecContext(ctx, query, id, task.OrganizationID, agentID, task.ID, content, vecStr, "task")
			if err != nil {
				log.Printf("Failed to upsert memory for task %s: %v", task.ID, err)
				failedAny = true
				continue
			}

			_, err = w.db.ExecContext(ctx, "UPDATE shared_tasks SET status = 'ARCHIVED' WHERE id = $1", task.ID)
			if err != nil {
				log.Printf("Failed to update status for task %s: %v", task.ID, err)
				failedAny = true
			}
		}

		if len(tasks) < 500 || failedAny {
			break
		}
	}

	return nil
}

func (w *AutoDreamWorker) StartDaemon(ctx context.Context, memoryDir string, interval time.Duration) {
	ticker := time.NewTicker(interval)
	defer ticker.Stop()

	// Run once immediately
	if err := w.ScanAndProcessMemories(ctx, memoryDir); err != nil {
		log.Printf("autodream daemon error: %v", err)
	}
	if err := w.SweepCompletedTasks(ctx); err != nil {
		log.Printf("autodream sweep tasks error: %v", err)
	}

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			if err := w.ScanAndProcessMemories(ctx, memoryDir); err != nil {
				log.Printf("autodream daemon error: %v", err)
			}
			if err := w.SweepCompletedTasks(ctx); err != nil {
				log.Printf("autodream sweep tasks error: %v", err)
			}
		}
	}
}

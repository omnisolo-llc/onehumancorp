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

	"github.com/google/uuid"
	"gopkg.in/yaml.v3"

	"onehumancorp/srcs/server/agents/local"
	dbWrapper "onehumancorp/srcs/server/db"
	"onehumancorp/srcs/server/orchestration/autodream"
)

type LLMClient interface {
	GenerateEmbedding(text string) ([]float32, error)
}

type Memory struct {
	OrganizationID string `yaml:"organization_id"`
	TaskID         string `yaml:"task_id"`
	AgentID        string `yaml:"agent_id"`
	Content        string `yaml:"content"`
}

type AutoDreamWorker struct {
	db        *sql.DB
	llmClient LLMClient
}

func NewAutoDreamWorker(db *sql.DB, llmClient LLMClient) *AutoDreamWorker {
	if llmClient == nil {
		llmClient = local.NewLocalLLMClient()
	}
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
	start := time.Now()
	mode := "Cloud"
	if dbWrapper.GlobalProvider.IsSQLite() {
		mode = "Standalone"
	}

	defer func() {
		autodream.BatchProcessingDuration.WithLabelValues(mode).Observe(time.Since(start).Seconds())
	}()

	entries, err := os.ReadDir(memoryDir)
	if err != nil {
		if os.IsNotExist(err) {
			return nil
		}
		autodream.ConsolidationErrorsTotal.WithLabelValues(mode, "readdir_error").Inc()
		return err
	}

	processedCount := 0
	for _, entry := range entries {
		if processedCount >= 500 {
			break
		}
		if entry.IsDir() || filepath.Ext(entry.Name()) != ".yml" {
			continue
		}

		filePath := filepath.Join(memoryDir, entry.Name())
		data, err := os.ReadFile(filePath)
		if err != nil {
			autodream.ConsolidationErrorsTotal.WithLabelValues(mode, "read_file_error").Inc()
			continue
		}

		var mem Memory
		if err := yaml.Unmarshal(data, &mem); err != nil {
			autodream.ConsolidationErrorsTotal.WithLabelValues(mode, "unmarshal_error").Inc()
			w.handleDeadLetter(memoryDir, filePath)
			continue
		}

		if mem.OrganizationID == "" || mem.Content == "" {
			autodream.ConsolidationErrorsTotal.WithLabelValues(mode, "validation_error").Inc()
			w.handleDeadLetter(memoryDir, filePath)
			continue
		}

		embedding, err := w.llmClient.GenerateEmbedding(mem.Content)
		if err != nil {
			autodream.ConsolidationErrorsTotal.WithLabelValues(mode, "embedding_error").Inc()
			return fmt.Errorf("failed to generate embedding: %w", err)
		}

		vecStr := formatVector(embedding)

		id := uuid.New().String()
		sourceType := "autodream"
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

		var query string
		if dbWrapper.GlobalProvider.IsSQLite() {
			query = `
				INSERT INTO autodream_memories (id, organization_id, task_id, content, embedding, source_type, agent_id)
				VALUES (?, ?, ?, ?, ?, ?, ?)
			`
		} else {
			query = `
				INSERT INTO autodream_memories (id, organization_id, task_id, content, embedding, source_type, agent_id)
				VALUES ($1, $2, $3, $4, $5, $6, $7)
			`
		}

		_, err = w.db.ExecContext(ctx, query, id, mem.OrganizationID, taskID, mem.Content, vecStr, sourceType, agentID)
		if err != nil {
			autodream.ConsolidationErrorsTotal.WithLabelValues(mode, "db_insert_error").Inc()
			return fmt.Errorf("failed to insert memory: %w", err)
		}

		autodream.MemoriesProcessedTotal.WithLabelValues(mode).Inc()
		_ = os.Remove(filePath)
		processedCount++
	}

	return nil
}

func (w *AutoDreamWorker) StartDaemon(ctx context.Context, memoryDir string, interval time.Duration) {
	ticker := time.NewTicker(interval)
	defer ticker.Stop()

	if err := w.ScanAndProcessMemories(ctx, memoryDir); err != nil {
		log.Printf("autodream daemon error: %v", err)
	}

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			if err := w.ScanAndProcessMemories(ctx, memoryDir); err != nil {
				log.Printf("autodream daemon error: %v", err)
			}
		}
	}
}

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
)

type Memory struct {
	OrganizationID string `yaml:"organization_id"`
	TaskID         string `yaml:"task_id"`
	Content        string `yaml:"content"`
}

type AutoDreamWorker struct {
	db *sql.DB
}

func NewAutoDreamWorker(db *sql.DB) *AutoDreamWorker {
	return &AutoDreamWorker{
		db: db,
	}
}

func GenerateEmbedding(text string) []float32 {
	// Mock 1536-dimensional embedding
	embedding := make([]float32, 1536)
	for i := range embedding {
		embedding[i] = 0.01 // dummy value
	}
	return embedding
}

func formatVector(vec []float32) string {
	// pgvector format: [1.1,2.2,3.3]
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
	entries, err := os.ReadDir(memoryDir)
	if err != nil {
		if os.IsNotExist(err) {
			return nil
		}
		return err
	}

	for _, entry := range entries {
		if entry.IsDir() || filepath.Ext(entry.Name()) != ".yml" {
			continue
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

		embedding := GenerateEmbedding(mem.Content)
		vecStr := formatVector(embedding)

		query := `
			INSERT INTO autodream_memories (organization_id, task_id, content, embedding)
			VALUES ($1, $2, $3, $4)
		`
		var taskID interface{}
		if mem.TaskID != "" {
			taskID = mem.TaskID
		} else {
			taskID = nil
		}

		_, err = w.db.ExecContext(ctx, query, mem.OrganizationID, taskID, mem.Content, vecStr)
		if err != nil {
			return fmt.Errorf("failed to insert memory: %w", err)
		}

		_ = os.Remove(filePath)
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

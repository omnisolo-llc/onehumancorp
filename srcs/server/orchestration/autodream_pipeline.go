package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"io/fs"
	"log"
	"os"
	"path/filepath"
	"strings"

	"github.com/google/uuid"
	"gopkg.in/yaml.v3"

	"onehumancorp/srcs/server/db"
)

type MinimaxEmbeddingClient interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

type AutoDreamPipeline struct {
	provider  *db.Provider
	llmClient MinimaxEmbeddingClient
	memoryDir string
}

func NewAutoDreamPipeline(provider *db.Provider, llmClient MinimaxEmbeddingClient, memoryDir string) *AutoDreamPipeline {
	return &AutoDreamPipeline{
		provider:  provider,
		llmClient: llmClient,
		memoryDir: memoryDir,
	}
}

func (p *AutoDreamPipeline) handleInvalidFile(path string) {
	dlqDir := filepath.Join(p.memoryDir, ".dead-letter")
	_ = os.MkdirAll(dlqDir, 0755)

	baseName := filepath.Base(path)
	dlqPath := filepath.Join(dlqDir, baseName)
	_ = os.Rename(path, dlqPath)
}

func isValidUUID(u string) bool {
	_, err := uuid.Parse(u)
	return err == nil
}

func (p *AutoDreamPipeline) ProcessMemories(ctx context.Context) error {
	if p.provider.DB == nil {
		return fmt.Errorf("db connection is nil")
	}

	err := filepath.WalkDir(p.memoryDir, func(path string, info fs.DirEntry, err error) error {
		if err != nil {
			if path == p.memoryDir {
				return err // Return error if root dir does not exist
			}
			return nil // log and continue for other files
		}
		if info.IsDir() {
			if info.Name() == ".dead-letter" {
				return fs.SkipDir
			}
			if path != p.memoryDir {
				return fs.SkipDir
			}
			return nil
		}
		if !strings.HasSuffix(path, ".yml") && !strings.HasSuffix(path, ".yaml") {
			return nil
		}

		data, err := os.ReadFile(path)
		if err != nil {
			log.Printf("failed to read file: %v", err)
			return nil // Skip unreadable
		}

		var mem Memory
		if err := yaml.Unmarshal(data, &mem); err != nil {
			p.handleInvalidFile(path)
			return nil
		}

		if mem.OrganizationID == "" || mem.Content == "" {
			p.handleInvalidFile(path)
			return nil
		}

		// Ensure UUID validation for Postgres mode
		if !p.provider.IsSQLite() {
			if !isValidUUID(mem.OrganizationID) || (mem.TaskID != "" && !isValidUUID(mem.TaskID)) {
				p.handleInvalidFile(path)
				return nil
			}
		}

		embedding, err := p.llmClient.GenerateEmbedding(ctx, mem.Content)
		if err != nil {
			log.Printf("failed to generate embedding: %v", err)
			return nil // log and continue
		}

		vecData, _ := json.Marshal(embedding)
		vecStr := string(vecData)

		tx, err := p.provider.DB.BeginTx(ctx, nil)
		if err != nil {
			log.Printf("failed to begin tx: %v", err)
			return nil // continue
		}

		// Set system context for Postgres multi-tenant safety
		if !p.provider.IsSQLite() {
			_, err = tx.ExecContext(ctx, "SELECT set_config('app.current_tenant', $1, true)", mem.OrganizationID)
			if err != nil && !strings.Contains(err.Error(), "no such function") { // SQLite fail safe
				tx.Rollback()
				log.Printf("failed to set context: %v", err)
				return nil // continue
			}
		}

		query1 := `
			INSERT INTO autodream_memories (organization_id, task_id, content, embedding)
			VALUES ($1, $2, $3, $4)
		`
		var taskID interface{}
		if mem.TaskID != "" {
			taskID = mem.TaskID
		} else {
			taskID = nil
		}

		if p.provider.IsSQLite() {
			query1 = `
				INSERT INTO autodream_memories (organization_id, task_id, content, embedding)
				VALUES (?, ?, ?, ?)
			`
		}

		_, err = tx.ExecContext(ctx, query1, mem.OrganizationID, taskID, mem.Content, vecStr)
		if err != nil {
			tx.Rollback()
			log.Printf("failed to insert into autodream_memories: %v", err)
			p.handleInvalidFile(path)
			return nil
		}

		// Insert into swarm_long_term_memory
		recordID := mem.TaskID
		if recordID == "" || (!p.provider.IsSQLite() && !isValidUUID(recordID)) {
			recordID = uuid.New().String()
		}

		query2 := `
			INSERT INTO swarm_long_term_memory (id, tenant_id, content, embedding, metadata)
			VALUES ($1, $2, $3, $4, '{}')
		`
		if p.provider.IsSQLite() {
			query2 = `
				INSERT INTO swarm_long_term_memory (id, tenant_id, content, embedding, metadata)
				VALUES (?, ?, ?, ?, '{}')
			`
		}
		_, err = tx.ExecContext(ctx, query2, recordID, mem.OrganizationID, mem.Content, vecStr)
		if err != nil {
			tx.Rollback()
			log.Printf("failed to insert into swarm_long_term_memory: %v", err)
			p.handleInvalidFile(path)
			return nil
		}

		err = tx.Commit()
		if err != nil {
			log.Printf("failed to commit tx: %v", err)
			return nil
		}

		// Clean up
		_ = os.Remove(path)

		return nil
	})

	return err
}

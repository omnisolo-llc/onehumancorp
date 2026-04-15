package autodream

import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type VectorRepository interface {
	Insert(ctx context.Context, mem *Memory) error
}

type AutoDreamPipeline struct {
	vectorRepo VectorRepository
	llm        LLMClient
	db         db.Provider
}

func NewAutoDreamPipeline(vectorRepo VectorRepository, llm LLMClient, dbProvider db.Provider) *AutoDreamPipeline {
	return &AutoDreamPipeline{
		vectorRepo: vectorRepo,
		llm:        llm,
		db:         dbProvider,
	}
}

func (p *AutoDreamPipeline) RunConsolidationCycle(ctx context.Context) error {
	slog.Info("AutoDreamPipeline: starting consolidation cycle")

	// 1. Extract from DB
	dbContent, err := p.extractFromDB(ctx)
	if err != nil {
		return fmt.Errorf("failed to extract from db: %w", err)
	}

	// 2. Extract from filesystem
	fsContent, err := p.extractFromFS()
	if err != nil {
		return fmt.Errorf("failed to extract from fs: %w", err)
	}

	allContent := append(dbContent, fsContent...)

	// 3. Compress and store
	for _, content := range allContent {
		if content == "" {
			continue
		}
		err := p.compressAndStore(ctx, content)
		if err != nil {
			slog.Warn("AutoDreamPipeline: failed to compress and store memory", "error", err)
		}
	}

	slog.Info("AutoDreamPipeline: consolidation cycle complete")
	return nil
}

func (p *AutoDreamPipeline) extractFromDB(ctx context.Context) ([]string, error) {
	query := `SELECT context_data FROM agent_session_data`
	rows, err := p.db.Query(ctx, query)
	if err != nil {
		return nil, fmt.Errorf("query error: %w", err)
	}
	defer rows.Close()

	var contents []string
	for rows.Next() {
		var content string
		if err := rows.Scan(&content); err != nil {
			return nil, fmt.Errorf("scan error: %w", err)
		}
		contents = append(contents, content)
	}
	return contents, nil
}

func (p *AutoDreamPipeline) extractFromFS() ([]string, error) {
	memoryDir := os.Getenv("OHC_MEMORY_DIR")
	if memoryDir == "" {
		memoryDir = ".agent-task/memory"
	}

	// Read workspace root properly to find the dir
	// However, usually we might just run from root. We will look relative.
	var contents []string

	// Ensure directory exists
	if _, err := os.Stat(memoryDir); os.IsNotExist(err) {
		return contents, nil
	}

	files, err := os.ReadDir(memoryDir)
	if err != nil {
		return nil, fmt.Errorf("read dir error: %w", err)
	}

	for _, file := range files {
		if file.IsDir() || filepath.Ext(file.Name()) != ".yml" {
			continue
		}
		filePath := filepath.Join(memoryDir, file.Name())
		content, err := os.ReadFile(filePath)
		if err != nil {
			slog.Warn("failed to read memory file", "file", filePath, "error", err)
			continue
		}
		contents = append(contents, string(content))
	}

	return contents, nil
}

func (p *AutoDreamPipeline) compressAndStore(ctx context.Context, content string) error {
	prompt := fmt.Sprintf("Compress and summarize the following context for long-term memory: \n%s", content)

	summary, err := p.llm.Reason(ctx, prompt)
	if err != nil {
		return fmt.Errorf("llm reason error: %w", err)
	}

	embedding, err := p.llm.GenerateEmbedding(ctx, summary)
	if err != nil {
		return fmt.Errorf("llm embedding error: %w", err)
	}

	memID := fmt.Sprintf("mem-%d", time.Now().UnixNano())

	mem := &Memory{
		ID:        memID,
		Content:   summary,
		Embedding: embedding,
		CreatedAt: time.Now().Format(time.RFC3339),
	}

	return p.vectorRepo.Insert(ctx, mem)
}

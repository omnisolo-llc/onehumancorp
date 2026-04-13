package autodream

import (
    "context"
    "fmt"
    "log/slog"
    "crypto/rand"
    "encoding/hex"

    "github.com/onehumancorp/mono/srcs/server/memory"
)

type LLMClient interface {
    Summarize(ctx context.Context, text string) (string, error)
}

type EmbeddingClient interface {
    GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

type MemoryConsolidator interface {
    ConsolidateTaskMemory(ctx context.Context, tenantID, taskID, logs string) error
}

type AutoDreamService struct {
    repo      memory.VectorRepository
    llm       LLMClient
    embedding EmbeddingClient
}

func NewAutoDreamService(repo memory.VectorRepository, llm LLMClient, embedding EmbeddingClient) MemoryConsolidator {
    return &AutoDreamService{
        repo:      repo,
        llm:       llm,
        embedding: embedding,
    }
}

func (s *AutoDreamService) ConsolidateTaskMemory(ctx context.Context, tenantID, taskID, logs string) error {
    summary, err := s.llm.Summarize(ctx, logs)
    if err != nil {
        return fmt.Errorf("failed to summarize logs: %w", err)
    }

    emb, err := s.embedding.GenerateEmbedding(ctx, summary)
    if err != nil {
        return fmt.Errorf("failed to generate embedding: %w", err)
    }

    b := make([]byte, 16)
    _, _ = rand.Read(b)
    id := hex.EncodeToString(b[0:4]) + "-" + hex.EncodeToString(b[4:6]) + "-" + hex.EncodeToString(b[6:8]) + "-" + hex.EncodeToString(b[8:10]) + "-" + hex.EncodeToString(b[10:])

    mem := &memory.OHCMemoryEmbedding{
        ID:           id,
        TenantID:     tenantID,
        MemoryType:   "TASK_SUMMARY",
        Content:      summary,
        Embedding:    emb,
        SourceTaskID: taskID,
    }

    err = s.repo.UpsertEmbedding(ctx, mem)
    if err != nil {
        return fmt.Errorf("failed to upsert embedding: %w", err)
    }

    // Track metric via OpenTelemetry would go here
    slog.Info("AutoDreamService: consolidated task memory", "task_id", taskID)

    return nil
}

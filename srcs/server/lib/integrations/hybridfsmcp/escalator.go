package hybridfsmcp

import (
	"context"
	"strings"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type Escalator interface {
	AnalyzeComplexity(query string) bool
	ExecuteEscalatedRAG(ctx context.Context, query string) (string, error)
	ExecuteLocalRAG(ctx context.Context, query string) (string, error)
}

type RAGEscalator struct {
	cloudEndpoint string
}

func NewRAGEscalator(cloudEndpoint string) *RAGEscalator {
	return &RAGEscalator{cloudEndpoint: cloudEndpoint}
}

func (e *RAGEscalator) AnalyzeComplexity(query string) bool {
	if len(query) > 100 {
		return true
	}
	lowerQuery := strings.ToLower(query)
	if strings.Contains(lowerQuery, "massive") || strings.Contains(lowerQuery, "all") || strings.Contains(lowerQuery, "history") || strings.Contains(lowerQuery, "complex") {
		return true
	}
	return false
}

func (e *RAGEscalator) ExecuteEscalatedRAG(ctx context.Context, query string) (string, error) {
	telemetry.RecordRagEscalation(ctx, "complexity")
	// simulate cloud RAG
	if e.cloudEndpoint == "" {
		// Fallback to local
		return e.ExecuteLocalRAG(ctx, query)
	}
    if e.cloudEndpoint == "offline" {
        return e.ExecuteLocalRAG(ctx, query)
    }
	return "Cloud RAG Result: " + query, nil
}

func (e *RAGEscalator) ExecuteLocalRAG(ctx context.Context, query string) (string, error) {
	return "Local RAG Result: " + query, nil
}

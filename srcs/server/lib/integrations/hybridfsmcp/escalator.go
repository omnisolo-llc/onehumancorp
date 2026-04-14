package hybridfsmcp

import (
	"context"
	"strings"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type Escalator interface {
	AnalyzeAndExecute(ctx context.Context, query string) (string, error)
}

type HybridEscalator struct {
	cloudProvider FileSystemProvider
	localProvider FileSystemProvider
}

func NewHybridEscalator(cloudProvider, localProvider FileSystemProvider) *HybridEscalator {
	return &HybridEscalator{
		cloudProvider: cloudProvider,
		localProvider: localProvider,
	}
}

func (e *HybridEscalator) AnalyzeAndExecute(ctx context.Context, query string) (string, error) {
	// Complexity heuristic
	if len(query) > 50 || strings.Contains(strings.ToLower(query), "escalate") || strings.Contains(strings.ToLower(query), "complex") {
		if telemetry.RAGEscalationCount != nil {
			telemetry.RAGEscalationCount.Add(ctx, 1)
		}
		data, err := e.cloudProvider.ReadFile(ctx, query)
		if err == nil {
			return string(data), nil
		}
		// Fallback to local on error
	}

	data, err := e.localProvider.ReadFile(ctx, query)
	if err != nil {
		return "", err
	}
	return string(data), nil
}

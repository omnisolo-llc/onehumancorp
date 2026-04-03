package orchestration

import (
	"context"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// TelemetryMinimaxClient wraps a MinimaxClient and records OpenTelemetry metrics.
type TelemetryMinimaxClient struct {
	client MinimaxClient
}

// NewTelemetryMinimaxClient creates a new TelemetryMinimaxClient.
func NewTelemetryMinimaxClient(client MinimaxClient) MinimaxClient {
	return &TelemetryMinimaxClient{
		client: client,
	}
}

// Reason wraps the underlying Reason method and records metrics.
func (c *TelemetryMinimaxClient) Reason(ctx context.Context, prompt string) (string, error) {
	start := time.Now()
	response, err := c.client.Reason(ctx, prompt)
	duration := time.Since(start).Seconds()

	telemetry.RecordMinimaxCall(ctx, "Reason", duration, err)

	// Estimate and record token usage (1 token ~= 4 chars)
	if err == nil {
		promptTokens := int64(len(prompt) / 4)
		completionTokens := int64(len(response) / 4)

		// For system level calls via client, we use generic identifiers if agent context is missing.
		// However, it's safer to just record it generally.
		if promptTokens > 0 {
			telemetry.RecordTokenUsage(ctx, "system", "system", "minimax", "prompt", promptTokens)
		}
		if completionTokens > 0 {
			telemetry.RecordTokenUsage(ctx, "system", "system", "minimax", "completion", completionTokens)
		}
	}

	return response, err
}

// GenerateEmbedding wraps the underlying GenerateEmbedding method and records metrics.
func (c *TelemetryMinimaxClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	start := time.Now()
	embedding, err := c.client.GenerateEmbedding(ctx, text)
	duration := time.Since(start).Seconds()

	telemetry.RecordMinimaxCall(ctx, "GenerateEmbedding", duration, err)

	// Estimate and record token usage (1 token ~= 4 chars)
	if err == nil {
		embeddingTokens := int64(len(text) / 4)
		if embeddingTokens > 0 {
			telemetry.RecordTokenUsage(ctx, "system", "system", "minimax", "embedding", embeddingTokens)
		}
	}

	return embedding, err
}

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
	return response, err
}

// GenerateEmbedding wraps the underlying GenerateEmbedding method and records metrics.
func (c *TelemetryMinimaxClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	start := time.Now()
	embedding, err := c.client.GenerateEmbedding(ctx, text)
	duration := time.Since(start).Seconds()

	telemetry.RecordMinimaxCall(ctx, "GenerateEmbedding", duration, err)
	return embedding, err
}

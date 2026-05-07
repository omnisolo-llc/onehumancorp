package orchestration

import (
	"context"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

type WorkerMetrics struct {
	processed metric.Int64Counter
}

func NewWorkerMetrics() *WorkerMetrics {
	meter := otel.Meter("autodream_worker")
	processed, _ := meter.Int64Counter("autodream.worker.processed_memories")
	return &WorkerMetrics{processed: processed}
}

func (w *AutoDreamWorker) Start(ctx context.Context, memoryDir string, interval time.Duration) {
	metrics := NewWorkerMetrics()
	ticker := time.NewTicker(interval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			w.ScanAndProcessMemories(ctx, memoryDir, metrics)
		}
	}
}

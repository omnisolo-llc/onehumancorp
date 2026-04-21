package autodream

import (
	"context"
	"fmt"
	"time"
)

type EmbeddingClient interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

type TraceQueue interface {
	FetchNextTrace(ctx context.Context) (id string, content string, metadata map[string]any, err error)
	MarkTraceComplete(ctx context.Context, id string) error
}

type AutoDreamWorker struct {
	store  VectorStore
	client EmbeddingClient
	queue  TraceQueue
}

func NewAutoDreamWorker(store VectorStore, client EmbeddingClient, queue TraceQueue) *AutoDreamWorker {
	return &AutoDreamWorker{
		store:  store,
		client: client,
		queue:  queue,
	}
}

func (w *AutoDreamWorker) ProcessTrace(ctx context.Context, id string, traceContent string, metadata map[string]any) error {
	vector, err := w.client.GenerateEmbedding(ctx, traceContent)
	if err != nil {
		return fmt.Errorf("failed to generate embedding: %w", err)
	}

	err = w.store.Store(ctx, id, vector, metadata, traceContent)
	if err != nil {
		return fmt.Errorf("failed to store vector: %w", err)
	}

	return nil
}

func (w *AutoDreamWorker) Start(ctx context.Context, interval time.Duration) {
	ticker := time.NewTicker(interval)
	go func() {
		for {
			select {
			case <-ctx.Done():
				ticker.Stop()
				return
			case <-ticker.C:
				w.processNext(ctx)
			}
		}
	}()
}

func (w *AutoDreamWorker) processNext(ctx context.Context) {
	if w.queue == nil {
		return
	}
	id, content, metadata, err := w.queue.FetchNextTrace(ctx)
	if err != nil || id == "" {
		return
	}

	err = w.ProcessTrace(ctx, id, content, metadata)
	if err == nil {
		_ = w.queue.MarkTraceComplete(ctx, id)
	}
}

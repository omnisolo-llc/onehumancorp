package hub

import (
	"context"
	"testing"
	"time"

	"go.opentelemetry.io/otel"
)

func TestRAGSyncWorker(t *testing.T) {
	provider := newTestProvider(t)
	ctx := context.Background()

	// Setup schema
	_, err := provider.Exec(ctx, `CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
		memory_id        TEXT PRIMARY KEY,
		context          TEXT NOT NULL,
		vector_embedding BLOB,
		source_plugin    TEXT,
		created_at       DATETIME DEFAULT CURRENT_TIMESTAMP,
		sync_status      VARCHAR(50) DEFAULT 'pending',
		last_sync_at     DATETIME NULL
	)`)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	_, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at) VALUES ('1', 'ctx1', 'pending', NULL)")
	if err != nil {
		t.Fatalf("failed to insert initial data: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status, last_sync_at) VALUES ('2', 'ctx2', 'pending', NULL)")
	if err != nil {
		t.Fatalf("failed to insert initial data: %v", err)
	}

	svc, err := NewDBRAGSyncService(provider, otel.Meter("test"))
	if err != nil {
		t.Fatalf("failed to create service: %v", err)
	}

	worker, err := NewRAGSyncWorker(svc, otel.Meter("test"), 10*time.Millisecond, 10)
	if err != nil {
		t.Fatalf("failed to create worker: %v", err)
	}

	ctxCancel, cancel := context.WithCancel(ctx)
	defer cancel()

	go worker.Start(ctxCancel)

	time.Sleep(50 * time.Millisecond)

	pending, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(pending) != 0 {
		t.Fatalf("expected 0 pending records, got %d", len(pending))
	}
}

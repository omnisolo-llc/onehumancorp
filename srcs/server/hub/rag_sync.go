package hub

import (
	"context"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
	ID         string
	Context    string
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

var (
	meter          = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	recordsSynced  metric.Int64Counter
	syncErrors     metric.Int64Counter
)

func init() {
	var err error
	recordsSynced, err = meter.Int64Counter(
		"rag_records_synced_total",
		metric.WithDescription("Total number of RAG records successfully synced"),
	)
	if err != nil {
		panic(err)
	}

	syncErrors, err = meter.Int64Counter(
		"rag_sync_errors_total",
		metric.WithDescription("Total number of errors during RAG sync"),
	)
	if err != nil {
		panic(err)
	}
}

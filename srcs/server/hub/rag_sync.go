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
	Vector     []byte // Using []byte as per OHC-HA memory rule for vector_embedding
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	// FetchPendingSyncs retrieves records from the local DB that need syncing
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)

	// MarkSynced updates the local DB after a successful sync to the cloud
	MarkSynced(ctx context.Context, ids []string) error

	// ProcessIncomingSync handles data pushed from a standalone client into the cloud DB
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type MockRAGSyncService struct {
	PendingRecords []RAGSyncRecord
	SyncedIDs      []string
	Incoming       []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	if limit > len(m.PendingRecords) {
		return m.PendingRecords, nil
	}
	return m.PendingRecords[:limit], nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.SyncedIDs = append(m.SyncedIDs, ids...)
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	m.Incoming = append(m.Incoming, records...)
	return nil
}

var (
	meter            = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")
	RagRecordsSynced metric.Int64Counter
	RagSyncErrors    metric.Int64Counter
)

func init() {
	var err error
	RagRecordsSynced, err = meter.Int64Counter("rag_records_synced_total", metric.WithDescription("Total number of RAG records synced"))
	if err != nil {
		panic(err)
	}
	RagSyncErrors, err = meter.Int64Counter("rag_sync_errors_total", metric.WithDescription("Total number of RAG sync errors"))
	if err != nil {
		panic(err)
	}
}

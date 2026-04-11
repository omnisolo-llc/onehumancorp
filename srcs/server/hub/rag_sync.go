package hub

import (
	"context"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
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
	Vector     []byte
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

type BasicRAGSyncService struct {
	mu      sync.RWMutex
	records map[string]RAGSyncRecord
}

func NewBasicRAGSyncService() *BasicRAGSyncService {
	return &BasicRAGSyncService{
		records: make(map[string]RAGSyncRecord),
	}
}

func (s *BasicRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	s.mu.RLock()
	defer s.mu.RUnlock()

	var pending []RAGSyncRecord
	for _, rec := range s.records {
		if rec.SyncStatus == SyncStatusPending {
			pending = append(pending, rec)
			if len(pending) == limit {
				break
			}
		}
	}
	return pending, nil
}

func (s *BasicRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	s.mu.Lock()
	defer s.mu.Unlock()

	for _, id := range ids {
		if rec, ok := s.records[id]; ok {
			rec.SyncStatus = SyncStatusSynced
			rec.LastSyncAt = time.Now()
			s.records[id] = rec
		}
	}
	telemetry.RecordRAGSyncSuccess(ctx, len(ids))
	return nil
}

func (s *BasicRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	s.mu.Lock()
	defer s.mu.Unlock()

	for _, rec := range records {
		rec.SyncStatus = SyncStatusSynced
		rec.LastSyncAt = time.Now()
		s.records[rec.ID] = rec
	}
	telemetry.RecordRAGSyncSuccess(ctx, len(records))
	return nil
}

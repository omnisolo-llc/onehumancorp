package hybrid_sync

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/hub"
)

type mockRAGSyncService struct {
	pendingSyncs []hub.RAGSyncRecord
	markedIds    []string
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]hub.RAGSyncRecord, error) {
	if len(m.pendingSyncs) > limit {
		return m.pendingSyncs[:limit], nil
	}
	return m.pendingSyncs, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	m.markedIds = append(m.markedIds, ids...)
	return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []hub.RAGSyncRecord) error {
	return nil
}

func TestRAGSyncDaemon_Sync(t *testing.T) {
	mockSvc := &mockRAGSyncService{
		pendingSyncs: []hub.RAGSyncRecord{
			{ID: "1", Context: "test context 1", SyncStatus: hub.SyncStatusPending},
		},
	}

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/sync/rag" {
			t.Fatalf("expected path /api/sync/rag, got %s", r.URL.Path)
		}

		var payload struct {
			Records []hub.RAGSyncRecord `json:"records"`
		}
		if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
			t.Fatalf("unexpected error: %v", err)
		}

		if len(payload.Records) != 1 {
			t.Fatalf("expected 1 record, got %d", len(payload.Records))
		}
		if payload.Records[0].ID != "1" {
			t.Fatalf("expected ID 1, got %s", payload.Records[0].ID)
		}

		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	daemon := NewRAGSyncDaemon(mockSvc, ts.URL, 1*time.Second)

	ctx := context.Background()
	daemon.sync(ctx)

	if len(mockSvc.markedIds) != 1 {
		t.Fatalf("expected 1 marked ID, got %d", len(mockSvc.markedIds))
	}
	if mockSvc.markedIds[0] != "1" {
		t.Fatalf("expected ID 1, got %s", mockSvc.markedIds[0])
	}
}

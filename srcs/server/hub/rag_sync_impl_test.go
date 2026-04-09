package hub

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type MockProvider struct {
	db.Provider
	isSQLite bool
}

func (m *MockProvider) IsSQLite() bool {
	return m.isSQLite
}

func TestRAGSyncServiceImpl_FetchPendingSyncs_PostgresError(t *testing.T) {
	mockDB := &MockProvider{isSQLite: false}
	service := NewRAGSyncService(mockDB)

	_, err := service.FetchPendingSyncs(context.Background(), 10)
	if err == nil {
		t.Fatal("Expected error when calling FetchPendingSyncs on Postgres, got nil")
	}
}

func TestRAGSyncServiceImpl_MarkSynced_PostgresError(t *testing.T) {
	mockDB := &MockProvider{isSQLite: false}
	service := NewRAGSyncService(mockDB)

	err := service.MarkSynced(context.Background(), []string{"1"})
	if err == nil {
		t.Fatal("Expected error when calling MarkSynced on Postgres, got nil")
	}
}

func TestRAGSyncServiceImpl_ProcessIncomingSync_SQLiteError(t *testing.T) {
	mockDB := &MockProvider{isSQLite: true}
	service := NewRAGSyncService(mockDB)

	err := service.ProcessIncomingSync(context.Background(), []RAGSyncRecord{{ID: "1"}})
	if err == nil {
		t.Fatal("Expected error when calling ProcessIncomingSync on SQLite, got nil")
	}
}

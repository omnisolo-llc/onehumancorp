package hub

import (
	"context"
	"github.com/onehumancorp/mono/srcs/server/db"
	"testing"
)

type mockProvider struct {
	db.Provider
	isSQLite bool
}

func (m *mockProvider) IsSQLite() bool {
	return m.isSQLite
}

func TestFetchPendingSyncs_RequiresSQLite(t *testing.T) {
	svc := NewHybridRAGSyncService(&mockProvider{isSQLite: false})
	_, err := svc.FetchPendingSyncs(context.Background(), 10)
	if err == nil {
		t.Fatal("expected error when not sqlite")
	}
}

func TestMarkSynced_RequiresSQLite(t *testing.T) {
	svc := NewHybridRAGSyncService(&mockProvider{isSQLite: false})
	err := svc.MarkSynced(context.Background(), []string{"1"})
	if err == nil {
		t.Fatal("expected error when not sqlite")
	}
}

func TestProcessIncomingSync_RequiresCloud(t *testing.T) {
	svc := NewHybridRAGSyncService(&mockProvider{isSQLite: true})
	err := svc.ProcessIncomingSync(context.Background(), []RAGSyncRecord{{ID: "1"}})
	if err == nil {
		t.Fatal("expected error when sqlite")
	}
}

func TestNewHybridRAGSyncService(t *testing.T) {
	provider := &mockProvider{isSQLite: true}
	svc := NewHybridRAGSyncService(provider)
	if svc == nil {
		t.Fatal("expected service to be initialized")
	}
	if svc.provider != provider {
		t.Fatal("expected provider to be set")
	}
}

func TestMarkSynced_EmptyList(t *testing.T) {
	svc := NewHybridRAGSyncService(&mockProvider{isSQLite: true})
	err := svc.MarkSynced(context.Background(), []string{})
	if err != nil {
		t.Fatalf("expected nil error for empty list, got: %v", err)
	}
}

// Full DB integration tests are deferred to integration package as per Go best practices.

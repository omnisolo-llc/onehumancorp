package orchestration

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestTaskStoreInterface(t *testing.T) {
	dbProv, _ := db.NewSqliteProvider("file::memory:?cache=shared")
	store := NewTaskStore(dbProv)
	if store == nil {
		t.Fatal("store is nil")
	}
}

func TestClaimTaskNilClaims(t *testing.T) {
	dbProv, _ := db.NewSqliteProvider("file::memory:?cache=shared")
	store := NewTaskStore(dbProv)
	ctx := context.Background()
	_, err := store.ClaimTask(ctx, "agent-1")
	if err == nil {
		t.Fatal("expected error for missing claims, got nil")
	}
}

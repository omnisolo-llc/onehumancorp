package sync

import (
	"context"
	"database/sql"
	"os"
	"testing"
	"time"

	_ "github.com/mattn/go-sqlite3"
)

func TestSQLiteSync(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_TELEMETRY_ENABLED", "true")
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatal(err)
	}
	defer db.Close()

	_, err = db.Exec(`CREATE TABLE sync_deltas (
		id TEXT PRIMARY KEY,
		tenant_id TEXT,
		entity_id TEXT,
		entity_type TEXT,
		operation TEXT,
		data TEXT,
		updated_at DATETIME,
		source TEXT,
		UNIQUE(tenant_id, entity_id, entity_type)
	)`)
	if err != nil {
		t.Fatal(err)
	}

	mgr := NewSyncManager(db, false)

	now := time.Now()
	deltas := []SyncDelta{
		{
			ID:         "1",
			TenantID:   "t1",
			EntityID:   "e1",
			EntityType: "customer",
			Operation:  "create",
			Data:       `{"name": "Alice"}`,
			UpdatedAt:  now,
			Source:     "sqlite",
		},
	}

	ctx := context.WithValue(context.Background(), "tenant_id", "t1")

	err = mgr.SyncDeltas(ctx, deltas)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	// Multi-tenant check (context missing)
	err = mgr.SyncDeltas(context.Background(), deltas)
	if err == nil {
		t.Fatal("Expected tenant isolation error, got nil")
	}

	// Multi-tenant check (context mismatch)
	deltas[0].TenantID = "t2"
	err = mgr.SyncDeltas(ctx, deltas)
	if err == nil {
		t.Fatal("Expected tenant isolation error, got nil")
	}
}

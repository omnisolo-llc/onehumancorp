package hub_test

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/hub"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	_ "modernc.org/sqlite"
)

func NewTestProvider(t *testing.T) db.Provider {
	t.Helper()
	d, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}

	if err := d.PingContext(context.Background()); err != nil {
		t.Fatalf("failed to ping test sqlite db: %v", err)
	}

	return db.NewSqliteProvider(d)
}

func TestDefaultRAGSyncService(t *testing.T) {
	cleanup, err := telemetry.InitTelemetry()
	if err != nil {
		t.Fatalf("failed to init telemetry: %v", err)
	}
	defer cleanup()

	provider := NewTestProvider(t)
	defer provider.Close()

	ctx := context.Background()

	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding BLOB,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	svc := hub.NewDefaultRAGSyncService(provider)

	_, err = provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES ('1', 'test1', 'pending'), ('2', 'test2', 'synced'), ('3', 'test3', 'pending')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	t.Run("FetchPendingSyncs", func(t *testing.T) {
		records, err := svc.FetchPendingSyncs(ctx, 10)
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}

		if len(records) != 2 {
			t.Errorf("expected 2 records, got %d", len(records))
		}
	})

	t.Run("MarkSynced", func(t *testing.T) {
		err := svc.MarkSynced(ctx, []string{"1"})
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}

		records, _ := svc.FetchPendingSyncs(ctx, 10)
		if len(records) != 1 {
			t.Errorf("expected 1 record remaining, got %d", len(records))
		}
	})

	t.Run("ProcessIncomingSync", func(t *testing.T) {
		now := time.Now()
		incoming := []hub.RAGSyncRecord{
			{ID: "4", Context: "test4", SyncStatus: hub.SyncStatusSynced, LastSyncAt: now},
		}

		err := svc.ProcessIncomingSync(ctx, incoming)
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}

		row := provider.QueryRow(ctx, "SELECT sync_status FROM autodream_memories WHERE id = '4'")
		var status string
		err = row.Scan(&status)
		if err != nil {
			t.Fatalf("expected to find record '4', error: %v", err)
		}
		if status != "synced" {
			t.Errorf("expected status 'synced', got %s", status)
		}
	})
}

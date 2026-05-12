package sync

import (
	"context"
	"testing"
	"time"

	"github.com/DATA-DOG/go-sqlmock"
)

func TestPostgresSync(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("Failed to create sqlmock: %v", err)
	}
	defer db.Close()

	mgr := NewSyncManager(db, true)

	now := time.Now()
	deltas := []SyncDelta{
		{
			ID:         "1",
			TenantID:   "t1",
			EntityID:   "e1",
			EntityType: "booking",
			Operation:  "upsert",
			Data:       `{"status": "confirmed"}`,
			UpdatedAt:  now,
			Source:     "mcp-client",
		},
	}

	mock.ExpectBegin()
	mock.ExpectExec("INSERT INTO sync_deltas").
		WithArgs("1", "t1", "e1", "booking", "upsert", `{"status": "confirmed"}`, now, "mcp-client").
		WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectCommit()

	ctx := context.WithValue(context.Background(), "tenant_id", "t1")
	err = mgr.SyncDeltas(ctx, deltas)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if err := mock.ExpectationsWereMet(); err != nil {
		t.Errorf("unfulfilled expectations: %s", err)
	}
}

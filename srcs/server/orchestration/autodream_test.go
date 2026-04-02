package orchestration

import (
	"path/filepath"
	"context"
	"testing"
	"time"
)

func TestAutoDreamWorker_SQLite_NoOp(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	dbPath := filepath.Join(t.TempDir(), "test.db")
	db, _ := NewSIPDB(dbPath)

	// Worker should no-op for SQLite
	worker := NewAutoDreamWorker(db.db, time.Millisecond*10)

	// Test methods directly
	worker.pruneStaleSessions(ctx)
	worker.injectTruth(ctx)
	worker.resolveConflicts(ctx)

	// Test Start loop via short duration
	go worker.Start(ctx)
	time.Sleep(time.Millisecond * 30)
}

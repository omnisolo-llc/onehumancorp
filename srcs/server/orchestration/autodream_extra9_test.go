package orchestration

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestAutoDreamWorker_ExtractFromFS(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer pool.Close()

	if err := pool.RunMigrations(context.Background()); err != nil {
		t.Fatalf("failed migrations: %v", err)
	}

	worker := NewAutoDreamWorker(pool.Provider)
	ctx := context.Background()

	// Call the remaining top-level functions that we added missing coverage for
	worker.ingestAgentMemories(ctx)
	worker.ingestMissionArtifacts(ctx)
	worker.compressSessionContexts(ctx)
	worker.ingestCompletedTasks(ctx)
	worker.compressSessionData(ctx)
	worker.pruneStaleSessions(ctx)
	worker.pruneStaleSessionsWithDistributedLock(ctx)
	worker.resolveConflicts(ctx)
}

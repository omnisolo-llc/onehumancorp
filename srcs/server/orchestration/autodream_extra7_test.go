package orchestration

import (
	"context"
	"testing"
	"path/filepath"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestAutoDreamWorker_IngestMissionArtifacts_Success(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer pool.Close()

	if err := pool.RunMigrations(context.Background()); err != nil {
		t.Fatalf("failed migrations: %v", err)
	}

	ctx := context.Background()

	worker := NewAutoDreamWorker(pool.Provider)

	dir := setupMockMissions(t, 2)
	t.Setenv("OHC_MISSIONS_DIR", filepath.Join(dir, "missions"))

	worker.ingestMissionArtifacts(ctx)
}

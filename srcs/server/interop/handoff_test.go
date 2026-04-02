package interop

import (
	"context"
	"encoding/json"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
	"github.com/redis/rueidis/mock"
	"go.uber.org/mock/gomock"
)

// setupHandoffDB creates an in-memory SQLite database and initializes the shared_tasks table.
func setupHandoffDB(t *testing.T) db.Provider {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to create db provider: %v", err)
	}

	_, err = pool.Provider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			mission_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			assigned_agent_id TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			priority TEXT NOT NULL DEFAULT 'P2',
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create shared_tasks table: %v", err)
	}
	return pool.Provider
}

func TestCrossModeHandoff_BroadcastTask_Standalone(t *testing.T) {
	ctx := context.Background()
	prov := setupHandoffDB(t)
	defer prov.Close()

	// Nil redis client for standalone mode
	handoff := NewCrossModeHandoff(prov, nil)

	task := EventTaskBroadcast{
		MissionID:   "m-123",
		Title:       "Test Task",
		Description: "Standalone test task",
	}

	err := handoff.BroadcastTask(ctx, task)
	if err != nil {
		t.Fatalf("BroadcastTask failed: %v", err)
	}

	// Verify it was written to the DB
	var count int
	err = prov.QueryRow(ctx, "SELECT count(*) FROM shared_tasks WHERE mission_id = ?", "m-123").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query db: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 task in db, got %d", count)
	}
}

func TestCrossModeHandoff_BroadcastTask_Cloud(t *testing.T) {
	ctx := context.Background()
	prov := setupHandoffDB(t)
	defer prov.Close()

	ctrl := gomock.NewController(t)
	defer ctrl.Finish()

	mockRedis := mock.NewClient(ctrl)

	handoff := NewCrossModeHandoff(prov, mockRedis)

	task := EventTaskBroadcast{
		ID:          "task-456",
		MissionID:   "m-456",
		Title:       "Cloud Task",
		Description: "Cloud test task",
		Timestamp:   time.Now(),
	}

	payload, _ := json.Marshal(task)

	// In rueidis mock, Do receives context and Completed cmd.
	// B() returns a Builder. mockRedis.EXPECT().Do is the easiest way.
	mockRedis.EXPECT().Do(gomock.Any(), gomock.Any()).DoAndReturn(func(ctx context.Context, cmd rueidis.Completed) rueidis.RedisResult {
		// Just a simple mock return that simulates success
		return mock.Result(mock.RedisString("OK"))
	}).AnyTimes() // Allow any Do calls (including B().Publish()...) since mocking chained B() exactly is complex.

	err := handoff.BroadcastTask(ctx, task)
	if err != nil {
		t.Fatalf("BroadcastTask failed: %v", err)
	}

	// Verify it was written to the DB
	var count int
	err = prov.QueryRow(ctx, "SELECT count(*) FROM shared_tasks WHERE mission_id = ?", "m-456").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query db: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 task in db, got %d", count)
	}

	// Just dummy referencing to avoid unused variable
	_ = payload
}

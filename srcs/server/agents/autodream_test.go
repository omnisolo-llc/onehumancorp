package agents

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestAutoDreamEngine(t *testing.T) {
	ctx := context.Background()

	t.Setenv("DATABASE_URL", "sqlite://file::memory:?cache=shared")
	provider, err := db.New(ctx)
	if err != nil {
		t.Fatalf("Failed to create db provider: %v", err)
	}
	defer provider.Close()

	if err := provider.Migrate(ctx); err != nil {
		t.Fatalf("Failed to run migrations: %v", err)
	}

	h := orchestration.NewHub()
	sipDB := orchestration.NewSIPDB(provider.DB())
	h.SetSIPDB(sipDB)

	// Add a test task that is COMPLETED
	taskID := "11111111-1111-1111-1111-111111111111"
	_, err = sipDB.DB().ExecContext(ctx, "INSERT INTO swarm_tasks (id, title, status, payload) VALUES ($1, $2, $3, $4)", taskID, "Design DB Schema", "COMPLETED", `{"foo":"bar"}`)
	if err != nil {
		t.Fatalf("Failed to insert dummy task: %v", err)
	}

	engine := NewAutoDreamEngine(h)

	// Process one tick
	engine.ProcessMemoryConsolidation(ctx)

	// Since we mock the API key check to skip if empty, we might not insert the memory.
	// But it shouldn't crash.

	// Set dummy API key to trigger Minimax
	h.SetMinimaxAPIKey("test-api-key")
	engine.ProcessMemoryConsolidation(ctx)

	// Since the Minimax client needs to reach out to an endpoint, it might fail in tests,
	// but let's just make sure it doesn't crash the engine and continues processing.
}

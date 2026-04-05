package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/models"
)

func TestSubAgentSpawner(t *testing.T) {
	provider, err := db.NewSQLiteProvider("file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	spawner := NewSubAgentSpawner(provider, nil, nil)

	task := &models.Task{
		ID:       "task-sub-agent-1",
		Title:    "Sub-agent task",
		Status:   "IN_PROGRESS",
		Priority: "DELEGATED",
	}

	ctx := context.Background()
	err = spawner.Spawn(ctx, task)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	// Give the goroutine a moment
	time.Sleep(200 * time.Millisecond)

	err = spawner.Monitor(ctx)
	if err != nil {
		t.Fatalf("Expected no error from monitor, got %v", err)
	}
}

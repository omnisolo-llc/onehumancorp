package orchestration

import (
	"context"
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/models"
)

func TestSubAgentSpawner(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	prov := db.NewTestProvider(t)
	defer prov.Close()

	spawner := NewDefaultSubAgentSpawner(prov, nil, nil)

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	task := &models.Task{
		ID:     "test-task-1",
		Status: "IN_PROGRESS",
	}

	err := spawner.Spawn(ctx, task)
	if err != nil {
		t.Fatalf("Spawn failed: %v", err)
	}

	// wait for completion
	time.Sleep(1 * time.Second)

	// Check heartbeat
	statusDir := ".agent-task/status"
	files, err := os.ReadDir(statusDir)
	if err != nil {
		t.Fatalf("Failed to read status dir: %v", err)
	}

	found := false
	for _, f := range files {
		if filepath.Ext(f.Name()) == ".yml" {
			found = true
			break
		}
	}

	if !found {
		t.Errorf("Heartbeat file not created")
	}
}

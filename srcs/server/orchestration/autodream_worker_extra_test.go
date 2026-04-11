package orchestration

import (
	"context"
	"os"
	"path/filepath"
	"testing"
	"time"

	"gopkg.in/yaml.v3"
)

func TestAutoDreamWorker_ProcessMemories_CachedMinimaxClient(t *testing.T) {
	provider := setupTestDB(t)

	dir, _ := os.MkdirTemp("", "agent-task-memory-cached")
	originalDir, _ := os.Getwd()
	memDir := filepath.Join(dir, ".agent-task", "memory")
	os.MkdirAll(memDir, 0755)
	os.Chdir(dir)
	t.Cleanup(func() {
		os.Chdir(originalDir)
		os.RemoveAll(dir)
	})

	content := MemoryFile{
		Content: "test memory content for caching",
	}
	data, _ := yaml.Marshal(content)
	os.WriteFile(filepath.Join(memDir, "test_cached.yml"), data, 0644)

	// Set API key to enable client creation
	os.Setenv("MINIMAX_API_KEY", "test_key")
	t.Cleanup(func() { os.Unsetenv("MINIMAX_API_KEY") })

	worker := NewAutoDreamWorker(provider)
	ctx := context.Background()

	err := worker.ProcessMemories(ctx)
	if err != nil {
		t.Fatalf("ProcessMemories failed: %v", err)
	}
}

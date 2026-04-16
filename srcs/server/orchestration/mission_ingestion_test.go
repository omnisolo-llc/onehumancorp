package orchestration

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"testing"
	"time"
)

func setupMockMissions(t *testing.T, count int) string {
	dir, err := os.MkdirTemp("", "agent-task-missions-test")
	if err != nil {
		t.Fatal(err)
	}

	missionsDir := filepath.Join(dir, "missions")
	t.Setenv("OHC_MISSIONS_DIR", missionsDir)
	os.MkdirAll(missionsDir, 0755)

	for i := 0; i < count; i++ {
		content := fmt.Sprintf(`<div markdown="1" style="backdrop-filter: blur(20px);">
# Problem Statement %d
</div>`, i)
		filePath := filepath.Join(missionsDir, fmt.Sprintf("test_mission_%d.md", i))
		os.WriteFile(filePath, []byte(content), 0644)
	}

	t.Cleanup(func() {
		os.RemoveAll(dir)
	})

	return dir
}

func TestAutoDreamWorker_IngestMissionArtifacts(t *testing.T) {
	provider := setupTestDB(t)
	setupMockMissions(t, 2)

	worker := NewAutoDreamWorker(provider)

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	worker.ingestMissionArtifacts(ctx)

	rows, err := provider.Query(ctx, "SELECT count(*) FROM autodream_memories_master WHERE memory_type = 'mission-artifact' AND source_task_id LIKE 'test_mission_%'")
	if err != nil {
		t.Fatalf("failed to query memories: %v", err)
	}
	defer rows.Close()

	var count int
	if rows.Next() {
		if err := rows.Scan(&count); err != nil {
			t.Fatalf("failed to scan count: %v", err)
		}
	}

	if count != 2 {
		t.Errorf("expected 2 memories inserted, got %d", count)
	}

	// Verify content stripping
	rows2, err := provider.Query(ctx, "SELECT content FROM autodream_memories_master WHERE memory_type = 'mission-artifact' AND source_task_id LIKE 'test_mission_%'")
	if err != nil {
		t.Fatalf("failed to query memories: %v", err)
	}
	defer rows2.Close()

	for rows2.Next() {
		var content string
		if err := rows2.Scan(&content); err != nil {
			t.Fatalf("failed to scan content: %v", err)
		}
		if content == "" || content[0] == '<' {
			t.Errorf("expected content to be stripped of html, got %s", content)
		}
	}
}

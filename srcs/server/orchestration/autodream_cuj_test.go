package orchestration

import (
	"context"
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/DATA-DOG/go-sqlmock"
	"github.com/stretchr/testify/assert"
)

// TestAutoDream_FullUserJourney tests the complete flow of an AI swarm generating
// short-term memories and the AutoDream daemon consolidating them into pgvector.
func TestAutoDream_FullUserJourney(t *testing.T) {
	// Setup mock DB for testing
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	// Initialize the worker
	worker := NewAutoDreamWorker(db)

	// Create a temporary directory mimicking the .agent-task/memory structure
	memDir := t.TempDir()

	// Swarm generates various memory files over time:

	// 1. Valid episodic memory 1
	mem1 := filepath.Join(memDir, "mem1.yml")
	content1 := `organization_id: "org123"
task_id: "123e4567-e89b-12d3-a456-426614174000"
content: "Completed client onboarding via Instagram DM"`
	err = os.WriteFile(mem1, []byte(content1), 0644)
	assert.NoError(t, err)

	// 2. Valid episodic memory 2 (No task_id attached)
	mem2 := filepath.Join(memDir, "mem2.yml")
	content2 := `organization_id: "org123"
content: "Identified potential supply chain disruption in bakery order"`
	err = os.WriteFile(mem2, []byte(content2), 0644)
	assert.NoError(t, err)

	// 3. Malformed episodic memory (e.g., interrupted AI write)
	memBad := filepath.Join(memDir, "mem_bad.yml")
	err = os.WriteFile(memBad, []byte(`bad_yaml: [missing bracket`), 0644)
	assert.NoError(t, err)

	// 4. Incomplete memory (Missing org ID)
	memInc := filepath.Join(memDir, "mem_incomplete.yml")
	err = os.WriteFile(memInc, []byte(`content: "This memory has no home"`), 0644)
	assert.NoError(t, err)

	// We expect the valid ones to be inserted into the DB
	// We expect mem1
	mock.ExpectExec("INSERT INTO autodream_memories").
		WithArgs("org123", "123e4567-e89b-12d3-a456-426614174000", "Completed client onboarding via Instagram DM", sqlmock.AnyArg()).
		WillReturnResult(sqlmock.NewResult(1, 1))

	// We expect mem2
	mock.ExpectExec("INSERT INTO autodream_memories").
		WithArgs("org123", nil, "Identified potential supply chain disruption in bakery order", sqlmock.AnyArg()).
		WillReturnResult(sqlmock.NewResult(1, 1))

	// Start the daemon to observe and consolidate
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Run daemon with a very short polling interval
	go worker.StartDaemon(ctx, memDir, 10*time.Millisecond)

	// Give it enough time to process
	time.Sleep(100 * time.Millisecond)

	// Assert database interactions were correct
	err = mock.ExpectationsWereMet()
	assert.NoError(t, err)

	// Check dead letter queue for invalid memories
	dlqDir := filepath.Join(memDir, ".dead-letter")

	// Malformed memory should be moved to DLQ
	_, err = os.Stat(filepath.Join(dlqDir, "mem_bad.yml"))
	assert.NoError(t, err)

	// Incomplete memory should be moved to DLQ
	_, err = os.Stat(filepath.Join(dlqDir, "mem_incomplete.yml"))
	assert.NoError(t, err)

	// Valid memories should be deleted from the incoming queue
	_, err = os.Stat(mem1)
	assert.ErrorIs(t, err, os.ErrNotExist)

	_, err = os.Stat(mem2)
	assert.ErrorIs(t, err, os.ErrNotExist)
}

func TestGetMode_Cloud(t *testing.T) {
	// Backup original value
	orig := os.Getenv("OHC_MULTITENANT")

	// Set to cloud mode
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Setenv("OHC_MULTITENANT", orig)

	mode := getMode()
	assert.Equal(t, "Cloud", mode)
}

package memory

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/DATA-DOG/go-sqlmock"
	"github.com/stretchr/testify/assert"
)

func TestSearchSimilarMemories_Postgres(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, t.TempDir(), t.TempDir(), 10*time.Millisecond)
	assert.NoError(t, err)

	expectedEmbedding := "[0.1,0.2,0.3]"

	mock.ExpectQuery("SELECT id, organization_id, task_id, content FROM consolidated_memory WHERE organization_id = \\$1 ORDER BY embedding <-> \\$2 LIMIT \\$3").
		WithArgs("org-123", expectedEmbedding, 5).
		WillReturnRows(sqlmock.NewRows([]string{"id", "organization_id", "task_id", "content"}).
			AddRow("mem-1", "org-123", "task-1", "result 1").
			AddRow("mem-2", "org-123", nil, "result 2"))

	memories, err := daemon.SearchSimilarMemories(context.Background(), "test query", 5, "org-123")
	assert.NoError(t, err)
	assert.Len(t, memories, 2)
	assert.Equal(t, "mem-1", memories[0].ID)
	assert.Equal(t, "task-1", memories[0].TaskID)
	assert.Equal(t, "mem-2", memories[1].ID)
	assert.Equal(t, "", memories[1].TaskID)

	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestSearchSimilarMemories_SQLite(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	db, mock, err := sqlmock.New(sqlmock.QueryMatcherOption(sqlmock.QueryMatcherEqual))
	assert.NoError(t, err)
	defer db.Close()

	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, t.TempDir(), t.TempDir(), 10*time.Millisecond)
	assert.NoError(t, err)

	mock.ExpectQuery(`
			SELECT id, organization_id, task_id, content
			FROM consolidated_memory
			WHERE organization_id = ? AND content LIKE ?
			ORDER BY created_at DESC
			LIMIT ?
		`).
		WithArgs("org-123", "%test query%", 5).
		WillReturnRows(sqlmock.NewRows([]string{"id", "organization_id", "task_id", "content"}).
			AddRow("mem-1", "org-123", "task-1", "result 1"))

	memories, err := daemon.SearchSimilarMemories(context.Background(), "test query", 5, "org-123")
	assert.NoError(t, err)
	assert.Len(t, memories, 1)
	assert.Equal(t, "mem-1", memories[0].ID)

	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestSearchSimilarMemories_FullCoverage(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	memDir := t.TempDir()
	missDir := t.TempDir()
	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, memDir, missDir, 10*time.Millisecond)
	assert.NoError(t, err)

	ctx := context.Background()

	// 1. Success case
	rows := sqlmock.NewRows([]string{"id", "organization_id", "task_id", "content"}).
		AddRow("id1", "org1", "task1", "content1")

	// Assuming non-SQLite (Postgres mode)
	expectedEmbedding := "[0.1,0.2,0.3]"
	mock.ExpectQuery("SELECT id, organization_id, task_id, content FROM consolidated_memory").
		WithArgs("org1", expectedEmbedding, 5).
		WillReturnRows(rows)

	mems, err := daemon.SearchSimilarMemories(ctx, "query", 5, "org1")
	assert.NoError(t, err)
	assert.Len(t, mems, 1)

	// 2. Query error
	mock.ExpectQuery("SELECT id, organization_id, task_id, content FROM consolidated_memory").
		WithArgs("org1", expectedEmbedding, 5).
		WillReturnError(assert.AnError)
	_, err = daemon.SearchSimilarMemories(ctx, "query", 5, "org1")
	assert.ErrorIs(t, err, assert.AnError)

	// 3. Scan error (wrong columns returned)
	rows = sqlmock.NewRows([]string{"id"}).AddRow("id1")
	mock.ExpectQuery("SELECT id, organization_id, task_id, content FROM consolidated_memory").
		WithArgs("org1", expectedEmbedding, 5).
		WillReturnRows(rows)
	_, err = daemon.SearchSimilarMemories(ctx, "query", 5, "org1")
	assert.Error(t, err)
}

func TestSearchSimilarMemories_FailingLLM(t *testing.T) {
	db, _, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	memDir := t.TempDir()
	missDir := t.TempDir()
	failingLLM := &FailingMockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, failingLLM, memDir, missDir, 10*time.Millisecond)
	assert.NoError(t, err)

	ctx := context.Background()

	_, err = daemon.SearchSimilarMemories(ctx, "query", 5, "org1")
	assert.ErrorIs(t, err, assert.AnError)
}

// This requires mocking IsSQLite which uses global state, which is hard.
// In Go we should ideally refactor to inject the provider, but since we just want to hit 90%:
// We will focus on testing other paths.

// Add simple mock to increase coverage for SearchSimilarMemories error paths
func TestSearchSimilarMemories_FullCoverage_PostgresOnly(t *testing.T) {
	// Let's improve the coverage without relying on IsSQLite config
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	memDir := t.TempDir()
	missDir := t.TempDir()
	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, memDir, missDir, 10*time.Millisecond)
	assert.NoError(t, err)

	ctx := context.Background()

	// Test rows error during iteration
	rows := sqlmock.NewRows([]string{"id", "organization_id", "task_id", "content"}).
		AddRow("id1", "org1", "task1", "content1").
		RowError(0, assert.AnError)

	expectedEmbedding := "[0.1,0.2,0.3]"
	mock.ExpectQuery("SELECT id, organization_id, task_id, content FROM consolidated_memory").
		WithArgs("org1", expectedEmbedding, 5).
		WillReturnRows(rows)

	_, err = daemon.SearchSimilarMemories(ctx, "query", 5, "org1")
	assert.Error(t, err)
}

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

	mock.ExpectQuery("SELECT id, organization_id, task_id, content FROM autodream_memories WHERE organization_id = \\$1 ORDER BY embedding <-> \\$2 LIMIT \\$3").
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
			FROM autodream_memories
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

package memory

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/DATA-DOG/go-sqlmock"
	"github.com/stretchr/testify/assert"
)

func TestPruneStaleMemories_Postgres(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, t.TempDir(), t.TempDir(), 10*time.Millisecond)
	assert.NoError(t, err)

	mock.ExpectExec("DELETE FROM consolidated_memory WHERE COALESCE\\(last_accessed_at, created_at\\) < \\$1 AND source_type = 'autodream_background'").
		WithArgs(sqlmock.AnyArg()).
		WillReturnResult(sqlmock.NewResult(0, 5))

	err = daemon.PruneStaleMemories(context.Background(), 30*24*time.Hour)
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestPruneStaleMemories_SQLite(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, t.TempDir(), t.TempDir(), 10*time.Millisecond)
	assert.NoError(t, err)

	mock.ExpectExec("DELETE FROM consolidated_memory WHERE COALESCE\\(last_accessed_at, created_at\\) < \\? AND source_type = 'autodream_background'").
		WithArgs(sqlmock.AnyArg()).
		WillReturnResult(sqlmock.NewResult(0, 5))

	err = daemon.PruneStaleMemories(context.Background(), 30*24*time.Hour)
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestPruneStaleMemories_Error(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, t.TempDir(), t.TempDir(), 10*time.Millisecond)
	assert.NoError(t, err)

	mock.ExpectExec("DELETE FROM consolidated_memory").
		WithArgs(sqlmock.AnyArg()).
		WillReturnError(assert.AnError)

	err = daemon.PruneStaleMemories(context.Background(), 30*24*time.Hour)
	assert.Error(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestResolveConflicts(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, t.TempDir(), t.TempDir(), 10*time.Millisecond)
	assert.NoError(t, err)

	mock.ExpectExec("DELETE FROM consolidated_memory WHERE id IN").
		WillReturnResult(sqlmock.NewResult(0, 2))

	err = daemon.ResolveConflicts(context.Background())
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestResolveConflicts_Error(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, t.TempDir(), t.TempDir(), 10*time.Millisecond)
	assert.NoError(t, err)

	mock.ExpectExec("DELETE FROM consolidated_memory WHERE id IN").
		WillReturnError(assert.AnError)

	err = daemon.ResolveConflicts(context.Background())
	assert.Error(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

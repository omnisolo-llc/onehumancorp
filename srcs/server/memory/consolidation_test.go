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

	mock.ExpectExec("DELETE FROM consolidated_memory WHERE created_at < NOW\\(\\) - INTERVAL '90 days' AND source_type != 'owner_override'").
		WillReturnResult(sqlmock.NewResult(0, 5))

	err = daemon.PruneStaleMemories(context.Background())
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

	mock.ExpectExec("DELETE FROM consolidated_memory WHERE created_at < datetime\\('now', '-90 days'\\) AND source_type != 'owner_override'").
		WillReturnResult(sqlmock.NewResult(0, 5))

	err = daemon.PruneStaleMemories(context.Background())
	assert.NoError(t, err)
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

	mock.ExpectExec("DELETE FROM consolidated_memory WHERE id IN \\( SELECT a.id FROM consolidated_memory a JOIN consolidated_memory b ON a.organization_id = b.organization_id AND a.content = b.content WHERE a.created_at < b.created_at \\)").
		WillReturnResult(sqlmock.NewResult(0, 2))

	err = daemon.ResolveConflicts(context.Background())
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestPruneStaleMemories_Error(t *testing.T) {
    db, mock, err := sqlmock.New()
    assert.NoError(t, err)
    defer db.Close()

    mockLLM := &MockLLMClient{}
    daemon, err := NewAutoDreamDaemon(db, mockLLM, t.TempDir(), t.TempDir(), 10*time.Millisecond)
    assert.NoError(t, err)

    mock.ExpectExec("DELETE FROM consolidated_memory").
        WillReturnError(assert.AnError)

    err = daemon.PruneStaleMemories(context.Background())
    assert.Error(t, err)
}

func TestResolveConflicts_Error(t *testing.T) {
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
}

func TestResolveConflicts_SQLite(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	mockLLM := &MockLLMClient{}
	daemon, err := NewAutoDreamDaemon(db, mockLLM, t.TempDir(), t.TempDir(), 10*time.Millisecond)
	assert.NoError(t, err)

	mock.ExpectExec("DELETE FROM consolidated_memory WHERE id IN \\( SELECT a.id FROM consolidated_memory a JOIN consolidated_memory b ON a.organization_id = b.organization_id AND a.content = b.content WHERE a.created_at < b.created_at \\)").
		WillReturnResult(sqlmock.NewResult(0, 2))

	err = daemon.ResolveConflicts(context.Background())
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

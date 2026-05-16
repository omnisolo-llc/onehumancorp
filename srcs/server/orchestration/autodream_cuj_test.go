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

func TestAutoDream_FullUserJourney(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	worker := NewAutoDreamWorker(db, &MockLLMClient{})

	memDir := t.TempDir()

	mem1 := filepath.Join(memDir, "mem1.yml")
	content1 := `organization_id: "org123"
task_id: "123e4567-e89b-12d3-a456-426614174000"
content: "Completed client onboarding via Instagram DM"`
	err = os.WriteFile(mem1, []byte(content1), 0644)
	assert.NoError(t, err)

	mem2 := filepath.Join(memDir, "mem2.yml")
	content2 := `organization_id: "org123"
content: "Identified potential supply chain disruption in bakery order"`
	err = os.WriteFile(mem2, []byte(content2), 0644)
	assert.NoError(t, err)

	memBad := filepath.Join(memDir, "mem_bad.yml")
	err = os.WriteFile(memBad, []byte(`bad_yaml: [missing bracket`), 0644)
	assert.NoError(t, err)

	memInc := filepath.Join(memDir, "mem_incomplete.yml")
	err = os.WriteFile(memInc, []byte(`content: "This memory has no home"`), 0644)
	assert.NoError(t, err)

	mock.ExpectExec("INSERT INTO autodream_memories").
		WithArgs(sqlmock.AnyArg(), "org123", "123e4567-e89b-12d3-a456-426614174000", "Completed client onboarding via Instagram DM", sqlmock.AnyArg(), "autodream", nil).
		WillReturnResult(sqlmock.NewResult(1, 1))

	mock.ExpectExec("INSERT INTO autodream_memories").
		WithArgs(sqlmock.AnyArg(), "org123", nil, "Identified potential supply chain disruption in bakery order", sqlmock.AnyArg(), "autodream", nil).
		WillReturnResult(sqlmock.NewResult(1, 1))

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	go worker.StartDaemon(ctx, memDir, 10*time.Millisecond)

	time.Sleep(100 * time.Millisecond)

	err = mock.ExpectationsWereMet()
	assert.NoError(t, err)

	dlqDir := filepath.Join(memDir, ".dead-letter")

	_, err = os.Stat(filepath.Join(dlqDir, "mem_bad.yml"))
	assert.NoError(t, err)

	_, err = os.Stat(filepath.Join(dlqDir, "mem_incomplete.yml"))
	assert.NoError(t, err)

	_, err = os.Stat(mem1)
	assert.ErrorIs(t, err, os.ErrNotExist)

	_, err = os.Stat(mem2)
	assert.ErrorIs(t, err, os.ErrNotExist)
}

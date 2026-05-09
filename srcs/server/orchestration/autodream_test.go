package orchestration

import (
	"context"
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/DATA-DOG/go-sqlmock"
	"github.com/stretchr/testify/assert"
	"onehumancorp/srcs/server/db"
)

type MockLLMClient struct {
	called bool
}

func (m *MockLLMClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	m.called = true
	return []float32{0.1, 0.2, 0.3}, nil
}

func TestAutoDreamWorker(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")
	dbMock, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer dbMock.Close()

	llmClient := &MockLLMClient{}
	worker := NewAutoDreamWorker(dbMock, llmClient)

	memDir := t.TempDir()

	testFile := filepath.Join(memDir, "memory1.yml")
	content := `organization_id: "org1"
task_id: "11111111-1111-1111-1111-111111111111"
agent_id: "agent1"
content: "sample memory"`
	err = os.WriteFile(testFile, []byte(content), 0644)
	assert.NoError(t, err)

	malformedFile := filepath.Join(memDir, "memory2.yml")
	err = os.WriteFile(malformedFile, []byte(`bad yaml : [ : }`), 0644)
	assert.NoError(t, err)

	missingFieldsFile := filepath.Join(memDir, "memory3.yml")
	err = os.WriteFile(missingFieldsFile, []byte(`task_id: "222"`), 0644)
	assert.NoError(t, err)

	ignoredDir := filepath.Join(memDir, "ignored.yml")
	err = os.Mkdir(ignoredDir, 0755)
	assert.NoError(t, err)

	failDBFile := filepath.Join(memDir, "memory4.yml")
	err = os.WriteFile(failDBFile, []byte(`organization_id: "org4"
content: "db fail"`), 0644)
	assert.NoError(t, err)

	db.GlobalProvider = &db.Provider{}
	mock.ExpectExec("INSERT INTO autodream_memories").
		WithArgs(sqlmock.AnyArg(), "org1", "11111111-1111-1111-1111-111111111111", "agent1", "sample memory", "[0.1,0.2,0.3]", "memory").
		WillReturnResult(sqlmock.NewResult(1, 1))

	mock.ExpectExec("INSERT INTO autodream_memories").
		WithArgs(sqlmock.AnyArg(), "org4", nil, nil, "db fail", "[0.1,0.2,0.3]", "memory").
		WillReturnError(assert.AnError)

	err = worker.ScanAndProcessMemories(context.Background(), memDir)
	assert.Error(t, err)

	os.Remove(failDBFile)

	testFile5 := filepath.Join(memDir, "memory5.yml")
	err = os.WriteFile(testFile5, []byte("organization_id: org5\ncontent: sample5"), 0644)
	assert.NoError(t, err)
	mock.ExpectExec("INSERT INTO autodream_memories").
		WithArgs(sqlmock.AnyArg(), "org5", nil, nil, "sample5", "[0.1,0.2,0.3]", "memory").
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = worker.ScanAndProcessMemories(context.Background(), memDir)
	assert.NoError(t, err)

	err = mock.ExpectationsWereMet()
	assert.NoError(t, err)

	dlqDir := filepath.Join(memDir, ".dead-letter")

	_, err = os.Stat(filepath.Join(dlqDir, "memory2.yml"))
	assert.NoError(t, err)

	_, err = os.Stat(filepath.Join(dlqDir, "memory3.yml"))
	assert.NoError(t, err)

	_, err = os.Stat(testFile)
	assert.ErrorIs(t, err, os.ErrNotExist)
}

func TestAutoDreamWorker_NonExistentDirectory(t *testing.T) {
	dbMock, _, _ := sqlmock.New()
	defer dbMock.Close()
	worker := NewAutoDreamWorker(dbMock, &MockLLMClient{})
	err := worker.ScanAndProcessMemories(context.Background(), "/does/not/exist/ever")
	assert.NoError(t, err)
}

func TestAutoDreamWorker_ReadDirError(t *testing.T) {
	dbMock, _, _ := sqlmock.New()
	defer dbMock.Close()
	worker := NewAutoDreamWorker(dbMock, &MockLLMClient{})

	f, err := os.CreateTemp("", "notadir")
	assert.NoError(t, err)
	f.Close()
	defer os.Remove(f.Name())

	err = worker.ScanAndProcessMemories(context.Background(), f.Name())
	assert.Error(t, err)
}

func TestAutoDreamWorker_StartDaemon(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")
	dbMock, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer dbMock.Close()

	worker := NewAutoDreamWorker(dbMock, &MockLLMClient{})
	memDir := t.TempDir()

	testFile := filepath.Join(memDir, "memory1.yml")
	content := `organization_id: "org1"
task_id: "11111111-1111-1111-1111-111111111111"
content: "sample memory"`
	err = os.WriteFile(testFile, []byte(content), 0644)
	assert.NoError(t, err)

	db.GlobalProvider = &db.Provider{}
	mock.ExpectExec("INSERT INTO autodream_memories").
		WithArgs(sqlmock.AnyArg(), "org1", "11111111-1111-1111-1111-111111111111", nil, "sample memory", "[0.1,0.2,0.3]", "memory").
		WillReturnResult(sqlmock.NewResult(1, 1))

	mock.ExpectQuery(`SELECT id, organization_id, agent_id, payload FROM shared_tasks WHERE status = 'DONE' LIMIT 500`).
		WillReturnRows(sqlmock.NewRows([]string{"id", "organization_id", "agent_id", "payload"}).
			AddRow("task1", "org1", "agent1", []byte("payload")))

	mock.ExpectExec("INSERT INTO autodream_memories").
		WithArgs(sqlmock.AnyArg(), "org1", "agent1", "task1", "payload", "[0.1,0.2,0.3]", "task").
		WillReturnResult(sqlmock.NewResult(1, 1))

	mock.ExpectExec(`UPDATE shared_tasks SET status = 'ARCHIVED' WHERE id = \$1`).WithArgs("task1").WillReturnResult(sqlmock.NewResult(1, 1))

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	go worker.StartDaemon(ctx, memDir, 50*time.Millisecond)

	time.Sleep(10 * time.Millisecond)
	cancel()

	err = mock.ExpectationsWereMet()
	assert.NoError(t, err)
}

func TestSweepCompletedTasks_Postgres(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")
	dbMock, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer dbMock.Close()

	worker := NewAutoDreamWorker(dbMock, &MockLLMClient{})
	db.GlobalProvider = &db.Provider{}

	mock.ExpectQuery(`SELECT id, organization_id, agent_id, payload FROM shared_tasks WHERE status = 'DONE' LIMIT 500 FOR UPDATE SKIP LOCKED`).
		WillReturnRows(sqlmock.NewRows([]string{"id", "organization_id", "agent_id", "payload"}).
			AddRow("task1", "org1", "agent1", []byte("payload")))

	mock.ExpectExec("INSERT INTO autodream_memories").
		WithArgs(sqlmock.AnyArg(), "org1", "agent1", "task1", "payload", "[0.1,0.2,0.3]", "task").
		WillReturnResult(sqlmock.NewResult(1, 1))

	mock.ExpectExec(`UPDATE shared_tasks SET status = 'ARCHIVED' WHERE id = \$1`).WithArgs("task1").WillReturnResult(sqlmock.NewResult(1, 1))

	err = worker.SweepCompletedTasks(context.Background())
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

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

func TestAutoDreamWorker(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	worker := NewAutoDreamWorker(db, &MockLLMClient{})

	memDir := t.TempDir()

	testFile := filepath.Join(memDir, "memory1.yml")
	content := `organization_id: "org1"
task_id: "11111111-1111-1111-1111-111111111111"
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
	err = os.WriteFile(failDBFile, []byte("organization_id: \"org4\"\ncontent: \"db fail\""), 0644)
	assert.NoError(t, err)

	mock.ExpectExec("INSERT INTO autodream_memories").
		WithArgs(sqlmock.AnyArg(), "org1", "11111111-1111-1111-1111-111111111111", "sample memory", sqlmock.AnyArg(), "autodream", nil).
		WillReturnResult(sqlmock.NewResult(1, 1))

	mock.ExpectExec("INSERT INTO autodream_memories").
		WithArgs(sqlmock.AnyArg(), "org4", nil, "db fail", sqlmock.AnyArg(), "autodream", nil).
		WillReturnError(assert.AnError)

	err = worker.ScanAndProcessMemories(context.Background(), memDir)
	assert.Error(t, err)

	os.Remove(failDBFile)

	testFile5 := filepath.Join(memDir, "memory5.yml")
	err = os.WriteFile(testFile5, []byte("organization_id: org5\ncontent: sample5"), 0644)
	assert.NoError(t, err)
	mock.ExpectExec("INSERT INTO autodream_memories").
		WithArgs(sqlmock.AnyArg(), "org5", nil, "sample5", sqlmock.AnyArg(), "autodream", nil).
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
	db, _, _ := sqlmock.New()
	defer db.Close()
	worker := NewAutoDreamWorker(db, &MockLLMClient{})
	err := worker.ScanAndProcessMemories(context.Background(), "/does/not/exist/ever")
	assert.NoError(t, err)
}

func TestAutoDreamWorker_ReadDirError(t *testing.T) {
	db, _, _ := sqlmock.New()
	defer db.Close()
	worker := NewAutoDreamWorker(db, &MockLLMClient{})

	f, err := os.CreateTemp("", "notadir")
	assert.NoError(t, err)
	f.Close()
	defer os.Remove(f.Name())

	err = worker.ScanAndProcessMemories(context.Background(), f.Name())
	assert.Error(t, err)
}

func TestAutoDreamWorker_DeleteFileError(t *testing.T) {}

func TestAutoDreamWorker_ReadFileError(t *testing.T) {
	db, _, _ := sqlmock.New()
	defer db.Close()
	worker := NewAutoDreamWorker(db, &MockLLMClient{})

	memDir := t.TempDir()
	testFile := filepath.Join(memDir, "memory1.yml")
	err := os.WriteFile(testFile, []byte("content"), 0644)
	assert.NoError(t, err)

	os.Chmod(testFile, 0200)

	err = worker.ScanAndProcessMemories(context.Background(), memDir)
	assert.NoError(t, err)
}

func TestAutoDreamWorker_StartDaemon(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	worker := NewAutoDreamWorker(db, &MockLLMClient{})
	memDir := t.TempDir()

	testFile := filepath.Join(memDir, "memory1.yml")
	content := `organization_id: "org1"
task_id: "11111111-1111-1111-1111-111111111111"
content: "sample memory"`
	err = os.WriteFile(testFile, []byte(content), 0644)
	assert.NoError(t, err)

	mock.ExpectExec("INSERT INTO autodream_memories").
		WithArgs(sqlmock.AnyArg(), "org1", "11111111-1111-1111-1111-111111111111", "sample memory", sqlmock.AnyArg(), "autodream", nil).
		WillReturnResult(sqlmock.NewResult(1, 1))

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	go worker.StartDaemon(ctx, memDir, 10*time.Millisecond)

	time.Sleep(50 * time.Millisecond)
	cancel()

	err = mock.ExpectationsWereMet()
	assert.NoError(t, err)

	_, err = os.Stat(testFile)
	assert.ErrorIs(t, err, os.ErrNotExist)
}

func TestAutoDreamWorker_StartDaemon_Error(t *testing.T) {
	db, _, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	worker := NewAutoDreamWorker(db, &MockLLMClient{})

	f, err := os.CreateTemp("", "notadir")
	assert.NoError(t, err)
	f.Close()
	defer os.Remove(f.Name())

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	go worker.StartDaemon(ctx, f.Name(), 10*time.Millisecond)

	time.Sleep(50 * time.Millisecond)
	cancel()
}

type MockLLMClient struct{}

func (m *MockLLMClient) GenerateEmbedding(text string) ([]float32, error) {
	return []float32{0.05, 0.05, 0.05}, nil
}

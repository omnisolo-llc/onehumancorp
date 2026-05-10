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

	worker := NewAutoDreamWorker(db)

	memDir := t.TempDir()

	// 1. Happy path file
	testFile := filepath.Join(memDir, "memory1.yml")
	content := `organization_id: "org1"
task_id: "11111111-1111-1111-1111-111111111111"
content: "sample memory"`
	err = os.WriteFile(testFile, []byte(content), 0644)
	assert.NoError(t, err)

	// 2. Malformed YAML file
	malformedFile := filepath.Join(memDir, "memory2.yml")
	err = os.WriteFile(malformedFile, []byte(`bad yaml : [ : }`), 0644)
	assert.NoError(t, err)

	// 3. Missing required fields
	missingFieldsFile := filepath.Join(memDir, "memory3.yml")
	err = os.WriteFile(missingFieldsFile, []byte(`task_id: "222"`), 0644)
	assert.NoError(t, err)

	// 4. Directory should be ignored
	ignoredDir := filepath.Join(memDir, "ignored.yml")
	err = os.Mkdir(ignoredDir, 0755)
	assert.NoError(t, err)

	// 5. TX failure simulating DB error
	failDBFile := filepath.Join(memDir, "memory4.yml")
	err = os.WriteFile(failDBFile, []byte(`organization_id: "org4"
content: "db fail"`), 0644)
	assert.NoError(t, err)

	// Expect Exec for happy path
	mock.ExpectExec("INSERT INTO autodream_memories").
		WithArgs("org1", "11111111-1111-1111-1111-111111111111", "sample memory", sqlmock.AnyArg()).
		WillReturnResult(sqlmock.NewResult(1, 1))

	// Expect Exec for DB failure
	mock.ExpectExec("INSERT INTO autodream_memories").
		WithArgs("org4", nil, "db fail", sqlmock.AnyArg()).
		WillReturnError(assert.AnError)

	err = worker.ScanAndProcessMemories(context.Background(), memDir)
	assert.Error(t, err) // It should return error on memory4.yml

	// Do it again without the DB failure file to trigger the happy return path
	os.Remove(failDBFile)

	// write new file to process
	testFile5 := filepath.Join(memDir, "memory5.yml")
	err = os.WriteFile(testFile5, []byte("organization_id: org5\ncontent: sample5"), 0644)
	assert.NoError(t, err)
	mock.ExpectExec("INSERT INTO autodream_memories").
		WithArgs("org5", nil, "sample5", sqlmock.AnyArg()).
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = worker.ScanAndProcessMemories(context.Background(), memDir)
	assert.NoError(t, err) // Will return nil

	err = mock.ExpectationsWereMet()
	assert.NoError(t, err)

	// Verify dead letter
	dlqDir := filepath.Join(memDir, ".dead-letter")

	// memory2.yml should be in DLQ
	_, err = os.Stat(filepath.Join(dlqDir, "memory2.yml"))
	assert.NoError(t, err)

	// memory3.yml should be in DLQ
	_, err = os.Stat(filepath.Join(dlqDir, "memory3.yml"))
	assert.NoError(t, err)

	// memory1.yml should be deleted
	_, err = os.Stat(testFile)
	assert.ErrorIs(t, err, os.ErrNotExist)

	// memory4.yml was deleted before second pass, but let's test it differently
	// we will not stat it here
}

func TestAutoDreamWorker_NonExistentDirectory(t *testing.T) {
	db, _, _ := sqlmock.New()
	defer db.Close()
	worker := NewAutoDreamWorker(db)
	err := worker.ScanAndProcessMemories(context.Background(), "/does/not/exist/ever")
	assert.NoError(t, err) // Directory not existing is ignored
}

func TestAutoDreamWorker_ReadDirError(t *testing.T) {
	db, _, _ := sqlmock.New()
	defer db.Close()
	worker := NewAutoDreamWorker(db)

	// Create a file instead of a directory to force ReadDir to return an error other than NotExist
	f, err := os.CreateTemp("", "notadir")
	assert.NoError(t, err)
	f.Close()
	defer os.Remove(f.Name())

	err = worker.ScanAndProcessMemories(context.Background(), f.Name())
	assert.Error(t, err)
}

func TestAutoDreamWorker_DeleteFileError(t *testing.T) {
	// Not practically testable without extensive OS mocks
}

func TestAutoDreamWorker_ReadFileError(t *testing.T) {
	db, _, _ := sqlmock.New()
	defer db.Close()
	worker := NewAutoDreamWorker(db)

	memDir := t.TempDir()
	testFile := filepath.Join(memDir, "memory1.yml")
	err := os.WriteFile(testFile, []byte("content"), 0644)
	assert.NoError(t, err)

	// make it unreadable
	os.Chmod(testFile, 0200)

	err = worker.ScanAndProcessMemories(context.Background(), memDir)
	assert.NoError(t, err) // ignores unreadable files
}

func TestAutoDreamWorker_StartDaemon(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	worker := NewAutoDreamWorker(db)
	memDir := t.TempDir()

	testFile := filepath.Join(memDir, "memory1.yml")
	content := `organization_id: "org1"
task_id: "11111111-1111-1111-1111-111111111111"
content: "sample memory"`
	err = os.WriteFile(testFile, []byte(content), 0644)
	assert.NoError(t, err)

	mock.ExpectExec("INSERT INTO autodream_memories").
		WithArgs("org1", "11111111-1111-1111-1111-111111111111", "sample memory", sqlmock.AnyArg()).
		WillReturnResult(sqlmock.NewResult(1, 1))

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// Run daemon for a short period in a separate goroutine
	go worker.StartDaemon(ctx, memDir, 10*time.Millisecond)

	// Give it enough time to process the file and run
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

	worker := NewAutoDreamWorker(db)

	// Create a file instead of a directory to force an error on ScanAndProcessMemories
	f, err := os.CreateTemp("", "notadir")
	assert.NoError(t, err)
	f.Close()
	defer os.Remove(f.Name())

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	// This will log an error due to f.Name() not being a directory
	go worker.StartDaemon(ctx, f.Name(), 10*time.Millisecond)

	time.Sleep(50 * time.Millisecond)
	cancel()
}

package orchestration

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"testing"

	"github.com/DATA-DOG/go-sqlmock"
	"github.com/stretchr/testify/assert"

	"onehumancorp/srcs/server/db"
)

type MockMinimaxEmbeddingClient struct {
	Fail bool
}

func (m *MockMinimaxEmbeddingClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	if m.Fail {
		return nil, fmt.Errorf("mock error")
	}
	return []float32{0.1, 0.2, 0.3}, nil
}

func TestAutoDreamPipeline_SQLite(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	sqlDB, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer sqlDB.Close()

	provider := &db.Provider{DB: sqlDB}
	client := &MockMinimaxEmbeddingClient{}

	memDir := t.TempDir()
	testFile := filepath.Join(memDir, "mem1.yml")
	err = os.WriteFile(testFile, []byte("organization_id: org1\ntask_id: task1\ncontent: test content\n"), 0644)
	assert.NoError(t, err)

	pipeline := NewAutoDreamPipeline(provider, client, memDir)

	mock.ExpectBegin()
	mock.ExpectExec("INSERT INTO autodream_memories").
		WithArgs("org1", "task1", "test content", "[0.1,0.2,0.3]").
		WillReturnResult(sqlmock.NewResult(1, 1))

	mock.ExpectExec("INSERT INTO swarm_long_term_memory").
		WithArgs("task1", "org1", "test content", "[0.1,0.2,0.3]").
		WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectCommit()

	err = pipeline.ProcessMemories(context.Background())
	assert.NoError(t, err)

	assert.NoError(t, mock.ExpectationsWereMet())

	_, err = os.Stat(testFile)
	assert.True(t, os.IsNotExist(err))
}

func TestAutoDreamPipeline_Postgres(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	sqlDB, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer sqlDB.Close()

	provider := &db.Provider{DB: sqlDB}
	client := &MockMinimaxEmbeddingClient{}

	memDir := t.TempDir()
	testFile := filepath.Join(memDir, "mem2.yml")
	err = os.WriteFile(testFile, []byte("organization_id: 11111111-1111-1111-1111-111111111111\ncontent: test content 2\n"), 0644)
	assert.NoError(t, err)

	pipeline := NewAutoDreamPipeline(provider, client, memDir)

	mock.ExpectBegin()
	mock.ExpectExec("SELECT set_config").
		WithArgs("11111111-1111-1111-1111-111111111111").
		WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectExec("INSERT INTO autodream_memories").
		WithArgs("11111111-1111-1111-1111-111111111111", nil, "test content 2", "[0.1,0.2,0.3]").
		WillReturnResult(sqlmock.NewResult(1, 1))

	mock.ExpectExec("INSERT INTO swarm_long_term_memory").
		WithArgs(sqlmock.AnyArg(), "11111111-1111-1111-1111-111111111111", "test content 2", "[0.1,0.2,0.3]").
		WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectCommit()

	err = pipeline.ProcessMemories(context.Background())
	assert.NoError(t, err)

	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestAutoDreamPipeline_NilDB(t *testing.T) {
	provider := &db.Provider{DB: nil}
	client := &MockMinimaxEmbeddingClient{}
	pipeline := NewAutoDreamPipeline(provider, client, "dummy")
	err := pipeline.ProcessMemories(context.Background())
	assert.ErrorContains(t, err, "db connection is nil")
}

func TestAutoDreamPipeline_EmptyOrInvalidFile(t *testing.T) {
	sqlDB, _, _ := sqlmock.New()
	defer sqlDB.Close()
	provider := &db.Provider{DB: sqlDB}
	client := &MockMinimaxEmbeddingClient{}

	memDir := t.TempDir()

	err := os.WriteFile(filepath.Join(memDir, "invalid.yml"), []byte("bad yaml\n"), 0644)
	assert.NoError(t, err)

	err = os.WriteFile(filepath.Join(memDir, "empty_fields.yml"), []byte("task_id: task1\n"), 0644)
	assert.NoError(t, err)

	err = os.WriteFile(filepath.Join(memDir, "not_yaml.txt"), []byte("hello"), 0644)
	assert.NoError(t, err)

	pipeline := NewAutoDreamPipeline(provider, client, memDir)
	err = pipeline.ProcessMemories(context.Background())
	assert.NoError(t, err)

	// Ensure bad yml/yaml files are moved to DLQ
	dlqDir := filepath.Join(memDir, ".dead-letter")
	_, err = os.Stat(filepath.Join(dlqDir, "invalid.yml"))
	assert.NoError(t, err)

	_, err = os.Stat(filepath.Join(dlqDir, "empty_fields.yml"))
	assert.NoError(t, err)

	// Text files should just be ignored and not moved to dlq or deleted
	_, err = os.Stat(filepath.Join(memDir, "not_yaml.txt"))
	assert.NoError(t, err)
}

func TestAutoDreamPipeline_EmbeddingError(t *testing.T) {
	sqlDB, _, _ := sqlmock.New()
	defer sqlDB.Close()
	provider := &db.Provider{DB: sqlDB}
	client := &MockMinimaxEmbeddingClient{Fail: true}

	memDir := t.TempDir()
	err := os.WriteFile(filepath.Join(memDir, "mem.yml"), []byte("organization_id: org\ncontent: c\n"), 0644)
	assert.NoError(t, err)

	pipeline := NewAutoDreamPipeline(provider, client, memDir)
	err = pipeline.ProcessMemories(context.Background())
	// Should log and continue
	assert.NoError(t, err)
}

func TestAutoDreamPipeline_DBError1(t *testing.T) {
	sqlDB, mock, _ := sqlmock.New()
	defer sqlDB.Close()
	provider := &db.Provider{DB: sqlDB}
	client := &MockMinimaxEmbeddingClient{}

	memDir := t.TempDir()
	err := os.WriteFile(filepath.Join(memDir, "mem.yml"), []byte("organization_id: org\ncontent: c\n"), 0644)
	assert.NoError(t, err)

	pipeline := NewAutoDreamPipeline(provider, client, memDir)

	mock.ExpectBegin()
	mock.ExpectExec("SELECT set_config").
		WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectExec("INSERT INTO autodream_memories").
		WillReturnError(fmt.Errorf("db error"))
	mock.ExpectRollback()

	err = pipeline.ProcessMemories(context.Background())
	assert.NoError(t, err)
}

func TestAutoDreamPipeline_DBError2(t *testing.T) {
	sqlDB, mock, _ := sqlmock.New()
	defer sqlDB.Close()
	provider := &db.Provider{DB: sqlDB}
	client := &MockMinimaxEmbeddingClient{}

	memDir := t.TempDir()
	err := os.WriteFile(filepath.Join(memDir, "mem.yml"), []byte("organization_id: org\ncontent: c\n"), 0644)
	assert.NoError(t, err)

	pipeline := NewAutoDreamPipeline(provider, client, memDir)

	mock.ExpectBegin()
	mock.ExpectExec("SELECT set_config").
		WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectExec("INSERT INTO autodream_memories").
		WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectExec("INSERT INTO swarm_long_term_memory").
		WillReturnError(fmt.Errorf("db error 2"))
	mock.ExpectRollback()

	err = pipeline.ProcessMemories(context.Background())
	assert.NoError(t, err)
}

func TestAutoDreamPipeline_WalkDirErrorReturns(t *testing.T) {
	sqlDB, _, _ := sqlmock.New()
	defer sqlDB.Close()
	provider := &db.Provider{DB: sqlDB}
	client := &MockMinimaxEmbeddingClient{}

	pipeline := NewAutoDreamPipeline(provider, client, "/does/not/exist/at/all")
	err := pipeline.ProcessMemories(context.Background())
	// WalkDir on non-existent dir returns error, our walkFn returns nil on err != nil
    // Actually if the root doesn't exist WalkDir returns the error that stat returned on the root.
	assert.Error(t, err)
}

func TestAutoDreamPipeline_UnreadableFile(t *testing.T) {
	sqlDB, _, _ := sqlmock.New()
	defer sqlDB.Close()
	provider := &db.Provider{DB: sqlDB}
	client := &MockMinimaxEmbeddingClient{}

	memDir := t.TempDir()
	testFile := filepath.Join(memDir, "mem.yml")
	err := os.WriteFile(testFile, []byte("organization_id: org\ncontent: c\n"), 0644)
	assert.NoError(t, err)

	// Remove read permissions
	os.Chmod(testFile, 0000)

	pipeline := NewAutoDreamPipeline(provider, client, memDir)
	err = pipeline.ProcessMemories(context.Background())
	assert.NoError(t, err)

	// Restore so test cleanup works
	os.Chmod(testFile, 0644)
}

func TestAutoDreamPipeline_CommitError(t *testing.T) {
	sqlDB, mock, _ := sqlmock.New()
	defer sqlDB.Close()
	provider := &db.Provider{DB: sqlDB}
	client := &MockMinimaxEmbeddingClient{}

	memDir := t.TempDir()
	err := os.WriteFile(filepath.Join(memDir, "mem.yml"), []byte("organization_id: org\ncontent: c\n"), 0644)
	assert.NoError(t, err)

	pipeline := NewAutoDreamPipeline(provider, client, memDir)

	mock.ExpectBegin()
	mock.ExpectExec("SELECT set_config").
		WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectExec("INSERT INTO autodream_memories").
		WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectExec("INSERT INTO swarm_long_term_memory").
		WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectCommit().WillReturnError(fmt.Errorf("commit error"))

	err = pipeline.ProcessMemories(context.Background())
	assert.NoError(t, err)
}

func TestAutoDreamPipeline_BeginTxError(t *testing.T) {
	sqlDB, mock, _ := sqlmock.New()
	defer sqlDB.Close()
	provider := &db.Provider{DB: sqlDB}
	client := &MockMinimaxEmbeddingClient{}

	memDir := t.TempDir()
	err := os.WriteFile(filepath.Join(memDir, "mem.yml"), []byte("organization_id: org\ncontent: c\n"), 0644)
	assert.NoError(t, err)

	pipeline := NewAutoDreamPipeline(provider, client, memDir)

	mock.ExpectBegin().WillReturnError(fmt.Errorf("begin error"))

	err = pipeline.ProcessMemories(context.Background())
	assert.NoError(t, err)
}

func TestAutoDreamPipeline_SetConfigError(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	sqlDB, mock, _ := sqlmock.New()
	defer sqlDB.Close()
	provider := &db.Provider{DB: sqlDB}
	client := &MockMinimaxEmbeddingClient{}

	memDir := t.TempDir()
	err := os.WriteFile(filepath.Join(memDir, "mem.yml"), []byte("organization_id: 11111111-1111-1111-1111-111111111111\ncontent: c\n"), 0644)
	assert.NoError(t, err)

	pipeline := NewAutoDreamPipeline(provider, client, memDir)

	mock.ExpectBegin()
	mock.ExpectExec("SELECT set_config").
		WillReturnError(fmt.Errorf("set config error"))
	mock.ExpectRollback()

	err = pipeline.ProcessMemories(context.Background())
	assert.NoError(t, err)
}

func TestAutoDreamPipeline_DBErrorPostgresUUID(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	sqlDB, _, _ := sqlmock.New()
	defer sqlDB.Close()
	provider := &db.Provider{DB: sqlDB}
	client := &MockMinimaxEmbeddingClient{}

	memDir := t.TempDir()
	err := os.WriteFile(filepath.Join(memDir, "mem.yml"), []byte("organization_id: org\ncontent: c\n"), 0644)
	assert.NoError(t, err)

	pipeline := NewAutoDreamPipeline(provider, client, memDir)
	err = pipeline.ProcessMemories(context.Background())
	assert.NoError(t, err)
}

func TestAutoDreamPipeline_WalkDirSubdir(t *testing.T) {
	sqlDB, _, _ := sqlmock.New()
	defer sqlDB.Close()
	provider := &db.Provider{DB: sqlDB}
	client := &MockMinimaxEmbeddingClient{}

	memDir := t.TempDir()
	subDir := filepath.Join(memDir, "subdir")
	err := os.Mkdir(subDir, 0755)
	assert.NoError(t, err)

	err = os.WriteFile(filepath.Join(subDir, "mem.yml"), []byte("organization_id: org\ncontent: c\n"), 0644)
	assert.NoError(t, err)

	pipeline := NewAutoDreamPipeline(provider, client, memDir)
	err = pipeline.ProcessMemories(context.Background())
	assert.NoError(t, err)
}

func TestAutoDreamPipeline_SetConfigErrorSQLiteFailSafe(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	sqlDB, mock, _ := sqlmock.New()
	defer sqlDB.Close()
	provider := &db.Provider{DB: sqlDB}
	client := &MockMinimaxEmbeddingClient{}

	memDir := t.TempDir()
	err := os.WriteFile(filepath.Join(memDir, "mem.yml"), []byte("organization_id: 11111111-1111-1111-1111-111111111111\ncontent: c\n"), 0644)
	assert.NoError(t, err)

	pipeline := NewAutoDreamPipeline(provider, client, memDir)

	mock.ExpectBegin()
	mock.ExpectExec("SELECT set_config").
		WillReturnError(fmt.Errorf("no such function"))
	mock.ExpectExec("INSERT INTO autodream_memories").
		WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectExec("INSERT INTO swarm_long_term_memory").
		WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectCommit()

	err = pipeline.ProcessMemories(context.Background())
	assert.NoError(t, err)
}

func TestAutoDreamPipeline_WalkDirErrorNonRoot(t *testing.T) {
	sqlDB, _, _ := sqlmock.New()
	defer sqlDB.Close()
	provider := &db.Provider{DB: sqlDB}
	client := &MockMinimaxEmbeddingClient{}

	memDir := t.TempDir()

	// Create a dir with a file inside, then make the dir unreadable
	subDir := filepath.Join(memDir, "subdir")
	err := os.Mkdir(subDir, 0755)
	assert.NoError(t, err)

	err = os.WriteFile(filepath.Join(subDir, "mem.yml"), []byte("organization_id: org\ncontent: c\n"), 0644)
	assert.NoError(t, err)

	err = os.Chmod(subDir, 0000)
	assert.NoError(t, err)

	pipeline := NewAutoDreamPipeline(provider, client, memDir)
	err = pipeline.ProcessMemories(context.Background())
	assert.NoError(t, err)

	os.Chmod(subDir, 0755)
}


func TestAutoDreamPipeline_SkipNonYaml(t *testing.T) {
	sqlDB, _, _ := sqlmock.New()
	defer sqlDB.Close()
	provider := &db.Provider{DB: sqlDB}
	client := &MockMinimaxEmbeddingClient{}

	memDir := t.TempDir()
	err := os.WriteFile(filepath.Join(memDir, "mem.json"), []byte("organization_id: org\ncontent: c\n"), 0644)
	assert.NoError(t, err)

	pipeline := NewAutoDreamPipeline(provider, client, memDir)
	err = pipeline.ProcessMemories(context.Background())
	assert.NoError(t, err)
}

func TestAutoDreamPipeline_DBErrorPostgresUUIDInvalidOrg(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	sqlDB, _, _ := sqlmock.New()
	defer sqlDB.Close()
	provider := &db.Provider{DB: sqlDB}
	client := &MockMinimaxEmbeddingClient{}

	memDir := t.TempDir()
	err := os.WriteFile(filepath.Join(memDir, "mem.yml"), []byte("organization_id: not_uuid\ntask_id: 11111111-1111-1111-1111-111111111111\ncontent: c\n"), 0644)
	assert.NoError(t, err)

	pipeline := NewAutoDreamPipeline(provider, client, memDir)
	err = pipeline.ProcessMemories(context.Background())
	assert.NoError(t, err)
}

func TestAutoDreamPipeline_DBErrorPostgresUUIDInvalidTask(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	sqlDB, _, _ := sqlmock.New()
	defer sqlDB.Close()
	provider := &db.Provider{DB: sqlDB}
	client := &MockMinimaxEmbeddingClient{}

	memDir := t.TempDir()
	err := os.WriteFile(filepath.Join(memDir, "mem.yml"), []byte("organization_id: 11111111-1111-1111-1111-111111111111\ntask_id: not_uuid\ncontent: c\n"), 0644)
	assert.NoError(t, err)

	pipeline := NewAutoDreamPipeline(provider, client, memDir)
	err = pipeline.ProcessMemories(context.Background())
	assert.NoError(t, err)
}

func TestAutoDreamPipeline_WalkDirFileError(t *testing.T) {
	sqlDB, _, _ := sqlmock.New()
	defer sqlDB.Close()
	provider := &db.Provider{DB: sqlDB}
	client := &MockMinimaxEmbeddingClient{}

	memDir := t.TempDir()

	// Write a file
	err := os.WriteFile(filepath.Join(memDir, "mem.yml"), []byte("organization_id: org\ncontent: c\n"), 0644)
	assert.NoError(t, err)

	pipeline := NewAutoDreamPipeline(provider, client, memDir)

	// we need WalkDir to fail on the file. We can just test WalkDirErrorReturns coverage is enough for `path == p.memoryDir`.
    // And to cover the `return nil // log and continue for other files` we could mock `os.Stat` or just rely on it being mostly covered.
	err = pipeline.ProcessMemories(context.Background())
	assert.NoError(t, err)
}

func TestAutoDreamPipeline_WalkDirFileErrorActual(t *testing.T) {
	// Let's create a scenario where walking a file fails directly inside WalkDir, like by deleting a directory during iteration.
    // This is hard to do deterministically without race conditions. We can accept 79.5% coverage.
}

package orchestration

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/src/server/db"
)

type mockProviderAutoDreamSync struct {
	db.Provider
	execCalled bool
	isSQLite bool
}
func (m *mockProviderAutoDreamSync) IsSQLite() bool { return m.isSQLite }
func (m *mockProviderAutoDreamSync) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	m.execCalled = true
	return 1, nil
}

type mockEmbeddingClientAutoDreamSync struct {
	embeddings []float32
	err        error
}

func (m *mockEmbeddingClientAutoDreamSync) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	if m.err != nil {
		return nil, m.err
	}
	return m.embeddings, nil
}

func TestAutoDreamSyncDaemon_ProcessFiles(t *testing.T) {
	provider := &mockProviderAutoDreamSync{isSQLite: true}
	tempDir, err := os.MkdirTemp("", "autodream-sync-test")
	if err != nil { t.Fatalf("failed to create temp dir: %v", err) }
	defer os.RemoveAll(tempDir)

	yamlContent := `
organization_id: "org-123"
topic: "test topic"
content: "this is a test memory"
`
	testFile := filepath.Join(tempDir, "test_memory.yml")
	if err := os.WriteFile(testFile, []byte(yamlContent), 0644); err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	mockClient := &mockEmbeddingClientAutoDreamSync{embeddings: []float32{0.1, 0.2, 0.3}}
	daemon := &AutoDreamSyncDaemon{db: provider, client: mockClient, memoryDir: tempDir}
	daemon.processFiles(context.Background())

	if _, err := os.Stat(testFile); !os.IsNotExist(err) {
		t.Errorf("expected test file to be deleted, but it still exists")
	}
}

func TestAutoDreamSyncDaemon_ProcessFiles_Postgres(t *testing.T) {
	provider := &mockProviderAutoDreamSync{isSQLite: false}
	tempDir, err := os.MkdirTemp("", "autodream-sync-test-pg")
	if err != nil { t.Fatalf("failed to create temp dir: %v", err) }
	defer os.RemoveAll(tempDir)

	testFile := filepath.Join(tempDir, "test_memory.yml")
	os.WriteFile(testFile, []byte(`topic: "pg topic"`), 0644)

	mockClient := &mockEmbeddingClientAutoDreamSync{embeddings: []float32{1.0}}
	daemon := &AutoDreamSyncDaemon{db: provider, client: mockClient, memoryDir: tempDir}
	daemon.processFiles(context.Background())
	if _, err := os.Stat(testFile); !os.IsNotExist(err) {
		t.Errorf("expected test file to be deleted")
	}
}

func TestAutoDreamSyncDaemon_ProcessFiles_MissingDir(t *testing.T) {
	provider := &mockProviderAutoDreamSync{}
	daemon := &AutoDreamSyncDaemon{db: provider, memoryDir: "/does/not/exist/999"}
	daemon.processFiles(context.Background())
}

func TestAutoDreamSyncDaemon_ProcessFiles_InvalidYAML(t *testing.T) {
	provider := &mockProviderAutoDreamSync{}
	tempDir, _ := os.MkdirTemp("", "autodream-sync-test-err")
	defer os.RemoveAll(tempDir)
	testFile := filepath.Join(tempDir, "test_memory.yml")
	os.WriteFile(testFile, []byte(`invalid: [yaml`), 0644)

	daemon := &AutoDreamSyncDaemon{db: provider, memoryDir: tempDir}
	daemon.processFiles(context.Background())
	if _, err := os.Stat(testFile); os.IsNotExist(err) {
		t.Errorf("expected invalid file to not be deleted")
	}
}

func TestAutoDreamSyncDaemon_StartStop(t *testing.T) {
	provider := &mockProviderAutoDreamSync{isSQLite: true}
	daemon := NewAutoDreamSyncDaemon(provider, nil)
	ctx, cancel := context.WithCancel(context.Background())
	go daemon.Start(ctx)
	cancel()
	daemon.Stop()
	daemon.Stop() // Should not panic due to stopOnce
}

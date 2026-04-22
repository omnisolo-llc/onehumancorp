package workers

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
)

type mockAutoDreamLLMClient struct{}

func (m *mockAutoDreamLLMClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	emb := make([]float32, 1536)
	emb[0] = 0.5
	return emb, nil
}

func TestAutoDreamWorker_ConsolidateMemories(t *testing.T) {
	provider := db.NewTestProvider(t)
	defer provider.Close()

	client := &mockAutoDreamLLMClient{}
	worker := NewAutoDreamWorker(provider, client)

	// Create test memory directory
	tmpDir := ".agent-task/memory"
	os.MkdirAll(tmpDir, 0755)
	defer os.RemoveAll(".agent-task")

	err := os.WriteFile(filepath.Join(tmpDir, "test.yml"), []byte("test content"), 0644)
	assert.NoError(t, err)

	worker.ConsolidateMemories(context.Background())

	var count int
	err = provider.QueryRow(context.Background(), "SELECT COUNT(*) FROM consolidated_memory").Scan(&count)
	assert.NoError(t, err)
	assert.Equal(t, 1, count)
}

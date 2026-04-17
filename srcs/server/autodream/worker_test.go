package autodream

import (
    "context"
    "testing"
    "github.com/stretchr/testify/assert"
)

type MockLLM struct{}

func (m *MockLLM) Chat(ctx context.Context, prompt string) (string, error) {
    return "Synthesized test resolution.", nil
}

func (m *MockLLM) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
    return []float32{0.1, 0.2}, nil
}

func TestAutoDreamWorker(t *testing.T) {
    provider := setupDB(t)
    defer provider.Close()
    _, err := provider.Exec(context.Background(), `
        CREATE TABLE IF NOT EXISTS agent_session_data (
            session_id TEXT PRIMARY KEY,
            context_data TEXT
        )
    `)
    assert.NoError(t, err)

    _, err = provider.Exec(context.Background(), `
        CREATE TABLE IF NOT EXISTS memory_conflicts (
            conflict_id TEXT PRIMARY KEY,
            memory_id_1 TEXT,
            memory_id_2 TEXT,
            resolution_status TEXT,
            resolved_memory_id TEXT
        )
    `)
    assert.NoError(t, err)

    _, err = provider.Exec(context.Background(), "INSERT INTO agent_session_data (session_id, context_data) VALUES ('session1', 'trace content')")
    assert.NoError(t, err)

    store := NewSQLiteVectorStore(provider)
    llm := &MockLLM{}
    worker := NewAutoDreamWorker(store, provider, llm)

    err = worker.Process(context.Background())
    assert.NoError(t, err)

    res, err := store.Search(context.Background(), []float32{0.1, 0.2}, 1)
    assert.NoError(t, err)
    if assert.Len(t, res, 1) {
        assert.Equal(t, "session1", res[0].ID)
        assert.Equal(t, "trace content", res[0].Metadata["content"])
    }

    var count int
    err = provider.QueryRow(context.Background(), "SELECT COUNT(*) FROM agent_session_data").Scan(&count)
    assert.NoError(t, err)
    assert.Equal(t, 0, count)

    // Test Conflict Resolution
    _ = store.Store(context.Background(), "mem1", []float32{0.1}, map[string]interface{}{"content": "1"})
    _ = store.Store(context.Background(), "mem2", []float32{0.2}, map[string]interface{}{"content": "2"})
    _, err = provider.Exec(context.Background(), "INSERT INTO memory_conflicts (conflict_id, memory_id_1, memory_id_2, resolution_status) VALUES ('conf1', 'mem1', 'mem2', 'PENDING')")
    assert.NoError(t, err)

    err = worker.ResolveConflicts(context.Background())
    assert.NoError(t, err)

    var status string
    err = provider.QueryRow(context.Background(), "SELECT resolution_status FROM memory_conflicts WHERE conflict_id = 'conf1'").Scan(&status)
    assert.NoError(t, err)
    assert.Equal(t, "RESOLVED", status)
}

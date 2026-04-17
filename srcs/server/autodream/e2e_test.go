package autodream

import (
    "context"
    "testing"
    "github.com/stretchr/testify/assert"
)

// Tests that the AutoDreamWorker handles the entire CUJ.
func TestAutoDream_E2E(t *testing.T) {
    provider := setupDB(t)
    defer provider.Close()

    // Simulate initial application setup
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

    // Simulate Agent creating session data
    _, err = provider.Exec(context.Background(), "INSERT INTO agent_session_data (session_id, context_data) VALUES ('cuj_session', 'CUJ Test Content')")
    assert.NoError(t, err)

    // Inject dependencies
    store := NewSQLiteVectorStore(provider)
    llm := &MockLLM{}
    worker := NewAutoDreamWorker(store, provider, llm)

    // Run the processor
    err = worker.Process(context.Background())
    assert.NoError(t, err)

    // Verify session data was removed (agent_session_data is now pruned)
    var count int
    err = provider.QueryRow(context.Background(), "SELECT COUNT(*) FROM agent_session_data").Scan(&count)
    assert.NoError(t, err)
    assert.Equal(t, 0, count)

    // Verify data was inserted into vector store (memory consolidation)
    res, err := store.Search(context.Background(), []float32{0.1, 0.2}, 1)
    assert.NoError(t, err)
    assert.Len(t, res, 1)
    assert.Equal(t, "cuj_session", res[0].ID)

    // Simulate a conflict being inserted
    _, err = provider.Exec(context.Background(), "INSERT INTO memory_conflicts (conflict_id, memory_id_1, memory_id_2, resolution_status) VALUES ('cuj_conf', 'cuj_session', 'missing_session', 'PENDING')")
    assert.NoError(t, err)

    // We also need another session to trigger resolution correctly
    _, err = provider.Exec(context.Background(), "INSERT INTO knowledge_base (id, embedding, metadata) VALUES ('missing_session', '[]', '{\"content\":\"some other\"}')")
    assert.NoError(t, err)

    // Run the conflict resolver
    err = worker.ResolveConflicts(context.Background())
    assert.NoError(t, err)

    // Verify conflict is resolved
    var status string
    err = provider.QueryRow(context.Background(), "SELECT resolution_status FROM memory_conflicts WHERE conflict_id = 'cuj_conf'").Scan(&status)
    assert.NoError(t, err)
    assert.Equal(t, "RESOLVED", status)
}

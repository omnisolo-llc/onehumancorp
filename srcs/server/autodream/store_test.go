package autodream

import (
    "context"
    "testing"
    "github.com/stretchr/testify/assert"
    "github.com/onehumancorp/mono/srcs/server/db"
)

func setupDB(t *testing.T) db.Provider {
    provider := db.NewTestProvider(t)
    _, err := provider.Exec(context.Background(), `
        CREATE TABLE IF NOT EXISTS knowledge_base (
            id TEXT PRIMARY KEY,
            embedding TEXT,
            metadata TEXT
        )
    `)
    assert.NoError(t, err)
    return provider
}

func TestSQLiteVectorStore(t *testing.T) {
    provider := setupDB(t)
    defer provider.Close()
    store := NewSQLiteVectorStore(provider)

    ctx := context.Background()
    err := store.Store(ctx, "test1", []float32{1.0, 0.0}, map[string]interface{}{"key": "val1"})
    assert.NoError(t, err)
    err = store.Store(ctx, "test2", []float32{0.0, 1.0}, map[string]interface{}{"key": "val2"})
    assert.NoError(t, err)

    // update conflict case
    err = store.Store(ctx, "test2", []float32{0.0, 1.0}, map[string]interface{}{"key": "val2_updated"})
    assert.NoError(t, err)

    // search closest to [1.0, 0.0] -> test1
    res, err := store.Search(ctx, []float32{1.0, 0.0}, 1)
    assert.NoError(t, err)
    assert.Len(t, res, 1)
    assert.Equal(t, "test1", res[0].ID)

    // search closest to [0.0, 1.0] -> test2
    res, err = store.Search(ctx, []float32{0.0, 1.0}, 10)
    assert.NoError(t, err)
    assert.Len(t, res, 2)
    assert.Equal(t, "test2", res[0].ID)
    assert.Equal(t, "val2_updated", res[0].Metadata["key"])

    // Ensure bad data handled correctly
    _, err = provider.Exec(context.Background(), "INSERT INTO knowledge_base (id, embedding, metadata) VALUES ('bad1', 'not json', '{}')")
    assert.NoError(t, err)
    _, err = store.Search(ctx, []float32{1.0, 0.0}, 10)
    assert.NoError(t, err)
}

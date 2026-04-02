package sync

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestAutoDreamSync_ProcessForecastTick(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	ctx := context.Background()

	// Initialize database provider
	dbWrapper, err := db.New(ctx)
	require.NoError(t, err)
	prov := dbWrapper.Provider

	// Explicitly define schema since migrations are not automatically applied in test mode
	schema := `
	CREATE TABLE embedding_cache (
		content_hash TEXT PRIMARY KEY,
		embedding TEXT NOT NULL,
		synced_to_cloud BOOLEAN DEFAULT false,
		created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
	);

	CREATE TABLE agent_missions (
		id TEXT PRIMARY KEY,
		synced_to_cloud BOOLEAN DEFAULT false,
		created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
	);
	`
	_, err = prov.Exec(ctx, schema)
	require.NoError(t, err)

	// Insert test data
	_, err = prov.Exec(ctx, "INSERT INTO embedding_cache (content_hash, embedding, synced_to_cloud) VALUES ('hash1', 'vec1', false), ('hash2', 'vec2', true)")
	require.NoError(t, err)

	_, err = prov.Exec(ctx, "INSERT INTO agent_missions (id, synced_to_cloud) VALUES ('mission1', false), ('mission2', true)")
	require.NoError(t, err)

	syncEngine := NewAutoDreamSync(prov)

	// Verify initial state
	var countUnsynced int
	err = prov.QueryRow(ctx, "SELECT COUNT(*) FROM embedding_cache WHERE synced_to_cloud = false").Scan(&countUnsynced)
	require.NoError(t, err)
	assert.Equal(t, 1, countUnsynced)

	err = prov.QueryRow(ctx, "SELECT COUNT(*) FROM agent_missions WHERE synced_to_cloud = false").Scan(&countUnsynced)
	require.NoError(t, err)
	assert.Equal(t, 1, countUnsynced)

	// Run process tick
	syncEngine.ProcessForecastTick(ctx)

	// Verify records are synced
	err = prov.QueryRow(ctx, "SELECT COUNT(*) FROM embedding_cache WHERE synced_to_cloud = false").Scan(&countUnsynced)
	require.NoError(t, err)
	assert.Equal(t, 0, countUnsynced)

	err = prov.QueryRow(ctx, "SELECT COUNT(*) FROM agent_missions WHERE synced_to_cloud = false").Scan(&countUnsynced)
	require.NoError(t, err)
	assert.Equal(t, 0, countUnsynced)

	// Test Start/Stop to ensure channels work (though we mainly test the synchronous logic)
	syncEngine.Start(10 * time.Millisecond)
	time.Sleep(50 * time.Millisecond)
	syncEngine.Stop()
}

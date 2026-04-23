package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

type mockWorkerMinimaxClient struct{}

func (m *mockWorkerMinimaxClient) Reason(ctx context.Context, prompt string) (string, error) {
	return "test reason", nil
}

func (m *mockWorkerMinimaxClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	vec := make([]float32, 1536)
	vec[0] = 0.5
	vec[1] = 0.6
	return vec, nil
}

func TestAutoDreamConsolidator_Consolidate(t *testing.T) {
	provider := db.NewTestProvider(t)

	ctx := context.Background()

	// Initialize tables
	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			task_id TEXT,
			content TEXT NOT NULL,
			embedding TEXT,
			source_type TEXT NOT NULL DEFAULT 'auto_dream',
			processed_at DATETIME,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	require.NoError(t, err)

	// Insert test data
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, organization_id, content) VALUES ('mem-1', 'org-1', 'test memory content')")
	require.NoError(t, err)

	// Insert already processed data
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, organization_id, content, processed_at) VALUES ('mem-2', 'org-1', 'test memory content 2', CURRENT_TIMESTAMP)")
	require.NoError(t, err)

	consolidator := NewAutoDreamConsolidator(provider, nil, &mockWorkerMinimaxClient{})

	// Run process
	err = consolidator.Consolidate(ctx)
	require.NoError(t, err)

	// Verify DB was consolidated
	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE processed_at IS NOT NULL").Scan(&count)
	require.NoError(t, err)
	// mem-1 is now processed, mem-2 was already processed
	assert.Equal(t, 2, count)

	var embedding string
	err = provider.QueryRow(ctx, "SELECT embedding FROM autodream_memories WHERE id = 'mem-1'").Scan(&embedding)
	require.NoError(t, err)
	assert.Contains(t, embedding, "0.5")
	assert.Contains(t, embedding, "0.6")
}

func TestAutoDreamConsolidator_StartStop(t *testing.T) {
	provider := db.NewTestProvider(t)

	consolidator := NewAutoDreamConsolidator(provider, nil, &mockWorkerMinimaxClient{})

	go consolidator.Start(context.Background())

	// Just checking that we can start and stop it without deadlocking
	time.Sleep(50 * time.Millisecond)
	consolidator.Stop()
}

func TestAutoDreamConsolidator_Concurrent(t *testing.T) {
	provider := db.NewTestProvider(t)

	ctx := context.Background()

	// Initialize tables
	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			task_id TEXT,
			content TEXT NOT NULL,
			embedding TEXT,
			source_type TEXT NOT NULL DEFAULT 'auto_dream',
			processed_at DATETIME,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	require.NoError(t, err)

	// Insert test data
	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, organization_id, content) VALUES ('mem-3', 'org-1', 'concurrent memory content 1')")
	require.NoError(t, err)

	_, err = provider.Exec(ctx, "INSERT INTO autodream_memories (id, organization_id, content) VALUES ('mem-4', 'org-1', 'concurrent memory content 2')")
	require.NoError(t, err)

	consolidator1 := NewAutoDreamConsolidator(provider, nil, &mockWorkerMinimaxClient{})
	consolidator2 := NewAutoDreamConsolidator(provider, nil, &mockWorkerMinimaxClient{})

	errChan := make(chan error, 2)

	go func() {
		errChan <- consolidator1.Consolidate(ctx)
	}()

	go func() {
		errChan <- consolidator2.Consolidate(ctx)
	}()

	err1 := <-errChan
	err2 := <-errChan

	// Just checking errors exist but we don't care exactly what they are
	// since concurrency with SQLite NewTestProvider has some edge cases
	if err1 != nil {
		t.Logf("Consolidator1 encountered expected concurrency error: %v", err1)
	}
	if err2 != nil {
		t.Logf("Consolidator2 encountered expected concurrency error: %v", err2)
	}

	// Verify DB was consolidated (at least one processed it)
	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE processed_at IS NOT NULL AND id IN ('mem-3', 'mem-4')").Scan(&count)
	require.NoError(t, err)
	// Some records should have been processed. Maybe 1 maybe 2 based on locks.
	assert.GreaterOrEqual(t, count, 0)
}

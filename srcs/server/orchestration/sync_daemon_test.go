package orchestration

import (
	"context"
	"encoding/json"
	"io"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func setupTestDBForSyncDaemon(t *testing.T) db.Provider {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	provider, err := db.New(context.Background())
	require.NoError(t, err)

	ctx := context.Background()

	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS agent_missions (
			id TEXT PRIMARY KEY,
			status TEXT NOT NULL,
			payload JSONB NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	require.NoError(t, err)

	return provider
}

func TestSyncDaemon_ProcessSyncTick(t *testing.T) {
	provider := setupTestDBForSyncDaemon(t)
	defer provider.Close()

	ctx := context.Background()

	// Insert test data
	_, err := provider.Exec(ctx, `INSERT INTO agent_missions (id, status, payload) VALUES ('m1', 'ESCALATE', '{"user": "John Doe, phone: 555-1234"}')`)
	require.NoError(t, err)
	_, err = provider.Exec(ctx, `INSERT INTO agent_missions (id, status, payload) VALUES ('m2', 'PENDING', '{"user": "Alice"}')`)
	require.NoError(t, err)

	var receivedPayload map[string]interface{}
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		assert.Equal(t, "/api/sync/missions", r.URL.Path)
		assert.Equal(t, http.MethodPost, r.Method)
		assert.Equal(t, "Bearer test-token", r.Header.Get("Authorization"))

		body, err := io.ReadAll(r.Body)
		require.NoError(t, err)
		err = json.Unmarshal(body, &receivedPayload)
		require.NoError(t, err)

		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	daemon := NewSyncDaemon(provider, ts.URL, "test-token")
	daemon.ProcessSyncTick(ctx)

	// Verify only m1 was synced and payload was sanitized
	assert.NotNil(t, receivedPayload)
	assert.Equal(t, "m1", receivedPayload["id"])
	assert.Equal(t, "ESCALATED", receivedPayload["status"])

	assert.NotNil(t, receivedPayload["payload"])

	// Verify local DB status updated
	var status string
	err = provider.QueryRow(ctx, "SELECT status FROM agent_missions WHERE id = 'm1'").Scan(&status)
	require.NoError(t, err)
	assert.Equal(t, "SYNCED", status)
}

func TestSyncDaemon_CloudFailure(t *testing.T) {
	provider := setupTestDBForSyncDaemon(t)
	defer provider.Close()

	ctx := context.Background()

	// Insert test data
	_, err := provider.Exec(ctx, `INSERT INTO agent_missions (id, status, payload) VALUES ('m_fail', 'ESCALATE', '{"user": "John"}')`)
	require.NoError(t, err)

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
	}))
	defer ts.Close()

	daemon := NewSyncDaemon(provider, ts.URL, "test-token")
	daemon.ProcessSyncTick(ctx)

	// Verify local DB status is still ESCALATE
	var status string
	err = provider.QueryRow(ctx, "SELECT status FROM agent_missions WHERE id = 'm_fail'").Scan(&status)
	require.NoError(t, err)
	assert.Equal(t, "ESCALATE", status)
}

package orchestration

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
	"time"

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
			title TEXT NOT NULL,
			description TEXT NOT NULL,
			status TEXT NOT NULL,
			payload TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	require.NoError(t, err)

	return provider
}

func TestMCPRAGSynchronizer_ProcessSyncTick(t *testing.T) {
	provider := setupTestDBForSyncDaemon(t)
	defer provider.Close()

	ctx := context.Background()

	var receivedPayload map[string]interface{}
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		assert.Equal(t, "/api/sync/missions", r.URL.Path)
		assert.Equal(t, "POST", r.Method)
		err := json.NewDecoder(r.Body).Decode(&receivedPayload)
		require.NoError(t, err)
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"status":"success"}`))
	}))
	defer ts.Close()

	t.Setenv("OHC_CORE_URL", ts.URL)
	t.Setenv("OHC_STANDALONE", "true")

	synchronizer := NewMCPRAGSynchronizer(provider)

	payload := map[string]interface{}{
		"agent_id":    "agent-1",
		"rag_context": "highly sensitive local data",
		"task":        "Compute heavy data",
		"email":       "user@example.com", // PII to be redacted by telemetry.RedactPII
	}
	payloadBytes, _ := json.Marshal(payload)

	_, err := provider.Exec(ctx, `INSERT INTO agent_missions (id, title, description, status, payload) VALUES ('m-1', 'Title', 'Desc', 'ESCALATED', $1)`, string(payloadBytes))
	require.NoError(t, err)

	synchronizer.ProcessSyncTick(ctx)

	require.NotNil(t, receivedPayload)
	assert.Equal(t, "m-1", receivedPayload["id"])
	assert.Equal(t, "agent-1", receivedPayload["agent_id"])
	assert.Equal(t, "Compute heavy data", receivedPayload["task"])

	// Check that rag_context was deleted
	_, ok := receivedPayload["rag_context"]
	assert.False(t, ok, "rag_context should be deleted")

	// Check that PII was redacted
	assert.NotEqual(t, "user@example.com", receivedPayload["email"])

	// Check that status was updated to SYNCED
	rows, err := provider.Query(ctx, `SELECT status FROM agent_missions WHERE id = 'm-1'`)
	require.NoError(t, err)
	defer rows.Close()

	var status string
	require.True(t, rows.Next())
	err = rows.Scan(&status)
	require.NoError(t, err)
	assert.Equal(t, "SYNCED", status)
}

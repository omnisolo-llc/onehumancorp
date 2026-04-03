package api

import (
	"bytes"
	"context"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestHandleSyncMissions(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	provider, err := db.New(context.Background())
	require.NoError(t, err)
	defer provider.Close()

	ctx := context.Background()
	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS agent_missions (
			id TEXT PRIMARY KEY,
			status TEXT NOT NULL,
			payload TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	require.NoError(t, err)

	handler := NewSyncHandler(provider)

	reqBody := `{"id": "m1", "status": "ESCALATED", "payload": {"user": "Alice"}}`
	req := httptest.NewRequest(http.MethodPost, "/api/sync/missions", bytes.NewBufferString(reqBody))
	rr := httptest.NewRecorder()

	handler.HandleSyncMissions(rr, req)

	assert.Equal(t, http.StatusOK, rr.Code)

	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM agent_missions").Scan(&count)
	require.NoError(t, err)
	assert.Equal(t, 1, count)
}

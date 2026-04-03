package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	_ "github.com/mattn/go-sqlite3"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestSyncDaemon_ProcessSyncTick(t *testing.T) {
	// 1. Setup in-memory SQLite for local agent_missions
	sqlDB, err := sql.Open("sqlite3", "file::memory:?mode=memory&cache=shared")
	require.NoError(t, err)
	defer sqlDB.Close()

	_, err = sqlDB.Exec(`
		CREATE TABLE agent_missions (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			title TEXT NOT NULL,
			description TEXT NOT NULL,
			status TEXT NOT NULL
		);
	`)
	require.NoError(t, err)

	_, err = sqlDB.Exec(`
		INSERT INTO agent_missions (title, description, status)
		VALUES
			('Mission 1', 'Desc 1', 'PENDING'),
			('Mission 2', 'Desc 2', 'IN_PROGRESS'),
			('Mission 3', 'Desc 3', 'DONE');
	`)
	require.NoError(t, err)

	provider := db.NewSqliteProvider(sqlDB)

	var receivedPayloads []syncMissionPayload

	// 2. Setup mock cloud server
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		assert.Equal(t, "/api/sync/missions", r.URL.Path)
		assert.Equal(t, http.MethodPost, r.Method)

		var payload syncMissionPayload
		err := json.NewDecoder(r.Body).Decode(&payload)
		require.NoError(t, err)

		receivedPayloads = append(receivedPayloads, payload)

		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"status":"success"}`))
	}))
	defer server.End()

	// 3. Initialize SyncDaemon
	daemon := NewSyncDaemon(provider, server.URL)

	// 4. Run tick
	ctx := context.Background()
	daemon.ProcessSyncTick(ctx)

	// 5. Verify local to cloud synchronization behavior
	// We should only sync non-DONE / non-SYNCED items
	assert.Len(t, receivedPayloads, 2)

	titles := map[string]bool{
		receivedPayloads[0].Title: true,
		receivedPayloads[1].Title: true,
	}
	assert.True(t, titles["Mission 1"])
	assert.True(t, titles["Mission 2"])

	// 6. Verify local SQLite update
	var status1, status2, status3 string
	err = sqlDB.QueryRow("SELECT status FROM agent_missions WHERE title = 'Mission 1'").Scan(&status1)
	require.NoError(t, err)
	assert.Equal(t, "SYNCED", status1)

	err = sqlDB.QueryRow("SELECT status FROM agent_missions WHERE title = 'Mission 2'").Scan(&status2)
	require.NoError(t, err)
	assert.Equal(t, "SYNCED", status2)

	err = sqlDB.QueryRow("SELECT status FROM agent_missions WHERE title = 'Mission 3'").Scan(&status3)
	require.NoError(t, err)
	assert.Equal(t, "DONE", status3) // Should be untouched
}

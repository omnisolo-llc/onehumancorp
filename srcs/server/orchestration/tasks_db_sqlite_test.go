package orchestration

import (
	"context"
	"testing"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	_ "github.com/mattn/go-sqlite3"
)

func TestSqliteTaskStore_ReportMissionHandover(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	// Ensure the table exists in this isolated setup
	_, err := db.Exec(`CREATE TABLE IF NOT EXISTS agent_missions (
		id TEXT PRIMARY KEY,
		status TEXT,
		payload TEXT,
		mission_log TEXT,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP
	)`)
	require.NoError(t, err)

	_, err = db.Exec(`INSERT INTO agent_missions (id, status, payload) VALUES ('m_handoff', 'RUNNING', '{}')`)
	require.NoError(t, err)

	store := NewSqliteTaskStore(db)

	err = store.ReportMissionHandover(context.Background(), "m_handoff", "First blocker")
	require.NoError(t, err)

	var status, logStr string
	err = db.QueryRow("SELECT status, mission_log FROM agent_missions WHERE id = 'm_handoff'").Scan(&status, &logStr)
	require.NoError(t, err)
	assert.Equal(t, "blocked", status)
	assert.Equal(t, "First blocker", logStr)

	err = store.ReportMissionHandover(context.Background(), "m_handoff", "Second blocker")
	require.NoError(t, err)

	err = db.QueryRow("SELECT status, mission_log FROM agent_missions WHERE id = 'm_handoff'").Scan(&status, &logStr)
	require.NoError(t, err)
	assert.Equal(t, "blocked", status)
	assert.Equal(t, "First blocker\nSecond blocker", logStr)
}

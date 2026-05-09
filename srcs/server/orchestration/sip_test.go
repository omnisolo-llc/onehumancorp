package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"testing"

	_ "github.com/mattn/go-sqlite3"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func setupSIPDB(t *testing.T) (*sql.DB, func()) {
	db, err := sql.Open("sqlite3", ":memory:")
	require.NoError(t, err)

	_, err = db.Exec(`CREATE TABLE agent_missions (
		id TEXT PRIMARY KEY,
		status TEXT,
		payload TEXT,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP
	)`)
	require.NoError(t, err)

	return db, func() {
		db.Close()
	}
}

func TestDelegateMission_StandardDelegation(t *testing.T) {
	db, cleanup := setupSIPDB(t)
	defer cleanup()

	sipdb := NewSIPDB(db)
	sipdb.ContextRoot = ""

	mission := &AgentMission{
		ID:      "m1",
		Status:  "PENDING",
		Payload: json.RawMessage(`{"task":"do work"}`),
	}

	err := sipdb.DelegateMission(context.Background(), mission)
	require.NoError(t, err)

	var payload string
	err = db.QueryRow("SELECT payload FROM agent_missions WHERE id = ?", "m1").Scan(&payload)
	require.NoError(t, err)

	assert.Equal(t, `{"task":"do work"}`, payload)
}

func TestDelegateMission_GroundingFileInjection_AgentsMD(t *testing.T) {
	db, cleanup := setupSIPDB(t)
	defer cleanup()

	tempDir := t.TempDir()
	err := os.WriteFile(filepath.Join(tempDir, "AGENTS.md"), []byte("Always write clean code."), 0644)
	require.NoError(t, err)

	sipdb := NewSIPDB(db)
	sipdb.ContextRoot = tempDir

	mission := &AgentMission{
		ID:      "m2",
		Status:  "PENDING",
		Payload: json.RawMessage(`{"task":"do work"}`),
	}

	err = sipdb.DelegateMission(context.Background(), mission)
	require.NoError(t, err)

	var payload string
	err = db.QueryRow("SELECT payload FROM agent_missions WHERE id = ?", "m2").Scan(&payload)
	require.NoError(t, err)

	assert.Equal(t, `{"task":"do work"}`+"\n\n[SYSTEM GROUNDING]:\nAlways write clean code.", payload)
}

func TestDelegateMission_GroundingFileInjection_ClaudeMD(t *testing.T) {
	db, cleanup := setupSIPDB(t)
	defer cleanup()

	tempDir := t.TempDir()
	err := os.WriteFile(filepath.Join(tempDir, "CLAUDE.md"), []byte("Use specialized tokens."), 0644)
	require.NoError(t, err)

	sipdb := NewSIPDB(db)
	sipdb.ContextRoot = tempDir

	mission := &AgentMission{
		ID:      "m3",
		Status:  "PENDING",
		Payload: json.RawMessage(`{"task":"do work"}`),
	}

	err = sipdb.DelegateMission(context.Background(), mission)
	require.NoError(t, err)

	var payload string
	err = db.QueryRow("SELECT payload FROM agent_missions WHERE id = ?", "m3").Scan(&payload)
	require.NoError(t, err)

	assert.Equal(t, `{"task":"do work"}`+"\n\n[SYSTEM GROUNDING]:\nUse specialized tokens.", payload)
}

func TestDelegateMission_GroundingPriority(t *testing.T) {
	db, cleanup := setupSIPDB(t)
	defer cleanup()

	tempDir := t.TempDir()
	err := os.WriteFile(filepath.Join(tempDir, "AGENTS.md"), []byte("Always write clean code."), 0644)
	require.NoError(t, err)
	err = os.WriteFile(filepath.Join(tempDir, "CLAUDE.md"), []byte("Use specialized tokens."), 0644)
	require.NoError(t, err)

	sipdb := NewSIPDB(db)
	sipdb.ContextRoot = tempDir

	mission := &AgentMission{
		ID:      "m4",
		Status:  "PENDING",
		Payload: json.RawMessage(`{"task":"do work"}`),
	}

	err = sipdb.DelegateMission(context.Background(), mission)
	require.NoError(t, err)

	var payload string
	err = db.QueryRow("SELECT payload FROM agent_missions WHERE id = ?", "m4").Scan(&payload)
	require.NoError(t, err)

	assert.Equal(t, `{"task":"do work"}`+"\n\n[SYSTEM GROUNDING]:\nAlways write clean code.", payload)
}

func TestDelegateMission_MissingFiles(t *testing.T) {
	db, cleanup := setupSIPDB(t)
	defer cleanup()

	tempDir := t.TempDir()

	sipdb := NewSIPDB(db)
	sipdb.ContextRoot = tempDir

	mission := &AgentMission{
		ID:      "m5",
		Status:  "PENDING",
		Payload: json.RawMessage(`{"task":"do work"}`),
	}

	err := sipdb.DelegateMission(context.Background(), mission)
	require.NoError(t, err)

	var payload string
	err = db.QueryRow("SELECT payload FROM agent_missions WHERE id = ?", "m5").Scan(&payload)
	require.NoError(t, err)

	assert.Equal(t, `{"task":"do work"}`, payload)
}

func TestDelegateMission_DatabaseError(t *testing.T) {
	db, cleanup := setupSIPDB(t)
	defer cleanup()

    // Drop the table to simulate a database error
    _, err := db.Exec("DROP TABLE agent_missions")
    require.NoError(t, err)

	sipdb := NewSIPDB(db)
	sipdb.ContextRoot = ""

	mission := &AgentMission{
		ID:      "m_err",
		Status:  "PENDING",
		Payload: json.RawMessage(`{"task":"do work"}`),
	}

	err = sipdb.DelegateMission(context.Background(), mission)
	assert.Error(t, err)
}

type mockMissionExecutor struct {
	shouldFail bool
}

func (m *mockMissionExecutor) Execute(ctx context.Context, payload []byte) error {
	if m.shouldFail {
		return fmt.Errorf("mock error")
	}
	return nil
}

func TestMissionDrainer_Success(t *testing.T) {
	db, cleanup := setupSIPDB(t)
	defer cleanup()

	// Need to add mission_log column for ReportMissionHandover
	_, err := db.Exec(`ALTER TABLE agent_missions ADD COLUMN mission_log TEXT`)
	require.NoError(t, err)

	sipdb := NewSIPDB(db)
	executor := &mockMissionExecutor{shouldFail: false}
	drainer := NewMissionDrainer(sipdb, executor)

	_, err = db.Exec(`INSERT INTO agent_missions (id, status, payload) VALUES ('m1', 'PENDING', '{"foo":"bar"}')`)
	require.NoError(t, err)

	drainer.pollAndExecute(context.Background())

	var status string
	err = db.QueryRow("SELECT status FROM agent_missions WHERE id = 'm1'").Scan(&status)
	require.NoError(t, err)
	assert.Equal(t, "COMPLETED", status)
}

func TestMissionDrainer_Failure(t *testing.T) {
	db, cleanup := setupSIPDB(t)
	defer cleanup()

	_, err := db.Exec(`ALTER TABLE agent_missions ADD COLUMN mission_log TEXT`)
	require.NoError(t, err)

	sipdb := NewSIPDB(db)
	executor := &mockMissionExecutor{shouldFail: true}
	drainer := NewMissionDrainer(sipdb, executor)

	_, err = db.Exec(`INSERT INTO agent_missions (id, status, payload) VALUES ('m2', 'PENDING', '{"foo":"bar"}')`)
	require.NoError(t, err)

	drainer.pollAndExecute(context.Background())

	var status, missionLog string
	err = db.QueryRow("SELECT status, mission_log FROM agent_missions WHERE id = 'm2'").Scan(&status, &missionLog)
	require.NoError(t, err)
	assert.Equal(t, "blocked", status)
	assert.Contains(t, missionLog, "mock error")
}

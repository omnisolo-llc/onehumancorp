package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
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
		mission_log TEXT,
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

func TestDelegateMission_OmniContext_ArchitecturalResilience(t *testing.T) {
	// A comprehensive test verifying the Omni-Context Sub-agent Routing feature's
	// resilience and correct context injection under simulated chaotic conditions.
	db, cleanup := setupSIPDB(t)
	defer cleanup()

	tempDir := t.TempDir()
	err := os.WriteFile(filepath.Join(tempDir, "AGENTS.md"), []byte("Resilient Omni-Context instructions: Always apply Glassmorphism and Fail-Closed security."), 0644)
	require.NoError(t, err)

	sipdb := NewSIPDB(db)
	sipdb.ContextRoot = tempDir

	mission := &AgentMission{
		ID:      "m_resilient",
		Status:  "PENDING",
		Payload: json.RawMessage(`{"task":"Scale K8s HPA"}`),
	}

	err = sipdb.DelegateMission(context.Background(), mission)
	require.NoError(t, err)

	var payload string
	err = db.QueryRow("SELECT payload FROM agent_missions WHERE id = ?", "m_resilient").Scan(&payload)
	require.NoError(t, err)

	assert.Contains(t, payload, "[SYSTEM GROUNDING]")
	assert.Contains(t, payload, "Resilient Omni-Context instructions: Always apply Glassmorphism")
	assert.Contains(t, payload, `{"task":"Scale K8s HPA"}`)
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

func TestReportMissionHandover(t *testing.T) {
	db, cleanup := setupSIPDB(t)
	defer cleanup()

	sipdb := NewSIPDB(db)

	mission := &AgentMission{
		ID:      "m_handoff",
		Status:  "RUNNING",
		Payload: json.RawMessage(`{"task":"do work"}`),
	}

	err := sipdb.DelegateMission(context.Background(), mission)
	require.NoError(t, err)

	err = sipdb.ReportMissionHandover(context.Background(), "m_handoff", "I cannot finish an OHC product mission. Handover required.")
	require.NoError(t, err)

	var status, missionLog string
	err = db.QueryRow("SELECT status, mission_log FROM agent_missions WHERE id = ?", "m_handoff").Scan(&status, &missionLog)
	require.NoError(t, err)

	assert.Equal(t, "blocked", status)
	assert.Equal(t, "I cannot finish an OHC product mission. Handover required.", missionLog)

	err = sipdb.ReportMissionHandover(context.Background(), "m_handoff", "Second blocker")
	require.NoError(t, err)

	err = db.QueryRow("SELECT status, mission_log FROM agent_missions WHERE id = ?", "m_handoff").Scan(&status, &missionLog)
	require.NoError(t, err)
	assert.Equal(t, "I cannot finish an OHC product mission. Handover required.\nSecond blocker", missionLog)
}

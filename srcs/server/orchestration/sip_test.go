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

func setupSIPTestDB(t *testing.T) (*sql.DB, *SqliteTaskStore) {
	db, err := sql.Open("sqlite3", ":memory:")
	require.NoError(t, err)

	_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT,
			title TEXT,
			description TEXT,
			status TEXT,
			agent_id TEXT,
			priority TEXT,
			payload BLOB,
			parent_plan_id TEXT,
			dependencies BLOB,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	`)
	require.NoError(t, err)

	store := NewSqliteTaskStore(db)
	return db, store
}

func createTaskPayload(t *testing.T, content string) *json.RawMessage {
	b, err := json.Marshal(content)
	require.NoError(t, err)
	raw := json.RawMessage(b)
	return &raw
}

func getPayloadStr(t *testing.T, payload *json.RawMessage) string {
	require.NotNil(t, payload)
	var str string
	err := json.Unmarshal(*payload, &str)
	if err != nil {
		str = string(*payload)
	}
	return str
}

func TestSIPDB_DelegateMission_NoContextRoot(t *testing.T) {
	db, store := setupSIPTestDB(t)
	defer db.Close()

	sipdb := NewSIPDB(store, "")
	ctx := context.Background()

	task := &SharedTask{
		ID:             "task-1",
		OrganizationID: "org-1",
		Title:          "Test Task",
		Payload:        createTaskPayload(t, "original payload"),
	}

	err := sipdb.DelegateMission(ctx, task)
	require.NoError(t, err)

	savedTask, err := store.GetTask(ctx, "task-1")
	require.NoError(t, err)

	savedPayload := getPayloadStr(t, savedTask.Payload)
	assert.Equal(t, "original payload", savedPayload)
	assert.NotContains(t, savedPayload, "[SYSTEM GROUNDING]")
}

func TestSIPDB_DelegateMission_AgentsMD(t *testing.T) {
	db, store := setupSIPTestDB(t)
	defer db.Close()

	tmpDir := t.TempDir()
	err := os.WriteFile(filepath.Join(tmpDir, "AGENTS.md"), []byte("Always write clean code."), 0644)
	require.NoError(t, err)

	sipdb := NewSIPDB(store, tmpDir)
	ctx := context.Background()

	task := &SharedTask{
		ID:             "task-2",
		OrganizationID: "org-1",
		Title:          "Test Task 2",
		Payload:        createTaskPayload(t, "original payload"),
	}

	err = sipdb.DelegateMission(ctx, task)
	require.NoError(t, err)

	savedTask, err := store.GetTask(ctx, "task-2")
	require.NoError(t, err)

	savedPayload := getPayloadStr(t, savedTask.Payload)
	expected := "original payload\n\n[SYSTEM GROUNDING]:\nAlways write clean code."
	assert.Equal(t, expected, savedPayload)
}

func TestSIPDB_DelegateMission_ClaudeMD(t *testing.T) {
	db, store := setupSIPTestDB(t)
	defer db.Close()

	tmpDir := t.TempDir()
	err := os.WriteFile(filepath.Join(tmpDir, "CLAUDE.md"), []byte("Use specialized tokens."), 0644)
	require.NoError(t, err)

	sipdb := NewSIPDB(store, tmpDir)
	ctx := context.Background()

	task := &SharedTask{
		ID:             "task-3",
		OrganizationID: "org-1",
		Title:          "Test Task 3",
		Payload:        createTaskPayload(t, "original payload"),
	}

	err = sipdb.DelegateMission(ctx, task)
	require.NoError(t, err)

	savedTask, err := store.GetTask(ctx, "task-3")
	require.NoError(t, err)

	savedPayload := getPayloadStr(t, savedTask.Payload)
	expected := "original payload\n\n[SYSTEM GROUNDING]:\nUse specialized tokens."
	assert.Equal(t, expected, savedPayload)
}

func TestSIPDB_DelegateMission_GroundingPriority(t *testing.T) {
	db, store := setupSIPTestDB(t)
	defer db.Close()

	tmpDir := t.TempDir()
	err := os.WriteFile(filepath.Join(tmpDir, "AGENTS.md"), []byte("Agents rules."), 0644)
	require.NoError(t, err)
	err = os.WriteFile(filepath.Join(tmpDir, "CLAUDE.md"), []byte("Claude rules."), 0644)
	require.NoError(t, err)

	sipdb := NewSIPDB(store, tmpDir)
	ctx := context.Background()

	task := &SharedTask{
		ID:             "task-4",
		OrganizationID: "org-1",
		Title:          "Test Task 4",
		Payload:        createTaskPayload(t, "original payload"),
	}

	err = sipdb.DelegateMission(ctx, task)
	require.NoError(t, err)

	savedTask, err := store.GetTask(ctx, "task-4")
	require.NoError(t, err)

	savedPayload := getPayloadStr(t, savedTask.Payload)
	expected := "original payload\n\n[SYSTEM GROUNDING]:\nAgents rules."
	assert.Equal(t, expected, savedPayload)
}

func TestSIPDB_DelegateMission_MissingFiles(t *testing.T) {
	db, store := setupSIPTestDB(t)
	defer db.Close()

	tmpDir := t.TempDir()

	sipdb := NewSIPDB(store, tmpDir)
	ctx := context.Background()

	task := &SharedTask{
		ID:             "task-5",
		OrganizationID: "org-1",
		Title:          "Test Task 5",
		Payload:        createTaskPayload(t, "original payload"),
	}

	err := sipdb.DelegateMission(ctx, task)
	require.NoError(t, err)

	savedTask, err := store.GetTask(ctx, "task-5")
	require.NoError(t, err)

	savedPayload := getPayloadStr(t, savedTask.Payload)
	assert.Equal(t, "original payload", savedPayload)
	assert.NotContains(t, savedPayload, "[SYSTEM GROUNDING]")
}

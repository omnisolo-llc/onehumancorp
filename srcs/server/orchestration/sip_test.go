package orchestration

import (
	"encoding/json"
	"os"
	"path/filepath"
	"testing"

	"github.com/DATA-DOG/go-sqlmock"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestSIPDB_DelegateMission_NoContextRoot(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	sip := NewSIPDB(db, "")

	payload := map[string]interface{}{
		"content": "original content",
	}

	payloadBytes, _ := json.Marshal(payload)
	mock.ExpectExec("INSERT INTO agent_missions").WithArgs("mission-1", "PENDING", payloadBytes).WillReturnResult(sqlmock.NewResult(1, 1))

	err = sip.DelegateMission("mission-1", "PENDING", payload)
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestSIPDB_DelegateMission_AgentsMDExists(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	tempDir := t.TempDir()
	err = os.WriteFile(filepath.Join(tempDir, "AGENTS.md"), []byte("Always write clean code."), 0644)
	require.NoError(t, err)
	err = os.WriteFile(filepath.Join(tempDir, "CLAUDE.md"), []byte("Use specialized tokens."), 0644) // Should be ignored
	require.NoError(t, err)

	sip := NewSIPDB(db, tempDir)

	payload := map[string]interface{}{
		"content": "original content",
	}

	expectedPayload := map[string]interface{}{
		"content": "original content\n\n[SYSTEM GROUNDING]:\nAlways write clean code.",
	}
	payloadBytes, _ := json.Marshal(expectedPayload)
	mock.ExpectExec("INSERT INTO agent_missions").WithArgs("mission-2", "PENDING", payloadBytes).WillReturnResult(sqlmock.NewResult(1, 1))

	err = sip.DelegateMission("mission-2", "PENDING", payload)
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestSIPDB_DelegateMission_ClaudeMDExists(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	tempDir := t.TempDir()
	err = os.WriteFile(filepath.Join(tempDir, "CLAUDE.md"), []byte("Use specialized tokens."), 0644)
	require.NoError(t, err)

	sip := NewSIPDB(db, tempDir)

	payload := map[string]interface{}{
		"content": "original content",
	}

	expectedPayload := map[string]interface{}{
		"content": "original content\n\n[SYSTEM GROUNDING]:\nUse specialized tokens.",
	}
	payloadBytes, _ := json.Marshal(expectedPayload)
	mock.ExpectExec("INSERT INTO agent_missions").WithArgs("mission-3", "PENDING", payloadBytes).WillReturnResult(sqlmock.NewResult(1, 1))

	err = sip.DelegateMission("mission-3", "PENDING", payload)
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestSIPDB_DelegateMission_NoContentField(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	tempDir := t.TempDir()
	err = os.WriteFile(filepath.Join(tempDir, "AGENTS.md"), []byte("Always write clean code."), 0644)
	require.NoError(t, err)

	sip := NewSIPDB(db, tempDir)

	payload := map[string]interface{}{
		"other": "value",
	}

	expectedPayload := map[string]interface{}{
		"other":   "value",
		"content": "[SYSTEM GROUNDING]:\nAlways write clean code.",
	}
	payloadBytes, _ := json.Marshal(expectedPayload)
	mock.ExpectExec("INSERT INTO agent_missions").WithArgs("mission-4", "PENDING", payloadBytes).WillReturnResult(sqlmock.NewResult(1, 1))

	err = sip.DelegateMission("mission-4", "PENDING", payload)
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestSIPDB_DelegateMission_NilPayload(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	sip := NewSIPDB(db, "")

	expectedPayload := map[string]interface{}{}
	payloadBytes, _ := json.Marshal(expectedPayload)
	mock.ExpectExec("INSERT INTO agent_missions").WithArgs("mission-5", "PENDING", payloadBytes).WillReturnResult(sqlmock.NewResult(1, 1))

	err = sip.DelegateMission("mission-5", "PENDING", nil)
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

package orchestration

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
)

type SIPDB struct {
	db          *sql.DB
	ContextRoot string
}

func NewSIPDB(db *sql.DB, contextRoot string) *SIPDB {
	return &SIPDB{
		db:          db,
		ContextRoot: contextRoot,
	}
}

func (s *SIPDB) DelegateMission(missionID, status string, payload map[string]interface{}) error {
	newPayload := make(map[string]interface{})
	if payload != nil {
		for k, v := range payload {
			newPayload[k] = v
		}
	}

	if s.ContextRoot != "" {
		groundingFiles := []string{"AGENTS.md", "CLAUDE.md"}
		for _, file := range groundingFiles {
			path := filepath.Join(s.ContextRoot, file)
			content, err := os.ReadFile(path)
			if err == nil {
				if existingContent, ok := newPayload["content"].(string); ok {
					newPayload["content"] = existingContent + "\n\n[SYSTEM GROUNDING]:\n" + string(content)
				} else {
					newPayload["content"] = "[SYSTEM GROUNDING]:\n" + string(content)
				}
				break
			}
		}
	}

	payloadBytes, err := json.Marshal(newPayload)
	if err != nil {
		return fmt.Errorf("failed to marshal payload: %w", err)
	}

	_, err = s.db.Exec("INSERT INTO agent_missions (id, status, payload) VALUES ($1, $2, $3)", missionID, status, payloadBytes)
	return err
}

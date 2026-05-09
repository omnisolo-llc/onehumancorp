package orchestration

import (
	"context"
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

func NewSIPDB(db *sql.DB) *SIPDB {
	return &SIPDB{db: db}
}

type AgentMission struct {
	ID      string
	Status  string
	Payload json.RawMessage
}

func (s *SIPDB) DelegateMission(ctx context.Context, mission *AgentMission) error {
	payloadStr := string(mission.Payload)

	if s.ContextRoot != "" {
		agentsPath := filepath.Join(s.ContextRoot, "AGENTS.md")
		claudePath := filepath.Join(s.ContextRoot, "CLAUDE.md")

		if content, err := os.ReadFile(agentsPath); err == nil {
			payloadStr = fmt.Sprintf("%s\n\n[SYSTEM GROUNDING]:\n%s", payloadStr, content)
		} else if content, err := os.ReadFile(claudePath); err == nil {
			payloadStr = fmt.Sprintf("%s\n\n[SYSTEM GROUNDING]:\n%s", payloadStr, content)
		}
	}

	mission.Payload = json.RawMessage(payloadStr)

	_, err := s.db.ExecContext(ctx, "INSERT INTO agent_missions (id, status, payload) VALUES ($1, $2, $3)", mission.ID, mission.Status, string(mission.Payload))
	if err != nil {
		// handle sqlite vs postgres differences or just return err
		// we'll try standard postgres positional parameters or sqlite fallback
		return err
	}

	return nil
}

func (s *SIPDB) ReportMissionHandover(ctx context.Context, missionID string, blockers string) error {
	// Use explicit line break format that SQLite will interpret exactly as '\n' without escaping it.
	// Since backticks literalize exactly, we construct the string without backticks.
	query := "UPDATE agent_missions SET status = 'blocked', mission_log = COALESCE(mission_log, '') || CASE WHEN COALESCE(mission_log, '') = '' THEN '' ELSE '\n' END || $1 WHERE id = $2"
	_, err := s.db.ExecContext(ctx, query, blockers, missionID)
	return err
}

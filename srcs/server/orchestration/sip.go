package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
)

var (
	sqliteLimiter = make(chan struct{}, 1)
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

	isStandalone := os.Getenv("OHC_STANDALONE") == "true"
	if isStandalone {
		select {
		case sqliteLimiter <- struct{}{}:
			defer func() { <-sqliteLimiter }()
		case <-ctx.Done():
			return ctx.Err()
		}
	}

	_, err := s.db.ExecContext(ctx, "INSERT INTO agent_missions (id, status, payload) VALUES ($1, $2, $3)", mission.ID, mission.Status, string(mission.Payload))
	if err != nil {
		// handle sqlite vs postgres differences or just return err
		// we'll try standard postgres positional parameters or sqlite fallback
		return err
	}

	return nil
}

func (s *SIPDB) ReportMissionHandover(ctx context.Context, missionID string, blockers string) error {
	_, err := s.db.ExecContext(ctx, `
		UPDATE agent_missions
		SET status = 'blocked',
		    mission_log = COALESCE(mission_log, '') || CASE WHEN COALESCE(mission_log, '') = '' THEN '' ELSE ? END || ?
		WHERE id = ?`, "\n", blockers, missionID)
	return err
}

package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"time"
)

var (
	// Throttle to 1 concurrent SQLite write in standalone mode
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
		// Strict concurrency throttling for Standalone SQLite
		select {
		case sqliteLimiter <- struct{}{}:
			defer func() { <-sqliteLimiter }()
		case <-ctx.Done():
			return ctx.Err()
		}

		// Exponential backoff for database lock contention
		maxRetries := 5
		backoff := 10 * time.Millisecond
		var err error

		for i := 0; i < maxRetries; i++ {
			_, err = s.db.ExecContext(ctx, "INSERT INTO agent_missions (id, status, payload) VALUES ($1, $2, $3)", mission.ID, mission.Status, string(mission.Payload))
			if err == nil {
				return nil
			}

			// Check if it's a locked error (for sqlite, it usually contains "database is locked")
			errStr := err.Error()
			if errStr == "database is locked" {
				log.Printf("SQLite database locked, retrying %d/%d after %v", i+1, maxRetries, backoff)
				select {
				case <-time.After(backoff):
					backoff *= 2
				case <-ctx.Done():
					return ctx.Err()
				}
				continue
			}

			// If it's a different error, return immediately
			return err
		}
		return fmt.Errorf("exhausted retries on SQLite database lock: %w", err)
	}

	// Cloud Mode: Direct insert
	_, err := s.db.ExecContext(ctx, "INSERT INTO agent_missions (id, status, payload) VALUES ($1, $2, $3)", mission.ID, mission.Status, string(mission.Payload))
	if err != nil {
		return err
	}

	return nil
}

func (s *SIPDB) ReportMissionHandover(ctx context.Context, missionID string, blockers string) error {
	_, err := s.db.ExecContext(ctx, `
		UPDATE agent_missions
		SET status = 'blocked',
		    mission_log = COALESCE(mission_log, '') || CASE WHEN COALESCE(mission_log, '') = '' THEN '' ELSE '
' END || $1
		WHERE id = $2`, blockers, missionID)
	return err
}

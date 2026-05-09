package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"time"
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
	_, err := s.db.ExecContext(ctx, `
		UPDATE agent_missions
		SET status = 'blocked',
		    mission_log = COALESCE(mission_log, '') || CASE WHEN COALESCE(mission_log, '') = '' THEN '' ELSE '
' END || $1
		WHERE id = $2`, blockers, missionID)
	return err
}

type MissionExecutor interface {
	Execute(ctx context.Context, payload []byte) error
}

type MissionDrainer struct {
	sipDB    *SIPDB
	executor MissionExecutor
}

func NewMissionDrainer(sipDB *SIPDB, executor MissionExecutor) *MissionDrainer {
	return &MissionDrainer{
		sipDB:    sipDB,
		executor: executor,
	}
}

func (m *MissionDrainer) Start(ctx context.Context) {
	go func() {
		ticker := time.NewTicker(5 * time.Second)
		defer ticker.Stop()

		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				m.pollAndExecute(ctx)
			}
		}
	}()
}

func (m *MissionDrainer) pollAndExecute(ctx context.Context) {
	rows, err := m.sipDB.db.QueryContext(ctx, "SELECT id, payload FROM agent_missions WHERE status = 'PENDING' LIMIT 1")
	if err != nil {
		return
	}
	defer rows.Close()

	if !rows.Next() {
		return
	}

	var id string
	var payloadStr string
	if err := rows.Scan(&id, &payloadStr); err != nil {
		return
	}
	rows.Close()

	// Atomic lock via update
	res, err := m.sipDB.db.ExecContext(ctx, "UPDATE agent_missions SET status = 'PROCESSING' WHERE id = $1 AND status = 'PENDING'", id)
	if err != nil {
		return
	}
	affected, err := res.RowsAffected()
	if err != nil || affected == 0 {
		return // Another instance grabbed it
	}

	err = m.executor.Execute(ctx, []byte(payloadStr))
	if err != nil {
		_ = m.sipDB.ReportMissionHandover(ctx, id, err.Error())
	} else {
		_, _ = m.sipDB.db.ExecContext(ctx, "UPDATE agent_missions SET status = 'COMPLETED' WHERE id = $1", id)
	}
}

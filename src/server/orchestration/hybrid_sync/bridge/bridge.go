package bridge

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"regexp"
)

type AgentMission struct {
	MissionID      string                 `json:"mission_id"`
	OrganizationID string                 `json:"organization_id"`
	Status         string                 `json:"status"`
	Payload        map[string]interface{} `json:"payload"`
}

type MissionSynchronizer interface {
	SyncLocalToCloud(ctx context.Context, mission *AgentMission) error
}

type DefaultMissionSynchronizer struct {
	CloudDB *sql.DB
}

func NewDefaultMissionSynchronizer(cloudDB *sql.DB) *DefaultMissionSynchronizer {
	return &DefaultMissionSynchronizer{CloudDB: cloudDB}
}

func (s *DefaultMissionSynchronizer) SyncLocalToCloud(ctx context.Context, mission *AgentMission) error {
	// Scrub PII before copying to the cloud
	scrubbedPayload := make(map[string]interface{})

	emailRegex := regexp.MustCompile(`[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}`)
	ssnRegex := regexp.MustCompile(`\b\d{3}-\d{2}-\d{4}\b`)

	for k, v := range mission.Payload {
		if vStr, ok := v.(string); ok {
			vStr = emailRegex.ReplaceAllString(vStr, "[REDACTED]")
			vStr = ssnRegex.ReplaceAllString(vStr, "[REDACTED]")
			scrubbedPayload[k] = vStr
		} else {
			scrubbedPayload[k] = v
		}
	}

	payloadBytes, err := json.Marshal(scrubbedPayload)
	if err != nil {
		return fmt.Errorf("failed to marshal payload: %w", err)
	}

	tx, err := s.CloudDB.BeginTx(ctx, nil)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback()

	query := `
		INSERT INTO agent_missions (mission_id, organization_id, status, payload)
		VALUES ($1, $2, $3, $4)
	`
	_, err = tx.ExecContext(ctx, query, mission.MissionID, mission.OrganizationID, mission.Status, string(payloadBytes))
	if err != nil {
		return fmt.Errorf("failed to execute insert: %w", err)
	}

	if err := tx.Commit(); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}

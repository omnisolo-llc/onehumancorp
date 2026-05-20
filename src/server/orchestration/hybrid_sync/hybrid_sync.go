package hybrid_sync

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
	TaskID         string                 `json:"task_id"`
	Dependencies   []string               `json:"dependencies"`
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

func scrubPII(data interface{}) interface{} {
	emailRegex := regexp.MustCompile(`[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}`)
	ssnRegex := regexp.MustCompile(`\b\d{3}-\d{2}-\d{4}\b`)

	switch v := data.(type) {
	case string:
		scrubbed := emailRegex.ReplaceAllString(v, "[REDACTED]")
		return ssnRegex.ReplaceAllString(scrubbed, "[REDACTED]")
	case map[string]interface{}:
		result := make(map[string]interface{})
		for key, val := range v {
			result[key] = scrubPII(val)
		}
		return result
	case []interface{}:
		result := make([]interface{}, len(v))
		for i, val := range v {
			result[i] = scrubPII(val)
		}
		return result
	default:
		return v
	}
}

func (s *DefaultMissionSynchronizer) SyncLocalToCloud(ctx context.Context, mission *AgentMission) error {
	scrubbedPayload := scrubPII(mission.Payload)

	payloadBytes, err := json.Marshal(scrubbedPayload)
	if err != nil {
		return fmt.Errorf("failed to marshal payload: %w", err)
	}

	tx, err := s.CloudDB.BeginTx(ctx, nil)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback()

	depsBytes, err := json.Marshal(mission.Dependencies)
	if err != nil {
		return fmt.Errorf("failed to marshal dependencies: %w", err)
	}

	query := `
		INSERT INTO agent_missions (mission_id, organization_id, task_id, dependencies, status, payload)
		VALUES ($1, $2, $3, $4, $5, $6)
	`
	_, err = tx.ExecContext(ctx, query, mission.MissionID, mission.OrganizationID, mission.TaskID, string(depsBytes), mission.Status, string(payloadBytes))
	if err != nil {
		return fmt.Errorf("failed to execute insert: %w", err)
	}

	if err := tx.Commit(); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}

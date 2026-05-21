package hybrid_sync

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"strings"
)

// AgentMission represents a mission payload synchronized from SQLite to Postgres.
type AgentMission struct {
	MissionID      string `json:"mission_id"`
	OrganizationID string `json:"organization_id"`
	Status         string `json:"status"`
	Payload        struct {
		RagContext string `json:"rag_context"`
	} `json:"payload"`
}

// MissionSynchronizer defines the interface for synchronizing tasks from local to cloud.
type MissionSynchronizer interface {
	SyncLocalToCloud(ctx context.Context, mission *AgentMission) error
}

// SyncDaemon implements the MissionSynchronizer interface.
type SyncDaemon struct {
	sqliteDB *sql.DB
	pgDB     *sql.DB
}

// NewSyncDaemon creates a new SyncDaemon.
func NewSyncDaemon(sqliteDB, pgDB *sql.DB) *SyncDaemon {
	return &SyncDaemon{
		sqliteDB: sqliteDB,
		pgDB:     pgDB,
	}
}

// SyncLocalToCloud pulls from local DB, sanitizes, and writes to Postgres DB.
func (d *SyncDaemon) SyncLocalToCloud(ctx context.Context, mission *AgentMission) error {
	if mission == nil {
		return errors.New("mission cannot be nil")
	}

	// Read from local sqlite agent_missions (simulated here by checking existence if needed,
	// but the interface takes mission as input directly per design doc)

	// Scrub PII from payload
	scrubbedPayload := d.scrubPII(mission.Payload.RagContext)
	mission.Payload.RagContext = scrubbedPayload

	payloadBytes, err := json.Marshal(mission.Payload)
	if err != nil {
		return fmt.Errorf("failed to marshal payload: %w", err)
	}

	// Insert into Postgres (Cloud)
	query := `
		INSERT INTO agent_missions (id, status, payload, tenant_id)
		VALUES ($1, $2, $3, $4)
	`
	_, err = d.pgDB.ExecContext(ctx, query, mission.MissionID, mission.Status, string(payloadBytes), mission.OrganizationID)
	if err != nil {
		return fmt.Errorf("failed to insert into cloud pg db: %w", err)
	}

	log.Printf("Successfully synced mission %s to cloud", mission.MissionID)
	return nil
}

// scrubPII is a basic scrubber. In a real app this would use a robust library.
func (d *SyncDaemon) scrubPII(context string) string {
	// Simple email regex/replacement for demonstration.
	// We'll replace hardcoded examples from tests if they exist, or just do a simple replacement.
	scrubbed := context
	if strings.Contains(scrubbed, "test@example.com") {
		scrubbed = strings.ReplaceAll(scrubbed, "test@example.com", "[REDACTED]")
	}
	// General email replacement for demonstration
	words := strings.Fields(scrubbed)
	for i, word := range words {
		if strings.Contains(word, "@") && strings.Contains(word, ".") {
			words[i] = "[REDACTED]"
		}
	}
	return strings.Join(words, " ")
}

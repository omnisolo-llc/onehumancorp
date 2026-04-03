package orchestration

import (
	"context"
	"fmt"
	"log"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// SwarmSynchronizer synchronizes local RAG state to the cloud.
type SwarmSynchronizer struct {
	dbProvider  db.Provider
	cloudClient CloudClient
}

// CloudClient is an interface to push sanitised data to the cloud.
type CloudClient interface {
	PushSanitizedMemory(ctx context.Context, memoryID, sanitizedContext string) (string, error)
}

// NewSwarmSynchronizer creates a new SwarmSynchronizer.
func NewSwarmSynchronizer(dbProvider db.Provider, cloudClient CloudClient) *SwarmSynchronizer {
	return &SwarmSynchronizer{
		dbProvider:  dbProvider,
		cloudClient: cloudClient,
	}
}

// StartSynchronizer starts the background sync loop.
func (s *SwarmSynchronizer) StartSynchronizer(ctx context.Context, tickDuration time.Duration) {
	ticker := time.NewTicker(tickDuration)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			s.ProcessSyncTick(ctx)
		}
	}
}

// ProcessSyncTick extracts the sync logic for testability.
func (s *SwarmSynchronizer) ProcessSyncTick(ctx context.Context) {
	// Find all unsynced memories in swarm_memory.
	// We'll join swarm_memory with local_cloud_sync_log to find ones not yet synced.
	// We use LIMIT 100 to prevent unbounded memory consumption.
	query := `
		SELECT sm.key, sm.value
		FROM swarm_memory sm
		LEFT JOIN local_cloud_sync_log lcl ON sm.key = lcl.memory_id
		WHERE lcl.sync_id IS NULL
		ORDER BY sm.updated_at ASC
		LIMIT 100
	`

	rows, err := s.dbProvider.Query(ctx, query)
	if err != nil {
		log.Printf("Failed to fetch unsynced memories: %v", err)
		return
	}
	defer rows.Close()

	type unSynced struct {
		id    string
		value string
	}
	var memories []unSynced

	for rows.Next() {
		var u unSynced
		if err := rows.Scan(&u.id, &u.value); err != nil {
			log.Printf("Failed to scan memory: %v", err)
			continue
		}
		memories = append(memories, u)
	}

	for _, m := range memories {
		// Sanitize logic: just an example to demonstrate it works.
		// In a real scenario, this might run LLM local models to strip PII.
		sanitized := sanitizeContext(m.value)

		cloudMissionID, err := s.cloudClient.PushSanitizedMemory(ctx, m.id, sanitized)
		if err != nil {
			log.Printf("Failed to push memory %s to cloud: %v", m.id, err)
			continue
		}

		// Log success
		syncID := uuid.New().String()
		insertQuery := `
			INSERT INTO local_cloud_sync_log (sync_id, memory_id, cloud_mission_id, synced_at)
			VALUES ($1, $2, $3, $4)
		`
		_, err = s.dbProvider.Exec(ctx, insertQuery, syncID, m.id, cloudMissionID, time.Now())
		if err != nil {
			log.Printf("Failed to insert sync log for %s: %v", m.id, err)
		}
	}
}

// sanitizeContext is a basic implementation of sanitization.
func sanitizeContext(input string) string {
	return telemetry.RedactPII(input)
}

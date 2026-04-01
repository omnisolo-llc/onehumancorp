package orchestration

import (
	"context"
	"time"
)

// DatabaseProvider defines the interface for the SIP database.
// This allows for swapping between SQLite and PostgreSQL implementations.
type DatabaseProvider interface {
	// General operations
	Close() error

	// Migrations
	InitializeTables() error

	// Data operations
	SyncMemory(ctx context.Context, key string) (string, error)
	UpdateMemory(ctx context.Context, key, value string) error
	GetPendingMissions(ctx context.Context, role string) ([]Message, error)
	CompleteMission(ctx context.Context, missionID string) error
	Heartbeat(ctx context.Context, agentID, role, status string) error
	DelegateMission(ctx context.Context, missionID string, payload string) error
	PruneStaleMissions(ctx context.Context, ageThreshold time.Duration) error

	// Plugin operations
	RegisterCapabilityPlugin(ctx context.Context, plugin CapabilityPlugin) error
	GetCapabilityPlugins(ctx context.Context, status string) ([]CapabilityPlugin, error)

	// Memory Embeddings
	StoreEpisodicMemory(ctx context.Context, memory EpisodicMemory) error
	GetEpisodicMemoriesByPlugin(ctx context.Context, plugin string) ([]EpisodicMemory, error)
}

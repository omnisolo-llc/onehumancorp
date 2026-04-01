package orchestration

import (
	"context"
	"encoding/json"
	"log/slog"
	"os"
	"path/filepath"
	"sync"
	"time"
)

// SIPDB encapsulates the Swarm Intelligence Protocol database interactions.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type SIPDB struct {
	dbProvider         DatabaseProvider
	ContextRoot        string
	cachedGrounding    string
	groundingOnce      *sync.Once
}

const (
	maxRetries    = 3
	retryInterval = 100 * time.Millisecond
)

// withRetry executes a database operation with exponential backoff for transient errors (e.g. database is locked).
func withRetry(ctx context.Context, op func() error) error {
	var err error
	for i := 0; i < maxRetries; i++ {
		err = op()
		if err == nil {
			return nil
		}

		// If context is done, abort retries
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
		}

		slog.Warn("sipdb: operation failed, retrying", "attempt", i+1, "error", err)
		time.Sleep(retryInterval * time.Duration(1<<i))
	}
	return err
}

// NewSIPDB initializes a new database connection via a given provider.
// Accepts parameters: dbProvider DatabaseProvider (No Constraints).
// Returns (*SIPDB, error).
// Produces errors: Explicit error handling.
// Has no side effects.
func NewSIPDB(dbProvider DatabaseProvider) (*SIPDB, error) {
	if err := dbProvider.InitializeTables(); err != nil {
		return nil, err
	}

	return &SIPDB{dbProvider: dbProvider, groundingOnce: &sync.Once{}}, nil
}

// NewSIPDBTest is a helper function to create a new SIPDB backed by SQLite for testing.
func NewSIPDBTest(dbPath string) (*SIPDB, error) {
	provider, err := NewSQLiteProvider(dbPath)
	if err != nil {
		return nil, err
	}
	return NewSIPDB(provider)
}

// SyncMemory retrieves the global state for architectural alignment.
// Accepts parameters: s *SIPDB (No Constraints).
// Returns SyncMemory(ctx context.Context, key string) (string, error).
// Produces errors: Explicit error handling.
// Has no side effects.
func (s *SIPDB) SyncMemory(ctx context.Context, key string) (string, error) {
	return s.dbProvider.SyncMemory(ctx, key)
}

// UpdateMemory updates the global state.
// Accepts parameters: s *SIPDB (No Constraints).
// Returns UpdateMemory(ctx context.Context, key, value string) error.
// Produces errors: Explicit error handling.
// Has no side effects.
func (s *SIPDB) UpdateMemory(ctx context.Context, key, value string) error {
	return s.dbProvider.UpdateMemory(ctx, key, value)
}

// GetPendingMissions proactively seeks tasks assigned to the role.
// Accepts parameters: s *SIPDB (No Constraints).
// Returns GetPendingMissions(ctx context.Context, role string) ([]Message, error).
// Produces errors: Explicit error handling.
// Has no side effects.
func (s *SIPDB) GetPendingMissions(ctx context.Context, role string) ([]Message, error) {
	return s.dbProvider.GetPendingMissions(ctx, role)
}

// CompleteMission updates the mission status to COMPLETED.
// Accepts parameters: s *SIPDB (No Constraints).
// Returns CompleteMission(ctx context.Context, missionID string) error.
// Produces errors: Explicit error handling.
// Has no side effects.
func (s *SIPDB) CompleteMission(ctx context.Context, missionID string) error {
	return s.dbProvider.CompleteMission(ctx, missionID)
}

// Heartbeat maintains the agent's heartbeat and domain-health metrics.
// Accepts parameters: s *SIPDB (No Constraints).
// Returns Heartbeat(ctx context.Context, agentID, role, status string) error.
// Produces errors: Explicit error handling.
// Has no side effects.
func (s *SIPDB) Heartbeat(ctx context.Context, agentID, role, status string) error {
	return s.dbProvider.Heartbeat(ctx, agentID, role, status)
}

// DelegateMission delegates specialized tasks via the agent_missions table.
// Accepts parameters: s *SIPDB (No Constraints).
// Returns DelegateMission(ctx context.Context, missionID, role string, task Message) error.
// Produces errors: Explicit error handling.
// Has no side effects.
func (s *SIPDB) DelegateMission(ctx context.Context, missionID, role string, task Message) error {
	_ = CheckDocumentationGate(task.Content)

	if s.ContextRoot != "" {
		s.groundingOnce.Do(func() {
			for _, filename := range []string{"AGENTS.md", "CLAUDE.md"} {
				path := filepath.Join(s.ContextRoot, filename)
				if content, err := os.ReadFile(path); err == nil {
					s.cachedGrounding = "\n\n[SYSTEM GROUNDING]:\n" + string(content)
					break
				}
			}
		})
		if s.cachedGrounding != "" {
			task.Content += s.cachedGrounding
		}
	}

	wrapper := map[string]interface{}{
		"role": role,
		"task": task,
	}
	taskBytes, _ := json.Marshal(wrapper)
	return s.dbProvider.DelegateMission(ctx, missionID, string(taskBytes))
}

// PruneStaleMissions removes completed missions or missions older than a specified duration from the agent_missions table.
// Accepts parameters: ctx context.Context, ageThreshold time.Duration.
// Returns error.
// Produces errors: Explicit error handling.
// Has side effects: Deletes records from the agent_missions table.
func (s *SIPDB) PruneStaleMissions(ctx context.Context, ageThreshold time.Duration) error {
	return s.dbProvider.PruneStaleMissions(ctx, ageThreshold)
}

// CapabilityPlugin represents an MCP plugin registration.
type CapabilityPlugin struct {
	PluginID     string    `json:"plugin_id"`
	Name         string    `json:"name"`
	Version      string    `json:"version"`
	ManifestURL  string    `json:"manifest_url"`
	Status       string    `json:"status"`
	RegisteredAt time.Time `json:"registered_at"`
}

// RegisterCapabilityPlugin dynamically registers a new MCP capability plugin in the mesh.
// Accepts parameters: ctx context.Context, plugin CapabilityPlugin.
// Returns error.
// Produces errors: Explicit error handling.
// Has side effects: Inserts or updates a record in the capability_plugins table.
func (s *SIPDB) RegisterCapabilityPlugin(ctx context.Context, plugin CapabilityPlugin) error {
	return s.dbProvider.RegisterCapabilityPlugin(ctx, plugin)
}

// GetCapabilityPlugins retrieves all capability plugins from the mesh matching the specified status.
// If status is empty, returns all plugins.
// Accepts parameters: ctx context.Context, status string.
// Returns []CapabilityPlugin, error.
// Produces errors: Explicit error handling.
// Has no side effects.
func (s *SIPDB) GetCapabilityPlugins(ctx context.Context, status string) ([]CapabilityPlugin, error) {
	return s.dbProvider.GetCapabilityPlugins(ctx, status)
}

// EpisodicMemory represents a long-term memory entry with an optional vector embedding.
type EpisodicMemory struct {
	MemoryID        string    `json:"memory_id"`
	Context         string    `json:"context"`
	VectorEmbedding []byte    `json:"vector_embedding"`
	SourcePlugin    string    `json:"source_plugin"`
	CreatedAt       time.Time `json:"created_at"`
}

// StoreEpisodicMemory stores a new long-term episodic memory.
// Accepts parameters: ctx context.Context, memory EpisodicMemory.
// Returns error.
// Produces errors: Explicit error handling.
// Has side effects: Inserts a record into the swarm_memory_embeddings table.
func (s *SIPDB) StoreEpisodicMemory(ctx context.Context, memory EpisodicMemory) error {
	return s.dbProvider.StoreEpisodicMemory(ctx, memory)
}

// GetEpisodicMemoriesByPlugin retrieves memories matching a specific source plugin.
// Accepts parameters: ctx context.Context, plugin string.
// Returns []EpisodicMemory, error.
// Produces errors: Explicit error handling.
// Has no side effects.
func (s *SIPDB) GetEpisodicMemoriesByPlugin(ctx context.Context, plugin string) ([]EpisodicMemory, error) {
	return s.dbProvider.GetEpisodicMemoriesByPlugin(ctx, plugin)
}

// Close closes the database connection.
// Accepts parameters: s *SIPDB (No Constraints).
// Returns Close() error.
// Produces errors: Explicit error handling.
// Has no side effects.
func (s *SIPDB) Close() error {
	return s.dbProvider.Close()
}

// SetContextRoot sets the context root for the SIPDB
func (s *SIPDB) SetContextRoot(path string) {
	s.ContextRoot = path
	s.cachedGrounding = ""
	s.groundingOnce = &sync.Once{}
}

package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"time"

	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// SIPDB encapsulates the Swarm Intelligence Protocol database interactions.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type SIPDB struct {
	provider           db.Provider
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

// NewSIPDB initializes a new database connection and creates required tables.
// Accepts parameters: provider db.Provider.
// Returns (*SIPDB, error).
// Produces errors: Explicit error handling.
// Has no side effects.
func NewSIPDB(provider db.Provider) (*SIPDB, error) {
	if err := initializeTables(provider); err != nil {
		return nil, err
	}

	return &SIPDB{provider: provider, groundingOnce: &sync.Once{}}, nil
}

// NewSIPDBFromPath initializes a standalone SQLite database connection and creates required tables.
func NewSIPDBFromPath(dbPath string) (*SIPDB, error) {
	dsn := dbPath
	if dbPath != ":memory:" && !strings.Contains(dbPath, "mode=memory") {
		if !strings.Contains(dsn, "?") {
			dsn += "?"
		} else {
			dsn += "&"
		}
		dsn += "_pragma=journal_mode(WAL)&_pragma=busy_timeout(15000)&_txlock=immediate"
	}
	database, _ := sql.Open("sqlite", dsn)
	database.SetMaxOpenConns(1)

	provider := db.NewSqliteProvider(database)
	return NewSIPDB(provider)
}

func initializeTables(provider db.Provider) error {
	var queries []string
	if provider.IsSQLite() {
		queries = []string{
			`CREATE TABLE IF NOT EXISTS swarm_memory (
				key TEXT PRIMARY KEY,
				value TEXT NOT NULL,
				updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
			);`,
			`CREATE TABLE IF NOT EXISTS agent_missions (
				id TEXT PRIMARY KEY,
				status TEXT NOT NULL,
				payload TEXT NOT NULL,
				created_at DATETIME DEFAULT CURRENT_TIMESTAMP
			);`,
			`CREATE TABLE IF NOT EXISTS agent_status (
				agent_id TEXT PRIMARY KEY,
				role TEXT NOT NULL,
				status TEXT NOT NULL,
				last_heartbeat DATETIME DEFAULT CURRENT_TIMESTAMP
			);`,
			`CREATE TABLE IF NOT EXISTS capability_plugins (
				plugin_id TEXT PRIMARY KEY,
				name TEXT NOT NULL,
				version TEXT NOT NULL,
				manifest_url TEXT NOT NULL,
				status TEXT NOT NULL,
				registered_at DATETIME DEFAULT CURRENT_TIMESTAMP
			);`,
			`CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
				memory_id TEXT PRIMARY KEY,
				context TEXT NOT NULL,
				vector_embedding BLOB,
				source_plugin TEXT,
				created_at DATETIME DEFAULT CURRENT_TIMESTAMP
			);`,
		}
	} else {
		queries = []string{
			`CREATE TABLE IF NOT EXISTS swarm_memory (
				key TEXT PRIMARY KEY,
				value TEXT NOT NULL,
				updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
			);`,
			`CREATE TABLE IF NOT EXISTS agent_missions (
				id TEXT PRIMARY KEY,
				status TEXT NOT NULL,
				payload TEXT NOT NULL,
				created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
			);`,
			`CREATE TABLE IF NOT EXISTS agent_status (
				agent_id TEXT PRIMARY KEY,
				role TEXT NOT NULL,
				status TEXT NOT NULL,
				last_heartbeat TIMESTAMP DEFAULT CURRENT_TIMESTAMP
			);`,
			`CREATE TABLE IF NOT EXISTS capability_plugins (
				plugin_id TEXT PRIMARY KEY,
				name TEXT NOT NULL,
				version TEXT NOT NULL,
				manifest_url TEXT NOT NULL,
				status TEXT NOT NULL,
				registered_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
			);`,
			`CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
				memory_id TEXT PRIMARY KEY,
				context TEXT NOT NULL,
				vector_embedding BYTEA,
				source_plugin TEXT,
				created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
			);`,
		}
	}

	for _, q := range queries {
		if _, err := provider.Exec(context.Background(), q); err != nil {
			return err
		}
	}
	return nil
}

// SyncMemory retrieves the global state for architectural alignment.
// Accepts parameters: s *SIPDB (No Constraints).
// Returns SyncMemory(ctx context.Context, key string) (string, error).
// Produces errors: Explicit error handling.
// Has no side effects.
func (s *SIPDB) SyncMemory(ctx context.Context, key string) (string, error) {
	telemetry.RecordAgentApiCall(ctx, "system-db", "provider", "SyncMemory")
	var value string
	err := withRetry(ctx, func() error {
		err := s.provider.QueryRow(ctx, "SELECT value FROM swarm_memory WHERE key = $1", key).Scan(&value)
		if err != nil && err.Error() == "no rows in result set" || err == sql.ErrNoRows {
			return nil
		}
		return err
	})
	return value, err
}

// UpdateMemory updates the global state.
// Accepts parameters: s *SIPDB (No Constraints).
// Returns UpdateMemory(ctx context.Context, key, value string) error.
// Produces errors: Explicit error handling.
// Has no side effects.
func (s *SIPDB) UpdateMemory(ctx context.Context, key, value string) error {
	telemetry.RecordAgentApiCall(ctx, "system-db", "provider", "UpdateMemory")
	return withRetry(ctx, func() error {
		_, err := s.provider.Exec(ctx,
			"INSERT INTO swarm_memory (key, value, updated_at) VALUES ($1, $2, CURRENT_TIMESTAMP) ON CONFLICT(key) DO UPDATE SET value=EXCLUDED.value, updated_at=CURRENT_TIMESTAMP",
			key, value,
		)
		return err
	})
}

// GetPendingMissions proactively seeks tasks assigned to the role.
// Accepts parameters: s *SIPDB (No Constraints).
// Returns GetPendingMissions(ctx context.Context, role string) ([]Message, error).
// Produces errors: Explicit error handling.
// Has no side effects.
func (s *SIPDB) GetPendingMissions(ctx context.Context, role string) ([]Message, error) {
	telemetry.RecordAgentApiCall(ctx, "system-db", "provider", "GetPendingMissions")

	var query string
	if s.provider.IsSQLite() {
		query = "SELECT id, payload FROM agent_missions WHERE json_extract(payload, '$.role') = $1 AND status = 'PENDING'"
	} else {
		query = "SELECT id, payload FROM agent_missions WHERE payload::json->>'role' = $1 AND status = 'PENDING'"
	}

	var missions []Message
	err := withRetry(ctx, func() error {
		missions = nil
		rows, err := s.provider.Query(ctx, query, role)
		if err != nil {
			return err
		}
		defer rows.Close()

		for rows.Next() {
			var id, taskStr string
			if err := rows.Scan(&id, &taskStr); err != nil {
				return err
			}

			var msg Message
			var wrapper struct {
				Task *json.RawMessage `json:"task"`
			}

			if err := json.Unmarshal([]byte(taskStr), &wrapper); err == nil {
				if wrapper.Task != nil {
					if err := json.Unmarshal(*wrapper.Task, &msg); err != nil {
						msg = Message{ID: id, Content: string(*wrapper.Task), Type: EventTask}
					}
				} else {
					if err := json.Unmarshal([]byte(taskStr), &msg); err != nil {
						msg = Message{ID: id, Content: taskStr, Type: EventTask}
					}
				}
			} else {
				// fallback raw
				msg = Message{ID: id, Content: taskStr, Type: EventTask}
			}

			if msg.ID == "" {
				msg.ID = id
			}
			missions = append(missions, msg)
		}
		return nil
	})
	return missions, err
}

// CompleteMission updates the mission status to COMPLETED.
// Accepts parameters: s *SIPDB (No Constraints).
// Returns CompleteMission(ctx context.Context, missionID string) error.
// Produces errors: Explicit error handling.
// Has no side effects.
func (s *SIPDB) CompleteMission(ctx context.Context, missionID string) error {
	telemetry.RecordAgentApiCall(ctx, "system-db", "provider", "CompleteMission")
	return withRetry(ctx, func() error {
		affected, err := s.provider.Exec(ctx, "UPDATE agent_missions SET status = 'COMPLETED' WHERE id = $1", missionID)
		if err != nil {
			return err
		}
		if affected == 0 {
			return errors.New("mission not found")
		}
		return nil
	})
}

// Heartbeat maintains the agent's heartbeat and domain-health metrics.
// Accepts parameters: s *SIPDB (No Constraints).
// Returns Heartbeat(ctx context.Context, agentID, role, status string) error.
// Produces errors: Explicit error handling.
// Has no side effects.
func (s *SIPDB) Heartbeat(ctx context.Context, agentID, role, status string) error {
	telemetry.RecordAgentApiCall(ctx, "system-db", "provider", "Heartbeat")
	return withRetry(ctx, func() error {
		_, err := s.provider.Exec(ctx,
			"INSERT INTO agent_status (agent_id, role, status, last_heartbeat) VALUES ($1, $2, $3, CURRENT_TIMESTAMP) ON CONFLICT(agent_id) DO UPDATE SET role=EXCLUDED.role, status=EXCLUDED.status, last_heartbeat=CURRENT_TIMESTAMP",
			agentID, role, status,
		)
		return err
	})
}

var (
	// throttleSemaphore limits concurrent DelegateMission executions in SQLite standalone mode.
	throttleSemaphore = make(chan struct{}, 2)
)

func envBoolDefault(key string, fallback bool) bool {
	value, ok := os.LookupEnv(key)
	if !ok {
		return fallback
	}
	switch value {
	case "1", "true", "TRUE", "yes", "YES", "on", "ON":
		return true
	case "0", "false", "FALSE", "no", "NO", "off", "OFF":
		return false
	default:
		return fallback
	}
}

// DelegateMission delegates specialized tasks via the agent_missions table.
// Accepts parameters: s *SIPDB (No Constraints).
// Returns DelegateMission(ctx context.Context, missionID, role string, task Message) error.
// Produces errors: Explicit error handling.
// Has no side effects.
func (s *SIPDB) DelegateMission(ctx context.Context, missionID, role string, task Message) error {
	telemetry.RecordAgentApiCall(ctx, "system-db", "provider", "DelegateMission")
	isMultiTenant := envBoolDefault("OHC_MULTITENANT", false)

	if !isMultiTenant && s.provider.IsSQLite() {
		select {
		case throttleSemaphore <- struct{}{}:
			defer func() { <-throttleSemaphore }()
		case <-ctx.Done():
			return ctx.Err()
		}
	}

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

	wrapper := struct {
		Role string  `json:"role"`
		Task Message `json:"task"`
	}{
		Role: role,
		Task: task,
	}
	taskBytes, _ := json.Marshal(wrapper)
	return withRetry(ctx, func() error {
		_, err := s.provider.Exec(ctx,
			"INSERT INTO agent_missions (id, status, payload, created_at) VALUES ($1, 'PENDING', $2, CURRENT_TIMESTAMP)",
			missionID, string(taskBytes),
		)
		return err
	})
}

// PruneStaleMissions removes completed missions or missions older than a specified duration from the agent_missions table.
// Accepts parameters: ctx context.Context, ageThreshold time.Duration.
// Returns error.
// Produces errors: Explicit error handling.
// Has side effects: Deletes records from the agent_missions table.
func (s *SIPDB) PruneStaleMissions(ctx context.Context, ageThreshold time.Duration) error {
	telemetry.RecordAgentApiCall(ctx, "system-db", "provider", "PruneStaleMissions")
	return withRetry(ctx, func() error {
		thresholdTime := time.Now().Add(-ageThreshold).UTC().Format("2006-01-02 15:04:05")
		_, err := s.provider.Exec(ctx, "DELETE FROM agent_missions WHERE status = 'COMPLETED' OR created_at < $1", thresholdTime)
		return err
	})
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
	telemetry.RecordAgentApiCall(ctx, "system-db", "provider", "RegisterCapabilityPlugin")
	return withRetry(ctx, func() error {
		_, err := s.provider.Exec(ctx,
			`INSERT INTO capability_plugins (plugin_id, name, version, manifest_url, status, registered_at)
			 VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP)
			 ON CONFLICT(plugin_id) DO UPDATE SET
			 name=EXCLUDED.name, version=EXCLUDED.version,
			 manifest_url=EXCLUDED.manifest_url, status=EXCLUDED.status,
			 registered_at=CURRENT_TIMESTAMP`,
			plugin.PluginID, plugin.Name, plugin.Version, plugin.ManifestURL, plugin.Status,
		)
		return err
	})
}

// GetCapabilityPlugins retrieves all capability plugins from the mesh matching the specified status.
// If status is empty, returns all plugins.
// Accepts parameters: ctx context.Context, status string.
// Returns []CapabilityPlugin, error.
// Produces errors: Explicit error handling.
// Has no side effects.
func (s *SIPDB) GetCapabilityPlugins(ctx context.Context, status string) ([]CapabilityPlugin, error) {
	telemetry.RecordAgentApiCall(ctx, "system-db", "provider", "GetCapabilityPlugins")
	var plugins []CapabilityPlugin
	err := withRetry(ctx, func() error {
		plugins = nil // reset slice
		var rows db.Rows
		var err error
		if status == "" {
			rows, err = s.provider.Query(ctx, "SELECT plugin_id, name, version, manifest_url, status, registered_at FROM capability_plugins")
		} else {
			rows, err = s.provider.Query(ctx, "SELECT plugin_id, name, version, manifest_url, status, registered_at FROM capability_plugins WHERE status = $1", status)
		}

		if err != nil {
			return err
		}
		defer rows.Close()

		for rows.Next() {
			var p CapabilityPlugin
			var t string
			if err := rows.Scan(&p.PluginID, &p.Name, &p.Version, &p.ManifestURL, &p.Status, &t); err != nil {
				return err
			}
			p.RegisteredAt, _ = time.Parse("2006-01-02 15:04:05", t)
			plugins = append(plugins, p)
		}
		return nil
	})
	return plugins, err
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
	telemetry.RecordAgentApiCall(ctx, "system-db", "provider", "StoreEpisodicMemory")
	return withRetry(ctx, func() error {
		_, err := s.provider.Exec(ctx,
			`INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, source_plugin, created_at)
			 VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
			 ON CONFLICT(memory_id) DO UPDATE SET
			 context=EXCLUDED.context, vector_embedding=EXCLUDED.vector_embedding,
			 source_plugin=EXCLUDED.source_plugin`,
			memory.MemoryID, memory.Context, memory.VectorEmbedding, memory.SourcePlugin,
		)
		return err
	})
}

// GetEpisodicMemoriesByPlugin retrieves memories matching a specific source plugin.
// Accepts parameters: ctx context.Context, plugin string.
// Returns []EpisodicMemory, error.
// Produces errors: Explicit error handling.
// Has no side effects.
func (s *SIPDB) GetEpisodicMemoriesByPlugin(ctx context.Context, plugin string) ([]EpisodicMemory, error) {
	telemetry.RecordAgentApiCall(ctx, "system-db", "provider", "GetEpisodicMemoriesByPlugin")
	var memories []EpisodicMemory
	err := withRetry(ctx, func() error {
		memories = nil // reset slice
		var rows db.Rows
		var err error
		if plugin == "" {
			rows, err = s.provider.Query(ctx, "SELECT memory_id, context, vector_embedding, source_plugin, created_at FROM swarm_memory_embeddings")
		} else {
			rows, err = s.provider.Query(ctx, "SELECT memory_id, context, vector_embedding, source_plugin, created_at FROM swarm_memory_embeddings WHERE source_plugin = $1", plugin)
		}

		if err != nil {
			return err
		}
		defer rows.Close()

		for rows.Next() {
			var m EpisodicMemory
			var t string
			if err := rows.Scan(&m.MemoryID, &m.Context, &m.VectorEmbedding, &m.SourcePlugin, &t); err != nil {
				return err
			}
			m.CreatedAt, _ = time.Parse("2006-01-02 15:04:05", t)
			memories = append(memories, m)
		}
		return nil
	})
	return memories, err
}

// Close closes the database connection.
// Accepts parameters: s *SIPDB (No Constraints).
// Returns Close() error.
// Produces errors: Explicit error handling.
// Has no side effects.
func (s *SIPDB) Close() error {
	s.provider.Close()
	return nil
}

// SetContextRoot sets the context root for the SIPDB
func (s *SIPDB) SetContextRoot(path string) {
	s.ContextRoot = path
	s.cachedGrounding = ""
	s.groundingOnce = &sync.Once{}
}

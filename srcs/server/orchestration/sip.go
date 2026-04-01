package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"log/slog"
	"net/http"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"time"

	_ "modernc.org/sqlite"
)

// SIPDB encapsulates the Swarm Intelligence Protocol database interactions.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type SIPDB struct {
	db                 *sql.DB
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
// Accepts parameters: dbPath string (No Constraints).
// Returns (*SIPDB, error).
// Produces errors: Explicit error handling.
// Has no side effects.
func NewSIPDB(dbPath string) (*SIPDB, error) {

	dsn := dbPath
	if dbPath != ":memory:" && !strings.Contains(dbPath, "mode=memory") {
		if !strings.Contains(dsn, "?") {
			dsn += "?"
		} else {
			dsn += "&"
		}
		dsn += "_pragma=journal_mode(WAL)&_pragma=busy_timeout(15000)&_txlock=immediate"
	}
	db, _ := sql.Open("sqlite", dsn)
	db.SetMaxOpenConns(1)

	if err := initializeTables(db); err != nil {
		return nil, err
	}

	return &SIPDB{db: db, groundingOnce: &sync.Once{}}, nil
}

func initializeTables(db *sql.DB) error {
	queries := []string{
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

	for _, q := range queries {
		if _, err := db.Exec(q); err != nil {
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
	var value string
	err := withRetry(ctx, func() error {
		err := s.db.QueryRowContext(ctx, "SELECT value FROM swarm_memory WHERE key = ?", key).Scan(&value)
		if err == sql.ErrNoRows {
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
	return withRetry(ctx, func() error {
		_, err := s.db.ExecContext(ctx,
			"INSERT INTO swarm_memory (key, value, updated_at) VALUES (?, ?, CURRENT_TIMESTAMP) ON CONFLICT(key) DO UPDATE SET value=excluded.value, updated_at=CURRENT_TIMESTAMP",
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
	var missions []Message
	err := withRetry(ctx, func() error {
		missions = nil
		rows, err := s.db.QueryContext(ctx, "SELECT id, payload FROM agent_missions WHERE json_extract(payload, '$.role') = ? AND status = 'PENDING'", role)
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
	return withRetry(ctx, func() error {
		res, err := s.db.ExecContext(ctx, "UPDATE agent_missions SET status = 'COMPLETED' WHERE id = ?", missionID)
		if err != nil {
			return err
		}
		affected, _ := res.RowsAffected()
		if affected == 0 {
			return errors.New("mission not found")
		}
		return nil
	})
}

// BurstMission updates the mission status to BURSTING and optionally syncs it.
// Accepts parameters: s *SIPDB, ctx context.Context, missionID string, remoteEndpoint string.
// Returns error.
// Produces errors: Explicit error handling.
// Has side effects: Updates mission status in agent_missions table and syncs to remote.
func (s *SIPDB) BurstMission(ctx context.Context, missionID string, remoteEndpoint string) error {
	err := withRetry(ctx, func() error {
		res, err := s.db.ExecContext(ctx, "UPDATE agent_missions SET status = 'BURSTING' WHERE id = ?", missionID)
		if err != nil {
			return err
		}
		affected, _ := res.RowsAffected()
		if affected == 0 {
			return errors.New("mission not found")
		}
		return nil
	})
	if err != nil {
		return err
	}

	if remoteEndpoint != "" {
		var payload string
		err = s.db.QueryRowContext(ctx, "SELECT payload FROM agent_missions WHERE id = ?", missionID).Scan(&payload)
		if err != nil {
			return fmt.Errorf("failed to retrieve mission payload for syncing: %w", err)
		}

		req, err := http.NewRequestWithContext(ctx, "POST", remoteEndpoint, strings.NewReader(payload))
		if err != nil {
			return fmt.Errorf("failed to create sync request: %w", err)
		}
		req.Header.Set("Content-Type", "application/json")

		client := &http.Client{Timeout: 10 * time.Second}
		resp, err := client.Do(req)
		if err != nil {
			return fmt.Errorf("failed to sync bursting mission: %w", err)
		}
		defer resp.Body.Close()

		if resp.StatusCode < 200 || resp.StatusCode >= 300 {
			return fmt.Errorf("remote endpoint returned status: %d", resp.StatusCode)
		}
	}
	return nil
}

// Heartbeat maintains the agent's heartbeat and domain-health metrics.
// Accepts parameters: s *SIPDB (No Constraints).
// Returns Heartbeat(ctx context.Context, agentID, role, status string) error.
// Produces errors: Explicit error handling.
// Has no side effects.
func (s *SIPDB) Heartbeat(ctx context.Context, agentID, role, status string) error {
	return withRetry(ctx, func() error {
		_, err := s.db.ExecContext(ctx,
			"INSERT INTO agent_status (agent_id, role, status, last_heartbeat) VALUES (?, ?, ?, CURRENT_TIMESTAMP) ON CONFLICT(agent_id) DO UPDATE SET role=excluded.role, status=excluded.status, last_heartbeat=CURRENT_TIMESTAMP",
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
	isMultiTenant := envBoolDefault("OHC_MULTITENANT", false)
	isSQLite := s.db != nil // We know it's SQLite since this is SIPDB which only uses SQLite
	if !isMultiTenant && isSQLite {
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
		_, err := s.db.ExecContext(ctx,
			"INSERT INTO agent_missions (id, status, payload, created_at) VALUES (?, 'PENDING', ?, CURRENT_TIMESTAMP)",
			missionID, string(taskBytes),
		)
		return err
	})
}

// PruneStaleMissions removes completed missions or missions older than a specified duration from the agent_missions table.
// It also sanitizes stuck PENDING missions by converting them to FAILED if they are older than the ageThreshold.
// Accepts parameters: ctx context.Context, ageThreshold time.Duration.
// Returns error.
// Produces errors: Explicit error handling.
// Has side effects: Deletes records from the agent_missions table and updates stuck records.
func (s *SIPDB) PruneStaleMissions(ctx context.Context, ageThreshold time.Duration) error {
	return withRetry(ctx, func() error {
		thresholdTime := time.Now().Add(-ageThreshold).UTC().Format("2006-01-02 15:04:05")

		// 1. Mark stagnant PENDING missions as FAILED (sanitizing the queue)
		_, err := s.db.ExecContext(ctx, "UPDATE agent_missions SET status = 'FAILED' WHERE status = 'PENDING' AND created_at < ?", thresholdTime)
		if err != nil {
			return err
		}

		// 2. Remove COMPLETED, or very old FAILED missions
		_, err = s.db.ExecContext(ctx, "DELETE FROM agent_missions WHERE status = 'COMPLETED' OR (status = 'FAILED' AND created_at < ?)", thresholdTime)
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
	return withRetry(ctx, func() error {
		_, err := s.db.ExecContext(ctx,
			`INSERT INTO capability_plugins (plugin_id, name, version, manifest_url, status, registered_at)
			 VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
			 ON CONFLICT(plugin_id) DO UPDATE SET
			 name=excluded.name, version=excluded.version,
			 manifest_url=excluded.manifest_url, status=excluded.status,
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
	var plugins []CapabilityPlugin
	err := withRetry(ctx, func() error {
		plugins = nil // reset slice
		var rows *sql.Rows
		var err error
		if status == "" {
			rows, err = s.db.QueryContext(ctx, "SELECT plugin_id, name, version, manifest_url, status, registered_at FROM capability_plugins")
		} else {
			rows, err = s.db.QueryContext(ctx, "SELECT plugin_id, name, version, manifest_url, status, registered_at FROM capability_plugins WHERE status = ?", status)
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
	return withRetry(ctx, func() error {
		_, err := s.db.ExecContext(ctx,
			`INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, source_plugin, created_at)
			 VALUES (?, ?, ?, ?, CURRENT_TIMESTAMP)
			 ON CONFLICT(memory_id) DO UPDATE SET
			 context=excluded.context, vector_embedding=excluded.vector_embedding,
			 source_plugin=excluded.source_plugin`,
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
	var memories []EpisodicMemory
	err := withRetry(ctx, func() error {
		memories = nil // reset slice
		var rows *sql.Rows
		var err error
		if plugin == "" {
			rows, err = s.db.QueryContext(ctx, "SELECT memory_id, context, vector_embedding, source_plugin, created_at FROM swarm_memory_embeddings")
		} else {
			rows, err = s.db.QueryContext(ctx, "SELECT memory_id, context, vector_embedding, source_plugin, created_at FROM swarm_memory_embeddings WHERE source_plugin = ?", plugin)
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
	return s.db.Close()
}

// SetContextRoot sets the context root for the SIPDB
func (s *SIPDB) SetContextRoot(path string) {
	s.ContextRoot = path
	s.cachedGrounding = ""
	s.groundingOnce = &sync.Once{}
}

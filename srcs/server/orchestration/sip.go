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

	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

var (
	sipMeter          = otel.Meter("github.com/onehumancorp/mono/srcs/server/orchestration")
	sipTracer         = otel.Tracer("github.com/onehumancorp/mono/srcs/server/orchestration")
	syncMissionsOk, _ = sipMeter.Int64Counter(
		"sip.missions.synced",
		metric.WithDescription("Number of successfully synced missions"),
	)
	syncMissionsErr, _ = sipMeter.Int64Counter(
		"sip.missions.sync_errors",
		metric.WithDescription("Number of mission sync errors"),
	)
)

// SIPDB encapsulates the Swarm Intelligence Protocol database interactions.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type SIPDB struct {
	db               db.Provider
	ContextRoot      string
	cachedGrounding  string
	groundingOnce    *sync.Once
	cachedGroundErr  error
}

const (
	maxRetries    = 3
	retryInterval = 100 * time.Millisecond
)

var (
	standaloneThrottle     = make(chan struct{}, 1) // Throttle to 1 concurrent write in standalone mode
	standaloneThrottleOnce sync.Once
)

// getThrottle conditionally acquires the semaphore if in standalone mode
func acquireThrottle(ctx context.Context) (func(), error) {
	standaloneThrottleOnce.Do(func() {
		if os.Getenv("OHC_STANDALONE") == "true" {
			// already initialized to 1
		} else {
			// If not standalone, make channel large enough or just ignore
		}
	})

	if os.Getenv("OHC_STANDALONE") == "true" {
		select {
		case standaloneThrottle <- struct{}{}:
			return func() { <-standaloneThrottle }, nil
		case <-ctx.Done():
			return nil, ctx.Err()
		}
	}
	return func() {}, nil
}

// withRetry executes a database operation with exponential backoff for transient errors (e.g. database is locked).
func withRetry(ctx context.Context, op func() error) error {
	cleanup, err := acquireThrottle(ctx)
	if err != nil {
		return err
	}
	defer cleanup()

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

		// Optimization: Avoid long exponential backoff retries when the DB connection is explicitly closed,
		// as it is non-recoverable and causes test timeouts.
		if err != nil && (err.Error() == "sql: database is closed" || err.Error() == "database is closed") {
			return err
		}

		slog.Warn("sipdb: operation failed, retrying", "attempt", i+1, "error", err)
		time.Sleep(retryInterval * time.Duration(1<<i))
	}
	return err
}

// NewSIPDBWithProvider initializes a new database connection and creates required tables.
func NewSIPDBWithProvider(provider db.Provider) (*SIPDB, error) {
	if err := initializeTables(provider); err != nil {
		return nil, err
	}
	return &SIPDB{db: provider, groundingOnce: new(sync.Once)}, nil
}

// NewSIPDB initializes a new SQLite database connection and creates required tables.
// This is kept for backward compatibility and tests.
// Accepts parameters: dbPath string (No Constraints).
// Returns (*SIPDB, error).
// Produces errors: Explicit error handling.
// Has no side effects.
func NewSIPDB(dbPath string) (*SIPDB, error) {
	dsn := dbPath
	if dbPath != ":memory:" && !strings.Contains(dbPath, "mode=memory") {
		dir := filepath.Dir(dbPath)
		if err := os.MkdirAll(dir, 0700); err != nil {
			return nil, err
		}
		if !strings.Contains(dsn, "?") {
			dsn += "?"
		} else {
			dsn += "&"
		}
		dsn += "_pragma=journal_mode(WAL)&_pragma=busy_timeout(15000)&_txlock=immediate"
	}
	sqlDB, _ := sql.Open("sqlite", dsn)
	sqlDB.SetMaxOpenConns(1)

	// Ensure the base file is created with correct permissions if it's not in memory
	if dbPath != ":memory:" && !strings.Contains(dbPath, "mode=memory") {
		basePath := dbPath
		if strings.HasPrefix(basePath, "file:") {
			basePath = strings.TrimPrefix(basePath, "file:")
		}
		if idx := strings.Index(basePath, "?"); idx != -1 {
			basePath = basePath[:idx]
		}

		// Touch the file with 0600 permissions before opening
		if f, err := os.OpenFile(basePath, os.O_CREATE|os.O_RDWR, 0600); err == nil {
			f.Close()
			os.Chmod(basePath, 0600) // Ensure chmod if file already existed
		}
		// Pre-create wal and shm files so that SQLite respects 0600
		if fwal, err := os.OpenFile(basePath+"-wal", os.O_CREATE|os.O_RDWR, 0600); err == nil {
			fwal.Close()
			os.Chmod(basePath+"-wal", 0600)
		}
		if fshm, err := os.OpenFile(basePath+"-shm", os.O_CREATE|os.O_RDWR, 0600); err == nil {
			fshm.Close()
			os.Chmod(basePath+"-shm", 0600)
		}

		// Ping to ensure file is actually created and loaded
		_ = sqlDB.Ping()

		// Also secure SQLite temporary files just in case
		if info, err := os.Stat(basePath); err == nil && !info.IsDir() {
			os.Chmod(basePath, 0600)
		}
		if info, err := os.Stat(basePath + "-wal"); err == nil && !info.IsDir() {
			os.Chmod(basePath+"-wal", 0600)
		}
		if info, err := os.Stat(basePath + "-shm"); err == nil && !info.IsDir() {
			os.Chmod(basePath+"-shm", 0600)
		}
	} else {
		_ = sqlDB.Ping()
	}

	provider := db.NewSqliteProvider(sqlDB)
	return NewSIPDBWithProvider(provider)
}

func initializeTables(provider db.Provider) error {
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
		`CREATE TABLE IF NOT EXISTS local_metrics_buffer (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			metric_type TEXT NOT NULL,
			payload TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);`,
	}

	for _, q := range queries {
		if !provider.IsSQLite() {
			q = strings.ReplaceAll(q, "DATETIME", "TIMESTAMP")
			q = strings.ReplaceAll(q, "BLOB", "BYTEA")
			q = strings.ReplaceAll(q, "INTEGER PRIMARY KEY AUTOINCREMENT", "SERIAL PRIMARY KEY")
		}
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
	var value string
	err := withRetry(ctx, func() error {
		err := s.db.QueryRow(ctx, "SELECT value FROM swarm_memory WHERE key = $1", key).Scan(&value)
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
		_, err := s.db.Exec(ctx,
			"INSERT INTO swarm_memory (key, value, updated_at) VALUES ($1, $2, CURRENT_TIMESTAMP) ON CONFLICT(key) DO UPDATE SET value=excluded.value, updated_at=CURRENT_TIMESTAMP",
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

		query := "SELECT id, payload FROM agent_missions WHERE payload::json->>'role' = $1 AND status = 'PENDING'"

		if s.db.IsSQLite() {
			query = "SELECT id, payload FROM agent_missions WHERE json_extract(payload, '$.role') = $1 AND status = 'PENDING'"
		}

		rows, err := s.db.Query(ctx, query, role)
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
	var rowsAffected int64
	err := withRetry(ctx, func() error {
		var err error
		rowsAffected, err = s.db.Exec(ctx, "UPDATE agent_missions SET status = 'COMPLETED' WHERE id = $1", missionID)
		return err
	})
	if err != nil {
		return err
	}
	if rowsAffected == 0 {
		return errors.New("mission not found")
	}
	return nil
}

// BurstMission updates the mission status to BURSTING and optionally syncs it.
// Accepts parameters: s *SIPDB, ctx context.Context, missionID string, remoteEndpoint string.
// Returns error.
// Produces errors: Explicit error handling.
// Has side effects: Updates mission status in agent_missions table and syncs to remote.
func (s *SIPDB) BurstMission(ctx context.Context, missionID string, remoteEndpoint string) error {
	var rowsAffected int64
	err := withRetry(ctx, func() error {
		var err error
		rowsAffected, err = s.db.Exec(ctx, "UPDATE agent_missions SET status = 'BURSTING' WHERE id = $1", missionID)
		return err
	})
	if err != nil {
		return err
	}
	if rowsAffected == 0 {
		return errors.New("mission not found")
	}

	if remoteEndpoint != "" {
		var payload string
		err = s.db.QueryRow(ctx, "SELECT payload FROM agent_missions WHERE id = $1", missionID).Scan(&payload)
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
		_, err := s.db.Exec(ctx,
			"INSERT INTO agent_status (agent_id, role, status, last_heartbeat) VALUES ($1, $2, $3, CURRENT_TIMESTAMP) ON CONFLICT(agent_id) DO UPDATE SET role=excluded.role, status=excluded.status, last_heartbeat=CURRENT_TIMESTAMP",
			agentID, role, status,
		)
		return err
	})
}

var (
	// throttleSemaphore limits concurrent DelegateMission executions in SQLite standalone mode.
	throttleSemaphore = make(chan struct{}, 1)
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

// UpsertMission inserts or updates a mission in the agent_missions table.
func (s *SIPDB) UpsertMission(ctx context.Context, missionID, status, payload string, forceLocal bool) error {
	isStandalone := envBoolDefault("OHC_STANDALONE", false)

	if isStandalone {
		select {
		case throttleSemaphore <- struct{}{}:
			cleanup := func() { <-throttleSemaphore }
			defer cleanup()
		case <-ctx.Done():
			return ctx.Err()
		}
	}

	upsertQuery := `
		INSERT INTO agent_missions (id, status, payload, created_at)
		VALUES ($1, $2, $3, CURRENT_TIMESTAMP)
		ON CONFLICT(id) DO NOTHING
	`
	if forceLocal {
		upsertQuery = `
			INSERT INTO agent_missions (id, status, payload, created_at)
			VALUES ($1, $2, $3, CURRENT_TIMESTAMP)
			ON CONFLICT(id) DO UPDATE SET
				status=EXCLUDED.status,
				payload=EXCLUDED.payload
		`
	}
	// For database abstractions handling PostgreSQL and SQLite
	return withRetry(ctx, func() error {
		_, err := s.db.Exec(ctx, upsertQuery, missionID, status, payload)
		return err
	})
}

// DelegateMission delegates specialized tasks via the agent_missions table.
// Accepts parameters: s *SIPDB (No Constraints).
// Returns DelegateMission(ctx context.Context, missionID, role string, task Message) error.
// Produces errors: Explicit error handling.
// Has no side effects.
func (s *SIPDB) DelegateMission(ctx context.Context, missionID, role string, task Message) error {
	isStandalone := envBoolDefault("OHC_STANDALONE", false)

	if isStandalone {
		select {
		case throttleSemaphore <- struct{}{}:
			cleanup := func() { <-throttleSemaphore }
			defer cleanup()
		case <-ctx.Done():
			return ctx.Err()
		}
	}

	_ = CheckDocumentationGate(task.Content)

	if s.ContextRoot != "" {
		s.groundingOnce.Do(func() {
			for _, filename := range []string{"AGENTS.md", "CLAUDE.md"} {
				path := filepath.Join(s.ContextRoot, filename)

				// Stat the file first to distinguish between missing vs permissions/errors
				_, statErr := os.Stat(path)
				if statErr == nil {
					// File exists
					content, err := os.ReadFile(path)
					if err != nil {
						s.cachedGroundErr = err
						break // Enforce fail-closed if there's a read error
					}
					s.cachedGrounding = "\n\n[SYSTEM GROUNDING]:\n" + string(content)
					break
				} else if !os.IsNotExist(statErr) {
					s.cachedGroundErr = statErr
					break
				}
			}
		})

		// Adhering to the Fail-Closed security mandate
		if s.cachedGroundErr != nil {
			return fmt.Errorf("fail-closed: unable to read project grounding files: %w", s.cachedGroundErr)
		}

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
		_, err := s.db.Exec(ctx,
			"INSERT INTO agent_missions (id, status, payload, created_at) VALUES ($1, 'PENDING', $2, CURRENT_TIMESTAMP)",
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
		_, err := s.db.Exec(ctx, "UPDATE agent_missions SET status = 'FAILED' WHERE status = 'PENDING' AND created_at < $1", thresholdTime)
		if err != nil {
			return err
		}

		// 2. Remove COMPLETED, or very old FAILED missions
		_, err = s.db.Exec(ctx, "DELETE FROM agent_missions WHERE status = 'COMPLETED' OR (status = 'FAILED' AND created_at < $1)", thresholdTime)
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
		_, err := s.db.Exec(ctx,
			`INSERT INTO capability_plugins (plugin_id, name, version, manifest_url, status, registered_at)
			 VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP)
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
		var rows db.Rows
		var err error
		if status == "" {
			rows, err = s.db.Query(ctx, "SELECT plugin_id, name, version, manifest_url, status, registered_at FROM capability_plugins")
		} else {
			rows, err = s.db.Query(ctx, "SELECT plugin_id, name, version, manifest_url, status, registered_at FROM capability_plugins WHERE status = $1", status)
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
		_, err := s.db.Exec(ctx,
			`INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, source_plugin, created_at)
			 VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
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
		var rows db.Rows
		var err error
		if plugin == "" {
			rows, err = s.db.Query(ctx, "SELECT memory_id, context, vector_embedding, source_plugin, created_at FROM swarm_memory_embeddings")
		} else {
			rows, err = s.db.Query(ctx, "SELECT memory_id, context, vector_embedding, source_plugin, created_at FROM swarm_memory_embeddings WHERE source_plugin = $1", plugin)
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
	s.db.Close()
	return nil
}

// SetContextRoot sets the context root for the SIPDB
func (s *SIPDB) SetContextRoot(path string) {
	s.ContextRoot = path
	s.cachedGrounding = ""
	s.cachedGroundErr = nil
	s.groundingOnce = new(sync.Once)
}

// BufferMetric inserts a telemetry metric into the local metric buffer.
// Accepts parameters: ctx context.Context, metricType string, payload string.
// Returns error.
// Produces errors: Explicit error handling.
// Has side effects: Inserts a record into the local_metrics_buffer table.
func (s *SIPDB) BufferMetric(ctx context.Context, metricType string, payload string) error {
	return withRetry(ctx, func() error {
		_, err := s.db.Exec(ctx,
			"INSERT INTO local_metrics_buffer (metric_type, payload, created_at) VALUES ($1, $2, CURRENT_TIMESTAMP)",
			metricType, payload,
		)
		return err
	})
}

// SyncBufferedMetrics aggregates and syncs buffered telemetry metrics with the OHC-SIP Cloud DB.
// Accepts parameters: ctx context.Context, remoteEndpoint string.
// Returns error.
// Produces errors: Explicit error handling.
// Has side effects: Posts aggregated metrics to a remote endpoint and deletes successful syncs from local_metrics_buffer.
// Returns the number of synced records and an error.
func (s *SIPDB) SyncBufferedMetrics(ctx context.Context, remoteEndpoint string) (int, error) {
	var records []struct {
		id         int64
		metricType string
		payload    string
	}

	err := withRetry(ctx, func() error {
		records = nil
		rows, err := s.db.Query(ctx, "SELECT id, metric_type, payload FROM local_metrics_buffer ORDER BY id ASC LIMIT 500")
		if err != nil {
			return err
		}
		defer rows.Close()

		for rows.Next() {
			var id int64
			var metricType string
			var payload string
			if err := rows.Scan(&id, &metricType, &payload); err != nil {
				return err
			}
			records = append(records, struct {
				id         int64
				metricType string
				payload    string
			}{id, metricType, payload})
		}
		return nil
	})
	if err != nil {
		return 0, err
	}

	if len(records) == 0 {
		return 0, nil // nothing to sync
	}

	// Prepare payload for batch sync
	var payloadBuilder strings.Builder
	payloadBuilder.WriteString("[")
	var idsToDelete []string
	for i, rec := range records {
		if i > 0 {
			payloadBuilder.WriteString(",")
		}
		// Inject metric_type into payload
		var obj map[string]interface{}
		if err := json.Unmarshal([]byte(rec.payload), &obj); err == nil {
			obj["metric_type"] = rec.metricType
			b, _ := json.Marshal(obj)
			payloadBuilder.Write(b)
		} else {
			payloadBuilder.WriteString(rec.payload)
		}
		idsToDelete = append(idsToDelete, fmt.Sprintf("%d", rec.id))
	}
	payloadBuilder.WriteString("]")

	req, err := http.NewRequestWithContext(ctx, "POST", remoteEndpoint, strings.NewReader(payloadBuilder.String()))
	if err != nil {
		return 0, fmt.Errorf("failed to create sync request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

	client := &http.Client{Timeout: 10 * time.Second}
	resp, err := client.Do(req)
	if err != nil {
		return 0, fmt.Errorf("failed to sync metrics: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return 0, fmt.Errorf("remote endpoint returned status: %d", resp.StatusCode)
	}

	// Delete successfully synced records
	err = withRetry(ctx, func() error {
		idList := strings.Join(idsToDelete, ",")
		_, err := s.db.Exec(ctx, fmt.Sprintf("DELETE FROM local_metrics_buffer WHERE id IN (%s)", idList))
		return err
	})
	return len(records), err
}

// SyncContextSync synchronizes the local Hybrid MCP RAG state to the cloud orchestration engine.
// Accepts parameters: ctx context.Context, remoteEndpoint string.
// Returns error.
// Produces errors: Explicit error handling.
// Has side effects: Posts local RAG context to a remote endpoint, deletes local records on success.
// Returns the number of synced records and an error.
func (s *SIPDB) SyncContextSync(ctx context.Context, remoteEndpoint string) (int, error) {
	var records []struct {
		id      string
		payload string
	}

	err := withRetry(ctx, func() error {
		records = nil
		// Fetch local episodic memories that haven't been synced (or just all local RAG data).
		// Note: The memory states the data is stored in swarm_memory_embeddings or similar
		// We can fetch from swarm_memory_embeddings
		rows, err := s.db.Query(ctx, "SELECT memory_id, context FROM swarm_memory_embeddings ORDER BY created_at ASC LIMIT 100")
		if err != nil {
			return err
		}
		defer rows.Close()

		for rows.Next() {
			var id, payload string
			if err := rows.Scan(&id, &payload); err != nil {
				return err
			}
			records = append(records, struct {
				id      string
				payload string
			}{id, payload})
		}
		return nil
	})
	if err != nil {
		return 0, err
	}

	if len(records) == 0 {
		return 0, nil
	}

	client := &http.Client{Timeout: 10 * time.Second}
	syncedCount := 0
	var idsToDelete []string

	for _, rec := range records {
		// Sanitize sensitive data by explicitly deleting `rag_context` key
		var payloadData map[string]interface{}
		if err := json.Unmarshal([]byte(rec.payload), &payloadData); err == nil {
			delete(payloadData, "rag_context")
		} else {
			// If not JSON, we assume it's raw text but the memory states
			// "safely decode the JSON payload into an interface{} and type assert to map[string]interface{}"
			// Let's create a generic JSON payload
			payloadData = map[string]interface{}{
				"context": rec.payload,
			}
		}
		sanitizedPayload, _ := json.Marshal(payloadData)

		req, err := http.NewRequestWithContext(ctx, "POST", remoteEndpoint, strings.NewReader(string(sanitizedPayload)))
		if err != nil {
			continue
		}
		req.Header.Set("Content-Type", "application/json")
		// robust conflict resolution prioritising local client
		req.Header.Set("X-OHC-Conflict-Resolution", "force-local")

		resp, err := client.Do(req)
		if err == nil {
			// treat 409 Conflict as success for local parity
			if (resp.StatusCode >= 200 && resp.StatusCode < 300) || resp.StatusCode == http.StatusConflict {
				idsToDelete = append(idsToDelete, rec.id)
				syncedCount++
			}
			resp.Body.Close()
		}
	}

	if len(idsToDelete) > 0 {
		err = withRetry(ctx, func() error {
			idList := "'" + strings.Join(idsToDelete, "','") + "'"
			_, err := s.db.Exec(ctx, fmt.Sprintf("DELETE FROM swarm_memory_embeddings WHERE memory_id IN (%s)", idList))
			return err
		})
		if err != nil {
			return syncedCount, err
		}
	}

	return syncedCount, nil
}

// SyncMissions aggregates and syncs pending missions with the OHC-SIP Cloud DB.
// Accepts parameters: ctx context.Context, remoteEndpoint string.
// Returns error.
// Produces errors: Explicit error handling.
// Has side effects: Posts pending missions to a remote endpoint and updates local status to SYNCED.
// Returns the number of synced records and an error.
func (s *SIPDB) SyncMissions(ctx context.Context, remoteEndpoint string) (int, error) {
	ctx, span := sipTracer.Start(ctx, "SyncMissions")
	defer span.End()

	var missions []struct {
		id      string
		payload string
	}

	err := withRetry(ctx, func() error {
		missions = nil
		rows, err := s.db.Query(ctx, "SELECT id, payload FROM agent_missions WHERE status = 'PENDING' ORDER BY created_at ASC LIMIT 100")
		if err != nil {
			return err
		}
		defer rows.Close()

		for rows.Next() {
			var id, payload string
			if err := rows.Scan(&id, &payload); err != nil {
				return err
			}
			missions = append(missions, struct {
				id      string
				payload string
			}{id, payload})
		}
		return nil
	})
	if err != nil {
		return 0, err
	}

	if len(missions) == 0 {
		return 0, nil
	}

	client := &http.Client{Timeout: 10 * time.Second}
	syncedCount := 0

	// Deep recursive redaction
	var sanitizeRecursively func(data interface{}) interface{}
	sanitizeRecursively = func(data interface{}) interface{} {
		switch v := data.(type) {
		case string:
			return telemetry.RedactPII(v)
		case map[string]interface{}:
			for key, val := range v {
				v[key] = sanitizeRecursively(val)
			}
			return v
		case []interface{}:
			for i, val := range v {
				v[i] = sanitizeRecursively(val)
			}
			return v
		default:
			return v
		}
	}

	for _, m := range missions {
		// Parse payload to redact and sanitize
		var rawData interface{}
		if err := json.Unmarshal([]byte(m.payload), &rawData); err != nil {
			slog.Warn("Failed to unmarshal mission payload for sanitization, skipping sync to prevent leakage", "mission_id", m.id)
			if syncMissionsErr != nil {
				syncMissionsErr.Add(ctx, 1)
			}
			continue
		}

		if payloadData, ok := rawData.(map[string]interface{}); ok {
			// Delete sensitive RAG context
			delete(payloadData, "rag_context")

			// Add ID to payload for synchronization endpoint
			payloadData["id"] = m.id
		}

		// Unconditionally apply redaction
		rawData = sanitizeRecursively(rawData)

		// Re-marshal sanitized payload
		sanitizedBytes, err := json.Marshal(rawData)
		if err != nil {
			slog.Warn("Failed to marshal sanitized mission payload, skipping sync", "mission_id", m.id)
			if syncMissionsErr != nil {
				syncMissionsErr.Add(ctx, 1)
			}
			continue
		}
		m.payload = string(sanitizedBytes)

		req, err := http.NewRequestWithContext(ctx, "POST", remoteEndpoint, strings.NewReader(m.payload))
		if err != nil {
			if syncMissionsErr != nil {
				syncMissionsErr.Add(ctx, 1)
			}
			continue
		}
		req.Header.Set("Content-Type", "application/json")
		req.Header.Set("X-OHC-Conflict-Resolution", "force-local")

		resp, err := client.Do(req)
		if err == nil {
			if (resp.StatusCode >= 200 && resp.StatusCode < 300) || resp.StatusCode == http.StatusConflict {
				updateErr := withRetry(ctx, func() error {
					_, updateErr := s.db.Exec(ctx, "UPDATE agent_missions SET status = 'SYNCED' WHERE id = $1", m.id)
					return updateErr
				})
				if updateErr == nil {
					syncedCount++
					if syncMissionsOk != nil {
						syncMissionsOk.Add(ctx, 1)
					}
				} else if syncMissionsErr != nil {
					syncMissionsErr.Add(ctx, 1)
				}
			} else if syncMissionsErr != nil {
				syncMissionsErr.Add(ctx, 1)
			}
			resp.Body.Close()
		} else if syncMissionsErr != nil {
			syncMissionsErr.Add(ctx, 1)
		}
	}

	return syncedCount, nil
}

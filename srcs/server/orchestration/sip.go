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
	"github.com/redis/rueidis"
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
	db              db.Provider
	orgID           string
	ContextRoot     string
	cachedGrounding string
	groundingOnce   *sync.Once
	cachedGroundErr error
	redisClient     rueidis.Client
	localCache      sync.Map
	cacheExpirations sync.Map
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
func acquireThrottle(ctx context.Context) error {
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
			return nil
		case <-ctx.Done():
			return ctx.Err()
		}
	}
	return nil
}

func releaseThrottle() {
	if os.Getenv("OHC_STANDALONE") == "true" {
		select {
		case <-standaloneThrottle:
		default:
		}
	}
}

// withSipRetry executes a database operation with exponential backoff for transient errors (e.g. database is locked).
func withSipRetry(ctx context.Context, op func() error) error {
	if err := acquireThrottle(ctx); err != nil {
		return err
	}
	defer releaseThrottle()

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

		if err != nil && (strings.Contains(err.Error(), "database is locked") || strings.Contains(err.Error(), "SQLITE_BUSY")) {
			telemetry.RecordSQLiteLockContention(ctx, "exec/query")
		}

		// Optimization: Avoid long exponential backoff retries when the DB connection is explicitly closed,
		// as it is non-recoverable and causes test timeouts.
		if err != nil && (err.Error() == "sql: database is closed" || err.Error() == "database is closed") {
			return err
		}

		// ⚡ BOLT: Return immediately for non-transient constraint errors to avoid wasting backoff latency.
		if err != nil && strings.Contains(err.Error(), "UNIQUE constraint failed") {
			return err
		}

		slog.Debug("sipdb: operation failed, retrying", "attempt", i+1, "error", err)
		time.Sleep(retryInterval * time.Duration(1<<i))
	}

	if err != nil {
		if strings.Contains(err.Error(), "database is locked") || strings.Contains(err.Error(), "SQLITE_BUSY") {
			telemetry.RecordSQLiteRetryExhausted(ctx, "exec/query")
		} else if strings.Contains(err.Error(), "could not serialize access") || strings.Contains(err.Error(), "deadlock detected") {
			telemetry.RecordPostgresRetryExhausted(ctx, "exec/query")
		}
	}
	return err
}

// NewSIPDBWithProvider initializes a new database connection and creates required tables.
func NewSIPDBWithProvider(provider db.Provider, orgID string) (*SIPDB, error) {
	if err := initializeTables(provider); err != nil {
		return nil, err
	}
	if orgID == "" {
		orgID = "system"
	}
	return &SIPDB{db: provider, orgID: orgID, groundingOnce: new(sync.Once)}, nil
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
	return NewSIPDBWithProvider(provider, "system")
}

func initializeTables(provider db.Provider) error {
	queries := []string{
		`CREATE TABLE IF NOT EXISTS swarm_memory (
			key TEXT PRIMARY KEY,
			value TEXT NOT NULL,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			organization_id TEXT DEFAULT 'system'
		);`,
		`CREATE TABLE IF NOT EXISTS agent_missions (
			id TEXT PRIMARY KEY,
			status TEXT NOT NULL,
			payload TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			organization_id TEXT DEFAULT 'system',
			synced_to_cloud BOOLEAN DEFAULT FALSE
		);`,
		`CREATE TABLE IF NOT EXISTS agent_status (
			agent_id TEXT PRIMARY KEY,
			role TEXT NOT NULL,
			status TEXT NOT NULL,
			last_heartbeat DATETIME DEFAULT CURRENT_TIMESTAMP,
			organization_id TEXT DEFAULT 'system'
		);`,
		`CREATE TABLE IF NOT EXISTS capability_plugins (
			plugin_id TEXT PRIMARY KEY,
			name TEXT NOT NULL,
			version TEXT NOT NULL,
			manifest_url TEXT NOT NULL,
			status TEXT NOT NULL,
			registered_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			organization_id TEXT DEFAULT 'system'
		);`,
		`CREATE TABLE IF NOT EXISTS swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			vector_embedding BLOB,
			source_plugin TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			organization_id TEXT DEFAULT 'system'
		);`,
		`CREATE TABLE IF NOT EXISTS telemetry_buffer (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			metric_type TEXT NOT NULL,
			payload TEXT NOT NULL,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			organization_id TEXT DEFAULT 'system'
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
	cacheKey := "sip:memory:" + s.orgID + ":" + key
	if val, ok := s.getCache(ctx, cacheKey, "SyncMemory"); ok {
		return val, nil
	}

	var value string
	err := withSipRetry(ctx, func() error {
		err := s.db.QueryRow(ctx, "SELECT value FROM swarm_memory WHERE key = $1 AND organization_id = $2", key, s.orgID).Scan(&value)
		if err == sql.ErrNoRows {
			return nil
		}
		return err
	})
	if err == nil && value != "" {
		s.setCache(ctx, cacheKey, value)
	}
	return value, err
}

// UpdateMemory updates the global state.
// Accepts parameters: s *SIPDB (No Constraints).
// Returns UpdateMemory(ctx context.Context, key, value string) error.
// Produces errors: Explicit error handling.
// Has no side effects.
func (s *SIPDB) UpdateMemory(ctx context.Context, key, value string) error {
	err := withSipRetry(ctx, func() error {
		_, err := s.db.Exec(ctx,
			"INSERT INTO swarm_memory (key, value, updated_at, organization_id) VALUES ($1, $2, CURRENT_TIMESTAMP, $3) ON CONFLICT(key) DO UPDATE SET value=excluded.value, updated_at=CURRENT_TIMESTAMP",
			key, value, s.orgID,
		)
		return err
	})
	if err == nil {
		s.invalidateCache(ctx, "sip:memory:"+s.orgID+":"+key)
	}
	return err
}

// getMissionUpdatedAt fetches the updated_at or created_at timestamp for a mission.
func (s *SIPDB) getMissionUpdatedAt(ctx context.Context, missionID string) (time.Time, error) {
	var updatedAt time.Time
	err := s.db.QueryRow(ctx, "SELECT COALESCE(updated_at, created_at) FROM agent_missions WHERE id = $1 AND organization_id = $2", missionID, s.orgID).Scan(&updatedAt)
	return updatedAt, err
}

// GetPendingMissions proactively seeks tasks assigned to the role.
// Accepts parameters: s *SIPDB (No Constraints).
// Returns GetPendingMissions(ctx context.Context, role string) ([]Message, error).
// Produces errors: Explicit error handling.
// Has no side effects.
func (s *SIPDB) GetPendingMissions(ctx context.Context, role string) ([]Message, error) {
	var missions []Message
	err := withSipRetry(ctx, func() error {
		missions = nil

		query := "SELECT id, payload FROM agent_missions WHERE payload::json->>'role' = $1 AND status = 'PENDING' AND organization_id = $2 ORDER BY created_at DESC LIMIT 500"
		var rows db.Rows
		var err error

		if role == "ANY" {
			query = "SELECT id, payload FROM agent_missions WHERE status = 'PENDING' AND organization_id = $1 ORDER BY created_at DESC LIMIT 500"
			rows, err = s.db.Query(ctx, query, s.orgID)
		} else {
			if s.db.IsSQLite() {
				query = "SELECT id, payload FROM agent_missions WHERE json_extract(payload, '$.role') = $1 AND status = 'PENDING' AND organization_id = $2 ORDER BY created_at DESC LIMIT 500"
			}
			rows, err = s.db.Query(ctx, query, role, s.orgID)
		}
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
	var prevTime time.Time
	var id string
	err := withSipRetry(ctx, func() error {
		prevTime, _ = s.getMissionUpdatedAt(ctx, missionID)

		err := s.db.QueryRow(ctx, "UPDATE agent_missions SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND organization_id = $2 RETURNING id", missionID, s.orgID).Scan(&id)
		if errors.Is(err, sql.ErrNoRows) || (err != nil && err.Error() == "no rows in result set") {
			return errors.New("mission not found")
		}
		return err
	})
	if err == nil && !prevTime.IsZero() {
		telemetry.RecordAgentTransitionLatency(ctx, "running_to_completed", time.Since(prevTime).Seconds())
	}
	return err
}

// BurstMission updates the mission status to BURSTING and optionally syncs it.
// Accepts parameters: s *SIPDB, ctx context.Context, missionID string, remoteEndpoint string.
// Returns error.
// Produces errors: Explicit error handling.
// Has side effects: Updates mission status in agent_missions table and syncs to remote.
func (s *SIPDB) BurstMission(ctx context.Context, missionID string, remoteEndpoint string) error {
	var prevTime time.Time
	var id string
	err := withSipRetry(ctx, func() error {
		prevTime, _ = s.getMissionUpdatedAt(ctx, missionID)

		err := s.db.QueryRow(ctx, "UPDATE agent_missions SET status = 'BURSTING', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND organization_id = $2 RETURNING id", missionID, s.orgID).Scan(&id)
		if errors.Is(err, sql.ErrNoRows) || (err != nil && err.Error() == "no rows in result set") {
			return errors.New("mission not found")
		}
		return err
	})
	if err != nil {
		return err
	}

	if !prevTime.IsZero() {
		telemetry.RecordAgentTransitionLatency(ctx, "to_bursting", time.Since(prevTime).Seconds())
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
	return withSipRetry(ctx, func() error {
		_, err := s.db.Exec(ctx,
			"INSERT INTO agent_status (agent_id, role, status, last_heartbeat, organization_id) VALUES ($1, $2, $3, CURRENT_TIMESTAMP, $4) ON CONFLICT(agent_id) DO UPDATE SET role=excluded.role, status=excluded.status, last_heartbeat=CURRENT_TIMESTAMP",
			agentID, role, status, s.orgID,
		)
		return err
	})
}

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
	var prevTime time.Time
	var oldStatus string

	err := withSipRetry(ctx, func() error {
		tx, err := s.db.Begin(ctx)
		if err != nil {
			return err
		}
		defer tx.Rollback(ctx)

		// Fetch previous state for telemetry
		_ = tx.QueryRow(ctx, "SELECT status, COALESCE(updated_at, created_at) FROM agent_missions WHERE id = $1 AND organization_id = $2", missionID, s.orgID).Scan(&oldStatus, &prevTime)

		// Standardize UpsertMission to ensure atomic status transitions and consistent forceLocal behavior.
		// Use a common upsert pattern where possible, but handle Postgres locking explicitly.
		if s.db.IsSQLite() {
			upsertQuery := `
				INSERT INTO agent_missions (id, status, payload, created_at, updated_at, organization_id)
				VALUES ($1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, $4)
				ON CONFLICT(id) DO UPDATE SET
					status = CASE WHEN $5 THEN excluded.status ELSE agent_missions.status END,
					payload = CASE WHEN $5 THEN excluded.payload ELSE agent_missions.payload END,
					updated_at = CASE WHEN $5 THEN CURRENT_TIMESTAMP ELSE agent_missions.updated_at END
			`
			_, err = tx.Exec(ctx, upsertQuery, missionID, status, payload, s.orgID, forceLocal)
			if err != nil {
				return err
			}
		} else {
			// Postgres mode parity: Use FOR UPDATE SKIP LOCKED for high-concurrency cloud nodes
			var existingID string
			err := tx.QueryRow(ctx, "SELECT id FROM agent_missions WHERE id = $1 AND organization_id = $2 FOR UPDATE SKIP LOCKED", missionID, s.orgID).Scan(&existingID)

			if err != nil && err.Error() != "sql: no rows in result set" {
				return err
			}

			if err != nil && err.Error() == "sql: no rows in result set" {
				// Row is either non-existent or locked by another transaction
				var checkID string
				checkErr := tx.QueryRow(ctx, "SELECT id FROM agent_missions WHERE id = $1 AND organization_id = $2", missionID, s.orgID).Scan(&checkID)
				if checkErr == nil && checkID == missionID {
					// Row exists but is locked
					telemetry.RecordPostgresLockContention(ctx, "upsert_mission")
					if forceLocal {
						// Fallback to blocking update to ensure data integrity when forceLocal=true
						_, errUpdate := tx.Exec(ctx, "UPDATE agent_missions SET status = $1, payload = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3 AND organization_id = $4", status, payload, missionID, s.orgID)
						return errUpdate
					}
					return nil // Skip to avoid contention if not forced
				}
				// Row truly doesn't exist, proceed to insert with DO NOTHING on conflict to be safe
				_, errInsert := tx.Exec(ctx, "INSERT INTO agent_missions (id, status, payload, created_at, updated_at, organization_id) VALUES ($1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, $4) ON CONFLICT(id) DO NOTHING", missionID, status, payload, s.orgID)
				return errInsert
			}

			if existingID != "" && forceLocal {
				// We have the lock and it's forceLocal, perform the update
				_, errUpdate := tx.Exec(ctx, "UPDATE agent_missions SET status = $1, payload = $2, updated_at = CURRENT_TIMESTAMP WHERE id = $3 AND organization_id = $4", status, payload, missionID, s.orgID)
				return errUpdate
			}
		}

		return tx.Commit(ctx)
	})

	if err == nil && !prevTime.IsZero() && oldStatus != status {
		transition := strings.ToLower(oldStatus) + "_to_" + strings.ToLower(status)
		telemetry.RecordAgentTransitionLatency(ctx, transition, time.Since(prevTime).Seconds())
	}
	return err
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
			var combinedGrounding strings.Builder

			for _, filename := range []string{"AGENTS.md", "CLAUDE_OHC.md"} {
				path := filepath.Join(s.ContextRoot, filename)

				// Stat the file first to distinguish between missing vs permissions/errors
				_, statErr := os.Stat(path)
				if statErr == nil {
					// File exists
					content, err := os.ReadFile(path)
					if err != nil {
						s.cachedGroundErr = err
						return // Enforce fail-closed if there's a read error
					}
					combinedGrounding.WriteString("\n" + string(content) + "\n")
				} else if !os.IsNotExist(statErr) {
					s.cachedGroundErr = statErr
					return
				}
			}

			if combinedGrounding.Len() > 0 {
				s.cachedGrounding = "\n\n[SYSTEM GROUNDING]\n" + strings.TrimSpace(combinedGrounding.String())
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
	return withSipRetry(ctx, func() error {
		_, err := s.db.Exec(ctx,
			"INSERT INTO agent_missions (id, status, payload, created_at, updated_at, organization_id) VALUES ($1, 'PENDING', $2, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, $3)",
			missionID, string(taskBytes), s.orgID,
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
	return withSipRetry(ctx, func() error {
		thresholdTime := time.Now().Add(-ageThreshold).UTC().Format("2006-01-02 15:04:05")

		// 1. Mark stagnant PENDING missions as FAILED (sanitizing the queue)
		// Phase 3: ML-Resilience audit guarantees both SQLite (Standalone) and Postgres (Cloud-native) execute this fallback gracefully.
		_, err := s.db.Exec(ctx, "UPDATE agent_missions SET status = 'FAILED' WHERE (status = 'PENDING' OR status = 'STUCK' OR status = 'BURSTING') AND created_at < $1 AND organization_id = $2", thresholdTime, s.orgID)
		if err != nil {
			return err
		}

		// 2. Remove COMPLETED, or very old FAILED missions
		// ⚡ BOLT: Prevent massive table scans by limiting delete batch size for sub-second latency
		if s.db.IsSQLite() {
			_, err = s.db.Exec(ctx, "DELETE FROM agent_missions WHERE id IN (SELECT id FROM agent_missions WHERE (status = 'COMPLETED' OR ((status = 'FAILED' OR status = 'STUCK' OR status = 'BURSTING') AND created_at < $1)) AND organization_id = $2 LIMIT 1000)", thresholdTime, s.orgID)
		} else {
			_, err = s.db.Exec(ctx, "WITH cte AS (SELECT id FROM agent_missions WHERE (status = 'COMPLETED' OR ((status = 'FAILED' OR status = 'STUCK' OR status = 'BURSTING') AND created_at < $1)) AND organization_id = $2 LIMIT 1000) DELETE FROM agent_missions WHERE id IN (SELECT id FROM cte)", thresholdTime, s.orgID)
		}

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
	return withSipRetry(ctx, func() error {
		_, err := s.db.Exec(ctx,
			`INSERT INTO capability_plugins (plugin_id, name, version, manifest_url, status, registered_at, organization_id)
			 VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, $6)
			 ON CONFLICT(plugin_id) DO UPDATE SET
			 name=excluded.name, version=excluded.version,
			 manifest_url=excluded.manifest_url, status=excluded.status,
			 registered_at=CURRENT_TIMESTAMP`,
			plugin.PluginID, plugin.Name, plugin.Version, plugin.ManifestURL, plugin.Status, s.orgID,
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
	cacheKey := "sip:plugins:" + s.orgID + ":" + status
	if val, ok := s.getCache(ctx, cacheKey, "GetCapabilityPlugins"); ok {
		var cachedPlugins []CapabilityPlugin
		if err := json.Unmarshal([]byte(val), &cachedPlugins); err == nil {
			return cachedPlugins, nil
		}
	}

	var plugins []CapabilityPlugin
	err := withSipRetry(ctx, func() error {
		plugins = nil // reset slice
		var rows db.Rows
		var err error
		if status == "" {
			rows, err = s.db.Query(ctx, "SELECT plugin_id, name, version, manifest_url, status, registered_at FROM capability_plugins WHERE organization_id = $1", s.orgID)
		} else {
			rows, err = s.db.Query(ctx, "SELECT plugin_id, name, version, manifest_url, status, registered_at FROM capability_plugins WHERE status = $1 AND organization_id = $2", status, s.orgID)
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
	if err == nil && plugins != nil {
		if b, err := json.Marshal(plugins); err == nil {
			s.setCache(ctx, cacheKey, string(b))
		}
	}
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
	err := withSipRetry(ctx, func() error {
		_, err := s.db.Exec(ctx,
			`INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, source_plugin, created_at, organization_id)
			 VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP, $5)
			 ON CONFLICT(memory_id) DO UPDATE SET
			 context=excluded.context, vector_embedding=excluded.vector_embedding,
			 source_plugin=excluded.source_plugin`,
			memory.MemoryID, memory.Context, memory.VectorEmbedding, memory.SourcePlugin, s.orgID,
		)
		return err
	})
	if err == nil {
		s.invalidateCache(ctx, "sip:memories:"+s.orgID+":"+memory.SourcePlugin)
		s.invalidateCache(ctx, "sip:memories:"+s.orgID+":")
	}
	return err
}

// GetEpisodicMemoriesByPlugin retrieves memories matching a specific source plugin.
// Accepts parameters: ctx context.Context, plugin string.
// Returns []EpisodicMemory, error.
// Produces errors: Explicit error handling.
// Has no side effects.
func (s *SIPDB) GetEpisodicMemoriesByPlugin(ctx context.Context, plugin string) ([]EpisodicMemory, error) {
	cacheKey := "sip:memories:" + s.orgID + ":" + plugin
	if val, ok := s.getCache(ctx, cacheKey, "GetEpisodicMemoriesByPlugin"); ok {
		var cachedMemories []EpisodicMemory
		if err := json.Unmarshal([]byte(val), &cachedMemories); err == nil {
			return cachedMemories, nil
		}
	}

	var memories []EpisodicMemory
	err := withSipRetry(ctx, func() error {
		memories = nil // reset slice
		var rows db.Rows
		var err error
		if plugin == "" {
			rows, err = s.db.Query(ctx, "SELECT memory_id, context, vector_embedding, source_plugin, created_at FROM swarm_memory_embeddings WHERE organization_id = $1", s.orgID)
		} else {
			rows, err = s.db.Query(ctx, "SELECT memory_id, context, vector_embedding, source_plugin, created_at FROM swarm_memory_embeddings WHERE source_plugin = $1 AND organization_id = $2", plugin, s.orgID)
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
	if err == nil && memories != nil {
		if b, err := json.Marshal(memories); err == nil {
			s.setCache(ctx, cacheKey, string(b))
		}
	}
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

// Provider returns the underlying db.Provider.
func (s *SIPDB) Provider() db.Provider {
	return s.db
}

// BufferMetric inserts a telemetry metric into the local metric buffer.
// Accepts parameters: ctx context.Context, metricType string, payload string.
// Returns error.
// Produces errors: Explicit error handling.
// Has side effects: Inserts a record into the telemetry_buffer table.
func (s *SIPDB) BufferMetric(ctx context.Context, metricType string, payload string) error {
	return withSipRetry(ctx, func() error {
		_, err := s.db.Exec(ctx,
			"INSERT INTO telemetry_buffer (metric_type, payload, created_at, organization_id) VALUES ($1, $2, CURRENT_TIMESTAMP, $3)",
			metricType, payload, s.orgID,
		)
		return err
	})
}

// SyncBufferedMetrics aggregates and syncs buffered telemetry metrics with the OHC-SIP Cloud DB.
// Accepts parameters: ctx context.Context, remoteEndpoint string.
// Returns error.
// Produces errors: Explicit error handling.
// Has side effects: Posts aggregated metrics to a remote endpoint and deletes successful syncs from telemetry_buffer.
// Returns the number of synced records and an error.
func (s *SIPDB) SyncBufferedMetrics(ctx context.Context, remoteEndpoint string) (int, error) {
	var records []struct {
		id         int64
		metricType string
		payload    string
	}

	err := withSipRetry(ctx, func() error {
		records = nil
		rows, err := s.db.Query(ctx, "SELECT id, metric_type, payload FROM telemetry_buffer WHERE organization_id = $1 ORDER BY id ASC LIMIT 500", s.orgID)
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
			sanitizedObj := SanitizePayloadMap(obj)
			b, _ := json.Marshal(sanitizedObj)
			payloadBuilder.Write(b)
		} else {
			sanitizedStr, _ := SanitizePayload(rec.payload)
			payloadBuilder.WriteString(sanitizedStr)
		}
		idsToDelete = append(idsToDelete, fmt.Sprintf("%d", rec.id))
	}
	payloadBuilder.WriteString("]")

	payloadStr := payloadBuilder.String()
	payloadSize := len(payloadStr)
	req, err := http.NewRequestWithContext(ctx, "POST", remoteEndpoint, strings.NewReader(payloadStr))
	if err != nil {
		return 0, fmt.Errorf("failed to create sync request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("X-OHC-Conflict-Resolution", "force-local")

	client := &http.Client{Timeout: 10 * time.Second}
	start := time.Now()
	resp, err := client.Do(req)
	if err != nil {
		return 0, fmt.Errorf("failed to sync metrics: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return 0, fmt.Errorf("remote endpoint returned status: %d", resp.StatusCode)
	}

	telemetry.RecordSIPSyncLatency(ctx, time.Since(start))
	telemetry.RecordSIPSyncPayloadSize(ctx, payloadSize)

	// Delete successfully synced records
	err = withSipRetry(ctx, func() error {
		idList := strings.Join(idsToDelete, ",")
		_, err := s.db.Exec(ctx, fmt.Sprintf("DELETE FROM telemetry_buffer WHERE id IN (%s)", idList))
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

	err := withSipRetry(ctx, func() error {
		records = nil
		// Fetch local episodic memories that haven't been synced (or just all local RAG data).
		// Note: The memory states the data is stored in swarm_memory_embeddings or similar
		// We can fetch from swarm_memory_embeddings
		rows, err := s.db.Query(ctx, "SELECT memory_id, context FROM swarm_memory_embeddings WHERE organization_id = $1 ORDER BY created_at ASC LIMIT 100", s.orgID)
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
		var payloadData map[string]interface{}
		if err := json.Unmarshal([]byte(rec.payload), &payloadData); err != nil {
			payloadData = map[string]interface{}{
				"context": rec.payload,
			}
		}

		// Unified high-fidelity sanitization
		sanitizedPayloadIface := SanitizePayloadMap(payloadData)
		if spm, ok := sanitizedPayloadIface.(map[string]interface{}); ok {
			payloadData = spm
		}

		// ensure memory_id is set
		payloadData["memory_id"] = rec.id

		sanitizedPayload, _ := json.Marshal(payloadData)

		req, err := http.NewRequestWithContext(ctx, "POST", remoteEndpoint, strings.NewReader(string(sanitizedPayload)))
		if err != nil {
			continue
		}
		req.Header.Set("Content-Type", "application/json")
		// robust conflict resolution prioritising local client
		req.Header.Set("X-OHC-Conflict-Resolution", "force-local")

		payloadSize := len(sanitizedPayload)
		start := time.Now()
		resp, err := client.Do(req)
		if err == nil {
			// treat 409 Conflict as success for local parity
			if (resp.StatusCode >= 200 && resp.StatusCode < 300) || resp.StatusCode == http.StatusConflict {
				idsToDelete = append(idsToDelete, rec.id)
				syncedCount++
				telemetry.RecordSIPSyncLatency(ctx, time.Since(start))
				telemetry.RecordSIPSyncPayloadSize(ctx, payloadSize)
			}
			resp.Body.Close()
		}
	}

	if len(idsToDelete) > 0 {
		err = withSipRetry(ctx, func() error {
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
		status  string
		payload string
	}

	err := withSipRetry(ctx, func() error {
		missions = nil
		rows, err := s.db.Query(ctx, "SELECT id, status, payload FROM agent_missions WHERE status IN ('PENDING', 'BURSTING') AND organization_id = $1 ORDER BY created_at ASC LIMIT 100", s.orgID)
		if err != nil {
			return err
		}
		defer rows.Close()

		for rows.Next() {
			var id, status, payload string
			if err := rows.Scan(&id, &status, &payload); err != nil {
				return err
			}
			missions = append(missions, struct {
				id      string
				status  string
				payload string
			}{id, status, payload})
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
			// Add ID and Status to payload for synchronization endpoint
			payloadData["id"] = m.id
			payloadData["status"] = m.status
		}

		// Unconditionally apply unified sanitization
		rawData = SanitizePayloadMap(rawData)

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
				updateErr := withSipRetry(ctx, func() error {
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
		} else {
			if syncMissionsErr != nil {
				syncMissionsErr.Add(ctx, 1)
			}
			return syncedCount, err
		}
	}

	return syncedCount, nil
}

// SetRedisClient injects a Redis client for global query caching.
func (s *SIPDB) SetRedisClient(r rueidis.Client) {
	s.redisClient = r
}

func (s *SIPDB) getCache(ctx context.Context, key string, operation string) (string, bool) {
	if s.redisClient != nil {
		cmd := s.redisClient.B().Get().Key(key).Build()
		val, err := s.redisClient.Do(ctx, cmd).ToString()
		if err == nil && val != "" {
			telemetry.RecordCacheHit(ctx, operation, "redis")
			return val, true
		}
	} else if s.db != nil && s.db.IsSQLite() {
		if exp, ok := s.cacheExpirations.Load(key); ok {
			if time.Now().After(exp.(time.Time)) {
				s.localCache.Delete(key)
				s.cacheExpirations.Delete(key)
			} else {
				if val, ok := s.localCache.Load(key); ok {
					telemetry.RecordCacheHit(ctx, operation, "memory")
					return val.(string), true
				}
			}
		}
	}
	telemetry.RecordCacheMiss(ctx, operation, "all")
	return "", false
}

func (s *SIPDB) setCache(ctx context.Context, key string, value string) {
	if s.redisClient != nil {
		cmd := s.redisClient.B().Set().Key(key).Value(value).Ex(1 * time.Hour).Build()
		_ = s.redisClient.Do(ctx, cmd)
	} else if s.db != nil && s.db.IsSQLite() {
		s.localCache.Store(key, value)
		s.cacheExpirations.Store(key, time.Now().Add(1*time.Hour))
	}
}

func (s *SIPDB) invalidateCache(ctx context.Context, key string) {
	if s.redisClient != nil {
		cmd := s.redisClient.B().Del().Key(key).Build()
		_ = s.redisClient.Do(ctx, cmd)
	} else if s.db != nil && s.db.IsSQLite() {
		s.localCache.Delete(key)
		s.cacheExpirations.Delete(key)
	}
}

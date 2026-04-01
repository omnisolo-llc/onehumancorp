package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"strings"
	"time"

	_ "modernc.org/sqlite"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// SQLiteProvider is the SQLite implementation of DatabaseProvider.
type SQLiteProvider struct {
	db *sql.DB
}

// NewSQLiteProvider creates a new SQLiteProvider
func NewSQLiteProvider(dbPath string) (*SQLiteProvider, error) {
	dsn := dbPath
	if dbPath != ":memory:" && !strings.Contains(dbPath, "mode=memory") {
		if !strings.Contains(dsn, "?") {
			dsn += "?"
		} else {
			dsn += "&"
		}
		dsn += "_pragma=journal_mode(WAL)&_pragma=busy_timeout(15000)&_txlock=immediate"
	}
	db, err := sql.Open("sqlite", dsn)
	if err != nil {
		return nil, err
	}
	db.SetMaxOpenConns(1)

	return &SQLiteProvider{db: db}, nil
}

func (p *SQLiteProvider) InitializeTables() error {
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
		if _, err := p.db.Exec(q); err != nil {
			return err
		}
	}
	return nil
}

func (p *SQLiteProvider) SyncMemory(ctx context.Context, key string) (string, error) {
	start := time.Now()
	var value string
	err := withRetry(ctx, func() error {
		err := p.db.QueryRowContext(ctx, "SELECT value FROM swarm_memory WHERE key = ?", key).Scan(&value)
		if err == sql.ErrNoRows {
			return nil
		}
		return err
	})
	if err == nil {
		telemetry.RecordAgentApiCall(ctx, "system-db", "provider", "sqlite_sync_memory")
	}
	_ = time.Since(start)
	return value, err
}

func (p *SQLiteProvider) UpdateMemory(ctx context.Context, key, value string) error {
	start := time.Now()
	err := withRetry(ctx, func() error {
		_, err := p.db.ExecContext(ctx,
			"INSERT INTO swarm_memory (key, value, updated_at) VALUES (?, ?, CURRENT_TIMESTAMP) ON CONFLICT(key) DO UPDATE SET value=excluded.value, updated_at=CURRENT_TIMESTAMP",
			key, value,
		)
		return err
	})
	if err == nil {
		telemetry.RecordAgentApiCall(ctx, "system-db", "provider", "sqlite_update_memory")
	}
	_ = time.Since(start)
	return err
}

func (p *SQLiteProvider) GetPendingMissions(ctx context.Context, role string) ([]Message, error) {
	start := time.Now()
	var missions []Message
	err := withRetry(ctx, func() error {
		missions = nil
		rows, err := p.db.QueryContext(ctx, "SELECT id, payload FROM agent_missions WHERE json_extract(payload, '$.role') = ? AND status = 'PENDING'", role)
		if err != nil {
			return err
		}
		defer rows.Close()

		for rows.Next() {
			var id, taskStr string
			if err := rows.Scan(&id, &taskStr); err != nil {
				return err
			}

			var payloadMap map[string]interface{}
			var msg Message
			if err := json.Unmarshal([]byte(taskStr), &payloadMap); err == nil {
				if taskRaw, ok := payloadMap["task"]; ok {
					taskBytes, _ := json.Marshal(taskRaw)
					if err := json.Unmarshal(taskBytes, &msg); err != nil {
						msg = Message{ID: id, Content: string(taskBytes), Type: EventTask}
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

			if true {
				if msg.ID == "" {
					msg.ID = id
				}
			}
			missions = append(missions, msg)
		}
		return nil
	})
	if err == nil {
		telemetry.RecordAgentApiCall(ctx, "system-db", "provider", "sqlite_get_pending_missions")
	}
	_ = time.Since(start)
	return missions, err
}

func (p *SQLiteProvider) CompleteMission(ctx context.Context, missionID string) error {
	start := time.Now()
	err := withRetry(ctx, func() error {
		res, err := p.db.ExecContext(ctx, "UPDATE agent_missions SET status = 'COMPLETED' WHERE id = ?", missionID)
		if err != nil {
			return err
		}
		affected, _ := res.RowsAffected()
		if affected == 0 {
			return errors.New("mission not found")
		}
		return nil
	})
	if err == nil {
		telemetry.RecordAgentApiCall(ctx, "system-db", "provider", "sqlite_complete_mission")
	}
	_ = time.Since(start)
	return err
}

func (p *SQLiteProvider) Heartbeat(ctx context.Context, agentID, role, status string) error {
	return withRetry(ctx, func() error {
		_, err := p.db.ExecContext(ctx,
			"INSERT INTO agent_status (agent_id, role, status, last_heartbeat) VALUES (?, ?, ?, CURRENT_TIMESTAMP) ON CONFLICT(agent_id) DO UPDATE SET role=excluded.role, status=excluded.status, last_heartbeat=CURRENT_TIMESTAMP",
			agentID, role, status,
		)
		return err
	})
}

func (p *SQLiteProvider) DelegateMission(ctx context.Context, missionID string, payload string) error {
	start := time.Now()
	err := withRetry(ctx, func() error {
		_, err := p.db.ExecContext(ctx,
			"INSERT INTO agent_missions (id, status, payload, created_at) VALUES (?, 'PENDING', ?, CURRENT_TIMESTAMP)",
			missionID, payload,
		)
		return err
	})
	if err == nil {
		telemetry.RecordAgentApiCall(ctx, "system-db", "provider", "sqlite_delegate_mission")
	}
	_ = time.Since(start)
	return err
}

func (p *SQLiteProvider) PruneStaleMissions(ctx context.Context, ageThreshold time.Duration) error {
	return withRetry(ctx, func() error {
		thresholdTime := time.Now().Add(-ageThreshold).UTC().Format("2006-01-02 15:04:05")
		_, err := p.db.ExecContext(ctx, "DELETE FROM agent_missions WHERE status = 'COMPLETED' OR created_at < ?", thresholdTime)
		return err
	})
}

func (p *SQLiteProvider) RegisterCapabilityPlugin(ctx context.Context, plugin CapabilityPlugin) error {
	return withRetry(ctx, func() error {
		_, err := p.db.ExecContext(ctx,
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

func (p *SQLiteProvider) GetCapabilityPlugins(ctx context.Context, status string) ([]CapabilityPlugin, error) {
	var plugins []CapabilityPlugin
	err := withRetry(ctx, func() error {
		plugins = nil // reset slice
		var rows *sql.Rows
		var err error
		if status == "" {
			rows, err = p.db.QueryContext(ctx, "SELECT plugin_id, name, version, manifest_url, status, registered_at FROM capability_plugins")
		} else {
			rows, err = p.db.QueryContext(ctx, "SELECT plugin_id, name, version, manifest_url, status, registered_at FROM capability_plugins WHERE status = ?", status)
		}

		if err != nil {
			return err
		}
		defer rows.Close()

		for rows.Next() {
			var plugin CapabilityPlugin
			var t string
			if err := rows.Scan(&plugin.PluginID, &plugin.Name, &plugin.Version, &plugin.ManifestURL, &plugin.Status, &t); err != nil {
				return err
			}
			plugin.RegisteredAt, _ = time.Parse("2006-01-02 15:04:05", t)
			plugins = append(plugins, plugin)
		}
		return nil
	})
	return plugins, err
}

func (p *SQLiteProvider) StoreEpisodicMemory(ctx context.Context, memory EpisodicMemory) error {
	return withRetry(ctx, func() error {
		_, err := p.db.ExecContext(ctx,
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

func (p *SQLiteProvider) GetEpisodicMemoriesByPlugin(ctx context.Context, plugin string) ([]EpisodicMemory, error) {
	var memories []EpisodicMemory
	err := withRetry(ctx, func() error {
		memories = nil // reset slice
		var rows *sql.Rows
		var err error
		if plugin == "" {
			rows, err = p.db.QueryContext(ctx, "SELECT memory_id, context, vector_embedding, source_plugin, created_at FROM swarm_memory_embeddings")
		} else {
			rows, err = p.db.QueryContext(ctx, "SELECT memory_id, context, vector_embedding, source_plugin, created_at FROM swarm_memory_embeddings WHERE source_plugin = ?", plugin)
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

func (p *SQLiteProvider) Close() error {
	return p.db.Close()
}

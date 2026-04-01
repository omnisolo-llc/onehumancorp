package orchestration

import (
	"context"
	"encoding/json"
	"errors"
	"time"

	"github.com/jackc/pgx/v5"
	"github.com/jackc/pgx/v5/pgxpool"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// PostgresProvider is the PostgreSQL implementation of DatabaseProvider.
type PostgresProvider struct {
	pool *pgxpool.Pool
}

// NewPostgresProvider creates a new PostgresProvider
func NewPostgresProvider(pool *pgxpool.Pool) *PostgresProvider {
	return &PostgresProvider{pool: pool}
}

func (p *PostgresProvider) InitializeTables() error {
	// Let migrations handle the tables, or run equivalent queries here
	// Assuming migrations handles schema creation.
	return nil
}

func (p *PostgresProvider) SyncMemory(ctx context.Context, key string) (string, error) {
	start := time.Now()
	var value string
	err := p.pool.QueryRow(ctx, "SELECT value FROM swarm_memory WHERE key = $1", key).Scan(&value)
	if err == pgx.ErrNoRows {
		err = nil
	}
	if err == nil {
		telemetry.RecordAgentApiCall(ctx, "system-db", "provider", "postgres_sync_memory")
	}
	_ = time.Since(start)
	return value, err
}

func (p *PostgresProvider) UpdateMemory(ctx context.Context, key, value string) error {
	start := time.Now()
	_, err := p.pool.Exec(ctx,
		"INSERT INTO swarm_memory (key, value, updated_at) VALUES ($1, $2, CURRENT_TIMESTAMP) ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value, updated_at = CURRENT_TIMESTAMP",
		key, value,
	)
	if err == nil {
		telemetry.RecordAgentApiCall(ctx, "system-db", "provider", "postgres_update_memory")
	}
	_ = time.Since(start)
	return err
}

func (p *PostgresProvider) GetPendingMissions(ctx context.Context, role string) ([]Message, error) {
	start := time.Now()
	var missions []Message
	rows, err := p.pool.Query(ctx, "SELECT id, payload FROM agent_missions WHERE payload::jsonb->>'role' = $1 AND status = 'PENDING'", role)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	for rows.Next() {
		var id, taskStr string
		if err := rows.Scan(&id, &taskStr); err != nil {
			return nil, err
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

		if msg.ID == "" {
			msg.ID = id
		}

		missions = append(missions, msg)
	}
	if err == nil {
		telemetry.RecordAgentApiCall(ctx, "system-db", "provider", "postgres_get_pending_missions")
	}
	_ = time.Since(start)
	return missions, nil
}

func (p *PostgresProvider) CompleteMission(ctx context.Context, missionID string) error {
	start := time.Now()
	tag, err := p.pool.Exec(ctx, "UPDATE agent_missions SET status = 'COMPLETED' WHERE id = $1", missionID)
	if err != nil {
		return err
	}
	if tag.RowsAffected() == 0 {
		return errors.New("mission not found")
	}
	telemetry.RecordAgentApiCall(ctx, "system-db", "provider", "postgres_complete_mission")
	_ = time.Since(start)
	return nil
}

func (p *PostgresProvider) Heartbeat(ctx context.Context, agentID, role, status string) error {
	_, err := p.pool.Exec(ctx,
		"INSERT INTO agent_status (agent_id, role, status, last_heartbeat) VALUES ($1, $2, $3, CURRENT_TIMESTAMP) ON CONFLICT (agent_id) DO UPDATE SET role = EXCLUDED.role, status = EXCLUDED.status, last_heartbeat = CURRENT_TIMESTAMP",
		agentID, role, status,
	)
	return err
}

func (p *PostgresProvider) DelegateMission(ctx context.Context, missionID string, payload string) error {
	start := time.Now()
	_, err := p.pool.Exec(ctx,
		"INSERT INTO agent_missions (id, status, payload, created_at) VALUES ($1, 'PENDING', $2, CURRENT_TIMESTAMP)",
		missionID, payload,
	)
	if err == nil {
		telemetry.RecordAgentApiCall(ctx, "system-db", "provider", "postgres_delegate_mission")
	}
	_ = time.Since(start)
	return err
}

func (p *PostgresProvider) PruneStaleMissions(ctx context.Context, ageThreshold time.Duration) error {
	thresholdTime := time.Now().Add(-ageThreshold).UTC()
	_, err := p.pool.Exec(ctx, "DELETE FROM agent_missions WHERE status = 'COMPLETED' OR created_at < $1", thresholdTime)
	return err
}

func (p *PostgresProvider) RegisterCapabilityPlugin(ctx context.Context, plugin CapabilityPlugin) error {
	_, err := p.pool.Exec(ctx,
		`INSERT INTO capability_plugins (plugin_id, name, version, manifest_url, status, registered_at)
		 VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP)
		 ON CONFLICT (plugin_id) DO UPDATE SET
		 name = EXCLUDED.name, version = EXCLUDED.version,
		 manifest_url = EXCLUDED.manifest_url, status = EXCLUDED.status,
		 registered_at = CURRENT_TIMESTAMP`,
		plugin.PluginID, plugin.Name, plugin.Version, plugin.ManifestURL, plugin.Status,
	)
	return err
}

func (p *PostgresProvider) GetCapabilityPlugins(ctx context.Context, status string) ([]CapabilityPlugin, error) {
	var plugins []CapabilityPlugin
	var rows pgx.Rows
	var err error

	if status == "" {
		rows, err = p.pool.Query(ctx, "SELECT plugin_id, name, version, manifest_url, status, registered_at FROM capability_plugins")
	} else {
		rows, err = p.pool.Query(ctx, "SELECT plugin_id, name, version, manifest_url, status, registered_at FROM capability_plugins WHERE status = $1", status)
	}

	if err != nil {
		return nil, err
	}
	defer rows.Close()

	for rows.Next() {
		var plugin CapabilityPlugin
		var t time.Time
		if err := rows.Scan(&plugin.PluginID, &plugin.Name, &plugin.Version, &plugin.ManifestURL, &plugin.Status, &t); err != nil {
			return nil, err
		}
		plugin.RegisteredAt = t
		plugins = append(plugins, plugin)
	}
	return plugins, nil
}

func (p *PostgresProvider) StoreEpisodicMemory(ctx context.Context, memory EpisodicMemory) error {
	_, err := p.pool.Exec(ctx,
		`INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, source_plugin, created_at)
		 VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)
		 ON CONFLICT (memory_id) DO UPDATE SET
		 context = EXCLUDED.context, vector_embedding = EXCLUDED.vector_embedding,
		 source_plugin = EXCLUDED.source_plugin`,
		memory.MemoryID, memory.Context, memory.VectorEmbedding, memory.SourcePlugin,
	)
	return err
}

func (p *PostgresProvider) GetEpisodicMemoriesByPlugin(ctx context.Context, plugin string) ([]EpisodicMemory, error) {
	var memories []EpisodicMemory
	var rows pgx.Rows
	var err error

	if plugin == "" {
		rows, err = p.pool.Query(ctx, "SELECT memory_id, context, vector_embedding, source_plugin, created_at FROM swarm_memory_embeddings")
	} else {
		rows, err = p.pool.Query(ctx, "SELECT memory_id, context, vector_embedding, source_plugin, created_at FROM swarm_memory_embeddings WHERE source_plugin = $1", plugin)
	}

	if err != nil {
		return nil, err
	}
	defer rows.Close()

	for rows.Next() {
		var m EpisodicMemory
		var t time.Time
		if err := rows.Scan(&m.MemoryID, &m.Context, &m.VectorEmbedding, &m.SourcePlugin, &t); err != nil {
			return nil, err
		}
		m.CreatedAt = t
		memories = append(memories, m)
	}
	return memories, nil
}

func (p *PostgresProvider) Close() error {
	// The pool is managed externally usually, but we could close it if we want
	// p.pool.Close()
	return nil
}

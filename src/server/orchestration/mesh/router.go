package mesh

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"

	"github.com/onehumancorp/mono/src/server/db"
)

type CapabilityRouter struct {
	mesh TeammateMesh
	db   db.Provider
}

type AgentProfile struct {
	AgentID string   `json:"agent_id"`
	Skills  []string `json:"skills"`
	Status  string   `json:"status"`
}

func NewCapabilityRouter(mesh TeammateMesh, dbProvider db.Provider) *CapabilityRouter {
	return &CapabilityRouter{
		mesh: mesh,
		db:   dbProvider,
	}
}

func (r *CapabilityRouter) EnsureTables(ctx context.Context) error {
	query := `CREATE TABLE IF NOT EXISTS agent_profiles (
		tenant_id TEXT NOT NULL,
		agent_id TEXT NOT NULL,
		skills TEXT NOT NULL DEFAULT '[]',
		status TEXT NOT NULL DEFAULT 'AVAILABLE',
		PRIMARY KEY (tenant_id, agent_id)
	)`
	if _, err := r.db.Exec(ctx, query); err != nil {
		return err
	}

	if !r.db.IsSQLite() {
		// Postgres specific: Enable row-level security
		rlsQuery := `ALTER TABLE agent_profiles ENABLE ROW LEVEL SECURITY;
		DROP POLICY IF EXISTS tenant_isolation_policy ON agent_profiles;
		CREATE POLICY tenant_isolation_policy ON agent_profiles
		    USING (tenant_id = current_setting('app.current_tenant', true));`
		if _, err := r.db.Exec(ctx, rlsQuery); err != nil {
			return err
		}
	}
	return nil
}

func (r *CapabilityRouter) RegisterAgentProfile(ctx context.Context, tenantID string, profile AgentProfile) error {
	skillsData, err := json.Marshal(profile.Skills)
	if err != nil {
		return err
	}

	if r.db.IsSQLite() {
		query := `INSERT INTO agent_profiles (tenant_id, agent_id, skills, status) VALUES (?, ?, ?, ?)
				  ON CONFLICT(tenant_id, agent_id) DO UPDATE SET skills=excluded.skills, status=excluded.status`
		_, err := r.db.Exec(ctx, query, tenantID, profile.AgentID, string(skillsData), profile.Status)
		return err
	}

	query := `INSERT INTO agent_profiles (tenant_id, agent_id, skills, status) VALUES ($1, $2, $3, $4)
			  ON CONFLICT(tenant_id, agent_id) DO UPDATE SET skills=excluded.skills, status=excluded.status`
	_, err = r.db.Exec(ctx, query, tenantID, profile.AgentID, string(skillsData), profile.Status)
	return err
}

func (r *CapabilityRouter) DispatchJob(ctx context.Context, tenantID string, jobID string, requiredSkill string, payload []byte) error {
	if tenantID == "" {
		return errors.New("tenant_id is required")
	}

	var selectedAgent string

	if r.db.IsSQLite() {
		// SQLite JSON fallback
		query := `SELECT agent_id FROM agent_profiles, json_each(agent_profiles.skills)
				  WHERE status = 'AVAILABLE' AND tenant_id = ? AND json_each.value = ? LIMIT 1`
		err := r.db.QueryRow(ctx, query, tenantID, requiredSkill).Scan(&selectedAgent)
		if err != nil && err.Error() != "sql: no rows in result set" {
			return fmt.Errorf("failed to query agent profiles: %w", err)
		}
	} else {
		// Postgres JSONB query
		query := `SELECT agent_id FROM agent_profiles
				  WHERE status = 'AVAILABLE' AND tenant_id = $1 AND skills::jsonb ? $2 LIMIT 1`
		err := r.db.QueryRow(ctx, query, tenantID, requiredSkill).Scan(&selectedAgent)
		if err != nil && err.Error() != "sql: no rows in result set" {
			// fallback if skills is not jsonb but just text
			// But it's defined as TEXT, so let's cast to jsonb in the query
			return fmt.Errorf("failed to query agent profiles: %w", err)
		}
	}

	if selectedAgent == "" {
		return fmt.Errorf("no available agent found with skill: %s", requiredSkill)
	}

	// Dispatch via Redis Teammate Mesh
	msg := map[string]interface{}{
		"tenant_id": tenantID,
		"job_id": jobID,
		"agent_id": selectedAgent,
		"required_skill": requiredSkill,
		"payload": string(payload),
	}
	data, err := json.Marshal(msg)
	if err != nil {
		return err
	}

	return r.mesh.Publish(ctx, "mesh:jobs", data)
}

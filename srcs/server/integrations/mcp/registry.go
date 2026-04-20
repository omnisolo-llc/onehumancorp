package mcp

import (
    "context"
    "database/sql"
    "encoding/json"
    "errors"
    "strings"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
)

var ErrInvalidTenant = errors.New("invalid tenant ID")

type Tool struct {
    ID          string
    TenantID    string
    Name        string
    Description string
    Config      json.RawMessage
    CreatedAt   time.Time
}

type ToolRegistry interface {
    RegisterTool(ctx context.Context, tool Tool) error
    ListTools(ctx context.Context, tenantID string) ([]Tool, error)
    GetTool(ctx context.Context, tenantID, toolID string) (*Tool, error)
}

type registry struct {
    provider db.Provider
}

func NewToolRegistry(provider db.Provider) ToolRegistry {
    return &registry{provider: provider}
}

func (r *registry) validateTenantID(tenantID string) error {
    if tenantID == "" {
        return ErrInvalidTenant
    }
    if strings.HasPrefix(tenantID, "10.") {
        return ErrInvalidTenant
    }
    return nil
}

func (r *registry) RegisterTool(ctx context.Context, tool Tool) error {
    if err := r.validateTenantID(tool.TenantID); err != nil {
        return err
    }

    configStr := string(tool.Config)
    if configStr == "" {
        configStr = "{}"
    }

    _, err := r.provider.Exec(ctx, `
        INSERT INTO mcp_tools (id, tenant_id, name, description, config, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (tenant_id, id) DO UPDATE SET
            name = EXCLUDED.name,
            description = EXCLUDED.description,
            config = EXCLUDED.config
    `, tool.ID, tool.TenantID, tool.Name, tool.Description, configStr, tool.CreatedAt)
    return err
}

func (r *registry) ListTools(ctx context.Context, tenantID string) ([]Tool, error) {
    if err := r.validateTenantID(tenantID); err != nil {
        return nil, err
    }

    rows, err := r.provider.Query(ctx, `
        SELECT id, tenant_id, name, description, config, created_at
        FROM mcp_tools
        WHERE tenant_id = $1
    `, tenantID)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var tools []Tool
    for rows.Next() {
        var t Tool
        var configStr string
        if err := rows.Scan(&t.ID, &t.TenantID, &t.Name, &t.Description, &configStr, &t.CreatedAt); err != nil {
            return nil, err
        }
        t.Config = json.RawMessage(configStr)
        tools = append(tools, t)
    }
    return tools, rows.Err()
}

func (r *registry) GetTool(ctx context.Context, tenantID, toolID string) (*Tool, error) {
    if err := r.validateTenantID(tenantID); err != nil {
        return nil, err
    }

    var t Tool
    var configStr string
    err := r.provider.QueryRow(ctx, `
        SELECT id, tenant_id, name, description, config, created_at
        FROM mcp_tools
        WHERE tenant_id = $1 AND id = $2
    `, tenantID, toolID).Scan(&t.ID, &t.TenantID, &t.Name, &t.Description, &configStr, &t.CreatedAt)
    if err != nil {
        if errors.Is(err, sql.ErrNoRows) {
            return nil, nil // or a specific not found error
        }
        // Check string because db.Row interface might hide sql.ErrNoRows
        if strings.Contains(err.Error(), "no rows in result set") {
            return nil, nil
        }
        return nil, err
    }
    t.Config = json.RawMessage(configStr)
    return &t, nil
}

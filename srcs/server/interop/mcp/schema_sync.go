package mcp

import (
    "context"
    "encoding/json"
    "fmt"
    "net/http"
    "net/url"
    "strings"
)

type SchemaSyncTool struct {
    proxy *McpSyncProxy
}

func NewSchemaSyncTool(proxy *McpSyncProxy) *SchemaSyncTool {
    return &SchemaSyncTool{proxy: proxy}
}

type Migration struct {
    Filename string `json:"filename"`
    Content  string `json:"content"`
}

func (t *SchemaSyncTool) Execute(ctx context.Context, targetVersion string) error {
    req, err := http.NewRequestWithContext(ctx, "GET", t.proxy.CloudEndpoint()+"/api/mcp/schema?target_version="+url.QueryEscape(targetVersion), nil)
    if err != nil {
        return fmt.Errorf("failed to create request: %w", err)
    }

    resp, err := t.proxy.HTTPClient().Do(req)
    if err != nil {
        return fmt.Errorf("failed to execute request: %w", err)
    }
    defer resp.Body.Close()

    if resp.StatusCode != http.StatusOK {
        return fmt.Errorf("unexpected status code: %d", resp.StatusCode)
    }

    var migrations []Migration
    if err := json.NewDecoder(resp.Body).Decode(&migrations); err != nil {
        return fmt.Errorf("failed to decode response: %w", err)
    }

    for _, m := range migrations {
        sqlStr := m.Content
        if t.proxy.DBProvider().IsSQLite() {
            sqlStr = strings.ReplaceAll(sqlStr, "JSONB", "TEXT")
        }

        tx, err := t.proxy.DBProvider().Begin(ctx)
        if err != nil {
            return fmt.Errorf("failed to begin transaction: %w", err)
        }

        execErr := func() error {
            defer tx.Rollback(ctx)
            if _, err := tx.Exec(ctx, sqlStr); err != nil {
                return fmt.Errorf("failed to execute migration %s: %w", m.Filename, err)
            }
            if err := tx.Commit(ctx); err != nil {
                return fmt.Errorf("failed to commit transaction: %w", err)
            }
            return nil
        }()

        if execErr != nil {
            return execErr
        }
    }

    return nil
}

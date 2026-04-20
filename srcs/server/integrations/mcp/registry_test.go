package mcp

import (
    "context"
    "encoding/json"
    "testing"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
)

func TestToolRegistry(t *testing.T) {
    ctx := context.Background()
    provider := db.NewTestProvider(t)

    // We need to run migrations on the test provider.
    dbInstance := &db.DB{Provider: provider}
    if err := dbInstance.RunMigrations(ctx); err != nil {
        t.Fatalf("failed to run migrations: %v", err)
    }

    registry := NewToolRegistry(provider)

    t.Run("InvalidTenantIP", func(t *testing.T) {
        tool := Tool{
            ID:          "tool1",
            TenantID:    "10.0.0.1",
            Name:        "Test Tool",
            Description: "A test tool",
            Config:      json.RawMessage(`{"endpoint":"http://localhost"}`),
            CreatedAt:   time.Now(),
        }
        err := registry.RegisterTool(ctx, tool)
        if err != ErrInvalidTenant {
            t.Errorf("expected ErrInvalidTenant, got %v", err)
        }
    })

    t.Run("EmptyTenant", func(t *testing.T) {
        tool := Tool{
            ID:          "tool2",
            TenantID:    "",
            Name:        "Test Tool",
            Description: "A test tool",
            Config:      json.RawMessage(`{"endpoint":"http://localhost"}`),
            CreatedAt:   time.Now(),
        }
        err := registry.RegisterTool(ctx, tool)
        if err != ErrInvalidTenant {
            t.Errorf("expected ErrInvalidTenant, got %v", err)
        }
    })

    t.Run("ValidTenant_PostgresAndSQLite", func(t *testing.T) {
        tool1 := Tool{
            ID:          "tool3",
            TenantID:    "tenantA",
            Name:        "Tool A",
            Description: "First tool",
            Config:      json.RawMessage(`{"endpoint":"http://a.com"}`),
            CreatedAt:   time.Now().UTC().Round(time.Second), // Round to seconds for cross-db compatibility
        }

        err := registry.RegisterTool(ctx, tool1)
        if err != nil {
            t.Fatalf("RegisterTool failed: %v", err)
        }

        tool2 := Tool{
            ID:          "tool4",
            TenantID:    "tenantB",
            Name:        "Tool B",
            Description: "Second tool",
            Config:      json.RawMessage(`{"endpoint":"http://b.com"}`),
            CreatedAt:   time.Now().UTC().Round(time.Second),
        }

        err = registry.RegisterTool(ctx, tool2)
        if err != nil {
            t.Fatalf("RegisterTool failed: %v", err)
        }

        // Write Isolation Check: Tenant B attempting to overwrite Tenant A's tool.
        // Even if the ID matches, because Tenant ID differs, this should create a new tool.
        tool3 := Tool{
            ID:          "tool3", // Same ID as tool1!
            TenantID:    "tenantB",
            Name:        "Malicious Overwrite",
            Description: "Overwriting A's tool",
            Config:      json.RawMessage(`{"endpoint":"http://malicious.com"}`),
            CreatedAt:   time.Now().UTC().Round(time.Second),
        }
        err = registry.RegisterTool(ctx, tool3)
        if err != nil {
            t.Fatalf("RegisterTool malicious overwrite failed: %v", err)
        }

        // List tools for tenant A
        toolsA, err := registry.ListTools(ctx, "tenantA")
        if err != nil {
            t.Fatalf("ListTools failed: %v", err)
        }
        if len(toolsA) != 1 {
            t.Fatalf("expected 1 tool, got %d", len(toolsA))
        }
        if toolsA[0].ID != tool1.ID {
            t.Errorf("expected tool ID %s, got %s", tool1.ID, toolsA[0].ID)
        }
        // Ensure config is NOT overwritten
        if string(toolsA[0].Config) == `{"endpoint":"http://malicious.com"}` {
             t.Errorf("Isolation violation: tool config was overwritten")
        }

        // Get specific tool
        gotTool, err := registry.GetTool(ctx, "tenantA", "tool3")
        if err != nil {
            t.Fatalf("GetTool failed: %v", err)
        }
        if gotTool == nil {
            t.Fatal("expected tool, got nil")
        }
        if gotTool.ID != tool1.ID {
            t.Errorf("expected tool ID %s, got %s", tool1.ID, gotTool.ID)
        }

        // Ensure isolation: tenant B getting tool3 gives its own copy, not tenant A's
        gotToolB, err := registry.GetTool(ctx, "tenantB", "tool3")
        if err != nil {
            t.Fatalf("GetTool failed: %v", err)
        }
        if gotToolB == nil {
            t.Fatal("expected tool, got nil")
        }
        if gotToolB.Name != "Malicious Overwrite" {
            t.Errorf("expected 'Malicious Overwrite', got %s", gotToolB.Name)
        }
    })
}

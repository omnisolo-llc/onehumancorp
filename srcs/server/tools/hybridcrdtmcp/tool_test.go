package hybridcrdtmcp

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// A quick mock provider for the test
type mockProvider struct {
	db.Provider
	execFunc     func(ctx context.Context, sql string, arguments ...any) (int64, error)
	queryRowFunc func(ctx context.Context, sql string, optionsAndArgs ...any) db.Row
}

func (m *mockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	if m.execFunc != nil {
		return m.execFunc(ctx, sql, arguments...)
	}
	return 1, nil
}

func (m *mockProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
	if m.queryRowFunc != nil {
		return m.queryRowFunc(ctx, sql, optionsAndArgs...)
	}
	return &mockRow{}
}

func (m *mockProvider) IsSQLite() bool {
	return false
}

type mockRow struct {
	scanFunc func(dest ...any) error
}

func (m *mockRow) Scan(dest ...any) error {
	if m.scanFunc != nil {
		return m.scanFunc(dest...)
	}
	// default: simulate empty string for crdt_vector
	for _, d := range dest {
		if sp, ok := d.(*string); ok {
			*sp = ""
		}
	}
	return nil
}

func TestMultiTenantCheck(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	provider := &mockProvider{}
	mcp := NewHybridCRDTMCP(provider)

	// Without org ID
	args := map[string]interface{}{"entity_id": "test"}
	_, err := mcp.CallTool(context.Background(), "crdt_pull", args)
	if err == nil {
		t.Error("expected error due to missing org ID")
	}

	// With org ID
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})
	_, err = mcp.CallTool(ctx, "crdt_pull", args)
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
}

func TestCRDTMerge(t *testing.T) {
	provider := &mockProvider{}
	mcp := NewHybridCRDTMCP(provider)

	args := map[string]interface{}{
		"local_vector":  map[string]interface{}{"clock": 1.0},
		"remote_vector": map[string]interface{}{"clock": 2.0},
	}
	res, err := mcp.CallTool(context.Background(), "crdt_merge", args)
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
	if res == nil {
		t.Error("expected result")
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map[string]interface{}")
	}
	merged, ok := resMap["merged_vector"].(map[string]interface{})
	if !ok {
		t.Fatalf("expected merged_vector")
	}
	if clock, ok := merged["clock"].(float64); !ok || clock != 3.0 {
		t.Errorf("expected clock 3.0, got %v", clock)
	}
}

func TestCRDTPushPull(t *testing.T) {
	storedClock := 0.0
	provider := &mockProvider{
		execFunc: func(ctx context.Context, sql string, arguments ...any) (int64, error) {
			// Extract clock from JSON mutations and save it
			return 1, nil
		},
		queryRowFunc: func(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
			return &mockRow{
				scanFunc: func(dest ...any) error {
					for _, d := range dest {
						if sp, ok := d.(*string); ok {
							*sp = `{"clock": 5.0}`
						}
					}
					return nil
				},
			}
		},
	}
	_ = storedClock

	mcp := NewHybridCRDTMCP(provider)
	ctx := context.Background()

	// Push
	argsPush := map[string]interface{}{
		"entity_id": "test-task",
		"mutations": map[string]interface{}{"clock": 5.0},
	}
	_, err := mcp.CallTool(ctx, "crdt_push", argsPush)
	if err != nil {
		t.Fatalf("failed to push: %v", err)
	}

	// Pull
	argsPull := map[string]interface{}{"entity_id": "test-task"}
	res, err := mcp.CallTool(ctx, "crdt_pull", argsPull)
	if err != nil {
		t.Fatalf("failed to pull: %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("expected map[string]interface{}")
	}
	vector, ok := resMap["vector"].(map[string]interface{})
	if !ok {
		t.Fatalf("expected vector")
	}
	if clock, ok := vector["clock"].(float64); !ok || clock != 5.0 {
		t.Errorf("expected clock 5.0, got %v", clock)
	}
}

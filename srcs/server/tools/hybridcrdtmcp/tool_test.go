package hybridcrdtmcp

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestListTools(t *testing.T) {
	mcp := NewHybridCRDTMCP()
	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}
}

func TestCRDTPull_NoTenant(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	mcp := NewHybridCRDTMCP()
	ctx := context.Background()
	args := map[string]interface{}{
		"entity_id": "test-entity-1",
	}

	result, err := mcp.CallTool(ctx, "crdt_pull", args)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	resMap, ok := result.(map[string]interface{})
	if !ok {
		t.Fatal("expected result to be a map")
	}

	if resMap["entity_id"] != "test-entity-1" {
		t.Fatalf("expected entity_id 'test-entity-1', got '%v'", resMap["entity_id"])
	}
}

func TestCRDTPull_MultiTenant_Unauthorized(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	mcp := NewHybridCRDTMCP()
	ctx := context.Background()
	args := map[string]interface{}{
		"entity_id": "test-entity-1",
	}

	_, err := mcp.CallTool(ctx, "crdt_pull", args)
	if err == nil {
		t.Fatal("expected error for missing tenant claims, got nil")
	}
}

func TestCRDTPull_MultiTenant_Authorized(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	mcp := NewHybridCRDTMCP()
	claims := &auth.Claims{OrganizationID: "org-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)
	args := map[string]interface{}{
		"entity_id": "test-entity-1",
	}

	result, err := mcp.CallTool(ctx, "crdt_pull", args)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	resMap, ok := result.(map[string]interface{})
	if !ok {
		t.Fatal("expected result to be a map")
	}

	if resMap["entity_id"] != "test-entity-1" {
		t.Fatalf("expected entity_id 'test-entity-1', got '%v'", resMap["entity_id"])
	}
}

func TestCRDTPush(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	mcp := NewHybridCRDTMCP()
	ctx := context.Background()
	args := map[string]interface{}{
		"entity_id": "test-entity-1",
		"mutations": map[string]interface{}{
			"key1": "value1",
		},
	}

	result, err := mcp.CallTool(ctx, "crdt_push", args)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	resMap, ok := result.(map[string]interface{})
	if !ok {
		t.Fatal("expected result to be a map")
	}

	if resMap["status"] != "success" {
		t.Fatalf("expected status 'success', got '%v'", resMap["status"])
	}
}

func TestCRDTMerge(t *testing.T) {
	mcp := NewHybridCRDTMCP()
	ctx := context.Background()
	args := map[string]interface{}{
		"local_vector": map[string]interface{}{
			"key1": "local_val1",
			"key2": "local_val2",
		},
		"remote_vector": map[string]interface{}{
			"key2": "remote_val2",
			"key3": "remote_val3",
		},
	}

	result, err := mcp.CallTool(ctx, "crdt_merge", args)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	resMap, ok := result.(map[string]interface{})
	if !ok {
		t.Fatal("expected result to be a map")
	}

	merged, ok := resMap["merged_vector"].(map[string]interface{})
	if !ok {
		t.Fatal("expected merged_vector to be a map")
	}

	if merged["key1"] != "local_val1" {
		t.Errorf("expected key1 to be 'local_val1', got '%v'", merged["key1"])
	}
	if merged["key2"] != "remote_val2" {
		t.Errorf("expected key2 to be 'remote_val2', got '%v'", merged["key2"])
	}
	if merged["key3"] != "remote_val3" {
		t.Errorf("expected key3 to be 'remote_val3', got '%v'", merged["key3"])
	}
}

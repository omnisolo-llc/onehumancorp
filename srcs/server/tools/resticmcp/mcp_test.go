package resticmcp

import (
	"context"
	"os"
	"os/exec"
	"testing"
)

func TestResticMCP_ListTools(t *testing.T) {
	mcp := NewResticMCP("test-repo", "test-pwd")
	tools := mcp.ListTools()

	expectedTools := map[string]bool{
		"ResticSnapshot": false,
		"ResticRestore":  false,
		"ResticStatus":   false,
	}

	for _, tool := range tools {
		if _, ok := expectedTools[tool.Name]; ok {
			expectedTools[tool.Name] = true
		} else {
			t.Errorf("Unexpected tool name: %s", tool.Name)
		}
	}

	for name, found := range expectedTools {
		if !found {
			t.Errorf("Expected tool %s not found", name)
		}
	}
}

func TestResticMCP_CallTool_MissingConfig(t *testing.T) {
	mcp := NewResticMCP("", "")
	_, err := mcp.CallTool(context.Background(), "ResticStatus", nil)
	if err == nil {
		t.Errorf("Expected error for missing config")
	}
}

func TestResticMCP_CallTool_CloudMode(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_MULTITENANT")
	defer os.Unsetenv("OHC_STANDALONE")

	mcp := NewResticMCP("repo", "pwd")
	_, err := mcp.CallTool(context.Background(), "ResticStatus", nil)
	if err == nil || err.Error() != "unsupported: restic MCP is only available in Standalone Mode" {
		t.Errorf("Expected unsupported error in Cloud Mode, got: %v", err)
	}
}

func TestResticMCP_CallTool_Unknown(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	mcp := NewResticMCP("repo", "pwd")
	_, err := mcp.CallTool(context.Background(), "UnknownTool", nil)
	if err == nil {
		t.Errorf("Expected error for unknown tool")
	}
}

// Since we cannot easily execute the real restic binary in our test environment reliably without it being installed,
// and since mocking exec.Command Context in Go tests can be complex when we parse CombinedOutput with varying returns,
// we will test the edge-case error handling and input validation in our mocked environment.

func TestResticMCP_CallTool_ResticSnapshot_Validation(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	mcp := NewResticMCP("repo", "pwd")

	// Missing paths arg
	_, err := mcp.CallTool(context.Background(), "ResticSnapshot", map[string]interface{}{})
	if err == nil {
		t.Errorf("Expected error for missing paths argument")
	}

	// Empty paths arg
	_, err = mcp.CallTool(context.Background(), "ResticSnapshot", map[string]interface{}{"paths": []interface{}{}})
	if err == nil {
		t.Errorf("Expected error for empty paths")
	}
}

func TestResticMCP_CallTool_ResticRestore_Validation(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	mcp := NewResticMCP("repo", "pwd")

	// Missing snapshot_id arg
	_, err := mcp.CallTool(context.Background(), "ResticRestore", map[string]interface{}{"target": "/fake/target"})
	if err == nil {
		t.Errorf("Expected error for missing snapshot_id argument")
	}

	// Missing target arg
	_, err = mcp.CallTool(context.Background(), "ResticRestore", map[string]interface{}{"snapshot_id": "fake-id"})
	if err == nil {
		t.Errorf("Expected error for missing target argument")
	}
}

// Mock test logic for testing execution without actually calling restic using exec.Command override in test runtime
// Note that we override ExecCommand temporarily to a stub
func TestResticMCP_CallTool_ExecMocking(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	ExecCommand = func(ctx context.Context, command string, args ...string) *exec.Cmd {
		// Mock a command that simply runs "echo" to succeed or "false" to fail
		if args[0] == "backup" {
			return exec.Command("echo", "success")
		}
		if args[0] == "restore" {
			return exec.Command("echo", "success")
		}
		if args[0] == "snapshots" {
			return exec.Command("echo", `[{"id":"fake"}]`)
		}
		return exec.Command("false")
	}
	defer func() { ExecCommand = exec.CommandContext }()

	mcp := NewResticMCP("repo", "pwd")

	// Test ResticSnapshot
	_, err := mcp.CallTool(context.Background(), "ResticSnapshot", map[string]interface{}{"paths": []interface{}{"/test"}})
	if err != nil {
		t.Errorf("Unexpected error for mock ResticSnapshot: %v", err)
	}

	// Test ResticRestore
	_, err = mcp.CallTool(context.Background(), "ResticRestore", map[string]interface{}{"snapshot_id": "id", "target": "/test"})
	if err != nil {
		t.Errorf("Unexpected error for mock ResticRestore: %v", err)
	}

	// Test ResticStatus
	res, err := mcp.CallTool(context.Background(), "ResticStatus", nil)
	if err != nil {
		t.Errorf("Unexpected error for mock ResticStatus: %v", err)
	}
	resMap, ok := res.(map[string]interface{})
	if !ok || resMap["status"] != "success" {
		t.Errorf("Expected success status, got %v", res)
	}
	if _, ok := resMap["snapshots"]; !ok {
		t.Errorf("Expected snapshots key in result")
	}
}

package codeexecmcp

import (
	"context"
	"os"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalExecutionProvider(t *testing.T) {
	provider := NewLocalExecutionProvider()
	ctx := context.Background()

	output, err := provider.RunShellCommand(ctx, "echo hello")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if !strings.Contains(output, "hello") {
		t.Errorf("expected output to contain 'hello', got %s", output)
	}
}

func TestLocalExecutionProvider_Error(t *testing.T) {
	provider := NewLocalExecutionProvider()
	ctx := context.Background()

	_, err := provider.RunShellCommand(ctx, "exit 1")
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestCloudExecutionProvider_NoClaims(t *testing.T) {
	provider := NewCloudExecutionProvider()
	ctx := context.Background() // no auth context

	_, err := provider.RunShellCommand(ctx, "echo hello")
	if err == nil {
		t.Fatal("expected error due to missing claims, got none")
	}
}

func TestCloudExecutionProvider_WithClaims(t *testing.T) {
	provider := NewCloudExecutionProvider()
	claims := &auth.Claims{OrganizationID: "test-org"}
	// Inject claims using the exported test key
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	_, err := provider.RunShellCommand(ctx, "echo hello")
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
	if !strings.Contains(err.Error(), "pending infrastructure integration") {
		t.Errorf("expected pending integration error, got %v", err)
	}
}

func TestHybridCodeExecMCP_ListTools(t *testing.T) {
	provider := NewLocalExecutionProvider()
	mcp := NewHybridCodeExecMCP(provider)

	tools := mcp.ListTools()
	if len(tools) != 1 {
		t.Fatalf("expected 1 tool, got %d", len(tools))
	}
	if tools[0].Name != "run_shell_command" {
		t.Errorf("expected tool name 'run_shell_command', got %s", tools[0].Name)
	}
}

func TestHybridCodeExecMCP_CallTool(t *testing.T) {
	provider := NewLocalExecutionProvider()
	mcp := NewHybridCodeExecMCP(provider)
	ctx := context.Background()

	// Test valid call
	args := map[string]interface{}{"command": "echo success"}
	res, err := mcp.CallTool(ctx, "run_shell_command", args)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	outputMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("expected result to be map[string]interface{}, got %T", res)
	}
	if !strings.Contains(outputMap["output"].(string), "success") {
		t.Errorf("expected 'success', got %s", outputMap["output"])
	}

	// Test missing argument
	_, err = mcp.CallTool(ctx, "run_shell_command", map[string]interface{}{})
	if err == nil {
		t.Fatal("expected error due to missing argument, got none")
	}

	// Test invalid type argument
	_, err = mcp.CallTool(ctx, "run_shell_command", map[string]interface{}{"command": 123})
	if err == nil {
		t.Fatal("expected error due to invalid argument type, got none")
	}

	// Test execution error returns error inside payload, not at top level
	res, err = mcp.CallTool(ctx, "run_shell_command", map[string]interface{}{"command": "exit 1"})
	if err != nil {
		t.Fatalf("expected no top level error, got %v", err)
	}
	outputMap, ok = res.(map[string]interface{})
	if !ok {
		t.Fatalf("expected result to be map[string]interface{}, got %T", res)
	}
	if _, hasErr := outputMap["error"]; !hasErr {
		t.Errorf("expected error field in payload, got %v", outputMap)
	}

	// Test invalid tool name
	_, err = mcp.CallTool(ctx, "invalid_tool", args)
	if err == nil {
		t.Fatal("expected error due to invalid tool name, got none")
	}
}

func TestNewProviderFactory(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	provider := NewProviderFactory()
	if _, ok := provider.(*CloudExecutionProvider); !ok {
		t.Errorf("expected CloudExecutionProvider, got %T", provider)
	}

	os.Setenv("OHC_MULTITENANT", "false")
	provider = NewProviderFactory()
	if _, ok := provider.(*LocalExecutionProvider); !ok {
		t.Errorf("expected LocalExecutionProvider, got %T", provider)
	}
}

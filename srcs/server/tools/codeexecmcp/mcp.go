package codeexecmcp

import (
	"context"
	"encoding/json"
	"errors"
	"os"
	"os/exec"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// CodeExecutionProvider abstracts code execution operations.
type CodeExecutionProvider interface {
	RunShellCommand(ctx context.Context, command string) (string, error)
}

// LocalExecutionProvider runs commands directly on the host.
type LocalExecutionProvider struct{}

func NewLocalExecutionProvider() *LocalExecutionProvider {
	return &LocalExecutionProvider{}
}

func (p *LocalExecutionProvider) RunShellCommand(ctx context.Context, command string) (string, error) {
	cmd := exec.CommandContext(ctx, "sh", "-c", command)
	output, err := cmd.CombinedOutput()
	return string(output), err
}

// CloudExecutionProvider runs commands in an isolated environment (simulated via strict timeout and tenant check).
type CloudExecutionProvider struct{}

func NewCloudExecutionProvider() *CloudExecutionProvider {
	return &CloudExecutionProvider{}
}

func (p *CloudExecutionProvider) RunShellCommand(ctx context.Context, command string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing claims or organization ID")
	}

	return "", errors.New("cloud execution requires ephemeral sandboxing (e.g., Firecracker), which is pending infrastructure integration")
}

// Tool describes an MCP tool.
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// HybridCodeExecMCP implements an MCP server for code execution.
type HybridCodeExecMCP struct {
	provider CodeExecutionProvider
}

// NewHybridCodeExecMCP creates a new HybridCodeExecMCP instance.
func NewHybridCodeExecMCP(provider CodeExecutionProvider) *HybridCodeExecMCP {
	return &HybridCodeExecMCP{
		provider: provider,
	}
}

// ListTools returns the available code execution tools.
func (m *HybridCodeExecMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "run_shell_command",
			Description: "Runs a shell command.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"command": {"type": "string"}}, "required": ["command"]}`),
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridCodeExecMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	if toolName == "run_shell_command" {
		command, ok := arguments["command"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'command' argument")
		}
		output, err := m.provider.RunShellCommand(ctx, command)
		result := map[string]interface{}{"output": output}
		if err != nil {
			result["error"] = err.Error()
		}
		return result, nil
	}
	return nil, errors.New("tool not found")
}

// NewProviderFactory returns the appropriate provider based on OHC_MULTITENANT.
func NewProviderFactory() CodeExecutionProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return NewCloudExecutionProvider()
	}
	return NewLocalExecutionProvider()
}

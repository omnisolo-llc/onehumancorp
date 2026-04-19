package resticmcp

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"os/exec"
	"strings"
)

// ExecCommand is a variable pointing to exec.CommandContext to allow mocking in tests.
var ExecCommand = exec.CommandContext

// ResticMCP implements the MCP interface for restic snapshot operations.
type ResticMCP struct {
	repo string
	pwd  string
}

// NewResticMCP creates a new ResticMCP instance.
func NewResticMCP(repo, pwd string) *ResticMCP {
	return &ResticMCP{
		repo: repo,
		pwd:  pwd,
	}
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

func envBoolDefault(key string, fallback bool) bool {
	val := os.Getenv(key)
	if val == "" {
		return fallback
	}
	return strings.ToLower(val) == "true" || val == "1"
}

// isCloudMode checks if the current environment is running in Cloud Mode
func isCloudMode() bool {
	return envBoolDefault("OHC_MULTITENANT", false) && !envBoolDefault("OHC_STANDALONE", false)
}

// ListTools returns the list of available tools.
func (m *ResticMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "ResticSnapshot",
			Description: "Creates a restic snapshot of the specified directories.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"paths": {"type": "array", "items": {"type": "string"}}}, "required": ["paths"]}`),
		},
		{
			Name:        "ResticRestore",
			Description: "Restores a restic snapshot to the specified target directory.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"snapshot_id": {"type": "string"}, "target": {"type": "string"}}, "required": ["snapshot_id", "target"]}`),
		},
		{
			Name:        "ResticStatus",
			Description: "Gets the status and lists snapshots in the restic repository.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {}}`),
		},
	}
}

// CallTool executes a tool by name.
func (m *ResticMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	if isCloudMode() {
		return nil, errors.New("unsupported: restic MCP is only available in Standalone Mode")
	}

	if m.repo == "" || m.pwd == "" {
		return nil, errors.New("restic repository or password not configured")
	}

	env := append(os.Environ(),
		fmt.Sprintf("RESTIC_REPOSITORY=%s", m.repo),
		fmt.Sprintf("RESTIC_PASSWORD=%s", m.pwd),
	)

	switch toolName {
	case "ResticSnapshot":
		pathsRaw, ok := arguments["paths"].([]interface{})
		if !ok {
			return nil, errors.New("missing or invalid 'paths' argument")
		}
		var paths []string
		for _, p := range pathsRaw {
			if str, ok := p.(string); ok {
				paths = append(paths, str)
			}
		}
		if len(paths) == 0 {
			return nil, errors.New("no paths provided for snapshot")
		}

		args := append([]string{"backup", "--"}, paths...)
		cmd := ExecCommand(ctx, "restic", args...)
		cmd.Env = env
		out, err := cmd.CombinedOutput()
		if err != nil {
			return nil, fmt.Errorf("restic backup failed: %v, output: %s", err, string(out))
		}
		return map[string]interface{}{"status": "success", "output": string(out)}, nil

	case "ResticRestore":
		snapshotID, ok := arguments["snapshot_id"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'snapshot_id' argument")
		}
		target, ok := arguments["target"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'target' argument")
		}

		cmd := ExecCommand(ctx, "restic", "restore", snapshotID, "--target", target)
		cmd.Env = env
		out, err := cmd.CombinedOutput()
		if err != nil {
			return nil, fmt.Errorf("restic restore failed: %v, output: %s", err, string(out))
		}
		return map[string]interface{}{"status": "success", "output": string(out)}, nil

	case "ResticStatus":
		cmd := ExecCommand(ctx, "restic", "snapshots", "--json")
		cmd.Env = env
		out, err := cmd.CombinedOutput()
		if err != nil {
			return nil, fmt.Errorf("restic status failed: %v, output: %s", err, string(out))
		}
		var snapshots []interface{}
		if err := json.Unmarshal(out, &snapshots); err != nil {
			// fallback to returning raw string if not json
			return map[string]interface{}{"status": "success", "output": string(out)}, nil
		}
		return map[string]interface{}{"status": "success", "snapshots": snapshots}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

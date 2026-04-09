package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

type MCPFactory struct {
	isCloud bool
	cloudProvider *CloudFSProvider
	localProvider *LocalFSProvider
}

func NewMCPFactory() *MCPFactory {
	isCloud := os.Getenv("OHC_MULTITENANT") == "true"

	var cloudProvider *CloudFSProvider
	var localProvider *LocalFSProvider

	if isCloud {
		baseVolume := os.Getenv("OHC_CLOUD_VOLUME_ROOT")
		if baseVolume == "" {
			baseVolume = "/mnt/cloud_volume" // Default for testing
		}
		cloudProvider = NewCloudFSProvider(baseVolume)
	} else {
		workspaceRoot := os.Getenv("OHC_WORKSPACE_ROOT")
		if workspaceRoot == "" {
			workspaceRoot = "/tmp/ohc_workspace" // Default for testing
		}
		localProvider = NewLocalFSProvider(workspaceRoot)
	}

	return &MCPFactory{
		isCloud:       isCloud,
		cloudProvider: cloudProvider,
		localProvider: localProvider,
	}
}

func (f *MCPFactory) GetProvider() FileSystemProvider {
	if f.isCloud {
		return f.cloudProvider
	}
	return f.localProvider
}

func (f *MCPFactory) ExecuteTool(ctx context.Context, toolName string, params map[string]interface{}) *mcp.ExecutionResult {
	provider := f.GetProvider()

	switch toolName {
	case "read_file":
		path, ok := params["path"].(string)
		if !ok {
			return mcp.FormatExecutionResult(toolName, "error", []byte(`{"error": "missing path parameter"}`), false)
		}
		data, err := provider.ReadFile(ctx, path)
		if err != nil {
			errBytes, _ := json.Marshal(map[string]interface{}{"error": err.Error()})
			return mcp.FormatExecutionResult(toolName, "error", errBytes, false)
		}
		resBytes, _ := json.Marshal(map[string]interface{}{"content": string(data)})
		return mcp.FormatExecutionResult(toolName, "success", resBytes, false)

	case "write_file":
		path, ok := params["path"].(string)
		if !ok {
			return mcp.FormatExecutionResult(toolName, "error", []byte(`{"error": "missing path parameter"}`), false)
		}
		content, ok := params["content"].(string)
		if !ok {
			return mcp.FormatExecutionResult(toolName, "error", []byte(`{"error": "missing content parameter"}`), false)
		}
		err := provider.WriteFile(ctx, path, []byte(content))
		if err != nil {
			errBytes, _ := json.Marshal(map[string]interface{}{"error": err.Error()})
			return mcp.FormatExecutionResult(toolName, "error", errBytes, false)
		}
		return mcp.FormatExecutionResult(toolName, "success", []byte(`{"status": "file written successfully"}`), false)

	case "list_directory":
		path, ok := params["path"].(string)
		if !ok {
			return mcp.FormatExecutionResult(toolName, "error", []byte(`{"error": "missing path parameter"}`), false)
		}
		entries, err := provider.ListDir(ctx, path)
		if err != nil {
			errBytes, _ := json.Marshal(map[string]interface{}{"error": err.Error()})
			return mcp.FormatExecutionResult(toolName, "error", errBytes, false)
		}
		b, _ := json.Marshal(map[string]interface{}{"entries": entries})
		return mcp.FormatExecutionResult(toolName, "success", b, false)

	default:
		return mcp.FormatExecutionResult(toolName, "error", []byte(`{"error": "unknown tool"}`), false)
	}
}

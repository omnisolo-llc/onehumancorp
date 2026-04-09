package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

type FileSystemMCPServer struct {
	provider FileSystemProvider
}

func NewFileSystemMCPServer(provider FileSystemProvider) *FileSystemMCPServer {
	return &FileSystemMCPServer{provider: provider}
}

// Ensure the server implements whatever MCP interface is required.
// For now, we'll expose standard methods.

type ToolRequest struct {
	ToolID string          `json:"tool_id"`
	Params json.RawMessage `json:"params"`
}

type ReadFileParams struct {
	Path string `json:"path"`
}

type WriteFileParams struct {
	Path string `json:"path"`
	Data string `json:"data"` // base64 or string depending on implementation, let's assume string
}

type ListDirParams struct {
	Path string `json:"path"`
}

type SearchFilesParams struct {
	Directory string `json:"directory"`
	Pattern   string `json:"pattern"`
}

func (s *FileSystemMCPServer) HandleTool(ctx context.Context, req ToolRequest) *mcp.ExecutionResult {
	var resultData []byte
	var status string
	var err error

	switch req.ToolID {
	case "read_file":
		var params ReadFileParams
		if err = json.Unmarshal(req.Params, &params); err == nil {
			resultData, err = s.provider.ReadFile(ctx, params.Path)
		}
	case "write_file":
		var params WriteFileParams
		if err = json.Unmarshal(req.Params, &params); err == nil {
			err = s.provider.WriteFile(ctx, params.Path, []byte(params.Data))
			if err == nil {
				resultData = []byte(`{"success": true}`)
			}
		}
	case "list_directory":
		var params ListDirParams
		if err = json.Unmarshal(req.Params, &params); err == nil {
			var entries []string
			entries, err = s.provider.ListDir(ctx, params.Path)
			if err == nil {
				resultData, err = json.Marshal(entries)
			}
		}
	case "search_files":
		var params SearchFilesParams
		if err = json.Unmarshal(req.Params, &params); err == nil {
			var matches []string
			matches, err = s.provider.SearchFiles(ctx, params.Directory, params.Pattern)
			if err == nil {
				resultData, err = json.Marshal(matches)
			}
		}
	default:
		err = fmt.Errorf("unknown tool_id: %s", req.ToolID)
	}

	if err != nil {
		status = "error"
		resultData = []byte(fmt.Sprintf(`{"error": %q}`, err.Error()))
	} else {
		status = "success"
	}

	return mcp.FormatExecutionResult(req.ToolID, status, resultData, false)
}

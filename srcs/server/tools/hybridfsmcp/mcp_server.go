package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

// FSMCPServer implements a standard MCP server wrapping the Hybrid FS Provider
type FSMCPServer struct {
	provider FileSystemProvider
}

func NewFSMCPServer(provider FileSystemProvider) *FSMCPServer {
	return &FSMCPServer{provider: provider}
}

// ReadFileArgs defines arguments for the read_file tool
type ReadFileArgs struct {
	Path string `json:"path"`
}

// WriteFileArgs defines arguments for the write_file tool
type WriteFileArgs struct {
	Path string `json:"path"`
	Data string `json:"data"`
}

// ListDirArgs defines arguments for the list_directory tool
type ListDirArgs struct {
	Path string `json:"path"`
}

func (s *FSMCPServer) ExecuteTool(ctx context.Context, toolName string, args json.RawMessage) (*mcp.ExecutionResult, error) {
	var escalation bool
	var status string
	var resultData []byte
	var err error

	switch toolName {
	case "read_file":
		var req ReadFileArgs
		if err := json.Unmarshal(args, &req); err != nil {
			return nil, fmt.Errorf("invalid arguments for read_file: %w", err)
		}
		data, readErr := s.provider.ReadFile(ctx, req.Path)
		if readErr != nil {
			status = "error"
			resultData = []byte(fmt.Sprintf(`{"error": %q}`, readErr.Error()))
			escalation = true
		} else {
			status = "success"
			resultData, _ = json.Marshal(map[string]string{"content": string(data)})
		}

	case "write_file":
		var req WriteFileArgs
		if err := json.Unmarshal(args, &req); err != nil {
			return nil, fmt.Errorf("invalid arguments for write_file: %w", err)
		}
		writeErr := s.provider.WriteFile(ctx, req.Path, []byte(req.Data))
		if writeErr != nil {
			status = "error"
			resultData = []byte(fmt.Sprintf(`{"error": %q}`, writeErr.Error()))
			escalation = true
		} else {
			status = "success"
			resultData, _ = json.Marshal(map[string]string{"message": "file written successfully"})
		}

	case "list_directory":
		var req ListDirArgs
		if err := json.Unmarshal(args, &req); err != nil {
			return nil, fmt.Errorf("invalid arguments for list_directory: %w", err)
		}
		entries, listErr := s.provider.ListDir(ctx, req.Path)
		if listErr != nil {
			status = "error"
			resultData = []byte(fmt.Sprintf(`{"error": %q}`, listErr.Error()))
			escalation = true
		} else {
			status = "success"
			var names []string
			for _, entry := range entries {
				names = append(names, entry.Name())
			}
			resultData, _ = json.Marshal(map[string][]string{"entries": names})
		}
	case "search_files":
		var req SearchFilesArgs
		if err := json.Unmarshal(args, &req); err != nil {
			return nil, fmt.Errorf("invalid arguments for search_files: %w", err)
		}
		matches, searchErr := s.provider.SearchFiles(ctx, req.Path, req.Pattern)
		if searchErr != nil {
			status = "error"
			resultData = []byte(fmt.Sprintf(`{"error": %q}`, searchErr.Error()))
			escalation = true
		} else {
			status = "success"
			resultData, _ = json.Marshal(map[string][]string{"matches": matches})
		}
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}

	return mcp.FormatExecutionResult(toolName, status, resultData, escalation), err
}

// SearchFilesArgs defines arguments for the search_files tool
type SearchFilesArgs struct {
	Path    string `json:"path"`
	Pattern string `json:"pattern"`
}

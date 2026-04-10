package hybridfsmcp

import (
	"context"
	"encoding/json"
	"encoding/base64"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

type FileSystemMCPServer struct {
	provider FileSystemProvider
}

func NewFileSystemMCPServer(provider FileSystemProvider) *FileSystemMCPServer {
	return &FileSystemMCPServer{provider: provider}
}

func (s *FileSystemMCPServer) ExecuteTool(ctx context.Context, toolID string, payload []byte) (*mcp.ExecutionResult, error) {
	switch toolID {
	case "read_file":
		var args struct {
			Path string `json:"path"`
		}
		if err := json.Unmarshal(payload, &args); err != nil {
			return nil, fmt.Errorf("invalid payload: %w", err)
		}
		data, err := s.provider.ReadFile(ctx, args.Path)
		if err != nil {
			return mcp.FormatExecutionResult(toolID, "error", []byte(err.Error()), false), nil
		}
		return mcp.FormatExecutionResult(toolID, "success", data, false), nil

	case "write_file":
		var args struct {
			Path string `json:"path"`
			Data string `json:"data"`
		}
		if err := json.Unmarshal(payload, &args); err != nil {
			return nil, fmt.Errorf("invalid payload: %w", err)
		}

		decoded, err := base64.StdEncoding.DecodeString(args.Data)
		if err != nil {
			return nil, fmt.Errorf("invalid base64 data: %w", err)
		}

		err = s.provider.WriteFile(ctx, args.Path, decoded, 0644)
		if err != nil {
			return mcp.FormatExecutionResult(toolID, "error", []byte(err.Error()), false), nil
		}
		return mcp.FormatExecutionResult(toolID, "success", []byte("file written successfully"), false), nil

	case "list_directory":
		var args struct {
			Path string `json:"path"`
		}
		if err := json.Unmarshal(payload, &args); err != nil {
			return nil, fmt.Errorf("invalid payload: %w", err)
		}
		infos, err := s.provider.ListDir(ctx, args.Path)
		if err != nil {
			return mcp.FormatExecutionResult(toolID, "error", []byte(err.Error()), false), nil
		}

		type fileEntry struct {
			Name  string `json:"name"`
			IsDir bool   `json:"is_dir"`
			Size  int64  `json:"size"`
		}

		var entries []fileEntry
		for _, info := range infos {
			entries = append(entries, fileEntry{
				Name:  info.Name(),
				IsDir: info.IsDir(),
				Size:  info.Size(),
			})
		}

		out, _ := json.Marshal(entries)
		return mcp.FormatExecutionResult(toolID, "success", out, false), nil

	case "search_files":
		// A simple search using ListDir as a basis (or you can expand this to walk dirs)
		return mcp.FormatExecutionResult(toolID, "error", []byte("search_files not fully implemented"), false), nil

	default:
		return nil, fmt.Errorf("unknown tool_id: %s", toolID)
	}
}

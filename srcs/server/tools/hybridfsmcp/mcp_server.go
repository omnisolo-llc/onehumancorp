package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

type Server struct {
	provider FileSystemProvider
}

func NewServer(provider FileSystemProvider) *Server {
	return &Server{provider: provider}
}

type ReadFileArgs struct {
	Path string `json:"path"`
}

type WriteFileArgs struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

type ListDirArgs struct {
	Path string `json:"path"`
}

type ListDirResult struct {
	Name  string `json:"name"`
	IsDir bool   `json:"is_dir"`
}

func (s *Server) ExecuteTool(ctx context.Context, toolID string, rawArgs []byte) *mcp.ExecutionResult {
	var err error
	var resultData []byte

	switch toolID {
	case "read_file":
		var args ReadFileArgs
		if err = json.Unmarshal(rawArgs, &args); err != nil {
			return mcp.FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf(`{"error": "%v"}`, err)), false)
		}
		var content []byte
		content, err = s.provider.ReadFile(ctx, args.Path)
		if err == nil {
			resultMap := map[string]string{"content": string(content)}
			resultData, _ = json.Marshal(resultMap)
		}

	case "write_file":
		var args WriteFileArgs
		if err = json.Unmarshal(rawArgs, &args); err != nil {
			return mcp.FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf(`{"error": "%v"}`, err)), false)
		}
		err = s.provider.WriteFile(ctx, args.Path, []byte(args.Content))
		if err == nil {
			resultData = []byte(`{"success": true}`)
		}

	case "list_directory":
		var args ListDirArgs
		if err = json.Unmarshal(rawArgs, &args); err != nil {
			return mcp.FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf(`{"error": "%v"}`, err)), false)
		}
		var entries []os.DirEntry
		entries, err = s.provider.ListDir(ctx, args.Path)
		if err == nil {
			var results []ListDirResult
			for _, entry := range entries {
				results = append(results, ListDirResult{
					Name:  entry.Name(),
					IsDir: entry.IsDir(),
				})
			}
			resultData, _ = json.Marshal(results)
		}

	default:
		return mcp.FormatExecutionResult(toolID, "error", []byte(`{"error": "unknown tool"}`), false)
	}

	if err != nil {
		return mcp.FormatExecutionResult(toolID, "error", []byte(fmt.Sprintf(`{"error": "%v"}`, err)), false)
	}

	return mcp.FormatExecutionResult(toolID, "success", resultData, false)
}

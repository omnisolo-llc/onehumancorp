package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

type FileSystemMCP struct {
	provider FileSystemProvider
}

func NewFileSystemMCP(provider FileSystemProvider) *FileSystemMCP {
	return &FileSystemMCP{
		provider: provider,
	}
}

type ReadFileArgs struct {
	Path string `json:"path"`
}

type WriteFileArgs struct {
	Path string `json:"path"`
	Data []byte `json:"data"`
}

type ListDirArgs struct {
	Path string `json:"path"`
}

type SearchFilesArgs struct {
	Path    string `json:"path"`
	Pattern string `json:"pattern"`
}

func (s *FileSystemMCP) HandleRequest(ctx context.Context, toolID string, args json.RawMessage) (*mcp.ExecutionResult, error) {
	var resultData []byte
	var err error

	switch toolID {
	case "read_file":
		var req ReadFileArgs
		if err := json.Unmarshal(args, &req); err != nil {
			return nil, fmt.Errorf("invalid arguments: %v", err)
		}
		data, err := s.provider.ReadFile(ctx, req.Path)
		if err != nil {
			return nil, err
		}
		resultData, err = json.Marshal(map[string]interface{}{"data": data})

	case "write_file":
		var req WriteFileArgs
		if err := json.Unmarshal(args, &req); err != nil {
			return nil, fmt.Errorf("invalid arguments: %v", err)
		}
		err = s.provider.WriteFile(ctx, req.Path, req.Data)
		if err != nil {
			return nil, err
		}
		resultData, err = json.Marshal(map[string]interface{}{"success": true})

	case "list_directory":
		var req ListDirArgs
		if err := json.Unmarshal(args, &req); err != nil {
			return nil, fmt.Errorf("invalid arguments: %v", err)
		}
		files, err := s.provider.ListDir(ctx, req.Path)
		if err != nil {
			return nil, err
		}
		resultData, err = json.Marshal(map[string]interface{}{"files": files})

	case "search_files":
		var req SearchFilesArgs
		if err := json.Unmarshal(args, &req); err != nil {
			return nil, fmt.Errorf("invalid arguments: %v", err)
		}

		var files []string
		if searcher, ok := s.provider.(interface {
			SearchFiles(ctx context.Context, path, pattern string) ([]string, error)
		}); ok {
			files, err = searcher.SearchFiles(ctx, req.Path, req.Pattern)
		} else {
			return nil, fmt.Errorf("underlying provider does not support SearchFiles")
		}

		if err != nil {
			return nil, err
		}
		resultData, err = json.Marshal(map[string]interface{}{"files": files})

	default:
		return nil, fmt.Errorf("unknown tool ID: %s", toolID)
	}

	if err != nil {
		return nil, err
	}

	return mcp.FormatExecutionResult(toolID, "success", resultData, false), nil
}

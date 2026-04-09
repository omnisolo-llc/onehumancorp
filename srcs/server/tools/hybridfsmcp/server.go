package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

type FileSystemMCPServer struct {
	provider FileSystemProvider
}

func NewFileSystemMCPServer() (*FileSystemMCPServer, error) {
	var provider FileSystemProvider
	var err error

	if os.Getenv("OHC_STANDALONE") == "true" {
		workspaceDir := os.Getenv("OHC_WORKSPACE_DIR")
		if workspaceDir == "" {
			workspaceDir = "./workspace"
		}
		provider, err = NewLocalFSProvider(workspaceDir)
	} else if os.Getenv("OHC_MULTITENANT") == "true" {
		volumeRoot := os.Getenv("OHC_VOLUME_ROOT")
		if volumeRoot == "" {
			volumeRoot = "/mnt/tenant-data"
		}
		provider, err = NewCloudFSProvider(volumeRoot)
	} else {
		return nil, fmt.Errorf("must specify OHC_STANDALONE or OHC_MULTITENANT")
	}

	if err != nil {
		return nil, err
	}

	return &FileSystemMCPServer{provider: provider}, nil
}

// Call handles tool execution using the filesystem provider
func (s *FileSystemMCPServer) Call(ctx context.Context, toolName string, params map[string]interface{}) (*mcp.ExecutionResult, error) {
	var resultData interface{}
	var err error

	switch toolName {
	case "read_file":
		path, ok := params["path"].(string)
		if !ok {
			return nil, fmt.Errorf("read_file requires string parameter 'path'")
		}
		data, readErr := s.provider.ReadFile(ctx, path)
		if readErr != nil {
			err = readErr
		} else {
			resultData = map[string]string{"content": string(data)}
		}

	case "write_file":
		path, ok := params["path"].(string)
		content, ok2 := params["content"].(string)
		if !ok || !ok2 {
			return nil, fmt.Errorf("write_file requires string parameters 'path' and 'content'")
		}
		err = s.provider.WriteFile(ctx, path, []byte(content))
		if err == nil {
			resultData = map[string]string{"status": "success"}
		}

	case "list_directory":
		path, ok := params["path"].(string)
		if !ok {
			return nil, fmt.Errorf("list_directory requires string parameter 'path'")
		}
		entries, listErr := s.provider.ListDir(ctx, path)
		if listErr != nil {
			err = listErr
		} else {
			resultData = map[string]interface{}{"entries": entries}
		}

	case "search_files":
		dir, ok := params["directory"].(string)
		pattern, ok2 := params["pattern"].(string)
		if !ok || !ok2 {
			return nil, fmt.Errorf("search_files requires string parameters 'directory' and 'pattern'")
		}
		matches, searchErr := s.provider.SearchFiles(ctx, dir, pattern)
		if searchErr != nil {
			err = searchErr
		} else {
			resultData = map[string]interface{}{"matches": matches}
		}

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}

	if err != nil {
		return nil, err
	}

	resultBytes, marshalErr := json.Marshal(resultData)
	if marshalErr != nil {
		return nil, marshalErr
	}

	return mcp.FormatExecutionResult(toolName, "success", resultBytes, false), nil
}

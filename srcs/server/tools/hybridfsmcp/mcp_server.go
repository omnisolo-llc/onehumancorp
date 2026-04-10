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

	// Determine mode
	if os.Getenv("OHC_MULTITENANT") == "true" {
		root := os.Getenv("OHC_CLOUD_STORAGE_ROOT")
		if root == "" {
			root = "/tmp/ohc_cloud_storage" // Default for testing
		}
		provider, err = NewCloudFSProvider(root)
	} else {
		root := os.Getenv("OHC_STANDALONE_WORKSPACE")
		if root == "" {
			root = "/tmp/ohc_workspace" // Default for testing
		}
		provider, err = NewLocalFSProvider(root)
	}

	if err != nil {
		return nil, fmt.Errorf("failed to initialize FS provider: %w", err)
	}

	return &FileSystemMCPServer{provider: provider}, nil
}

type ReadFileInput struct {
	Path string `json:"path"`
}

type WriteFileInput struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

type ListDirectoryInput struct {
	Path string `json:"path"`
}

func (s *FileSystemMCPServer) ExecuteTool(ctx context.Context, toolID string, input json.RawMessage) (*mcp.ExecutionResult, error) {
	switch toolID {
	case "read_file":
		var in ReadFileInput
		if err := json.Unmarshal(input, &in); err != nil {
			return nil, err
		}
		data, err := s.provider.ReadFile(ctx, in.Path)
		if err != nil {
			return nil, err
		}
		res, _ := json.Marshal(map[string]string{"content": string(data)})
		return mcp.FormatExecutionResult(toolID, "success", res, false), nil

	case "write_file":
		var in WriteFileInput
		if err := json.Unmarshal(input, &in); err != nil {
			return nil, err
		}
		err := s.provider.WriteFile(ctx, in.Path, []byte(in.Content))
		if err != nil {
			return nil, err
		}
		res, _ := json.Marshal(map[string]string{"message": "file written successfully"})
		return mcp.FormatExecutionResult(toolID, "success", res, false), nil

	case "list_directory":
		var in ListDirectoryInput
		if err := json.Unmarshal(input, &in); err != nil {
			return nil, err
		}
		entries, err := s.provider.ListDir(ctx, in.Path)
		if err != nil {
			return nil, err
		}
		var names []string
		for _, e := range entries {
			names = append(names, e.Name())
		}
		res, _ := json.Marshal(map[string][]string{"files": names})
		return mcp.FormatExecutionResult(toolID, "success", res, false), nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolID)
	}
}

// Add Server to provide a bundle
type HybridFSBundle struct {
	server *FileSystemMCPServer
}

func NewHybridFSBundle() (*HybridFSBundle, error) {
	server, err := NewFileSystemMCPServer()
	if err != nil {
		return nil, err
	}
	return &HybridFSBundle{server: server}, nil
}

// GetTools Returns available tools for this bundle
func (b *HybridFSBundle) GetTools() []map[string]interface{} {
	return []map[string]interface{}{
		{
			"name":        "read_file",
			"description": "Read content of a file",
			"inputSchema": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type":        "string",
						"description": "Path to the file to read",
					},
				},
				"required": []string{"path"},
			},
		},
		{
			"name":        "write_file",
			"description": "Write content to a file",
			"inputSchema": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type":        "string",
						"description": "Path to the file to write",
					},
					"content": map[string]interface{}{
						"type":        "string",
						"description": "Content to write",
					},
				},
				"required": []string{"path", "content"},
			},
		},
		{
			"name":        "list_directory",
			"description": "List files in a directory",
			"inputSchema": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type":        "string",
						"description": "Path to the directory",
					},
				},
				"required": []string{"path"},
			},
		},
	}
}

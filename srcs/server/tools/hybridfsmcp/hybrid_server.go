package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
)

// Factory method to choose the appropriate provider based on deployment mode
func NewProvider() (FileSystemProvider, error) {
	if os.Getenv("OHC_STANDALONE") == "true" {
		// In Standalone Mode, use the current working directory or an explicit local workspace
		workspace := os.Getenv("OHC_LOCAL_WORKSPACE")
		if workspace == "" {
			var err error
			workspace, err = os.Getwd()
			if err != nil {
				return nil, fmt.Errorf("failed to get working directory: %w", err)
			}
		}
		return NewLocalFSProvider(workspace)
	}

	// Cloud Native Mode
	mountPath := os.Getenv("OHC_CLOUD_FS_MOUNT")
	if mountPath == "" {
		mountPath = "/var/lib/ohc/fs" // default volume mount
	}
	return NewCloudFSProvider(mountPath)
}

// Server exposes FileSystemProvider via MCP Tool format
type Server struct {
	provider FileSystemProvider
}

func NewServer() (*Server, error) {
	provider, err := NewProvider()
	if err != nil {
		return nil, err
	}
	return &Server{provider: provider}, nil
}

type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

type ReadFileArgs struct {
	Path string `json:"path"`
}

type WriteFileArgs struct {
	Path string `json:"path"`
	Data string `json:"data"` // base64 encoded or string representation
}

type ListDirArgs struct {
	Path string `json:"path"`
}

// Tools returns the list of MCP tools this server provides
func (s *Server) Tools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Read the contents of a file in the hybrid filesystem.",
			InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string"}},"required":["path"]}`),
		},
		{
			Name:        "write_file",
			Description: "Write contents to a file in the hybrid filesystem.",
			InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string"},"data":{"type":"string"}},"required":["path","data"]}`),
		},
		{
			Name:        "list_directory",
			Description: "List files and directories in a given path.",
			InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string"}},"required":["path"]}`),
		},
	}
}

// Call executes the requested tool
func (s *Server) Call(ctx context.Context, toolName string, args []byte) ([]byte, error) {
	switch toolName {
	case "read_file":
		var req ReadFileArgs
		if err := json.Unmarshal(args, &req); err != nil {
			return nil, err
		}

		data, err := s.provider.ReadFile(ctx, req.Path)
		if err != nil {
			return nil, err
		}
		return data, nil

	case "write_file":
		var req WriteFileArgs
		if err := json.Unmarshal(args, &req); err != nil {
			return nil, err
		}

		err := s.provider.WriteFile(ctx, req.Path, []byte(req.Data))
		if err != nil {
			return nil, err
		}
		return []byte(`{"status":"success"}`), nil

	case "list_directory":
		var req ListDirArgs
		if err := json.Unmarshal(args, &req); err != nil {
			return nil, err
		}

		infos, err := s.provider.ListDir(ctx, req.Path)
		if err != nil {
			return nil, err
		}

		return json.Marshal(infos)

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

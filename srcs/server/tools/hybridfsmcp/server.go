package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

// HybridFSServer exposes filesystem operations via MCP
type HybridFSServer struct {
	provider FileSystemProvider
}

// NewHybridFSServer creates a new HybridFSServer, initializing the appropriate
// FileSystemProvider based on the OHC_MULTITENANT environment variable.
func NewHybridFSServer() (*HybridFSServer, error) {
	fsRoot := os.Getenv("OHC_FS_ROOT")
	if fsRoot == "" {
		fsRoot = os.TempDir()
	}

	var provider FileSystemProvider
	var err error

	if os.Getenv("OHC_MULTITENANT") == "true" {
		provider, err = NewCloudFSProvider(fsRoot)
	} else {
		provider, err = NewLocalFSProvider(fsRoot)
	}

	if err != nil {
		return nil, fmt.Errorf("failed to initialize FS provider: %w", err)
	}

	return &HybridFSServer{provider: provider}, nil
}

// HandleToolCall processes MCP tool invocations
func (s *HybridFSServer) HandleToolCall(ctx context.Context, toolName string, args json.RawMessage) *mcp.ExecutionResult {
	var err error
	var result interface{}

	switch toolName {
	case "read_file":
		result, err = s.handleReadFile(ctx, args)
	case "write_file":
		result, err = s.handleWriteFile(ctx, args)
	case "list_directory":
		result, err = s.handleListDirectory(ctx, args)
	case "search_files":
		result, err = s.handleSearchFiles(ctx, args)
	default:
		return mcp.FormatExecutionResult(toolName, "error", []byte(fmt.Sprintf("unknown tool: %s", toolName)), false)
	}

	if err != nil {
		return mcp.FormatExecutionResult(toolName, "error", []byte(err.Error()), false)
	}

	resultBytes, _ := json.Marshal(result)
	return mcp.FormatExecutionResult(toolName, "success", resultBytes, false)
}

func (s *HybridFSServer) handleReadFile(ctx context.Context, args json.RawMessage) (interface{}, error) {
	var input struct {
		Path string `json:"path"`
	}
	if err := json.Unmarshal(args, &input); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	data, err := s.provider.ReadFile(ctx, input.Path)
	if err != nil {
		return nil, err
	}

	return map[string]string{"content": string(data)}, nil
}

func (s *HybridFSServer) handleWriteFile(ctx context.Context, args json.RawMessage) (interface{}, error) {
	var input struct {
		Path    string `json:"path"`
		Content string `json:"content"`
	}
	if err := json.Unmarshal(args, &input); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	if err := s.provider.WriteFile(ctx, input.Path, []byte(input.Content)); err != nil {
		return nil, err
	}

	return map[string]string{"status": "success"}, nil
}

func (s *HybridFSServer) handleListDirectory(ctx context.Context, args json.RawMessage) (interface{}, error) {
	var input struct {
		Path string `json:"path"`
	}
	if err := json.Unmarshal(args, &input); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	// Handle empty path as root
	path := input.Path
	if path == "" {
		path = "."
	}

	infos, err := s.provider.ListDir(ctx, path)
	if err != nil {
		return nil, err
	}

	return map[string]interface{}{"files": infos}, nil
}

func (s *HybridFSServer) handleSearchFiles(ctx context.Context, args json.RawMessage) (interface{}, error) {
	var input struct {
		Path    string `json:"path"`
		Pattern string `json:"pattern"`
	}
	if err := json.Unmarshal(args, &input); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	path := input.Path
	if path == "" {
		path = "."
	}

	var results []string
	err := s.searchRecursive(ctx, path, input.Pattern, &results)
	if err != nil {
		return nil, err
	}

	return map[string]interface{}{"matches": results}, nil
}

func (s *HybridFSServer) searchRecursive(ctx context.Context, currentPath, pattern string, results *[]string) error {
	infos, err := s.provider.ListDir(ctx, currentPath)
	if err != nil {
		return err
	}

	for _, info := range infos {
		fullPath := filepath.Join(currentPath, info.Name)
		if strings.Contains(info.Name, pattern) {
			*results = append(*results, fullPath)
		}
		if info.IsDir {
			if err := s.searchRecursive(ctx, fullPath, pattern, results); err != nil {
				// Ignore permission errors for subdirectories during search to allow partial results
				continue
			}
		}
	}
	return nil
}

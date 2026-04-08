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

type HybridFSMCP struct {
	provider mcp.FileSystemProvider
}

func NewHybridFSMCP(baseDir string) *HybridFSMCP {
	var provider mcp.FileSystemProvider
	if os.Getenv("OHC_STANDALONE") == "true" {
		provider = mcp.NewLocalFSProvider(baseDir)
	} else {
		provider = mcp.NewCloudFSProvider(baseDir)
	}
	return &HybridFSMCP{provider: provider}
}

type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

type ReadFileArgs struct {
	Path string `json:"path"`
}

type WriteFileArgs struct {
	Path string `json:"path"`
	Data string `json:"data"`
}

type ListDirArgs struct {
	Path string `json:"path"`
}

type SearchFilesArgs struct {
	Path    string `json:"path"`
	Pattern string `json:"pattern"`
}

func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the content of a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Writes data to a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "data": {"type": "string"}}, "required": ["path", "data"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists files in a directory.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "search_files",
			Description: "Searches for files by pattern.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "pattern": {"type": "string"}}, "required": ["path", "pattern"]}`,
		},
	}
}

func (s *HybridFSMCP) ReadFile(ctx context.Context, args []byte) (*mcp.ExecutionResult, error) {
	var a ReadFileArgs
	if err := json.Unmarshal(args, &a); err != nil {
		return nil, err
	}
	data, err := s.provider.ReadFile(ctx, a.Path)
	if err != nil {
		return nil, err
	}
	res, _ := json.Marshal(map[string]string{"content": string(data)})
	return mcp.FormatExecutionResult("read_file", "success", res, false), nil
}

func (s *HybridFSMCP) WriteFile(ctx context.Context, args []byte) (*mcp.ExecutionResult, error) {
	var a WriteFileArgs
	if err := json.Unmarshal(args, &a); err != nil {
		return nil, err
	}
	err := s.provider.WriteFile(ctx, a.Path, []byte(a.Data))
	if err != nil {
		return nil, err
	}
	res, _ := json.Marshal(map[string]string{"status": "written"})
	return mcp.FormatExecutionResult("write_file", "success", res, false), nil
}

func (s *HybridFSMCP) ListDir(ctx context.Context, args []byte) (*mcp.ExecutionResult, error) {
	var a ListDirArgs
	if err := json.Unmarshal(args, &a); err != nil {
		return nil, err
	}
	entries, err := s.provider.ListDir(ctx, a.Path)
	if err != nil {
		return nil, err
	}
	res, _ := json.Marshal(map[string]interface{}{"entries": entries})
	return mcp.FormatExecutionResult("list_directory", "success", res, false), nil
}

func (s *HybridFSMCP) SearchFiles(ctx context.Context, args []byte) (*mcp.ExecutionResult, error) {
	var a SearchFilesArgs
	if err := json.Unmarshal(args, &a); err != nil {
		return nil, err
	}

	var matches []string
	var search func(dir string) error
	search = func(dir string) error {
		entries, err := s.provider.ListDir(ctx, dir)
		if err != nil {
			return err
		}
		for _, entry := range entries {
			fullPath := filepath.Join(dir, entry)
			if strings.Contains(entry, a.Pattern) {
				matches = append(matches, fullPath)
			}
			_ = search(fullPath)
		}
		return nil
	}
	_ = search(a.Path)

	res, _ := json.Marshal(map[string]interface{}{"matches": matches})
	return mcp.FormatExecutionResult("search_files", "success", res, false), nil
}

func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	argsBytes, _ := json.Marshal(arguments)
	switch toolName {
	case "read_file":
		return m.ReadFile(ctx, argsBytes)
	case "write_file":
		return m.WriteFile(ctx, argsBytes)
	case "list_directory":
		return m.ListDir(ctx, argsBytes)
	case "search_files":
		return m.SearchFiles(ctx, argsBytes)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

package hybridfsmcp

import (
	"context"
	"encoding/base64"
	"errors"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter           = otel.Meter("github.com/onehumancorp/mono/srcs/server/tools/hybridfsmcp")
	filesRead, _    = meter.Int64Counter("hybridfsmcp.files_read", metric.WithDescription("Number of files read"))
	filesWritten, _ = meter.Int64Counter("hybridfsmcp.files_written", metric.WithDescription("Number of files written"))
	dirsListed, _   = meter.Int64Counter("hybridfsmcp.dirs_listed", metric.WithDescription("Number of directory listings"))
	filesSearched, _ = meter.Int64Counter("hybridfsmcp.files_searched", metric.WithDescription("Number of file searches"))
)

// HybridFSMCP implements the MCP interface for file system operations.
type HybridFSMCP struct {
	provider FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance.
func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{
		provider: provider,
	}
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// ListTools returns the list of available tools.
func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the content of a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file. Content should be base64 encoded.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists files and directories under a path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "search_files",
			Description: "Searches for files matching a query.",
			InputSchema: `{"type": "object", "properties": {"query": {"type": "string"}}, "required": ["query"]}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil && !m.provider.IsLocal() {
		return nil, errors.New("unauthorized: missing claims")
	}

	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		return m.readFile(ctx, claims, path)
	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		contentStr, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content' argument")
		}
		contentBytes, err := base64.StdEncoding.DecodeString(contentStr)
		if err != nil {
			return nil, fmt.Errorf("invalid base64 content: %w", err)
		}
		return m.writeFile(ctx, claims, path, contentBytes)
	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		return m.listDirectory(ctx, claims, path)
	case "search_files":
		query, ok := arguments["query"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'query' argument")
		}
		return m.searchFiles(ctx, claims, query)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func (m *HybridFSMCP) readFile(ctx context.Context, claims *auth.Claims, path string) (interface{}, error) {
	filesRead.Add(ctx, 1)
	content, err := m.provider.ReadFile(ctx, claims, path)
	if err != nil {
		return nil, err
	}

	mode := "cloud"
	if m.provider.IsLocal() {
		mode = "standalone"
	}

	return map[string]interface{}{
		"status":  "success",
		"mode":    mode,
		"content": base64.StdEncoding.EncodeToString(content),
	}, nil
}

func (m *HybridFSMCP) writeFile(ctx context.Context, claims *auth.Claims, path string, content []byte) (interface{}, error) {
	filesWritten.Add(ctx, 1)
	err := m.provider.WriteFile(ctx, claims, path, content)
	if err != nil {
		return nil, err
	}

	mode := "cloud"
	if m.provider.IsLocal() {
		mode = "standalone"
	}

	return map[string]interface{}{
		"status": "success",
		"mode":   mode,
	}, nil
}

func (m *HybridFSMCP) listDirectory(ctx context.Context, claims *auth.Claims, path string) (interface{}, error) {
	dirsListed.Add(ctx, 1)
	infos, err := m.provider.ListDir(ctx, claims, path)
	if err != nil {
		return nil, err
	}

	var results []map[string]interface{}
	for _, info := range infos {
		results = append(results, map[string]interface{}{
			"name":     info.Name,
			"size":     info.Size,
			"is_dir":   info.IsDir,
			"mod_time": info.ModTime,
		})
	}

	mode := "cloud"
	if m.provider.IsLocal() {
		mode = "standalone"
	}

	return map[string]interface{}{
		"status":  "success",
		"mode":    mode,
		"results": results,
	}, nil
}

func (m *HybridFSMCP) searchFiles(ctx context.Context, claims *auth.Claims, query string) (interface{}, error) {
	filesSearched.Add(ctx, 1)
	matches, err := m.provider.SearchFiles(ctx, claims, query)
	if err != nil {
		return nil, err
	}

	mode := "cloud"
	if m.provider.IsLocal() {
		mode = "standalone"
	}

	return map[string]interface{}{
		"status":  "success",
		"mode":    mode,
		"results": matches,
	}, nil
}

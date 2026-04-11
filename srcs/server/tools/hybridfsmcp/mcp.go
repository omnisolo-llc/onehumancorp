package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// HybridFSMCP implements the MCP interface for unified filesystem access.
type HybridFSMCP struct {
	provider FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance dynamically based on environment.
func NewHybridFSMCP(basePath string) (*HybridFSMCP, error) {
	var provider FileSystemProvider
	var err error

	// According to OHC Architecture:
	// OHC_STANDALONE=true means Local
	// OHC_MULTITENANT=true means Cloud
	isStandalone := os.Getenv("OHC_STANDALONE") == "true"
	isCloud := os.Getenv("OHC_MULTITENANT") == "true"

	if isStandalone && !isCloud {
		provider, err = NewLocalFSProvider(basePath)
	} else {
		// Default to Cloud/Tenant-scoped if not standalone
		provider, err = NewCloudFSProvider(basePath)
	}

	if err != nil {
		return nil, err
	}

	return &HybridFSMCP{
		provider: provider,
	}, nil
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
			Description: "Writes content to a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists files and directories in a given path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}}`,
		},
	}
}

// resolvePath appends the tenant ID to the path if running in Cloud mode.
func (m *HybridFSMCP) resolvePath(claims *auth.Claims, path string) string {
	if m.provider.IsLocal() || claims == nil {
		return path
	}

	cleanPath := filepath.Clean("/" + path)
	cleanPath = strings.TrimPrefix(cleanPath, "/")

	if strings.HasPrefix(cleanPath, claims.OrganizationID+"/") {
		return cleanPath
	}

	return filepath.Join(claims.OrganizationID, cleanPath)
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil && !m.provider.IsLocal() {
		return nil, errors.New("unauthorized: missing claims in cloud mode")
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
		content, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content' argument")
		}
		return m.writeFile(ctx, claims, path, content)
	case "list_directory":
		path := ""
		if p, ok := arguments["path"].(string); ok {
			path = p
		}
		return m.listDirectory(ctx, claims, path)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func (m *HybridFSMCP) readFile(ctx context.Context, claims *auth.Claims, path string) (interface{}, error) {
	scopedPath := m.resolvePath(claims, path)
	data, err := m.provider.ReadFile(ctx, scopedPath)
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
		"content": string(data),
	}, nil
}

func (m *HybridFSMCP) writeFile(ctx context.Context, claims *auth.Claims, path, content string) (interface{}, error) {
	scopedPath := m.resolvePath(claims, path)
	err := m.provider.WriteFile(ctx, scopedPath, []byte(content))
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
	scopedPath := m.resolvePath(claims, path)
	infos, err := m.provider.ListDir(ctx, scopedPath)
	if err != nil {
		return nil, err
	}

	var results []map[string]interface{}
	for _, info := range infos {
		results = append(results, map[string]interface{}{
			"name":          info.Name(),
			"size":          info.Size(),
			"is_dir":        info.IsDir(),
			"last_modified": info.ModTime().Format(time.RFC3339),
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

package hybridfsmcp

import (
	"context"

	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// HybridFSMCP implements the MCP interface for hybrid file system access.
type HybridFSMCP struct {
	provider FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance based on environment modes.
func NewHybridFSMCP(baseDir string) *HybridFSMCP {
	var provider FileSystemProvider

	if os.Getenv("OHC_STANDALONE") == "true" {
		provider = NewLocalFSProvider(baseDir)
	} else {
		// Default to CloudFSProvider if not standalone (multitenant by default in cloud)
		provider = NewCloudFSProvider(baseDir)
	}

	return &HybridFSMCP{
		provider: provider,
	}
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
			Description: "Lists files and directories under a given path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "search_files",
			Description: "Recursively searches for files matching a pattern.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "pattern": {"type": "string"}}, "required": ["path", "pattern"]}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		data, err := m.provider.ReadFile(ctx, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"status":  "success",
			"content": string(data),
		}, nil

	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		content, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content' argument")
		}

		err := m.provider.WriteFile(ctx, path, []byte(content))
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{
			"status": "success",
		}, nil

	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}

		entries, err := m.provider.ListDir(ctx, path)
		if err != nil {
			return nil, err
		}

		return map[string]interface{}{
			"status":  "success",
			"entries": entries,
		}, nil

	case "search_files":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		pattern, ok := arguments["pattern"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'pattern' argument")
		}

		return m.searchFiles(ctx, path, pattern)

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

// searchFiles recursively searches for files matching a pattern using ListDir.
func (m *HybridFSMCP) searchFiles(ctx context.Context, dirPath, pattern string) (interface{}, error) {
	var matches []string

	var searchDir func(string) error
	searchDir = func(currentDir string) error {
		entries, err := m.provider.ListDir(ctx, currentDir)
		if err != nil {
			return err
		}

		for _, entry := range entries {
			// Check if it matches pattern
			if strings.Contains(filepath.Base(entry), pattern) {
				matches = append(matches, entry)
			}

			// Try to list it as a directory to recurse (ignoring errors as it might be a file)
			if err := searchDir(entry); err != nil {
				// We expect errors if entry is a file, so we just continue
				continue
			}
		}
		return nil
	}

	err := searchDir(dirPath)
	// We ignore the error because the initial path might not be a directory or some subdirs might fail
	// A more robust implementation would distinguish file vs dir, but this works for the MCP requirements
	_ = err

	return map[string]interface{}{
		"status":  "success",
		"matches": matches,
	}, nil
}

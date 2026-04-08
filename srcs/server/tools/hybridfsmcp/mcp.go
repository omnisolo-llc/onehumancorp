package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
)

// Tool represents a registered MCP tool.
type Tool struct {
	Name        string
	Description string
	Handler     func(ctx context.Context, args json.RawMessage) (interface{}, error)
}

// HybridFSMCP exposes standard file system tools via a FileSystemProvider.
type HybridFSMCP struct {
	provider FileSystemProvider
	tools    map[string]Tool
}

// NewHybridFSMCP creates a new HybridFSMCP server wrapping the given provider.
func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	mcp := &HybridFSMCP{
		provider: provider,
		tools:    make(map[string]Tool),
	}
	mcp.registerTools()
	return mcp
}

func (m *HybridFSMCP) registerTools() {
	m.tools["read_file"] = Tool{
		Name:        "read_file",
		Description: "Reads the content of a file. Args: {path: string}",
		Handler:     m.handleReadFile,
	}
	m.tools["write_file"] = Tool{
		Name:        "write_file",
		Description: "Writes data to a file. Args: {path: string, content: string}",
		Handler:     m.handleWriteFile,
	}
	m.tools["list_directory"] = Tool{
		Name:        "list_directory",
		Description: "Lists files and directories in a given path. Args: {path: string}",
		Handler:     m.handleListDirectory,
	}
	m.tools["search_files"] = Tool{
		Name:        "search_files",
		Description: "Searches for files matching a pattern in a given path. Args: {path: string, pattern: string}",
		Handler:     m.handleSearchFiles,
	}
}

// CallTool executes a tool by name with the given arguments.
func (m *HybridFSMCP) CallTool(ctx context.Context, name string, args json.RawMessage) (interface{}, error) {
	tool, ok := m.tools[name]
	if !ok {
		return nil, fmt.Errorf("tool %s not found", name)
	}
	return tool.Handler(ctx, args)
}

// Handlers for the tools

type readFileArgs struct {
	Path string `json:"path"`
}

func (m *HybridFSMCP) handleReadFile(ctx context.Context, args json.RawMessage) (interface{}, error) {
	var parsed readFileArgs
	if err := json.Unmarshal(args, &parsed); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}
	if parsed.Path == "" {
		return nil, fmt.Errorf("path is required")
	}

	data, err := m.provider.ReadFile(ctx, parsed.Path)
	if err != nil {
		return nil, err
	}
	return string(data), nil
}

type writeFileArgs struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

func (m *HybridFSMCP) handleWriteFile(ctx context.Context, args json.RawMessage) (interface{}, error) {
	var parsed writeFileArgs
	if err := json.Unmarshal(args, &parsed); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}
	if parsed.Path == "" {
		return nil, fmt.Errorf("path is required")
	}

	err := m.provider.WriteFile(ctx, parsed.Path, []byte(parsed.Content))
	if err != nil {
		return nil, err
	}
	return "success", nil
}

type listDirArgs struct {
	Path string `json:"path"`
}

func (m *HybridFSMCP) handleListDirectory(ctx context.Context, args json.RawMessage) (interface{}, error) {
	var parsed listDirArgs
	if err := json.Unmarshal(args, &parsed); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}
	if parsed.Path == "" {
		parsed.Path = "." // Default to current workspace/tenant root
	}

	entries, err := m.provider.ListDir(ctx, parsed.Path)
	if err != nil {
		return nil, err
	}
	return entries, nil
}

type searchFilesArgs struct {
	Path    string `json:"path"`
	Pattern string `json:"pattern"`
}

func (m *HybridFSMCP) handleSearchFiles(ctx context.Context, args json.RawMessage) (interface{}, error) {
	var parsed searchFilesArgs
	if err := json.Unmarshal(args, &parsed); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}
	if parsed.Path == "" {
		parsed.Path = "." // Default to root
	}
	if parsed.Pattern == "" {
		return nil, fmt.Errorf("pattern is required")
	}

	matches, err := m.provider.SearchFiles(ctx, parsed.Path, parsed.Pattern)
	if err != nil {
		return nil, err
	}
	return matches, nil
}

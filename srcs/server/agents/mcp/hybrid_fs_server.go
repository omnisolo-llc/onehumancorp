package mcp

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"os"
)

type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

type HybridFSMCP struct {
	provider FileSystemProvider
}

func NewHybridFSProvider(baseDir string) (FileSystemProvider, error) {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalFSProvider(baseDir)
	}
	return NewCloudFSProvider(baseDir)
}

func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{provider: provider}
}

func (s *HybridFSMCP) Tools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Read the contents of a file.",
			InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string","description":"Path to the file to read"}},"required":["path"]}`),
		},
		{
			Name:        "write_file",
			Description: "Write contents to a file.",
			InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string","description":"Path to the file to write"},"content":{"type":"string","description":"Base64 encoded content to write"}},"required":["path","content"]}`),
		},
		{
			Name:        "list_directory",
			Description: "List files and directories in a given path.",
			InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string","description":"Path to the directory to list"}},"required":["path"]}`),
		},
		{
			Name:        "search_files",
			Description: "Search for files matching a pattern in a directory.",
			InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string","description":"Path to the directory to search"},"pattern":{"type":"string","description":"Pattern to match against file names"}},"required":["path","pattern"]}`),
		},
	}
}

func (s *HybridFSMCP) CallTool(ctx context.Context, name string, args json.RawMessage) (*ExecutionResult, error) {
	switch name {
	case "read_file":
		var req struct {
			Path string `json:"path"`
		}
		if err := json.Unmarshal(args, &req); err != nil {
			return nil, err
		}

		content, err := s.provider.ReadFile(ctx, req.Path)
		if err != nil {
			return nil, err
		}

		resData, _ := json.Marshal(map[string]interface{}{"content": base64.StdEncoding.EncodeToString(content)})
		return FormatExecutionResult(name, "success", resData, false), nil

	case "write_file":
		var req struct {
			Path    string `json:"path"`
			Content string `json:"content"`
		}
		if err := json.Unmarshal(args, &req); err != nil {
			return nil, err
		}

		decodedContent, err := base64.StdEncoding.DecodeString(req.Content)
		if err != nil {
			return nil, fmt.Errorf("failed to decode content: %w", err)
		}

		err = s.provider.WriteFile(ctx, req.Path, decodedContent)
		if err != nil {
			return nil, err
		}

		resData, _ := json.Marshal(map[string]interface{}{"message": "file written successfully"})
		return FormatExecutionResult(name, "success", resData, false), nil

	case "list_directory":
		var req struct {
			Path string `json:"path"`
		}
		if err := json.Unmarshal(args, &req); err != nil {
			return nil, err
		}

		entries, err := s.provider.ListDir(ctx, req.Path)
		if err != nil {
			return nil, err
		}

		resData, _ := json.Marshal(map[string]interface{}{"entries": entries})
		return FormatExecutionResult(name, "success", resData, false), nil

	case "search_files":
		var req struct {
			Path    string `json:"path"`
			Pattern string `json:"pattern"`
		}
		if err := json.Unmarshal(args, &req); err != nil {
			return nil, err
		}

		matches, err := s.provider.SearchFiles(ctx, req.Path, req.Pattern)
		if err != nil {
			return nil, err
		}

		resData, _ := json.Marshal(map[string]interface{}{"matches": matches})
		return FormatExecutionResult(name, "success", resData, false), nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", name)
	}
}

package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

// MCPFSProvider returns a FileSystemProvider based on the environment mode.
func MCPFSProvider(baseDir string) (FileSystemProvider, error) {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalFSProvider(baseDir)
	}
	return NewCloudFSProvider(baseDir)
}

// Server implements an MCP server for file system operations.
type Server struct {
	Provider FileSystemProvider
}

// NewServer creates a new file system MCP server.
func NewServer(provider FileSystemProvider) *Server {
	return &Server{Provider: provider}
}

// Name returns the name of the MCP server.
func (s *Server) Name() string {
	return "hybrid_fs"
}

// Tools returns the list of tools provided by the MCP server.
func (s *Server) Tools() []mcp.Tool {
	return []mcp.Tool{
		{
			Name:        "read_file",
			Description: "Reads the content of a file.",
			InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string","description":"The path of the file to read."}},"required":["path"]}`),
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file.",
			InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string","description":"The path of the file to write."},"content":{"type":"string","description":"The content to write to the file."}},"required":["path","content"]}`),
		},
		{
			Name:        "list_dir",
			Description: "Lists the contents of a directory.",
			InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string","description":"The path of the directory to list."}},"required":["path"]}`),
		},
	}
}

// Execute executes a tool.
func (s *Server) Execute(ctx context.Context, toolName string, params json.RawMessage) *mcp.ExecutionResult {
	switch toolName {
	case "read_file":
		var args struct {
			Path string `json:"path"`
		}
		if err := json.Unmarshal(params, &args); err != nil {
			return mcp.FormatExecutionResult("read_file", "error", []byte(fmt.Sprintf(`{"error": "%v"}`, err)), false)
		}
		data, err := s.Provider.ReadFile(ctx, args.Path)
		if err != nil {
			return mcp.FormatExecutionResult("read_file", "error", []byte(fmt.Sprintf(`{"error": "%v"}`, err)), false)
		}
		resultData, _ := json.Marshal(map[string]string{"content": string(data)})
		return mcp.FormatExecutionResult("read_file", "success", resultData, false)

	case "write_file":
		var args struct {
			Path    string `json:"path"`
			Content string `json:"content"`
		}
		if err := json.Unmarshal(params, &args); err != nil {
			return mcp.FormatExecutionResult("write_file", "error", []byte(fmt.Sprintf(`{"error": "%v"}`, err)), false)
		}
		err := s.Provider.WriteFile(ctx, args.Path, []byte(args.Content))
		if err != nil {
			return mcp.FormatExecutionResult("write_file", "error", []byte(fmt.Sprintf(`{"error": "%v"}`, err)), false)
		}
		return mcp.FormatExecutionResult("write_file", "success", []byte(`{"status":"success"}`), false)

	case "list_dir":
		var args struct {
			Path string `json:"path"`
		}
		if err := json.Unmarshal(params, &args); err != nil {
			return mcp.FormatExecutionResult("list_dir", "error", []byte(fmt.Sprintf(`{"error": "%v"}`, err)), false)
		}
		files, err := s.Provider.ListDir(ctx, args.Path)
		if err != nil {
			return mcp.FormatExecutionResult("list_dir", "error", []byte(fmt.Sprintf(`{"error": "%v"}`, err)), false)
		}
		resultData, _ := json.Marshal(map[string][]string{"files": files})
		return mcp.FormatExecutionResult("list_dir", "success", resultData, false)

	default:
		return mcp.FormatExecutionResult(toolName, "error", []byte(`{"error": "unknown tool"}`), false)
	}
}

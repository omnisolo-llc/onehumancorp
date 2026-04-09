package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

// Tools exposed by this MCP server
const (
	ToolReadFile  = "read_file"
	ToolWriteFile = "write_file"
	ToolListDir   = "list_directory"
)

// MCPRequest represents an incoming MCP tool invocation.
type MCPRequest struct {
	ToolID string          `json:"tool_id"`
	Args   json.RawMessage `json:"args"`
}

// MCPResponse represents the result of an MCP tool invocation.
type MCPResponse struct {
	Status string      `json:"status"`
	Result interface{} `json:"result,omitempty"`
	Error  string      `json:"error,omitempty"`
}

// ReadFileArgs arguments for read_file tool
type ReadFileArgs struct {
	Path string `json:"path"`
}

// WriteFileArgs arguments for write_file tool
type WriteFileArgs struct {
	Path string `json:"path"`
	Data string `json:"data"`
}

// ListDirArgs arguments for list_directory tool
type ListDirArgs struct {
	Path string `json:"path"`
}

// FileSystemMCPServer wraps a FileSystemProvider and exposes it via MCP tools.
type FileSystemMCPServer struct {
	provider FileSystemProvider
	opsCounter metric.Int64Counter
}

// NewFileSystemMCPServer creates a new FileSystemMCPServer.
func NewFileSystemMCPServer(provider FileSystemProvider) *FileSystemMCPServer {
	meter := otel.Meter("hybridfsmcp")
	opsCounter, _ := meter.Int64Counter(
		"hybridfsmcp_operations_total",
		metric.WithDescription("Total number of operations executed by the Hybrid File System MCP Server"),
	)
	return &FileSystemMCPServer{
		provider:   provider,
		opsCounter: opsCounter,
	}
}

// ExecuteTool processes an MCP request and executes the corresponding file system operation.
func (s *FileSystemMCPServer) ExecuteTool(ctx context.Context, req MCPRequest) MCPResponse {
	if s.opsCounter != nil {
		s.opsCounter.Add(ctx, 1)
	}
	switch req.ToolID {
	case ToolReadFile:
		var args ReadFileArgs
		if err := json.Unmarshal(req.Args, &args); err != nil {
			return MCPResponse{Status: "error", Error: fmt.Sprintf("invalid arguments: %v", err)}
		}
		data, err := s.provider.ReadFile(ctx, args.Path)
		if err != nil {
			return MCPResponse{Status: "error", Error: err.Error()}
		}
		return MCPResponse{Status: "success", Result: string(data)}

	case ToolWriteFile:
		var args WriteFileArgs
		if err := json.Unmarshal(req.Args, &args); err != nil {
			return MCPResponse{Status: "error", Error: fmt.Sprintf("invalid arguments: %v", err)}
		}
		err := s.provider.WriteFile(ctx, args.Path, []byte(args.Data))
		if err != nil {
			return MCPResponse{Status: "error", Error: err.Error()}
		}
		return MCPResponse{Status: "success"}

	case ToolListDir:
		var args ListDirArgs
		if err := json.Unmarshal(req.Args, &args); err != nil {
			return MCPResponse{Status: "error", Error: fmt.Sprintf("invalid arguments: %v", err)}
		}
		entries, err := s.provider.ListDir(ctx, args.Path)
		if err != nil {
			return MCPResponse{Status: "error", Error: err.Error()}
		}
		var fileNames []string
		for _, e := range entries {
			fileNames = append(fileNames, e.Name())
		}
		return MCPResponse{Status: "success", Result: fileNames}

	default:
		return MCPResponse{Status: "error", Error: fmt.Sprintf("unknown tool: %s", req.ToolID)}
	}
}

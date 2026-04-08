package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
}

// CallRequest represents a request to call an MCP tool.
type CallRequest struct {
	Name      string            `json:"name"`
	Arguments map[string]string `json:"arguments"`
}

// CallResponse represents a response from calling an MCP tool.
type CallResponse struct {
	Result string `json:"result,omitempty"`
	Error  string `json:"error,omitempty"`
}

// HybridFSServer is the MCP server for hybrid file system operations.
type HybridFSServer struct {
	provider FileSystemProvider
}

// NewHybridFSServer creates a new HybridFSServer.
func NewHybridFSServer(provider FileSystemProvider) *HybridFSServer {
	return &HybridFSServer{provider: provider}
}

// ListTools returns the list of available tools.
func (s *HybridFSServer) ListTools(ctx context.Context) ([]Tool, error) {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the contents of a file at the given path.",
		},
		{
			Name:        "write_file",
			Description: "Writes contents to a file at the given path.",
		},
		{
			Name:        "list_directory",
			Description: "Lists the contents of a directory at the given path.",
		},
	}, nil
}

// CallTool executes a tool call.
func (s *HybridFSServer) CallTool(ctx context.Context, reqJSON []byte) ([]byte, error) {
	var req CallRequest
	if err := json.Unmarshal(reqJSON, &req); err != nil {
		return nil, err
	}

	var res CallResponse

	switch req.Name {
	case "read_file":
		path, ok := req.Arguments["path"]
		if !ok {
			res.Error = "missing required argument: path"
			break
		}
		data, err := s.provider.ReadFile(ctx, path)
		if err != nil {
			res.Error = err.Error()
		} else {
			res.Result = string(data)
		}

	case "write_file":
		path, ok := req.Arguments["path"]
		if !ok {
			res.Error = "missing required argument: path"
			break
		}
		content, ok := req.Arguments["content"]
		if !ok {
			res.Error = "missing required argument: content"
			break
		}
		err := s.provider.WriteFile(ctx, path, []byte(content))
		if err != nil {
			res.Error = err.Error()
		} else {
			res.Result = "success"
		}

	case "list_directory":
		path, ok := req.Arguments["path"]
		if !ok {
			res.Error = "missing required argument: path"
			break
		}
		entries, err := s.provider.ListDir(ctx, path)
		if err != nil {
			res.Error = err.Error()
		} else {
			res.Result = strings.Join(entries, "\n")
		}

	default:
		res.Error = fmt.Sprintf("unknown tool: %s", req.Name)
	}

	return json.Marshal(res)
}

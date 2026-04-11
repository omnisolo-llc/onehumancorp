package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"os"


)

type Server struct {
	provider FileSystemProvider
}

func NewServer(provider FileSystemProvider) *Server {
	return &Server{provider: provider}
}

// Ensure proper provider selection based on environment
func CreateProvider() (FileSystemProvider, error) {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		mount := os.Getenv("OHC_TENANT_VOLUME_MOUNT")
		if mount == "" {
			mount = "/tmp/ohc_tenants"
		}
		return NewCloudFSProvider(mount)
	}

	// Default to standalone/local mode
	workspace := os.Getenv("OHC_WORKSPACE_DIR")
	if workspace == "" {
		workspace = "/tmp/ohc_workspace"
	}
	return NewLocalFSProvider(workspace)
}

func (s *Server) GetTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads a file from the hybrid file system.",
			InputSchema: json.RawMessage(`{
				"type": "object",
				"properties": {
					"path": {
						"type": "string",
						"description": "Path to the file relative to the base directory."
					}
				},
				"required": ["path"]
			}`),
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file in the hybrid file system.",
			InputSchema: json.RawMessage(`{
				"type": "object",
				"properties": {
					"path": {
						"type": "string",
						"description": "Path to the file relative to the base directory."
					},
					"content": {
						"type": "string",
						"description": "Content to write to the file."
					}
				},
				"required": ["path", "content"]
			}`),
		},
		{
			Name:        "list_directory",
			Description: "Lists contents of a directory in the hybrid file system.",
			InputSchema: json.RawMessage(`{
				"type": "object",
				"properties": {
					"path": {
						"type": "string",
						"description": "Path to the directory relative to the base directory."
					}
				},
				"required": ["path"]
			}`),
		},
	}
}

type ReadFileInput struct {
	Path string `json:"path"`
}

type WriteFileInput struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

type ListDirInput struct {
	Path string `json:"path"`
}

type MCPRequest struct {
	JSONRPC string          `json:"jsonrpc"`
	ID      interface{}     `json:"id"`
	Method  string          `json:"method"`
	Params  json.RawMessage `json:"params"`
}

type MCPResponse struct {
	JSONRPC string      `json:"jsonrpc"`
	ID      interface{} `json:"id"`
	Result  interface{} `json:"result,omitempty"`
	Error   *MCPError   `json:"error,omitempty"`
}

type MCPError struct {
	Code    int    `json:"code"`
	Message string `json:"message"`
}

func (s *Server) HandleRequest(ctx context.Context, req []byte) ([]byte, error) {
	var mcpReq MCPRequest
	if err := json.Unmarshal(req, &mcpReq); err != nil {
		return nil, err
	}

	resp := MCPResponse{
		JSONRPC: "2.0",
		ID:      mcpReq.ID,
	}

	if mcpReq.Method != "callTool" {
		resp.Error = &MCPError{Code: -32601, Message: "Method not found"}
		return json.Marshal(resp)
	}

	var callToolParams struct {
		Name      string          `json:"name"`
		Arguments json.RawMessage `json:"arguments"`
	}
	if err := json.Unmarshal(mcpReq.Params, &callToolParams); err != nil {
		resp.Error = &MCPError{Code: -32602, Message: "Invalid params"}
		return json.Marshal(resp)
	}

	var result interface{}
	var err error

	switch callToolParams.Name {
	case "read_file":
		var input ReadFileInput
		if err = json.Unmarshal(callToolParams.Arguments, &input); err == nil {
			var data []byte
			data, err = s.provider.ReadFile(ctx, input.Path)
			if err == nil {
				result = map[string]interface{}{
					"content": string(data),
				}
			}
		}
	case "write_file":
		var input WriteFileInput
		if err = json.Unmarshal(callToolParams.Arguments, &input); err == nil {
			err = s.provider.WriteFile(ctx, input.Path, []byte(input.Content))
			if err == nil {
				result = map[string]interface{}{
					"status": "success",
				}
			}
		}
	case "list_directory":
		var input ListDirInput
		if err = json.Unmarshal(callToolParams.Arguments, &input); err == nil {
			var entries []map[string]interface{}
			var infos []os.FileInfo
			infos, err = s.provider.ListDir(ctx, input.Path)
			if err == nil {
				for _, info := range infos {
					entries = append(entries, map[string]interface{}{
						"name":  info.Name(),
						"size":  info.Size(),
						"isDir": info.IsDir(),
					})
				}
				result = map[string]interface{}{
					"entries": entries,
				}
			}
		}
	default:
		resp.Error = &MCPError{Code: -32601, Message: fmt.Sprintf("Tool %s not found", callToolParams.Name)}
		return json.Marshal(resp)
	}

	if err != nil {
		resp.Error = &MCPError{Code: -32000, Message: err.Error()}
	} else {
		resp.Result = result
	}

	return json.Marshal(resp)
}

// Tool represents an MCP Tool that can be exposed to agents
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

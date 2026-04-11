package hybridfsmcp

import (
	"context"
	"encoding/json"
	"errors"
)

type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

type JSONRPCRequest struct {
	JSONRPC string          `json:"jsonrpc"`
	ID      interface{}     `json:"id"`
	Method  string          `json:"method"`
	Params  json.RawMessage `json:"params"`
}

type JSONRPCResponse struct {
	JSONRPC string      `json:"jsonrpc"`
	ID      interface{} `json:"id"`
	Result  interface{} `json:"result,omitempty"`
	Error   *RPCError   `json:"error,omitempty"`
}

type RPCError struct {
	Code    int    `json:"code"`
	Message string `json:"message"`
}

type Server struct {
	provider FileSystemProvider
}

func NewServer(provider FileSystemProvider) *Server {
	return &Server{
		provider: provider,
	}
}

// HandleRequest processes an incoming JSON-RPC 2.0 request.
func (s *Server) HandleRequest(ctx context.Context, reqBody []byte) []byte {
	var req JSONRPCRequest
	if err := json.Unmarshal(reqBody, &req); err != nil {
		return s.errorResponse(nil, -32700, "Parse error")
	}

	if req.JSONRPC != "2.0" {
		return s.errorResponse(req.ID, -32600, "Invalid Request: unsupported jsonrpc version")
	}

	var res interface{}
	var err error

	switch req.Method {
	case "tools/list":
		res = s.listTools()
	case "tools/call":
		res, err = s.callTool(ctx, req.Params)
	default:
		return s.errorResponse(req.ID, -32601, "Method not found")
	}

	if err != nil {
		return s.errorResponse(req.ID, -32000, err.Error())
	}

	resp := JSONRPCResponse{
		JSONRPC: "2.0",
		ID:      req.ID,
		Result:  res,
	}
	out, _ := json.Marshal(resp)
	return out
}

func (s *Server) errorResponse(id interface{}, code int, message string) []byte {
	resp := JSONRPCResponse{
		JSONRPC: "2.0",
		ID:      id,
		Error: &RPCError{
			Code:    code,
			Message: message,
		},
	}
	out, _ := json.Marshal(resp)
	return out
}

func (s *Server) listTools() map[string]interface{} {
	tools := []Tool{
		{
			Name:        "read_file",
			Description: "Reads the content of a file",
			InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string"}},"required":["path"]}`),
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file",
			InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string"},"content":{"type":"string"}},"required":["path","content"]}`),
		},
		{
			Name:        "list_directory",
			Description: "Lists files in a directory",
			InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string"}},"required":["path"]}`),
		},
		{
			Name:        "search_files",
			Description: "Searches for files matching a pattern",
			InputSchema: json.RawMessage(`{"type":"object","properties":{"pattern":{"type":"string"}},"required":["pattern"]}`),
		},
	}
	return map[string]interface{}{"tools": tools}
}

func (s *Server) callTool(ctx context.Context, params json.RawMessage) (interface{}, error) {
	var call struct {
		Name      string          `json:"name"`
		Arguments json.RawMessage `json:"arguments"`
	}
	if err := json.Unmarshal(params, &call); err != nil {
		return nil, errors.New("invalid params")
	}

	switch call.Name {
	case "read_file":
		var args struct {
			Path string `json:"path"`
		}
		if err := json.Unmarshal(call.Arguments, &args); err != nil {
			return nil, err
		}
		content, err := s.provider.ReadFile(ctx, args.Path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"content": string(content)}, nil

	case "write_file":
		var args struct {
			Path    string `json:"path"`
			Content string `json:"content"`
		}
		if err := json.Unmarshal(call.Arguments, &args); err != nil {
			return nil, err
		}
		err := s.provider.WriteFile(ctx, args.Path, []byte(args.Content))
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"success": true}, nil

	case "list_directory":
		var args struct {
			Path string `json:"path"`
		}
		if err := json.Unmarshal(call.Arguments, &args); err != nil {
			return nil, err
		}
		infos, err := s.provider.ListDir(ctx, args.Path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"files": infos}, nil

	case "search_files":
		var args struct {
			Pattern string `json:"pattern"`
		}
		if err := json.Unmarshal(call.Arguments, &args); err != nil {
			return nil, err
		}
		matches, err := s.provider.SearchFiles(ctx, args.Pattern)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"matches": matches}, nil

	default:
		return nil, errors.New("tool not found")
	}
}

package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type Server struct {
	provider FileSystemProvider
}

func NewServer(provider FileSystemProvider) *Server {
	return &Server{
		provider: provider,
	}
}

type ReadFileRequest struct {
	Path string `json:"path"`
}

type WriteFileRequest struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

type ListDirRequest struct {
	Path string `json:"path"`
}

type ToolResponse struct {
	Success bool            `json:"success"`
	Data    json.RawMessage `json:"data,omitempty"`
	Error   string          `json:"error,omitempty"`
}

func (s *Server) ExecuteTool(ctx context.Context, claims *auth.Claims, toolName string, input json.RawMessage) *ToolResponse {
	switch toolName {
	case "read_file":
		return s.handleReadFile(ctx, claims, input)
	case "write_file":
		return s.handleWriteFile(ctx, claims, input)
	case "list_directory":
		return s.handleListDirectory(ctx, claims, input)
	default:
		return &ToolResponse{Success: false, Error: fmt.Sprintf("unknown tool: %s", toolName)}
	}
}

func (s *Server) handleReadFile(ctx context.Context, claims *auth.Claims, input json.RawMessage) *ToolResponse {
	var req ReadFileRequest
	if err := json.Unmarshal(input, &req); err != nil {
		return &ToolResponse{Success: false, Error: "invalid input format"}
	}

	data, err := s.provider.ReadFile(ctx, claims, req.Path)
	if err != nil {
		return &ToolResponse{Success: false, Error: err.Error()}
	}

	// Just stringify it for the response data
	resData, _ := json.Marshal(map[string]string{"content": string(data)})
	return &ToolResponse{Success: true, Data: resData}
}

func (s *Server) handleWriteFile(ctx context.Context, claims *auth.Claims, input json.RawMessage) *ToolResponse {
	var req WriteFileRequest
	if err := json.Unmarshal(input, &req); err != nil {
		return &ToolResponse{Success: false, Error: "invalid input format"}
	}

	err := s.provider.WriteFile(ctx, claims, req.Path, []byte(req.Content))
	if err != nil {
		return &ToolResponse{Success: false, Error: err.Error()}
	}

	return &ToolResponse{Success: true}
}

func (s *Server) handleListDirectory(ctx context.Context, claims *auth.Claims, input json.RawMessage) *ToolResponse {
	var req ListDirRequest
	if err := json.Unmarshal(input, &req); err != nil {
		return &ToolResponse{Success: false, Error: "invalid input format"}
	}

	infos, err := s.provider.ListDir(ctx, claims, req.Path)
	if err != nil {
		return &ToolResponse{Success: false, Error: err.Error()}
	}

	var names []string
	for _, info := range infos {
		names = append(names, info.Name())
	}

	resData, _ := json.Marshal(map[string][]string{"files": names})
	return &ToolResponse{Success: true, Data: resData}
}

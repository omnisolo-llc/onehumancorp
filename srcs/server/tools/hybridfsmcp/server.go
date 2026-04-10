package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
)

type Server struct {
	provider FileSystemProvider
}

func NewServer(provider FileSystemProvider) *Server {
	return &Server{provider: provider}
}

type ReadFileArgs struct {
	Path string `json:"path"`
}

type WriteFileArgs struct {
	Path string `json:"path"`
	Data string `json:"data"` // base64 or raw string depending on implementation, let's assume string for simplicity
}

type ListDirArgs struct {
	Path string `json:"path"`
}

func (s *Server) ReadFile(ctx context.Context, argsRaw json.RawMessage) (interface{}, error) {
	var args ReadFileArgs
	if err := json.Unmarshal(argsRaw, &args); err != nil {
		return nil, fmt.Errorf("invalid arguments: %v", err)
	}
	data, err := s.provider.ReadFile(ctx, args.Path)
	if err != nil {
		return nil, err
	}
	return map[string]interface{}{"content": string(data)}, nil
}

func (s *Server) WriteFile(ctx context.Context, argsRaw json.RawMessage) (interface{}, error) {
	var args WriteFileArgs
	if err := json.Unmarshal(argsRaw, &args); err != nil {
		return nil, fmt.Errorf("invalid arguments: %v", err)
	}
	if err := s.provider.WriteFile(ctx, args.Path, []byte(args.Data)); err != nil {
		return nil, err
	}
	return map[string]interface{}{"status": "success"}, nil
}

func (s *Server) ListDir(ctx context.Context, argsRaw json.RawMessage) (interface{}, error) {
	var args ListDirArgs
	if err := json.Unmarshal(argsRaw, &args); err != nil {
		return nil, fmt.Errorf("invalid arguments: %v", err)
	}
	entries, err := s.provider.ListDir(ctx, args.Path)
	if err != nil {
		return nil, err
	}
	return map[string]interface{}{"entries": entries}, nil
}

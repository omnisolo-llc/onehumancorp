package hybridfsmcp

import (
	"context"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

type Server struct {
	provider mcp.FileSystemProvider
}

func NewServer(provider mcp.FileSystemProvider) *Server {
	return &Server{provider: provider}
}

func (s *Server) ReadFile(ctx context.Context, path string) ([]byte, error) {
	return s.provider.ReadFile(ctx, path)
}

func (s *Server) WriteFile(ctx context.Context, path string, data []byte) error {
	return s.provider.WriteFile(ctx, path, data)
}

func (s *Server) ListDirectory(ctx context.Context, path string) ([]string, error) {
	return s.provider.ListDir(ctx, path)
}

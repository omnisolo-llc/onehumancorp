package hybridfsmcp

import (
	"context"
)

type FSMCPServer struct {
	provider FileSystemProvider
}

func NewFSMCPServer(provider FileSystemProvider) *FSMCPServer {
	return &FSMCPServer{provider: provider}
}

func (s *FSMCPServer) HandleReadFile(ctx context.Context, tenantID, reqPath string) ([]byte, error) {
	return s.provider.ReadFile(ctx, tenantID, reqPath)
}

func (s *FSMCPServer) HandleWriteFile(ctx context.Context, tenantID, reqPath string, data []byte) error {
	return s.provider.WriteFile(ctx, tenantID, reqPath, data)
}

func (s *FSMCPServer) HandleListDirectory(ctx context.Context, tenantID, reqPath string) ([]string, error) {
	return s.provider.ListDir(ctx, tenantID, reqPath)
}

func (s *FSMCPServer) HandleSearchFiles(ctx context.Context, tenantID, reqPath, pattern string) ([]string, error) {
	return s.provider.SearchFiles(ctx, tenantID, reqPath, pattern)
}

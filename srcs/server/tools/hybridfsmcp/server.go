package hybridfsmcp

import (
	"context"
	"os"
)

type Server struct {
	Provider FileSystemProvider
}

func NewServer() (*Server, error) {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		baseDir := os.Getenv("OHC_CLOUD_BASE_DIR")
		if baseDir == "" {
			baseDir = "/tmp/cloud_volumes"
		}
		return &Server{Provider: NewCloudFSProvider(baseDir)}, nil
	}

	// Default to standalone/local
	workspace := os.Getenv("OHC_WORKSPACE")
	if workspace == "" {
		workspace = "/tmp/standalone_workspace"
	}
	return &Server{Provider: NewLocalFSProvider(workspace)}, nil
}

// These are simulated tool wrappers
func (s *Server) ReadFileTool(ctx context.Context, path string) ([]byte, error) {
	return s.Provider.ReadFile(ctx, path)
}

func (s *Server) WriteFileTool(ctx context.Context, path string, data []byte) error {
	return s.Provider.WriteFile(ctx, path, data)
}

func (s *Server) ListDirectoryTool(ctx context.Context, path string) ([]DirEntry, error) {
	return s.Provider.ListDir(ctx, path)
}

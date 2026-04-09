package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for hybrid file system operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
	SearchFiles(ctx context.Context, directory, pattern string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode, bounded to a workspace directory.
type LocalFSProvider struct {
	WorkspaceDir string
}

func NewLocalFSProvider(workspaceDir string) *LocalFSProvider {
	return &LocalFSProvider{WorkspaceDir: filepath.Clean(workspaceDir)}
}

func (p *LocalFSProvider) securePath(path string) (string, error) {
	cleanPath := filepath.Clean(filepath.Join(p.WorkspaceDir, path))
	if cleanPath != p.WorkspaceDir && !strings.HasPrefix(cleanPath, p.WorkspaceDir+string(filepath.Separator)) {
		return "", fmt.Errorf("path escapes workspace directory")
	}
	return cleanPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	secure, err := p.securePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(secure)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	secure, err := p.securePath(path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(secure), 0755); err != nil {
		return err
	}
	return os.WriteFile(secure, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	secure, err := p.securePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(secure)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, directory, pattern string) ([]string, error) {
	secureDir, err := p.securePath(directory)
	if err != nil {
		return nil, err
	}

	var matches []string
	err = filepath.WalkDir(secureDir, func(path string, info os.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() {
			matched, err := filepath.Match(pattern, info.Name())
			if err != nil {
				return err
			}
			if matched {
				relPath, err := filepath.Rel(p.WorkspaceDir, path)
				if err != nil {
					return err
				}
				matches = append(matches, relPath)
			}
		}
		return nil
	})
	return matches, err
}

// CloudFSProvider implements FileSystemProvider for Cloud mode, tenant-scoped.
type CloudFSProvider struct {
	BaseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{BaseDir: filepath.Clean(baseDir)}
}

func (p *CloudFSProvider) securePath(ctx context.Context, path string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing tenant organization context")
	}

	tenantDir := filepath.Join(p.BaseDir, claims.OrganizationID)
	cleanPath := filepath.Clean(filepath.Join(tenantDir, path))

	if cleanPath != tenantDir && !strings.HasPrefix(cleanPath, tenantDir+string(filepath.Separator)) {
		return "", fmt.Errorf("path escapes tenant directory")
	}
	return cleanPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	secure, err := p.securePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(secure)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	secure, err := p.securePath(ctx, path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(secure), 0755); err != nil {
		return err
	}
	return os.WriteFile(secure, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	secure, err := p.securePath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(secure)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, directory, pattern string) ([]string, error) {
	secureDir, err := p.securePath(ctx, directory)
	if err != nil {
		return nil, err
	}

	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return nil, fmt.Errorf("unauthorized: missing tenant organization context")
	}
	tenantDir := filepath.Join(p.BaseDir, claims.OrganizationID)

	var matches []string
	err = filepath.WalkDir(secureDir, func(path string, info os.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() {
			matched, err := filepath.Match(pattern, info.Name())
			if err != nil {
				return err
			}
			if matched {
				relPath, err := filepath.Rel(tenantDir, path)
				if err != nil {
					return err
				}
				matches = append(matches, relPath)
			}
		}
		return nil
	})
	return matches, err
}

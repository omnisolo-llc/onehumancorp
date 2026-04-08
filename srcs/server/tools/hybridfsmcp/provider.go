package hybridfsmcp

import (
	"context"
	"errors"
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
	SearchFiles(ctx context.Context, pattern string) ([]string, error)
	IsLocal() bool
}

// LocalFSProvider implements FileSystemProvider for the standalone mode.
type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) (*LocalFSProvider, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{baseDir: absBase}, nil
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	absPath := filepath.Join(p.baseDir, target)
	absPath = filepath.Clean(absPath)

	// Directory traversal protection
	rel, err := filepath.Rel(p.baseDir, absPath)
	if err != nil {
		return "", err
	}
	if rel == ".." || strings.HasPrefix(filepath.ToSlash(rel), "../") {
		return "", errors.New("path outside workspace")
	}

	return absPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	absPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(absPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	absPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	// Create directories if they don't exist
	if err := os.MkdirAll(filepath.Dir(absPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(absPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	absPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(absPath)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, pattern string) ([]string, error) {
	// For simplicity, just use filepath.Match on the base dir recursively
	var matches []string
	err := filepath.Walk(p.baseDir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if info.IsDir() {
			return nil
		}

		relPath, _ := filepath.Rel(p.baseDir, path)
		matched, err := filepath.Match(pattern, relPath)
		if err != nil {
			return err
		}
		if matched {
			matches = append(matches, relPath)
		}
		return nil
	})

	return matches, err
}

func (p *LocalFSProvider) IsLocal() bool {
	return true
}

// CloudFSProvider implements FileSystemProvider for the multi-tenant cloud mode.
type CloudFSProvider struct {
	baseVolume string // e.g., a shared persistent volume mount path
}

func NewCloudFSProvider(baseVolume string) *CloudFSProvider {
	return &CloudFSProvider{baseVolume: baseVolume}
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing or invalid claims")
	}

	// Validate tenant ID
	tenantID := claims.OrganizationID
	// Basic regex validation could go here, for now just no slashes/dots
	if strings.Contains(tenantID, "/") || strings.Contains(tenantID, ".") {
		return "", errors.New("invalid tenant ID")
	}

	// Tenant-scoped base directory
	tenantDir := filepath.Join(p.baseVolume, tenantID)

	// Create tenant dir if it doesn't exist (in a real system this might be pre-provisioned)
	_ = os.MkdirAll(tenantDir, 0755)

	absPath := filepath.Join(tenantDir, target)
	absPath = filepath.Clean(absPath)

	rel, err := filepath.Rel(tenantDir, absPath)
	if err != nil {
		return "", err
	}
	if rel == ".." || strings.HasPrefix(filepath.ToSlash(rel), "../") {
		return "", errors.New("path outside tenant workspace")
	}

	return absPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	absPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(absPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	absPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(absPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(absPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	absPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(absPath)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, pattern string) ([]string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return nil, errors.New("unauthorized")
	}
	tenantDir := filepath.Join(p.baseVolume, claims.OrganizationID)

	var matches []string
	err := filepath.Walk(tenantDir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err // Ignore errors like permission denied in a real system, but fail here
		}
		if info.IsDir() {
			return nil
		}

		relPath, _ := filepath.Rel(tenantDir, path)
		matched, err := filepath.Match(pattern, relPath)
		if err != nil {
			return err
		}
		if matched {
			matches = append(matches, relPath)
		}
		return nil
	})

	return matches, err
}

func (p *CloudFSProvider) IsLocal() bool {
	return false
}

// Factory function
func NewHybridFSProvider(isStandalone bool, basePath string) (FileSystemProvider, error) {
	if isStandalone {
		return NewLocalFSProvider(basePath)
	}
	return NewCloudFSProvider(basePath), nil
}

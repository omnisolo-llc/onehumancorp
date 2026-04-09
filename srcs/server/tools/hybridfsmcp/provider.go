package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

var validTenantID = regexp.MustCompile(`^[a-zA-Z0-9_-]+$`)

// FileSystemProvider abstracts file operations for local and cloud modes.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
	SearchFiles(ctx context.Context, root string, pattern string) ([]string, error)
	IsLocal() bool
}

// LocalFSProvider implements FileSystemProvider for Standalone mode.
type LocalFSProvider struct {
	workspaceDir string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to workspaceDir.
func NewLocalFSProvider(workspaceDir string) (*LocalFSProvider, error) {
	absWorkspace, err := filepath.Abs(workspaceDir)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{workspaceDir: absWorkspace}, nil
}

func (p *LocalFSProvider) resolvePath(targetPath string) (string, error) {
	cleanPath := filepath.Clean(filepath.Join(p.workspaceDir, targetPath))
	if cleanPath != p.workspaceDir && !strings.HasPrefix(cleanPath, p.workspaceDir+string(filepath.Separator)) {
		return "", errors.New("path escapes workspace boundary")
	}
	return cleanPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var results []string
	for _, e := range entries {
		results = append(results, e.Name())
	}
	return results, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, root string, pattern string) ([]string, error) {
	fullRoot, err := p.resolvePath(root)
	if err != nil {
		return nil, err
	}

	regex, err := regexp.Compile(pattern)
	if err != nil {
		return nil, fmt.Errorf("invalid pattern: %w", err)
	}

	var results []string
	err = filepath.WalkDir(fullRoot, func(path string, d os.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() && regex.MatchString(d.Name()) {
			relPath, _ := filepath.Rel(p.workspaceDir, path)
			results = append(results, relPath)
		}
		return nil
	})
	return results, err
}

func (p *LocalFSProvider) IsLocal() bool {
	return true
}

// CloudFSProvider implements FileSystemProvider for Cloud mode with tenant isolation.
type CloudFSProvider struct {
	baseDir string
}

// NewCloudFSProvider creates a new CloudFSProvider.
func NewCloudFSProvider(baseDir string) (*CloudFSProvider, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{baseDir: absBase}, nil
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, targetPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing or empty tenant claims")
	}

	if !validTenantID.MatchString(claims.OrganizationID) {
		return "", errors.New("invalid tenant ID format")
	}

	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	cleanPath := filepath.Clean(filepath.Join(tenantDir, targetPath))
	if cleanPath != tenantDir && !strings.HasPrefix(cleanPath, tenantDir+string(filepath.Separator)) {
		return "", errors.New("path escapes tenant boundary")
	}
	return cleanPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var results []string
	for _, e := range entries {
		results = append(results, e.Name())
	}
	return results, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, root string, pattern string) ([]string, error) {
	fullRoot, err := p.resolvePath(ctx, root)
	if err != nil {
		return nil, err
	}

	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}
	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)

	regex, err := regexp.Compile(pattern)
	if err != nil {
		return nil, fmt.Errorf("invalid pattern: %w", err)
	}

	var results []string
	err = filepath.WalkDir(fullRoot, func(path string, d os.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() && regex.MatchString(d.Name()) {
			relPath, _ := filepath.Rel(tenantDir, path)
			results = append(results, relPath)
		}
		return nil
	})
	return results, err
}

func (p *CloudFSProvider) IsLocal() bool {
	return false
}

// NewFileSystemProvider returns the appropriate provider based on the environment.
func NewFileSystemProvider(workspaceDir string) (FileSystemProvider, error) {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalFSProvider(workspaceDir)
	}
	return NewCloudFSProvider(workspaceDir)
}

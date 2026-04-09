package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for hybrid file operations
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]os.DirEntry, error)
}

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

func (p *LocalFSProvider) resolvePath(targetPath string) (string, error) {
	cleanPath := filepath.Clean(filepath.Join(p.baseDir, targetPath))
	if cleanPath == p.baseDir {
		return cleanPath, nil
	}
	if !strings.HasPrefix(cleanPath, p.baseDir+string(filepath.Separator)) {
		return "", errors.New("access denied: path escapes base directory")
	}
	return cleanPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	safePath, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	// Enforce 0700 permissions as per memory rules
	if err := os.MkdirAll(filepath.Dir(safePath), 0700); err != nil {
		return err
	}
	return os.WriteFile(safePath, data, 0600)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]os.DirEntry, error) {
	safePath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(safePath)
}

type CloudFSProvider struct {
	baseDir string
}

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
		return "", errors.New("unauthorized: missing claims or organization id")
	}

	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)
	cleanPath := filepath.Clean(filepath.Join(tenantDir, targetPath))

	if cleanPath == tenantDir {
		return cleanPath, nil
	}
	if !strings.HasPrefix(cleanPath, tenantDir+string(filepath.Separator)) {
		return "", errors.New("access denied: path escapes tenant directory")
	}
	return cleanPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	safePath, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(safePath), 0700); err != nil {
		return err
	}
	return os.WriteFile(safePath, data, 0600)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]os.DirEntry, error) {
	safePath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadDir(safePath)
}

func NewProvider(baseDir string) (FileSystemProvider, error) {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalFSProvider(baseDir)
	}
	return NewCloudFSProvider(baseDir)
}

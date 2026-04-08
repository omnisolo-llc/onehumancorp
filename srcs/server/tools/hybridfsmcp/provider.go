package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the unified interface for file operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode, enforcing path bounding.
type LocalFSProvider struct {
	basePath string
}

// NewLocalFSProvider creates a new LocalFSProvider with the given base directory.
func NewLocalFSProvider(basePath string) (*LocalFSProvider, error) {
	absPath, err := filepath.Abs(basePath)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{basePath: absPath}, nil
}

func (p *LocalFSProvider) resolvePath(targetPath string) (string, error) {
	absPath, err := filepath.Abs(filepath.Join(p.basePath, targetPath))
	if err != nil {
		return "", err
	}

	rel, err := filepath.Rel(p.basePath, absPath)
	if err != nil {
		return "", err
	}

	relSlash := filepath.ToSlash(rel)
	if relSlash == ".." || strings.HasPrefix(relSlash, "../") {
		return "", errors.New("access denied: path escapes base directory")
	}

	return absPath, nil
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
	dir := filepath.Dir(safePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(safePath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	safePath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	var entries []string
	err = filepath.WalkDir(safePath, func(p string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if p == safePath {
			return nil // Skip root
		}

		rel, err := filepath.Rel(safePath, p)
		if err != nil {
			return err
		}

		// Only list direct children for ListDir
		if strings.Contains(rel, string(filepath.Separator)) {
			if d.IsDir() {
				return fs.SkipDir
			}
			return nil
		}

		if d.IsDir() {
			entries = append(entries, rel+"/")
		} else {
			entries = append(entries, rel)
		}
		return nil
	})

	if os.IsNotExist(err) {
		return nil, fmt.Errorf("directory not found: %s", path)
	}
	return entries, err
}


// CloudFSProvider implements FileSystemProvider for Cloud mode, scoping by tenant ID.
type CloudFSProvider struct {
	tenantRoot string
}

// NewCloudFSProvider creates a new CloudFSProvider.
func NewCloudFSProvider(tenantRoot string) (*CloudFSProvider, error) {
	absPath, err := filepath.Abs(tenantRoot)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{tenantRoot: absPath}, nil
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, targetPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("access denied: missing tenant claims")
	}

	tenantDir := filepath.Join(p.tenantRoot, claims.OrganizationID)

	absPath, err := filepath.Abs(filepath.Join(tenantDir, targetPath))
	if err != nil {
		return "", err
	}

	rel, err := filepath.Rel(tenantDir, absPath)
	if err != nil {
		return "", err
	}

	relSlash := filepath.ToSlash(rel)
	if relSlash == ".." || strings.HasPrefix(relSlash, "../") {
		return "", errors.New("access denied: path escapes tenant directory")
	}

	return absPath, nil
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
	dir := filepath.Dir(safePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(safePath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	safePath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		if os.IsNotExist(err) {
			return nil, fmt.Errorf("directory not found: %s", path)
		}
		return nil, err
	}

	var result []string
	for _, entry := range entries {
		if entry.IsDir() {
			result = append(result, entry.Name()+"/")
		} else {
			result = append(result, entry.Name())
		}
	}
	return result, nil
}

// NewProvider is a factory that returns the appropriate provider based on the mode.
func NewProvider(standalone bool, basePath string) (FileSystemProvider, error) {
	if standalone {
		return NewLocalFSProvider(basePath)
	}
	return NewCloudFSProvider(basePath)
}

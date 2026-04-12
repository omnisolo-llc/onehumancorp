package hybridfsmcp

import (
	"context"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
	SearchFiles(ctx context.Context, dir string, pattern string) ([]string, error)
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

func (p *LocalFSProvider) securePath(path string) (string, error) {
	cleanPath := filepath.Clean(path)
	if !filepath.IsAbs(cleanPath) {
		cleanPath = filepath.Join(p.baseDir, cleanPath)
	}

	// Ensure path is within baseDir
	if !strings.HasPrefix(cleanPath, p.baseDir+string(filepath.Separator)) && cleanPath != p.baseDir {
		return "", fmt.Errorf("path traversal attempt: %s", path)
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

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	secure, err := p.securePath(path)
	if err != nil {
		return err
	}

	// Ensure directory exists
	dir := filepath.Dir(secure)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(secure, content, 0644)
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

	var result []string
	for _, entry := range entries {
		result = append(result, entry.Name())
	}
	return result, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, dir string, pattern string) ([]string, error) {
	secure, err := p.securePath(dir)
	if err != nil {
		return nil, err
	}

	var result []string
	err = filepath.WalkDir(secure, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}

		if !d.IsDir() {
			matched, matchErr := filepath.Match(pattern, d.Name())
			if matchErr != nil {
				return matchErr
			}
			if matched {
				relPath, _ := filepath.Rel(secure, path)
				result = append(result, relPath)
			}
		}
		return nil
	})

	return result, err
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

func (p *CloudFSProvider) securePath(ctx context.Context, path string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing organization claims")
	}

	tenantDir := filepath.Join(p.baseDir, claims.OrganizationID)

	cleanPath := filepath.Clean(path)
	if !filepath.IsAbs(cleanPath) {
		cleanPath = filepath.Join(tenantDir, cleanPath)
	}

	// Ensure path is within tenantDir
	if !strings.HasPrefix(cleanPath, tenantDir+string(filepath.Separator)) && cleanPath != tenantDir {
		return "", fmt.Errorf("path traversal attempt: %s", path)
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

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	secure, err := p.securePath(ctx, path)
	if err != nil {
		return err
	}

	// Ensure directory exists
	dir := filepath.Dir(secure)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(secure, content, 0644)
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

	var result []string
	for _, entry := range entries {
		result = append(result, entry.Name())
	}
	return result, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, dir string, pattern string) ([]string, error) {
	secure, err := p.securePath(ctx, dir)
	if err != nil {
		return nil, err
	}

	var result []string
	err = filepath.WalkDir(secure, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}

		if !d.IsDir() {
			matched, matchErr := filepath.Match(pattern, d.Name())
			if matchErr != nil {
				return matchErr
			}
			if matched {
				relPath, _ := filepath.Rel(secure, path)
				result = append(result, relPath)
			}
		}
		return nil
	})

	return result, err
}

func NewFileSystemProvider(baseDir string) (FileSystemProvider, error) {
	isStandalone := os.Getenv("OHC_STANDALONE") == "true"
	if isStandalone {
		return NewLocalFSProvider(baseDir)
	}
	return NewCloudFSProvider(baseDir)
}

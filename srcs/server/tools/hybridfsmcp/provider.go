package hybridfsmcp

import (
	"context"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
	"fmt"
	"errors"
)

// FileSystemProvider defines the interface for our abstract filesystem.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]fs.FileInfo, error)
	IsLocal() bool
}

// LocalFSProvider implements the FileSystemProvider for Standalone mode.
type LocalFSProvider struct {
	basePath string
}

// NewLocalFSProvider creates a LocalFSProvider with a restricted base path.
func NewLocalFSProvider(basePath string) (*LocalFSProvider, error) {
	cleanPath := filepath.Clean(basePath)
	info, err := os.Stat(cleanPath)
	if err != nil {
		if os.IsNotExist(err) {
			err = os.MkdirAll(cleanPath, 0755)
			if err != nil {
				return nil, fmt.Errorf("failed to create base path: %w", err)
			}
		} else {
			return nil, fmt.Errorf("failed to stat base path: %w", err)
		}
	} else if !info.IsDir() {
		return nil, fmt.Errorf("base path is not a directory: %s", cleanPath)
	}

	return &LocalFSProvider{
		basePath: cleanPath,
	}, nil
}

func (p *LocalFSProvider) IsLocal() bool {
	return true
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	cleanTarget := filepath.Clean(filepath.Join(p.basePath, target))

	// Prevent path traversal
	if cleanTarget != p.basePath && !strings.HasPrefix(cleanTarget, p.basePath+string(filepath.Separator)) {
		return "", errors.New("access denied: path escapes workspace boundaries")
	}
	return cleanTarget, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create directory: %w", err)
	}

	return os.WriteFile(resolved, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolved)
	if err != nil {
		return nil, err
	}

	var infos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err == nil {
			infos = append(infos, info)
		}
	}
	return infos, nil
}

// CloudFSProvider implements the FileSystemProvider for Cloud mode using tenant-scoped paths.
type CloudFSProvider struct {
	pvBasePath string
}

// NewCloudFSProvider creates a CloudFSProvider with a PV base path.
func NewCloudFSProvider(pvBasePath string) (*CloudFSProvider, error) {
	cleanPath := filepath.Clean(pvBasePath)
	info, err := os.Stat(cleanPath)
	if err != nil {
		if os.IsNotExist(err) {
			err = os.MkdirAll(cleanPath, 0755)
			if err != nil {
				return nil, fmt.Errorf("failed to create pv base path: %w", err)
			}
		} else {
			return nil, fmt.Errorf("failed to stat pv base path: %w", err)
		}
	} else if !info.IsDir() {
		return nil, fmt.Errorf("pv base path is not a directory: %s", cleanPath)
	}

	return &CloudFSProvider{
		pvBasePath: cleanPath,
	}, nil
}

func (p *CloudFSProvider) IsLocal() bool {
	return false
}

// We expect the path to already include the tenant scope when calling resolvePath
// inside the cloud provider, or we rely on the MCP wrapper to prepend it.
// We'll let the MCP wrapper prepend tenant ID.
func (p *CloudFSProvider) resolvePath(target string) (string, error) {
	cleanTarget := filepath.Clean(filepath.Join(p.pvBasePath, target))

	// Prevent path traversal outside PV base
	if cleanTarget != p.pvBasePath && !strings.HasPrefix(cleanTarget, p.pvBasePath+string(filepath.Separator)) {
		return "", errors.New("access denied: path escapes pv boundaries")
	}
	return cleanTarget, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create directory: %w", err)
	}

	return os.WriteFile(resolved, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolved)
	if err != nil {
		return nil, err
	}

	var infos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err == nil {
			infos = append(infos, info)
		}
	}
	return infos, nil
}

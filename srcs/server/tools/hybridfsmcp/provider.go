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

// FileInfo represents metadata about a file or directory.
type FileInfo struct {
	Name  string
	IsDir bool
	Size  int64
}

// FileSystemProvider defines the interface for hybrid file operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]FileInfo, error)
	SearchFiles(ctx context.Context, path string, pattern string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone mode, bounding access to a workspace dir.
type LocalFSProvider struct {
	workspaceDir string
}

// NewLocalFSProvider creates a LocalFSProvider bound to workspaceDir.
func NewLocalFSProvider(workspaceDir string) (*LocalFSProvider, error) {
	abs, err := filepath.Abs(workspaceDir)
	if err != nil {
		return nil, fmt.Errorf("failed to get absolute workspace path: %w", err)
	}
	return &LocalFSProvider{workspaceDir: abs}, nil
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	full := filepath.Join(p.workspaceDir, target)
	clean := filepath.Clean(full)

	rel, err := filepath.Rel(p.workspaceDir, clean)
	if err != nil || strings.HasPrefix(rel, "../") || strings.HasPrefix(rel, "..\\") || rel == ".." {
		return "", errors.New("path escapes workspace boundary")
	}

	return clean, nil
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
		return err
	}
	return os.WriteFile(resolved, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(resolved)
	if err != nil {
		if os.IsNotExist(err) {
			return []FileInfo{}, nil
		}
		return nil, err
	}
	var res []FileInfo
	for _, e := range entries {
		info, err := e.Info()
		if err != nil {
			continue
		}
		res = append(res, FileInfo{
			Name:  e.Name(),
			IsDir: e.IsDir(),
			Size:  info.Size(),
		})
	}
	return res, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, path string, pattern string) ([]string, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	var matches []string
	err = filepath.WalkDir(resolved, func(walkPath string, d fs.DirEntry, walkErr error) error {
		if walkErr != nil {
			if os.IsNotExist(walkErr) {
				return nil
			}
			return walkErr
		}
		if !d.IsDir() && strings.Contains(d.Name(), pattern) {
			rel, _ := filepath.Rel(p.workspaceDir, walkPath)
			matches = append(matches, rel)
		}
		return nil
	})
	return matches, err
}

// CloudFSProvider implements FileSystemProvider for Cloud mode, bounding access based on tenant claims.
type CloudFSProvider struct {
	baseStorageDir string
}

// NewCloudFSProvider creates a CloudFSProvider.
func NewCloudFSProvider(baseStorageDir string) (*CloudFSProvider, error) {
	abs, err := filepath.Abs(baseStorageDir)
	if err != nil {
		return nil, fmt.Errorf("failed to get absolute base storage path: %w", err)
	}
	return &CloudFSProvider{baseStorageDir: abs}, nil
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing tenant context")
	}
	tenantDir := filepath.Join(p.baseStorageDir, claims.OrganizationID)
	full := filepath.Join(tenantDir, target)
	clean := filepath.Clean(full)

	rel, err := filepath.Rel(tenantDir, clean)
	if err != nil || strings.HasPrefix(rel, "../") || strings.HasPrefix(rel, "..\\") || rel == ".." {
		return "", errors.New("path escapes tenant boundary")
	}

	return clean, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(resolved, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(resolved)
	if err != nil {
		if os.IsNotExist(err) {
			return []FileInfo{}, nil
		}
		return nil, err
	}
	var res []FileInfo
	for _, e := range entries {
		info, err := e.Info()
		if err != nil {
			continue
		}
		res = append(res, FileInfo{
			Name:  e.Name(),
			IsDir: e.IsDir(),
			Size:  info.Size(),
		})
	}
	return res, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, path string, pattern string) ([]string, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return nil, errors.New("unauthorized: missing tenant context")
	}
	tenantDir := filepath.Join(p.baseStorageDir, claims.OrganizationID)

	var matches []string
	err = filepath.WalkDir(resolved, func(walkPath string, d fs.DirEntry, walkErr error) error {
		if walkErr != nil {
			if os.IsNotExist(walkErr) {
				return nil
			}
			return walkErr
		}
		if !d.IsDir() && strings.Contains(d.Name(), pattern) {
			rel, _ := filepath.Rel(tenantDir, walkPath)
			matches = append(matches, rel)
		}
		return nil
	})
	return matches, err
}

// NewProvider is the factory function that returns the appropriate provider based on OHC_STANDALONE.
func NewProvider(workspaceOrBaseDir string) (FileSystemProvider, error) {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalFSProvider(workspaceOrBaseDir)
	}
	return NewCloudFSProvider(workspaceOrBaseDir)
}

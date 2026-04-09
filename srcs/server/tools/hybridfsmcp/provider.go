package hybridfsmcp

import (
	"context"
	"errors"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for file operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]fs.FileInfo, error)
	SearchFiles(ctx context.Context, dir string, pattern string) ([]string, error)
	IsLocal() bool
}

// LocalFSProvider maps directly to the local filesystem using path bounding.
type LocalFSProvider struct {
	workspaceDir string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to the given workspace directory.
func NewLocalFSProvider(workspaceDir string) (*LocalFSProvider, error) {
	abs, err := filepath.Abs(workspaceDir)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{workspaceDir: abs}, nil
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	cleanPath := filepath.Clean(target)
	if filepath.IsAbs(cleanPath) {
		if !strings.HasPrefix(cleanPath, p.workspaceDir+string(filepath.Separator)) && cleanPath != p.workspaceDir {
			return "", errors.New("path escapes workspace boundary")
		}
		return cleanPath, nil
	}
	abs := filepath.Join(p.workspaceDir, cleanPath)
	if !strings.HasPrefix(abs, p.workspaceDir+string(filepath.Separator)) && abs != p.workspaceDir {
		return "", errors.New("path escapes workspace boundary")
	}
	return abs, nil
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
	// Ensure directory exists
	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
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
		if err != nil {
			continue
		}
		infos = append(infos, info)
	}
	return infos, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, dir string, pattern string) ([]string, error) {
	resolvedDir, err := p.resolvePath(dir)
	if err != nil {
		return nil, err
	}
	var matches []string
	err = filepath.WalkDir(resolvedDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if d.IsDir() {
			return nil
		}
		matched, err := filepath.Match(pattern, d.Name())
		if err != nil {
			return err
		}
		if matched {
			rel, err := filepath.Rel(p.workspaceDir, path)
			if err != nil {
				matches = append(matches, path)
			} else {
				matches = append(matches, rel)
			}
		}
		return nil
	})
	return matches, err
}

func (p *LocalFSProvider) IsLocal() bool {
	return true
}

// CloudFSProvider maps to Tenant-scoped paths.
type CloudFSProvider struct {
	baseStorageDir string
}

// NewCloudFSProvider creates a new CloudFSProvider bounded to a base storage directory (e.g., PVC mount).
func NewCloudFSProvider(baseStorageDir string) (*CloudFSProvider, error) {
	abs, err := filepath.Abs(baseStorageDir)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{baseStorageDir: abs}, nil
}

func (p *CloudFSProvider) resolvePath(tenantID string, target string) (string, error) {
	if tenantID == "" {
		return "", errors.New("unauthorized: missing tenant ID")
	}

	cleanPath := filepath.Clean(target)

	// Create the tenant's dedicated storage path
	tenantRoot := filepath.Join(p.baseStorageDir, tenantID)

	// Ensure target path resolves inside the tenant's root
	var abs string
	if filepath.IsAbs(cleanPath) {
		// If they provide an absolute path, we treat it as relative to their tenant root
		// by stripping the leading slash
		cleanPath = strings.TrimPrefix(cleanPath, "/")
		abs = filepath.Join(tenantRoot, cleanPath)
	} else {
		abs = filepath.Join(tenantRoot, cleanPath)
	}

	if !strings.HasPrefix(abs, tenantRoot+string(filepath.Separator)) && abs != tenantRoot {
		return "", errors.New("path escapes tenant boundary")
	}

	return abs, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}
	resolved, err := p.resolvePath(claims.OrganizationID, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return errors.New("unauthorized: missing claims")
	}
	resolved, err := p.resolvePath(claims.OrganizationID, path)
	if err != nil {
		return err
	}
	// Ensure directory exists
	dir := filepath.Dir(resolved)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(resolved, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}
	resolved, err := p.resolvePath(claims.OrganizationID, path)
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
		if err != nil {
			continue
		}
		infos = append(infos, info)
	}
	return infos, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, dir string, pattern string) ([]string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}
	resolvedDir, err := p.resolvePath(claims.OrganizationID, dir)
	if err != nil {
		return nil, err
	}
	var matches []string
	tenantRoot := filepath.Join(p.baseStorageDir, claims.OrganizationID)
	err = filepath.WalkDir(resolvedDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if d.IsDir() {
			return nil
		}
		matched, err := filepath.Match(pattern, d.Name())
		if err != nil {
			return err
		}
		if matched {
			rel, err := filepath.Rel(tenantRoot, path)
			if err != nil {
				matches = append(matches, path)
			} else {
				matches = append(matches, rel)
			}
		}
		return nil
	})
	return matches, err
}

func (p *CloudFSProvider) IsLocal() bool {
	return false
}

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

// FileInfo represents metadata for a file.
type FileInfo struct {
	Name  string
	IsDir bool
	Size  int64
}

// FileSystemProvider defines the unified interface for file operations.
type FileSystemProvider interface {
	IsLocal() bool
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]FileInfo, error)
	SearchFiles(ctx context.Context, claims *auth.Claims, path, pattern string) ([]string, error)
}

// LocalFSProvider implements the FileSystemProvider for Standalone mode.
type LocalFSProvider struct {
	baseDir string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to the baseDir.
func NewLocalFSProvider(baseDir string) (*LocalFSProvider, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, err
	}
	return &LocalFSProvider{baseDir: absBase}, nil
}

func (p *LocalFSProvider) IsLocal() bool { return true }

// resolvePath returns an absolute path ensuring it does not escape the baseDir.
func (p *LocalFSProvider) resolvePath(reqPath string) (string, error) {
	absPath, err := filepath.Abs(filepath.Join(p.baseDir, reqPath))
	if err != nil {
		return "", err
	}

	baseWithSep := p.baseDir
	if !strings.HasSuffix(baseWithSep, string(filepath.Separator)) {
		baseWithSep += string(filepath.Separator)
	}

	if absPath != p.baseDir && !strings.HasPrefix(absPath, baseWithSep) {
		return "", errors.New("path escapes base directory")
	}

	return absPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, reqPath string) ([]byte, error) {
	resolved, err := p.resolvePath(reqPath)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, reqPath string, data []byte) error {
	resolved, err := p.resolvePath(reqPath)
	if err != nil {
		return err
	}
	// Create directory structure if needed
	if err := os.MkdirAll(filepath.Dir(resolved), 0755); err != nil {
		return err
	}
	return os.WriteFile(resolved, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, reqPath string) ([]FileInfo, error) {
	resolved, err := p.resolvePath(reqPath)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolved)
	if err != nil {
		return nil, err
	}

	var results []FileInfo
	for _, e := range entries {
		info, err := e.Info()
		if err != nil {
			continue // Skip files we can't stat
		}
		results = append(results, FileInfo{
			Name:  e.Name(),
			IsDir: e.IsDir(),
			Size:  info.Size(),
		})
	}
	return results, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, claims *auth.Claims, reqPath, pattern string) ([]string, error) {
	resolved, err := p.resolvePath(reqPath)
	if err != nil {
		return nil, err
	}

	var matches []string
	err = filepath.WalkDir(resolved, func(path string, d fs.DirEntry, err error) error {
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
			rel, err := filepath.Rel(p.baseDir, path)
			if err != nil {
				return err
			}
			matches = append(matches, rel)
		}
		return nil
	})

	return matches, err
}

// CloudFSProvider implements the FileSystemProvider for Cloud mode with tenant isolation.
type CloudFSProvider struct {
	baseDir string // e.g., the root of the K8s PV mounted for tenant workspaces
}

func NewCloudFSProvider(baseDir string) (*CloudFSProvider, error) {
	absBase, err := filepath.Abs(baseDir)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{baseDir: absBase}, nil
}

func (p *CloudFSProvider) IsLocal() bool { return false }

// resolvePath ensures the path is scoped to the tenant's organization ID.
func (p *CloudFSProvider) resolvePath(claims *auth.Claims, reqPath string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing claims or organization ID")
	}

	tenantBase := filepath.Join(p.baseDir, claims.OrganizationID)

	// Create an absolute path under the tenant's base
	absPath, err := filepath.Abs(filepath.Join(tenantBase, reqPath))
	if err != nil {
		return "", err
	}

	tenantBaseWithSep := tenantBase
	if !strings.HasSuffix(tenantBaseWithSep, string(filepath.Separator)) {
		tenantBaseWithSep += string(filepath.Separator)
	}

	if absPath != tenantBase && !strings.HasPrefix(absPath, tenantBaseWithSep) {
		return "", errors.New("path escapes tenant directory")
	}

	return absPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, reqPath string) ([]byte, error) {
	resolved, err := p.resolvePath(claims, reqPath)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, reqPath string, data []byte) error {
	resolved, err := p.resolvePath(claims, reqPath)
	if err != nil {
		return err
	}
	// Ensure tenant directory exists
	if err := os.MkdirAll(filepath.Dir(resolved), 0755); err != nil {
		return err
	}
	return os.WriteFile(resolved, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, reqPath string) ([]FileInfo, error) {
	resolved, err := p.resolvePath(claims, reqPath)
	if err != nil {
		return nil, err
	}

	// Ensure tenant directory exists, if we are just listing the root
	if _, err := os.Stat(resolved); os.IsNotExist(err) {
		return []FileInfo{}, nil
	}

	entries, err := os.ReadDir(resolved)
	if err != nil {
		return nil, err
	}

	var results []FileInfo
	for _, e := range entries {
		info, err := e.Info()
		if err != nil {
			continue
		}
		results = append(results, FileInfo{
			Name:  e.Name(),
			IsDir: e.IsDir(),
			Size:  info.Size(),
		})
	}
	return results, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, claims *auth.Claims, reqPath, pattern string) ([]string, error) {
	resolved, err := p.resolvePath(claims, reqPath)
	if err != nil {
		return nil, err
	}

	tenantBase := filepath.Join(p.baseDir, claims.OrganizationID)

	if _, err := os.Stat(resolved); os.IsNotExist(err) {
		return []string{}, nil
	}

	var matches []string
	err = filepath.WalkDir(resolved, func(path string, d fs.DirEntry, err error) error {
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
			rel, err := filepath.Rel(tenantBase, path)
			if err != nil {
				return err
			}
			matches = append(matches, rel)
		}
		return nil
	})

	return matches, err
}

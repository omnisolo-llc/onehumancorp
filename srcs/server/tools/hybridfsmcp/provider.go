package hybridfsmcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"strings"
	"sync"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for hybrid file system operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error)
	SearchFiles(ctx context.Context, claims *auth.Claims, query string) ([]string, error)
	IsLocal() bool
}

// LocalFSProvider implements FileSystemProvider for Standalone mode.
type LocalFSProvider struct {
	workspaceDir string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to workspaceDir.
func NewLocalFSProvider(workspaceDir string) *LocalFSProvider {
	if workspaceDir == "" {
		workspaceDir = "."
	}
	abs, err := filepath.Abs(workspaceDir)
	if err == nil {
		workspaceDir = abs
	}
	return &LocalFSProvider{workspaceDir: workspaceDir}
}

func (p *LocalFSProvider) sanitizePath(path string) (string, error) {
	if strings.Contains(path, "..") {
		return "", errors.New("directory traversal attempt blocked")
	}
	cleanPath := filepath.Clean("/" + path)
	cleanPath = strings.TrimPrefix(cleanPath, "/")
	fullPath := filepath.Join(p.workspaceDir, cleanPath)

	if !strings.HasPrefix(fullPath, p.workspaceDir) {
		return "", errors.New("directory traversal attempt blocked")
	}

	return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	fullPath, err := p.sanitizePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	fullPath, err := p.sanitizePath(path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	fullPath, err := p.sanitizePath(path)
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

func (p *LocalFSProvider) SearchFiles(ctx context.Context, claims *auth.Claims, query string) ([]string, error) {
	var results []string
	err := filepath.Walk(p.workspaceDir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil // skip errors
		}
		if !info.IsDir() && strings.Contains(info.Name(), query) {
			rel, err := filepath.Rel(p.workspaceDir, path)
			if err == nil {
				results = append(results, rel)
			}
		}
		return nil
	})
	return results, err
}

func (p *LocalFSProvider) IsLocal() bool {
	return true
}

// CloudFSProvider implements FileSystemProvider for Cloud mode with tenant isolation.
// This is currently a mock/placeholder implementation simulating an S3-backed virtual FS.
type CloudFSProvider struct {
	mu          sync.RWMutex
	// In a real implementation, this would hold S3 client/bucket info.
	mockStorage map[string]map[string][]byte
}

func NewCloudFSProvider() *CloudFSProvider {
	return &CloudFSProvider{
		mockStorage: make(map[string]map[string][]byte),
	}
}

func (p *CloudFSProvider) enforceIsolation(claims *auth.Claims) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing organization claims")
	}
	return claims.OrganizationID, nil
}

func (p *CloudFSProvider) sanitizePath(path string) string {
	cleanPath := filepath.Clean("/" + path)
	return strings.TrimPrefix(cleanPath, "/")
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	orgID, err := p.enforceIsolation(claims)
	if err != nil {
		return nil, err
	}

	cleanPath := p.sanitizePath(path)

	p.mu.RLock()
	defer p.mu.RUnlock()

	if tenantStorage, ok := p.mockStorage[orgID]; ok {
		if data, ok := tenantStorage[cleanPath]; ok {
			return data, nil
		}
	}
	return nil, os.ErrNotExist
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	orgID, err := p.enforceIsolation(claims)
	if err != nil {
		return err
	}

	cleanPath := p.sanitizePath(path)

	p.mu.Lock()
	defer p.mu.Unlock()

	if _, ok := p.mockStorage[orgID]; !ok {
		p.mockStorage[orgID] = make(map[string][]byte)
	}

	// Create a copy of the data
	dataCopy := make([]byte, len(data))
	copy(dataCopy, data)

	p.mockStorage[orgID][cleanPath] = dataCopy
	return nil
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	orgID, err := p.enforceIsolation(claims)
	if err != nil {
		return nil, err
	}

	cleanPath := p.sanitizePath(path)
	if cleanPath != "" && !strings.HasSuffix(cleanPath, "/") {
		cleanPath += "/"
	}

	var results []string
	seen := make(map[string]bool)

	p.mu.RLock()
	defer p.mu.RUnlock()

	if tenantStorage, ok := p.mockStorage[orgID]; ok {
		for k := range tenantStorage {
			if cleanPath == "" || strings.HasPrefix(k, cleanPath) {
				rel := strings.TrimPrefix(k, cleanPath)
				parts := strings.SplitN(rel, "/", 2)
				if len(parts) > 0 {
					name := parts[0]
					if !seen[name] {
						results = append(results, name)
						seen[name] = true
					}
				}
			}
		}
	}
	return results, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, claims *auth.Claims, query string) ([]string, error) {
	orgID, err := p.enforceIsolation(claims)
	if err != nil {
		return nil, err
	}

	var results []string

	p.mu.RLock()
	defer p.mu.RUnlock()

	if tenantStorage, ok := p.mockStorage[orgID]; ok {
		for k := range tenantStorage {
			if strings.Contains(filepath.Base(k), query) {
				results = append(results, k)
			}
		}
	}
	return results, nil
}

func (p *CloudFSProvider) IsLocal() bool {
	return false
}

// NewFileSystemProvider acts as a factory based on environment variables.
func NewFileSystemProvider(workspaceDir string) FileSystemProvider {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalFSProvider(workspaceDir)
	}
	// Default to Cloud/Multitenant mode if OHC_MULTITENANT is set or if not standalone.
	return NewCloudFSProvider()
}

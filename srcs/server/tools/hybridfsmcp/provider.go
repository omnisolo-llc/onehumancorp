package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines the interface for hybrid file operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error
	ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error)
}

// LocalFSProvider implements the FileSystemProvider for Standalone mode.
// It maps directly to the local file system with safety bounds.
type LocalFSProvider struct {
	baseDir string
}

// NewLocalFSProvider creates a new LocalFSProvider bounded to baseDir.
func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	if baseDir == "" {
		baseDir = os.Getenv("OHC_FS_ROOT")
		if baseDir == "" {
			baseDir = os.TempDir()
		}
	}
	absBaseDir, err := filepath.Abs(baseDir)
	if err != nil {
		panic(err) // Ensure the base directory is valid at startup
	}
	return &LocalFSProvider{baseDir: absBaseDir}
}

// validatePath ensures the requested path is within the bounded baseDir.
func (p *LocalFSProvider) validatePath(reqPath string) (string, error) {
	fullPath := filepath.Join(p.baseDir, reqPath)
	cleanPath, err := filepath.Abs(fullPath)
	if err != nil {
		return "", fmt.Errorf("invalid path: %w", err)
	}

	// Safely verify directory boundaries to prevent path traversal
	if cleanPath != p.baseDir && !strings.HasPrefix(cleanPath, p.baseDir+string(filepath.Separator)) {
		return "", errors.New("access denied: path escapes base directory")
	}

	return cleanPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	safePath, err := p.validatePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	safePath, err := p.validatePath(path)
	if err != nil {
		return err
	}
	// Ensure the parent directory exists
	if err := os.MkdirAll(filepath.Dir(safePath), 0755); err != nil {
		return err
	}
	return os.WriteFile(safePath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	safePath, err := p.validatePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

// CloudFSProvider implements the FileSystemProvider for Cloud-Native mode.
// It uses tenant-scoped paths based on auth.Claims to prevent cross-tenant access.
type CloudFSProvider struct {
	cloudRoot string
}

// NewCloudFSProvider creates a new CloudFSProvider with a cloud root directory.
func NewCloudFSProvider(cloudRoot string) *CloudFSProvider {
	if cloudRoot == "" {
		cloudRoot = os.Getenv("OHC_FS_ROOT")
		if cloudRoot == "" {
			cloudRoot = "/data/tenants"
		}
	}
	absRoot, err := filepath.Abs(cloudRoot)
	if err != nil {
		panic(err)
	}
	return &CloudFSProvider{cloudRoot: absRoot}
}

// validateTenantPath scopes paths by organization_id from auth.Claims
func (p *CloudFSProvider) validateTenantPath(claims *auth.Claims, reqPath string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing or invalid tenant claims")
	}

	tenantBaseDir := filepath.Join(p.cloudRoot, claims.OrganizationID)
	// Important: Create tenant directory if it doesn't exist for tests
	// In reality this might be provisioned via K8s PVs
	os.MkdirAll(tenantBaseDir, 0755)

	fullPath := filepath.Join(tenantBaseDir, reqPath)
	cleanPath, err := filepath.Abs(fullPath)
	if err != nil {
		return "", fmt.Errorf("invalid path: %w", err)
	}

	// Safely verify directory boundaries
	if cleanPath != tenantBaseDir && !strings.HasPrefix(cleanPath, tenantBaseDir+string(filepath.Separator)) {
		return "", errors.New("access denied: path escapes tenant directory")
	}

	return cleanPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	safePath, err := p.validateTenantPath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	safePath, err := p.validateTenantPath(claims, path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(safePath), 0755); err != nil {
		return err
	}
	return os.WriteFile(safePath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	safePath, err := p.validateTenantPath(claims, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

// NewProvider creates the appropriate FileSystemProvider based on the OHC_MULTITENANT env var.
func NewProvider() FileSystemProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return NewCloudFSProvider("")
	}
	return NewLocalFSProvider("")
}

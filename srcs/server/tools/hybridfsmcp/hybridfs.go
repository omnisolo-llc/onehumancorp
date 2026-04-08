package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider defines an abstraction for reading and writing files,
// allowing unified file system operations across standalone and cloud deployments.
// Accepts contextual path and payload details.
// Returns file content bytes or error on failures.
// Produces an error if the path is unauthorized or reading fails.
// Has side effects by reading/writing to the underlying filesystem.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
	SearchFiles(ctx context.Context, dir string, pattern string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for Standalone modes,
// bounding access to a defined local workspace directory for safety.
// Accepts a root workspace directory parameter.
// Returns an initialized LocalFSProvider.
// Produces no errors.
// Has no side effects.
type LocalFSProvider struct {
	workspaceDir string
}

func (p *LocalFSProvider) resolvePath(path string) (string, error) {
	cleanPath := filepath.Clean(path)
	if filepath.IsAbs(cleanPath) || strings.HasPrefix(cleanPath, "..") {
		return "", fmt.Errorf("unauthorized path traversal or absolute path")
	}
	fullPath := filepath.Join(p.workspaceDir, cleanPath)
	return fullPath, nil
}

// ReadFile reads the file content from the local bounded filesystem.
// Accepts a context and a relative path string.
// Returns a byte slice of the content and an error on failure.
// Produces an error if the path is outside bounds or file doesn't exist.
// Has side effects by reading the filesystem.
func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

// WriteFile writes the given content to the specified path in the bounded filesystem.
// Accepts a context, path string, and content byte slice.
// Returns an error on failure.
// Produces an error if the path is outside bounds or permissions fail.
// Has side effects by writing to the filesystem.
func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create directories: %w", err)
	}
	return os.WriteFile(fullPath, content, 0644)
}

// ListDir lists the files in a local directory.
// Accepts a context and a path string.
// Returns a slice of strings containing the file names or an error on failure.
// Produces an error if the path is invalid or the directory doesn't exist.
// Has side effects by reading the filesystem.
func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

// SearchFiles searches for files matching a pattern in the bounded filesystem.
// Accepts a context, a directory path, and a search pattern.
// Returns a slice of matched file paths or an error on failure.
// Produces an error if the path is unauthorized or the search fails.
// Has side effects by recursively reading the filesystem.
func (p *LocalFSProvider) SearchFiles(ctx context.Context, dir string, pattern string) ([]string, error) {
	fullPath, err := p.resolvePath(dir)
	if err != nil {
		return nil, err
	}

	var matches []string
	err = filepath.Walk(fullPath, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() {
			rel, err := filepath.Rel(fullPath, path)
			if err != nil {
				return err
			}
			if strings.Contains(rel, pattern) {
				matches = append(matches, rel)
			}
		}
		return nil
	})
	return matches, err
}

// CloudFSProvider implements FileSystemProvider for multi-tenant cloud modes,
// scoping filesystem operations to a tenant-specific virtual directory structure.
// Accepts a base volume path parameter.
// Returns an initialized CloudFSProvider.
// Produces no errors.
// Has no side effects.
type CloudFSProvider struct {
	baseVolume string
}

func (p *CloudFSProvider) resolveTenantPath(ctx context.Context, path string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing tenant context")
	}

	cleanPath := filepath.Clean(path)
	if filepath.IsAbs(cleanPath) || strings.HasPrefix(cleanPath, "..") {
		return "", fmt.Errorf("unauthorized path traversal or absolute path")
	}

	tenantDir := filepath.Join(p.baseVolume, claims.OrganizationID)
	fullPath := filepath.Join(tenantDir, cleanPath)
	return fullPath, nil
}

// ReadFile reads the file content from the tenant-scoped cloud filesystem.
// Accepts a context (with auth.Claims) and a path string.
// Returns a byte slice of the content and an error on failure.
// Produces an error if tenant auth fails or the file doesn't exist.
// Has side effects by reading the filesystem.
func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

// WriteFile writes the given content to the specified path in the tenant-scoped cloud filesystem.
// Accepts a context (with auth.Claims), path string, and content byte slice.
// Returns an error on failure.
// Produces an error if tenant auth fails or permissions are invalid.
// Has side effects by writing to the filesystem.
func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	fullPath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create tenant directories: %w", err)
	}
	return os.WriteFile(fullPath, content, 0644)
}

// ListDir lists the files in a tenant-scoped cloud directory.
// Accepts a context (with auth.Claims) and a path string.
// Returns a slice of strings containing the file names or an error on failure.
// Produces an error if tenant auth fails or the directory doesn't exist.
// Has side effects by reading the filesystem.
func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

// SearchFiles searches for files matching a pattern in the tenant-scoped cloud filesystem.
// Accepts a context (with auth.Claims), a directory path, and a search pattern.
// Returns a slice of matched file paths or an error on failure.
// Produces an error if tenant auth fails or the search fails.
// Has side effects by recursively reading the filesystem.
func (p *CloudFSProvider) SearchFiles(ctx context.Context, dir string, pattern string) ([]string, error) {
	fullPath, err := p.resolveTenantPath(ctx, dir)
	if err != nil {
		return nil, err
	}

	var matches []string
	err = filepath.Walk(fullPath, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() {
			rel, err := filepath.Rel(fullPath, path)
			if err != nil {
				return err
			}
			if strings.Contains(rel, pattern) {
				matches = append(matches, rel)
			}
		}
		return nil
	})
	return matches, err
}

// NewProvider is a factory method that instantiates the correct FileSystemProvider
// based on the environment constraints (OHC_STANDALONE).
// Accepts no parameters.
// Returns a FileSystemProvider and an error if initialization fails.
// Produces an error if the underlying directories cannot be initialized.
// Has side effects by creating missing workspace/volume directories.
func NewProvider() (FileSystemProvider, error) {
	isStandalone := os.Getenv("OHC_STANDALONE") == "true"
	if isStandalone {
		workspace := os.Getenv("OHC_LOCAL_WORKSPACE")
		if workspace == "" {
			home, err := os.UserHomeDir()
			if err != nil {
				return nil, fmt.Errorf("failed to get user home dir: %w", err)
			}
			workspace = filepath.Join(home, ".ohc-workspace")
		}
		if err := os.MkdirAll(workspace, 0755); err != nil {
			return nil, fmt.Errorf("failed to init local workspace: %w", err)
		}
		return &LocalFSProvider{workspaceDir: workspace}, nil
	}

	baseVolume := os.Getenv("OHC_CLOUD_VOLUME")
	if baseVolume == "" {
		baseVolume = "/mnt/data/tenants" // Default K8s PVC mount point
	}
	if err := os.MkdirAll(baseVolume, 0755); err != nil {
		return nil, fmt.Errorf("failed to init cloud volume: %w", err)
	}
	return &CloudFSProvider{baseVolume: baseVolume}, nil
}

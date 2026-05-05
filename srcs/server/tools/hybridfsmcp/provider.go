package hybridfsmcp

import (
	"context"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
)

type Claims struct {
	OrganizationID string
}

type FileSystemProvider interface {
	ReadFile(ctx context.Context, claims *Claims, path string) ([]byte, error)
	WriteFile(ctx context.Context, claims *Claims, path string, data []byte) error
	ListDir(ctx context.Context, claims *Claims, path string) ([]fs.FileInfo, error)
	SearchFiles(ctx context.Context, claims *Claims, query string) ([]string, error)
}

// LocalFSProvider acts on the local filesystem. Binds operations to a specific WorkspaceDir.
type LocalFSProvider struct {
	WorkspaceDir string
}

func (p *LocalFSProvider) sanitizePath(targetPath string) (string, error) {
	workspaceClean := filepath.Clean(p.WorkspaceDir)
	cleanPath := filepath.Clean(filepath.Join(workspaceClean, targetPath))

	// Add trailing separator to ensure exact directory match and prevent prefix attacks
	// (e.g., matching /mnt/workspace-hacked against /mnt/workspace)
	if !strings.HasPrefix(cleanPath+string(filepath.Separator), workspaceClean+string(filepath.Separator)) {
		return "", fmt.Errorf("path escapes workspace directory")
	}
	return cleanPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *Claims, path string) ([]byte, error) {
	safePath, err := p.sanitizePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *Claims, path string, data []byte) error {
	safePath, err := p.sanitizePath(path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(safePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(safePath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, claims *Claims, path string) ([]fs.FileInfo, error) {
	safePath, err := p.sanitizePath(path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}

	var infos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			return nil, err
		}
		infos = append(infos, info)
	}

	return infos, nil
}

func (p *LocalFSProvider) SearchFiles(ctx context.Context, claims *Claims, query string) ([]string, error) {
	var results []string

	err := filepath.WalkDir(p.WorkspaceDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}

		if !d.IsDir() && strings.Contains(d.Name(), query) {
			relPath, err := filepath.Rel(p.WorkspaceDir, path)
			if err != nil {
				return err
			}
			results = append(results, relPath)
		}

		return nil
	})

	return results, err
}

// CloudFSProvider acts on tenant-scoped directories based on Claims.OrganizationID.
type CloudFSProvider struct {
	BaseCloudDir string
}

func (p *CloudFSProvider) sanitizePath(claims *Claims, targetPath string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("missing OrganizationID in claims")
	}
	tenantDir := filepath.Clean(filepath.Join(p.BaseCloudDir, claims.OrganizationID))
	cleanPath := filepath.Clean(filepath.Join(tenantDir, targetPath))

	if !strings.HasPrefix(cleanPath+string(filepath.Separator), tenantDir+string(filepath.Separator)) {
		return "", fmt.Errorf("path escapes tenant directory")
	}
	return cleanPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *Claims, path string) ([]byte, error) {
	safePath, err := p.sanitizePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *Claims, path string, data []byte) error {
	safePath, err := p.sanitizePath(claims, path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(safePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(safePath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *Claims, path string) ([]fs.FileInfo, error) {
	safePath, err := p.sanitizePath(claims, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}

	var infos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			return nil, err
		}
		infos = append(infos, info)
	}

	return infos, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, claims *Claims, query string) ([]string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return nil, fmt.Errorf("missing OrganizationID in claims")
	}

	tenantDir := filepath.Join(p.BaseCloudDir, claims.OrganizationID)

	// Check if tenant dir exists before searching
	if _, err := os.Stat(tenantDir); os.IsNotExist(err) {
		return []string{}, nil
	}

	var results []string

	err := filepath.WalkDir(tenantDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}

		if !d.IsDir() && strings.Contains(d.Name(), query) {
			relPath, err := filepath.Rel(tenantDir, path)
			if err != nil {
				return err
			}
			results = append(results, relPath)
		}

		return nil
	})

	return results, err
}

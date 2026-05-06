package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

type CloudFSProvider struct {
	mountPath string
}

func NewCloudFSProvider(mountPath string) (*CloudFSProvider, error) {
	absPath, err := filepath.Abs(mountPath)
	if err != nil {
		return nil, err
	}
	// Ensure mount path exists
	err = os.MkdirAll(absPath, 0755)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{mountPath: absPath}, nil
}

func (p *CloudFSProvider) IsLocal() bool {
	return false
}

func (p *CloudFSProvider) getTenantPath(tenantID string) string {
	return filepath.Join(p.mountPath, tenantID)
}

func (p *CloudFSProvider) sanitizePath(tenantID, inputPath string) (string, error) {
	if tenantID == "" {
		return "", fmt.Errorf("tenant ID is required")
	}

	cleanPath := filepath.Clean(inputPath)
	tenantPath := p.getTenantPath(tenantID)
	fullPath := filepath.Join(tenantPath, cleanPath)

	// Fix security vulnerability by comparing with Rel
	rel, err := filepath.Rel(tenantPath, fullPath)
	if err != nil || strings.HasPrefix(rel, "..") {
		return "", fmt.Errorf("invalid path: escapes tenant directory")
	}

	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	tenantID, _ := ctx.Value(tenantIDKey{}).(string)
	fullPath, err := p.sanitizePath(tenantID, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	tenantID, _ := ctx.Value(tenantIDKey{}).(string)
	fullPath, err := p.sanitizePath(tenantID, path)
	if err != nil {
		return err
	}

	// Ensure directory exists
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(fullPath, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	tenantID, _ := ctx.Value(tenantIDKey{}).(string)
	fullPath, err := p.sanitizePath(tenantID, path)
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

func (p *CloudFSProvider) SearchFiles(ctx context.Context, query string, path string) ([]string, error) {
	tenantID, _ := ctx.Value(tenantIDKey{}).(string)
	fullPath, err := p.sanitizePath(tenantID, path)
	if err != nil {
		return nil, err
	}
	_, statErr := os.Stat(fullPath)
	if statErr != nil {
		return nil, statErr
	}

	tenantPath := p.getTenantPath(tenantID)
	var results []string

	err = filepath.Walk(fullPath, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil // Skip errors in walk
		}

		if !info.IsDir() && strings.Contains(info.Name(), query) {
			relPath, err := filepath.Rel(tenantPath, path)
			if err == nil {
				results = append(results, relPath)
			}
		}
		return nil
	})

	return results, err
}

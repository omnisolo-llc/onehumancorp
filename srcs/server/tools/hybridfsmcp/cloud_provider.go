package hybridfsmcp

import (
	"context"
	"errors"
	"io/fs"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

var validTenantRegex = regexp.MustCompile(`^[a-zA-Z0-9_-]+$`)

type CloudFSProvider struct {
	baseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{baseDir: baseDir}
}

func (p *CloudFSProvider) resolvePath(tenantID, reqPath string) (string, error) {
	if !validTenantRegex.MatchString(tenantID) {
		return "", errors.New("invalid tenant ID format")
	}

	tenantBase := filepath.Join(p.baseDir, tenantID)
	absPath := filepath.Join(tenantBase, reqPath)

	rel, err := filepath.Rel(tenantBase, absPath)
	if err != nil {
		return "", err
	}

	relSlash := filepath.ToSlash(rel)
	if relSlash == ".." || strings.HasPrefix(relSlash, "../") {
		return "", errors.New("directory traversal detected")
	}

	return absPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, tenantID, reqPath string) ([]byte, error) {
	resolved, err := p.resolvePath(tenantID, reqPath)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, tenantID, reqPath string, data []byte) error {
	resolved, err := p.resolvePath(tenantID, reqPath)
	if err != nil {
		return err
	}

	if err := os.MkdirAll(filepath.Dir(resolved), 0755); err != nil {
		return err
	}

	return os.WriteFile(resolved, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, tenantID, reqPath string) ([]string, error) {
	resolved, err := p.resolvePath(tenantID, reqPath)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolved)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, tenantID, reqPath, pattern string) ([]string, error) {
	resolved, err := p.resolvePath(tenantID, reqPath)
	if err != nil {
		return nil, err
	}

	tenantBase := filepath.Join(p.baseDir, tenantID)

	var matches []string
	err = filepath.WalkDir(resolved, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}

		if !d.IsDir() {
			matched, err := filepath.Match(pattern, d.Name())
			if err != nil {
				return err
			}
			if matched {
				relPath, err := filepath.Rel(tenantBase, path)
				if err == nil {
					matches = append(matches, filepath.ToSlash(relPath))
				}
			}
		}
		return nil
	})

	if err != nil {
		return nil, err
	}

	return matches, nil
}

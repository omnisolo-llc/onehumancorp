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

type CloudFSProvider struct {
	BaseDir string
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, reqPath string) (string, error) {
	orgID := auth.OrganizationIDFromContext(ctx)
	if orgID == "" {
		return "", errors.New("unauthorized: missing organization ID")
	}

	tenantDir := filepath.Join(p.BaseDir, orgID)
	cleanPath := filepath.Clean(filepath.Join(tenantDir, reqPath))

	absBase, err := filepath.Abs(tenantDir)
	if err != nil {
		return "", err
	}

	absClean, err := filepath.Abs(cleanPath)
	if err != nil {
		return "", err
	}

	if !strings.HasPrefix(absClean, absBase+string(filepath.Separator)) && absClean != absBase {
		return "", fmt.Errorf("path escapes tenant directory: %s", reqPath)
	}

	return absClean, nil
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

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(resolved)
	if err != nil {
		return nil, err
	}

	var names []string
	for _, e := range entries {
		names = append(names, e.Name())
	}
	return names, nil
}

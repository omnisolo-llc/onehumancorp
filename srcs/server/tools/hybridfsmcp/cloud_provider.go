package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type CloudFSProvider struct {
	baseVolumeDir string
}

func NewCloudFSProvider(baseVolumeDir string) (*CloudFSProvider, error) {
	abs, err := filepath.Abs(baseVolumeDir)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{baseVolumeDir: abs}, nil
}

func (p *CloudFSProvider) resolvePath(claims *auth.Claims, target string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("missing organization ID in claims")
	}
	cleanTarget := filepath.Clean(target)
	if filepath.IsAbs(cleanTarget) {
		return "", fmt.Errorf("absolute paths not allowed")
	}
	tenantDir := filepath.Join(p.baseVolumeDir, claims.OrganizationID)
	fullPath := filepath.Join(tenantDir, cleanTarget)

	if !strings.HasPrefix(fullPath, tenantDir) || (len(fullPath) > len(tenantDir) && fullPath[len(tenantDir)] != filepath.Separator) {
		if fullPath != tenantDir {
			return "", fmt.Errorf("path escapes tenant boundary")
		}
	}
	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
	fullPath, err := p.resolvePath(claims, path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0700); err != nil {
		return err
	}
	return os.WriteFile(fullPath, content, 0600)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	fullPath, err := p.resolvePath(claims, path)
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

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

type CloudFSProvider struct {
	VolumeMount string
}

func NewCloudFSProvider(volumeMount string) (*CloudFSProvider, error) {
	absMount, err := filepath.Abs(volumeMount)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{VolumeMount: absMount}, nil
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	orgID := auth.OrganizationIDFromContext(ctx)
	if orgID == "" {
		return "", errors.New("unauthorized: missing organization ID in context")
	}

	if filepath.IsAbs(filepath.Clean(target)) {
		return "", errors.New("absolute paths are not allowed")
	}

	tenantBase := filepath.Join(p.VolumeMount, orgID)

	// Ensure tenant directory exists
	if err := os.MkdirAll(tenantBase, 0755); err != nil {
		return "", err
	}

	fullPath := filepath.Join(tenantBase, target)
	fullPath = filepath.Clean(fullPath)

	if fullPath != tenantBase && !strings.HasPrefix(fullPath, tenantBase+string(filepath.Separator)) {
		return "", errors.New("path escapes tenant directory")
	}

	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}
	return os.WriteFile(fullPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.FileInfo, error) {
	fullPath, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, err
	}

	var infos []fs.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err == nil {
			infos = append(infos, info)
		}
	}
	return infos, nil
}

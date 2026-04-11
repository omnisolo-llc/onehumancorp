package hybridfsmcp

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// CloudFSProvider represents the tenant-scoped persistent volume in Cloud Mode
type CloudFSProvider struct {
	VolumeMount string
}

func NewCloudFSProvider(mountPath string) (*CloudFSProvider, error) {
	absMount, err := filepath.Abs(mountPath)
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{VolumeMount: absMount}, nil
}

func (p *CloudFSProvider) resolveTenantPath(ctx context.Context, reqPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing tenant claims")
	}

	tenantDir := filepath.Join(p.VolumeMount, claims.OrganizationID)

	cleanReq := strings.TrimPrefix(filepath.Clean(reqPath), "/")
	fullPath := filepath.Join(tenantDir, cleanReq)

	checkBase := tenantDir
	if !strings.HasSuffix(checkBase, string(filepath.Separator)) {
		checkBase += string(filepath.Separator)
	}

	if fullPath != tenantDir && !strings.HasPrefix(fullPath, checkBase) {
		return "", fmt.Errorf("path traversal denied outside tenant directory")
	}

	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	safePath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(safePath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	safePath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return err
	}

	dir := filepath.Dir(safePath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}

	return os.WriteFile(safePath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
	safePath, err := p.resolveTenantPath(ctx, path)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(safePath)
	if err != nil {
		return nil, err
	}

	var infos []FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue
		}
		infos = append(infos, FileInfo{
			Name:  entry.Name(),
			IsDir: entry.IsDir(),
			Size:  info.Size(),
		})
	}

	return infos, nil
}

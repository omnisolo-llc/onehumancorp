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
    RootVolumePath string
}

func NewCloudFSProvider(rootVolumePath string) *CloudFSProvider {
    return &CloudFSProvider{RootVolumePath: rootVolumePath}
}

func (p *CloudFSProvider) resolveTenantPath(ctx context.Context, targetPath string) (string, error) {
    claims := auth.ClaimsFromContext(ctx)
    if claims == nil {
        return "", fmt.Errorf("unauthorized: missing claims")
    }

    tenantID := claims.OrganizationID
    if tenantID == "" {
        return "", fmt.Errorf("unauthorized: missing tenant ID")
    }

    // Construct tenant-specific path
    tenantBase := filepath.Join(p.RootVolumePath, "tenants", tenantID)
    cleanBase := filepath.Clean(tenantBase)
    resolved := filepath.Clean(filepath.Join(cleanBase, targetPath))

    if resolved != cleanBase && !strings.HasPrefix(resolved, cleanBase+string(filepath.Separator)) {
        return "", fmt.Errorf("path %s is outside tenant directory", targetPath)
    }

    return resolved, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
    fullPath, err := p.resolveTenantPath(ctx, path)
    if err != nil {
        return nil, err
    }
    return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
    fullPath, err := p.resolveTenantPath(ctx, path)
    if err != nil {
        return err
    }

    // Ensure dir exists
    if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
        return err
    }

    return os.WriteFile(fullPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
    fullPath, err := p.resolveTenantPath(ctx, path)
    if err != nil {
        return nil, err
    }

    entries, err := os.ReadDir(fullPath)
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

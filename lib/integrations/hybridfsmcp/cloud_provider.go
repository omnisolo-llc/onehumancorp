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
    basePath string
}

func NewCloudFSProvider(basePath string) *CloudFSProvider {
    return &CloudFSProvider{basePath: basePath}
}

func (p *CloudFSProvider) IsLocal() bool {
    return false
}

func (p *CloudFSProvider) resolvePath(claims *auth.Claims, path string) (string, error) {
    if claims == nil || claims.OrganizationID == "" {
        return "", errors.New("unauthorized: missing organization ID")
    }
    tenantPath := filepath.Join(p.basePath, claims.OrganizationID)
    cleanPath := filepath.Clean(filepath.Join(tenantPath, path))

    // Append separator to ensure exact boundary check, preventing prefix hijacking
    // e.g. /base/tenant1 vs /base/tenant10
    tenantWithSep := filepath.Clean(tenantPath) + string(filepath.Separator)
    cleanWithSep := cleanPath
    if cleanPath != filepath.Clean(tenantPath) {
        cleanWithSep += string(filepath.Separator)
    }

    if !strings.HasPrefix(cleanWithSep, tenantWithSep) && cleanPath != filepath.Clean(tenantPath) {
        return "", fmt.Errorf("path escapes tenant directory: %s", path)
    }
    return cleanPath, nil
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
    err = os.MkdirAll(filepath.Dir(fullPath), 0755)
    if err != nil {
        return err
    }
    return os.WriteFile(fullPath, content, 0644)
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
    var files []string
    for _, entry := range entries {
        files = append(files, entry.Name())
    }
    return files, nil
}

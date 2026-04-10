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
    BaseDir string
}

func (p *CloudFSProvider) getTenantPath(ctx context.Context, path string) (string, error) {
    claims := auth.ClaimsFromContext(ctx)
    if claims == nil || claims.OrganizationID == "" {
        return "", fmt.Errorf("missing organization ID in claims")
    }

    // Ensure BaseDir is absolute to prevent unpredictable Join behavior
    baseDir, err := filepath.Abs(p.BaseDir)
    if err != nil {
        return "", err
    }

    // Prevent organization ID traversal
    orgID := filepath.Clean("/" + claims.OrganizationID)
    if orgID == "/" || orgID == "." || orgID == ".." {
         return "", fmt.Errorf("invalid organization ID")
    }

    tenantDir := filepath.Join(baseDir, orgID)

    // Convert path to clean absolute representation relative to tenantDir
    cleanPath := filepath.Clean(path)
    var fullPath string
    if filepath.IsAbs(cleanPath) {
        // Strip leading slash for absolute paths within MCP context
        fullPath = filepath.Join(tenantDir, cleanPath[1:])
    } else {
        fullPath = filepath.Join(tenantDir, cleanPath)
    }

    // Path Traversal Security Check: Ensure fullPath starts with tenantDir + separator
    tenantDirWithSep := tenantDir
    if !strings.HasSuffix(tenantDirWithSep, string(filepath.Separator)) {
        tenantDirWithSep += string(filepath.Separator)
    }

    if !strings.HasPrefix(fullPath, tenantDirWithSep) && fullPath != tenantDir {
        return "", fmt.Errorf("path bounds error: %s is outside %s", fullPath, tenantDirWithSep)
    }

    return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
    fullPath, err := p.getTenantPath(ctx, path)
    if err != nil {
        return nil, err
    }
    return os.ReadFile(fullPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
    fullPath, err := p.getTenantPath(ctx, path)
    if err != nil {
        return err
    }
    if err := os.MkdirAll(filepath.Dir(fullPath), 0755); err != nil {
        return err
    }
    return os.WriteFile(fullPath, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
    fullPath, err := p.getTenantPath(ctx, path)
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

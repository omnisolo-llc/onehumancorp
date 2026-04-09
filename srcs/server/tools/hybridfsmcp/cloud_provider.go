package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"path/filepath"
	"strings"
	"sync"
)

// CloudFSProvider implements a tenant-scoped mock virtual file system.
// In a real implementation, this could map to S3 or a K8s PVC scoped to the tenant.
type CloudFSProvider struct {
	mu    sync.RWMutex
	files map[string][]byte
}

// NewCloudFSProvider creates a new CloudFSProvider.
func NewCloudFSProvider() *CloudFSProvider {
	return &CloudFSProvider{
		files: make(map[string][]byte),
	}
}

// secureTenantPath resolves a tenant-specific path and enforces isolation.
func (p *CloudFSProvider) secureTenantPath(tenantID, target string) (string, error) {
	if tenantID == "" {
		return "", errors.New("missing tenant ID")
	}

	// Clean the path to prevent directory traversal
	cleanTarget := filepath.Clean(target)
	if strings.HasPrefix(cleanTarget, "..") || strings.Contains(cleanTarget, "/../") {
		return "", errors.New("access denied: path escapes bounds")
	}

	// Virtual path rooted at /tenant/{tenantID}/
	return fmt.Sprintf("/tenant/%s/%s", tenantID, cleanTarget), nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	tenantID := extractTenantID(ctx)
	secure, err := p.secureTenantPath(tenantID, path)
	if err != nil {
		return nil, err
	}

	p.mu.RLock()
	defer p.mu.RUnlock()

	data, ok := p.files[secure]
	if !ok {
		return nil, errors.New("file not found")
	}
	return data, nil
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	tenantID := extractTenantID(ctx)
	secure, err := p.secureTenantPath(tenantID, path)
	if err != nil {
		return err
	}

	p.mu.Lock()
	defer p.mu.Unlock()

	// Clone the content to prevent external modification
	clone := make([]byte, len(content))
	copy(clone, content)
	p.files[secure] = clone
	return nil
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	tenantID := extractTenantID(ctx)
	securePrefix, err := p.secureTenantPath(tenantID, path)
	if err != nil {
		return nil, err
	}

	if !strings.HasSuffix(securePrefix, "/") && securePrefix != fmt.Sprintf("/tenant/%s/.", tenantID) {
		securePrefix += "/"
	}
	// normalize root dir
	if securePrefix == fmt.Sprintf("/tenant/%s/./", tenantID) {
		securePrefix = fmt.Sprintf("/tenant/%s/", tenantID)
	}

	p.mu.RLock()
	defer p.mu.RUnlock()

	var entries []string
	seen := make(map[string]bool)

	for k := range p.files {
		if strings.HasPrefix(k, securePrefix) {
			rel := strings.TrimPrefix(k, securePrefix)
			// Extract the first path segment (either a file or a sub-directory)
			parts := strings.Split(rel, "/")
			if len(parts) > 0 && parts[0] != "" {
				name := parts[0]
				if !seen[name] {
					seen[name] = true
					entries = append(entries, name)
				}
			}
		}
	}

	return entries, nil
}

func (p *CloudFSProvider) IsLocal() bool {
	return false
}

// contextKey type for context value retrieval
type contextKey string

const TenantIDKey contextKey = "tenant_id"

// extractTenantID is a helper to pull the tenant ID from context
func extractTenantID(ctx context.Context) string {
	if val := ctx.Value(TenantIDKey); val != nil {
		if s, ok := val.(string); ok {
			return s
		}
	}
	return ""
}

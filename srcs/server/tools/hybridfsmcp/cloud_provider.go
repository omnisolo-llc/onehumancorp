package hybridfsmcp

import (
	"context"
	"fmt"
	"path/filepath"
	"strings"
	"sync"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type CloudFSProvider struct {
	mu sync.RWMutex
	// A mock representation of a virtual file system
	store map[string][]byte
}

func NewCloudFSProvider() *CloudFSProvider {
	return &CloudFSProvider{
		store: make(map[string][]byte),
	}
}

func (p *CloudFSProvider) getTenantPrefix(claims *auth.Claims) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing or invalid claims")
	}
	return fmt.Sprintf("/tenant/%s/", claims.OrganizationID), nil
}

func (p *CloudFSProvider) securePath(claims *auth.Claims, path string) (string, error) {
	prefix, err := p.getTenantPrefix(claims)
	if err != nil {
		return "", err
	}
    // Clean and normalize the provided path
	cleanPath := filepath.Clean(filepath.Join("/", path))
    // Form the absolute tenant path
	fullPath := filepath.Join(prefix, cleanPath)

	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	fullPath, err := p.securePath(claims, path)
	if err != nil {
		return nil, err
	}

	p.mu.RLock()
	defer p.mu.RUnlock()

	data, ok := p.store[fullPath]
	if !ok {
		return nil, fmt.Errorf("file not found: %s", path)
	}
	return data, nil
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	fullPath, err := p.securePath(claims, path)
	if err != nil {
		return err
	}

	p.mu.Lock()
	defer p.mu.Unlock()

	p.store[fullPath] = data
	return nil
}

func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	fullPath, err := p.securePath(claims, path)
	if err != nil {
		return nil, err
	}

	// Ensure fullPath ends with a slash for prefix matching
	if !strings.HasSuffix(fullPath, "/") {
		fullPath += "/"
	}

	var entries []string
    seen := make(map[string]bool)

	p.mu.RLock()
	defer p.mu.RUnlock()

	for key := range p.store {
		if strings.HasPrefix(key, fullPath) {
			relPath := strings.TrimPrefix(key, fullPath)
            if relPath == "" {
                continue // The directory itself (if it were a file entry)
            }
			parts := strings.SplitN(relPath, "/", 2)
            if len(parts) > 0 && !seen[parts[0]] {
                entries = append(entries, parts[0])
                seen[parts[0]] = true
            }
		}
	}
	return entries, nil
}

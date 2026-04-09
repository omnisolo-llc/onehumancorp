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
	baseVolume string
}

func NewCloudFSProvider(baseVolume string) *CloudFSProvider {
	return &CloudFSProvider{baseVolume: filepath.Clean(baseVolume)}
}

func (p *CloudFSProvider) validatePath(ctx context.Context, targetPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing tenant context")
	}
	cleanTarget := filepath.Clean(targetPath)
	if filepath.IsAbs(cleanTarget) {
		return "", fmt.Errorf("path must be relative to tenant root")
	}

	tenantRoot := filepath.Join(p.baseVolume, claims.OrganizationID)
	absPath := filepath.Join(tenantRoot, cleanTarget)

	if !strings.HasPrefix(filepath.Clean(absPath), tenantRoot+string(filepath.Separator)) && filepath.Clean(absPath) != tenantRoot {
		return "", fmt.Errorf("path access denied: outside tenant bounds")
	}

	return absPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	validPath, err := p.validatePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(validPath)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	validPath, err := p.validatePath(ctx, path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(validPath), 0755); err != nil {
		return err
	}
	return os.WriteFile(validPath, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	validPath, err := p.validatePath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(validPath)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

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
	mountPath string
}

func NewCloudFSProvider(mountPath string) (*CloudFSProvider, error) {
	abs, err := filepath.Abs(mountPath)
	if err != nil {
		return nil, err
	}
	if err := os.MkdirAll(abs, 0700); err != nil {
		return nil, err
	}
	return &CloudFSProvider{mountPath: abs}, nil
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, key string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing claims or organization id")
	}

	cleanKey := filepath.Clean("/" + key)
	cleanKey = strings.TrimPrefix(cleanKey, "/")

	orgID := claims.OrganizationID
	if cleanKey == orgID || strings.HasPrefix(cleanKey, orgID+"/") {
		// Key already contains the org ID correctly scoped
	} else {
		cleanKey = filepath.Join(orgID, cleanKey)
	}

	fullPath := filepath.Join(p.mountPath, cleanKey)
	orgBasePath := filepath.Join(p.mountPath, orgID)

	if !strings.HasPrefix(fullPath, orgBasePath+string(filepath.Separator)) && fullPath != orgBasePath {
		return "", fmt.Errorf("path escapes tenant boundary")
	}
	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, key string) ([]byte, error) {
	path, err := p.resolvePath(ctx, key)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(path)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, key string, content []byte) error {
	path, err := p.resolvePath(ctx, key)
	if err != nil {
		return err
	}
	dir := filepath.Dir(path)
	if err := os.MkdirAll(dir, 0700); err != nil {
		return err
	}
	return os.WriteFile(path, content, 0600)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, prefix string) ([]string, error) {
	path, err := p.resolvePath(ctx, prefix)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(path)
	if err != nil {
		return nil, err
	}
	var res []string
	for _, e := range entries {
		res = append(res, e.Name())
	}
	return res, nil
}

func (p *CloudFSProvider) SearchFiles(ctx context.Context, pattern string) ([]string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return nil, errors.New("unauthorized: missing claims")
	}

	orgBasePath := filepath.Join(p.mountPath, claims.OrganizationID)
	var res []string

	// Ensure directory exists so walk doesn't fail
	if _, err := os.Stat(orgBasePath); os.IsNotExist(err) {
		return res, nil
	}

	err := filepath.Walk(orgBasePath, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if !info.IsDir() && strings.Contains(info.Name(), pattern) {
			rel, err := filepath.Rel(orgBasePath, path)
			if err == nil {
				res = append(res, rel)
			}
		}
		return nil
	})
	return res, err
}

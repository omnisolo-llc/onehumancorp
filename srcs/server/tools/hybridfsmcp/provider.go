package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// FileSystemProvider abstracts the file system operations for the MCP server.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string, claims *auth.Claims) (map[string]interface{}, error)
	WriteFile(ctx context.Context, path string, content string, claims *auth.Claims) (map[string]interface{}, error)
	ListDir(ctx context.Context, path string, claims *auth.Claims) (map[string]interface{}, error)
}

// LocalFSProvider implements FileSystemProvider mapping to local file system with safety bounds.
type LocalFSProvider struct {
	basePath string
}

func NewLocalFSProvider(basePath string) *LocalFSProvider {
	if basePath == "" {
		basePath = os.TempDir()
	}
	return &LocalFSProvider{basePath: filepath.Clean(basePath)}
}

func (p *LocalFSProvider) validatePath(reqPath string) (string, error) {
	if filepath.IsAbs(reqPath) {
		return "", fmt.Errorf("absolute paths are not allowed: %s", reqPath)
	}

	fullPath := filepath.Join(p.basePath, filepath.Clean(reqPath))
	// Ensure the resolved path doesn't escape the base path.
	if fullPath != p.basePath && !strings.HasPrefix(fullPath, p.basePath+string(filepath.Separator)) {
		return "", fmt.Errorf("path traversal attempt: %s", reqPath)
	}
	return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, reqPath string, claims *auth.Claims) (map[string]interface{}, error) {
	fullPath, err := p.validatePath(reqPath)
	if err != nil {
		return nil, err
	}

	data, err := os.ReadFile(fullPath)
	if err != nil {
		return nil, fmt.Errorf("read file failed: %w", err)
	}

	return map[string]interface{}{
		"status":  "success",
		"content": string(data),
	}, nil
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, reqPath string, content string, claims *auth.Claims) (map[string]interface{}, error) {
	fullPath, err := p.validatePath(reqPath)
	if err != nil {
		return nil, err
	}

	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return nil, fmt.Errorf("create directory failed: %w", err)
	}

	if err := os.WriteFile(fullPath, []byte(content), 0644); err != nil {
		return nil, fmt.Errorf("write file failed: %w", err)
	}

	return map[string]interface{}{
		"status":  "success",
		"message": fmt.Sprintf("File %s written successfully", reqPath),
	}, nil
}

func (p *LocalFSProvider) ListDir(ctx context.Context, reqPath string, claims *auth.Claims) (map[string]interface{}, error) {
	fullPath, err := p.validatePath(reqPath)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		return nil, fmt.Errorf("read directory failed: %w", err)
	}

	var fileList []map[string]interface{}
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue // skip if we can't get info
		}
		fileList = append(fileList, map[string]interface{}{
			"name":  entry.Name(),
			"isDir": entry.IsDir(),
			"size":  info.Size(),
			"mode":  uint32(info.Mode()),
		})
	}

	return map[string]interface{}{
		"status": "success",
		"files":  fileList,
	}, nil
}

// CloudFSProvider implements FileSystemProvider mapping to Tenant-scoped storage.
// It uses auth.Claims to chroot the path per tenant.
type CloudFSProvider struct {
	cloudRoot string
}

func NewCloudFSProvider(cloudRoot string) *CloudFSProvider {
	if cloudRoot == "" {
		cloudRoot = "/mnt/cloud-volumes" // Example default persistent volume path
	}
	return &CloudFSProvider{cloudRoot: filepath.Clean(cloudRoot)}
}

func (p *CloudFSProvider) validatePath(reqPath string, claims *auth.Claims) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing or empty organization ID in claims")
	}

	if filepath.IsAbs(reqPath) {
		return "", fmt.Errorf("absolute paths are not allowed: %s", reqPath)
	}

	tenantRoot := filepath.Join(p.cloudRoot, claims.OrganizationID)
	fullPath := filepath.Join(tenantRoot, filepath.Clean(reqPath))

	// Ensure the resolved path doesn't escape the tenant root.
	if fullPath != tenantRoot && !strings.HasPrefix(fullPath, tenantRoot+string(filepath.Separator)) {
		return "", fmt.Errorf("path traversal attempt: %s", reqPath)
	}
	return fullPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, reqPath string, claims *auth.Claims) (map[string]interface{}, error) {
	fullPath, err := p.validatePath(reqPath, claims)
	if err != nil {
		return nil, err
	}

	data, err := os.ReadFile(fullPath)
	if err != nil {
		if errors.Is(err, fs.ErrNotExist) {
			return nil, fmt.Errorf("file not found: %s", reqPath)
		}
		return nil, fmt.Errorf("read file failed: %w", err)
	}

	return map[string]interface{}{
		"status":  "success",
		"content": string(data),
	}, nil
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, reqPath string, content string, claims *auth.Claims) (map[string]interface{}, error) {
	fullPath, err := p.validatePath(reqPath, claims)
	if err != nil {
		return nil, err
	}

	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return nil, fmt.Errorf("create directory failed: %w", err)
	}

	if err := os.WriteFile(fullPath, []byte(content), 0644); err != nil {
		return nil, fmt.Errorf("write file failed: %w", err)
	}

	return map[string]interface{}{
		"status":  "success",
		"message": fmt.Sprintf("File %s written successfully", reqPath),
	}, nil
}

func (p *CloudFSProvider) ListDir(ctx context.Context, reqPath string, claims *auth.Claims) (map[string]interface{}, error) {
	fullPath, err := p.validatePath(reqPath, claims)
	if err != nil {
		return nil, err
	}

	entries, err := os.ReadDir(fullPath)
	if err != nil {
		if errors.Is(err, fs.ErrNotExist) {
			return nil, fmt.Errorf("directory not found: %s", reqPath)
		}
		return nil, fmt.Errorf("read directory failed: %w", err)
	}

	var fileList []map[string]interface{}
	for _, entry := range entries {
		info, err := entry.Info()
		if err != nil {
			continue // skip if we can't get info
		}
		fileList = append(fileList, map[string]interface{}{
			"name":  entry.Name(),
			"isDir": entry.IsDir(),
			"size":  info.Size(),
			"mode":  uint32(info.Mode()),
		})
	}

	return map[string]interface{}{
		"status": "success",
		"files":  fileList,
	}, nil
}

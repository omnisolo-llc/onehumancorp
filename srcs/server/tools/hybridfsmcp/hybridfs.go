package hybridfsmcp


import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	fileSystemReadsTotal metric.Int64Counter
	fileSystemWritesTotal metric.Int64Counter
	fileSystemErrorsTotal metric.Int64Counter
)

func init() {
	meter := otel.Meter("github.com/onehumancorp/mono/ohc")
	var err error
	fileSystemReadsTotal, err = meter.Int64Counter("ohc_mcp_fs_reads_total", metric.WithDescription("Total file system reads by MCP"))
	if err != nil {
		panic(err)
	}
	fileSystemWritesTotal, err = meter.Int64Counter("ohc_mcp_fs_writes_total", metric.WithDescription("Total file system writes by MCP"))
	if err != nil {
		panic(err)
	}
	fileSystemErrorsTotal, err = meter.Int64Counter("ohc_mcp_fs_errors_total", metric.WithDescription("Total file system errors by MCP"))
	if err != nil {
		panic(err)
	}
}


type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

type LocalFSProvider struct {
	BaseDir string
}

func (l *LocalFSProvider) resolvePath(reqPath string) (string, error) {
	if filepath.IsAbs(filepath.Clean(reqPath)) {
		return "", fmt.Errorf("absolute paths are not allowed")
	}
	fullPath := filepath.Join(l.BaseDir, reqPath)
	rel, err := filepath.Rel(l.BaseDir, fullPath)
	if err != nil || strings.HasPrefix(rel, "..") {
		return "", fmt.Errorf("path traversal attempt detected")
	}
	return fullPath, nil
}

func (l *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	if fileSystemReadsTotal != nil {
		fileSystemReadsTotal.Add(ctx, 1)
	}
	fullPath, err := l.resolvePath(path)
	if err != nil {
		if fileSystemErrorsTotal != nil {
			fileSystemErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "ReadFile"), attribute.String("error", err.Error())))
		}
		return nil, err
	}
	data, err := os.ReadFile(fullPath)
	if err != nil && fileSystemErrorsTotal != nil {
		fileSystemErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "ReadFile"), attribute.String("error", err.Error())))
	}
	return data, err
}

func (l *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	if fileSystemWritesTotal != nil {
		fileSystemWritesTotal.Add(ctx, 1)
	}
	fullPath, err := l.resolvePath(path)
	if err != nil {
		if fileSystemErrorsTotal != nil {
			fileSystemErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "WriteFile"), attribute.String("error", err.Error())))
		}
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	err = os.WriteFile(fullPath, content, 0644)
	if err != nil && fileSystemErrorsTotal != nil {
		fileSystemErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "WriteFile"), attribute.String("error", err.Error())))
	}
	return err
}

func (l *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := l.resolvePath(path)
	if err != nil {
		if fileSystemErrorsTotal != nil {
			fileSystemErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "ReadFile"), attribute.String("error", err.Error())))
		}
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		if fileSystemErrorsTotal != nil {
			fileSystemErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "ReadFile"), attribute.String("error", err.Error())))
		}
		return nil, err
	}
	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

type CloudFSProvider struct {
	BaseDir string
}

func (c *CloudFSProvider) resolvePath(ctx context.Context, reqPath string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized or missing organization ID")
	}

	if filepath.IsAbs(filepath.Clean(reqPath)) {
		return "", fmt.Errorf("absolute paths are not allowed")
	}

	tenantDir := filepath.Join(c.BaseDir, claims.OrganizationID)
	fullPath := filepath.Join(tenantDir, reqPath)

	rel, err := filepath.Rel(tenantDir, fullPath)
	if err != nil || strings.HasPrefix(rel, "..") {
		return "", fmt.Errorf("path traversal attempt detected")
	}
	return fullPath, nil
}

func (c *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	if fileSystemReadsTotal != nil {
		fileSystemReadsTotal.Add(ctx, 1)
	}
	fullPath, err := c.resolvePath(ctx, path)
	if err != nil {
		if fileSystemErrorsTotal != nil {
			fileSystemErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "ReadFile"), attribute.String("error", err.Error())))
		}
		return nil, err
	}
	data, err := os.ReadFile(fullPath)
	if err != nil && fileSystemErrorsTotal != nil {
		fileSystemErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "ReadFile"), attribute.String("error", err.Error())))
	}
	return data, err
}

func (c *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	if fileSystemWritesTotal != nil {
		fileSystemWritesTotal.Add(ctx, 1)
	}
	fullPath, err := c.resolvePath(ctx, path)
	if err != nil {
		if fileSystemErrorsTotal != nil {
			fileSystemErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "WriteFile"), attribute.String("error", err.Error())))
		}
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	err = os.WriteFile(fullPath, content, 0644)
	if err != nil && fileSystemErrorsTotal != nil {
		fileSystemErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "WriteFile"), attribute.String("error", err.Error())))
	}
	return err
}

func (c *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := c.resolvePath(ctx, path)
	if err != nil {
		if fileSystemErrorsTotal != nil {
			fileSystemErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "ReadFile"), attribute.String("error", err.Error())))
		}
		return nil, err
	}
	entries, err := os.ReadDir(fullPath)
	if err != nil {
		if fileSystemErrorsTotal != nil {
			fileSystemErrorsTotal.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "ReadFile"), attribute.String("error", err.Error())))
		}
		return nil, err
	}
	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

func NewFileSystemProvider(mode string, baseDir string) (FileSystemProvider, error) {
	if mode == "OHC_STANDALONE" {
		return &LocalFSProvider{BaseDir: baseDir}, nil
	} else if mode == "OHC_MULTITENANT" {
		return &CloudFSProvider{BaseDir: baseDir}, nil
	}
	return nil, fmt.Errorf("unknown mode: %s", mode)
}

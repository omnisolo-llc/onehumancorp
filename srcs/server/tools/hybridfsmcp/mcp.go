package hybridfsmcp

import (
	"bytes"
	"context"
	"github.com/onehumancorp/mono/srcs/server/agents/mcp/proxy"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"

	"github.com/minio/minio-go/v7"
	"github.com/minio/minio-go/v7/pkg/credentials"
	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/utils"
)

// FileSystemProvider abstracts the file system operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
}

// LocalFSProvider implements FileSystemProvider for a bounded local directory.
type LocalFSProvider struct {
	baseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{baseDir: baseDir}
}

func (p *LocalFSProvider) sanitizePath(path string) (string, error) {
	if strings.Contains(path, "..") {
		return "", errors.New("directory traversal not allowed")
	}
	cleanPath := filepath.Clean("/" + path)
	cleanPath = strings.TrimPrefix(cleanPath, "/")
	if cleanPath == "" {
		return p.baseDir, nil
	}
	fullPath := filepath.Join(p.baseDir, cleanPath)
	return fullPath, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	fullPath, err := p.sanitizePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(fullPath)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	fullPath, err := p.sanitizePath(path)
	if err != nil {
		return err
	}
	dir := filepath.Dir(fullPath)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return err
	}
	return utils.WriteFileAtomic(fullPath, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	fullPath, err := p.sanitizePath(path)
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

// S3ClientInterface abstracts S3 methods used by CloudFSProvider to enable testing.
type S3ClientInterface interface {
	GetObject(ctx context.Context, bucketName, objectName string, opts minio.GetObjectOptions) (io.ReadCloser, error)
	PutObject(ctx context.Context, bucketName, objectName string, reader io.Reader, objectSize int64, opts minio.PutObjectOptions) (minio.UploadInfo, error)
	ListObjects(ctx context.Context, bucketName string, opts minio.ListObjectsOptions) <-chan minio.ObjectInfo
}

// CloudFSProvider implements FileSystemProvider using an S3 backend.
type CloudFSProvider struct {
	client     S3ClientInterface
	bucketName string
}

// NewCloudFSProvider creates a new CloudFSProvider.
func NewCloudFSProvider(client S3ClientInterface, bucketName string) *CloudFSProvider {
	return &CloudFSProvider{client: client, bucketName: bucketName}
}

func (p *CloudFSProvider) sanitizePath(ctx context.Context, path string) (string, error) {
	if strings.Contains(path, "..") {
		return "", errors.New("directory traversal not allowed")
	}
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing claims or organization ID")
	}

	cleanPath := filepath.Clean("/" + path)
	cleanPath = strings.TrimPrefix(cleanPath, "/")

	// S3 keys must be tenant/{org_id}/fs/{path}
	if cleanPath == "" {
		return fmt.Sprintf("tenant/%s/fs/", claims.OrganizationID), nil
	}
	s3Key := fmt.Sprintf("tenant/%s/fs/%s", claims.OrganizationID, cleanPath)
	return s3Key, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	s3Key, err := p.sanitizePath(ctx, path)
	if err != nil {
		return nil, err
	}
	object, err := p.client.GetObject(ctx, p.bucketName, s3Key, minio.GetObjectOptions{})
	if err != nil {
		return nil, err
	}
	defer object.Close()

	// MinIO GetObject returns a *minio.Object which implements io.Reader
	// Need to check for NotFound error when actually reading
	data, err := io.ReadAll(object)
	if err != nil {
		// minio-go returns error responses that we can check
		errResp := minio.ToErrorResponse(err)
		if errResp.Code == "NoSuchKey" {
			return nil, os.ErrNotExist
		}
		return nil, err
	}
	return data, nil
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	s3Key, err := p.sanitizePath(ctx, path)
	if err != nil {
		return err
	}

	reader := bytes.NewReader(data)
	_, err = p.client.PutObject(ctx, p.bucketName, s3Key, reader, int64(len(data)), minio.PutObjectOptions{
		ContentType: "application/octet-stream",
	})
	return err
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	s3Key, err := p.sanitizePath(ctx, path)
	if err != nil {
		return nil, err
	}

	// For S3 directory listing, we append a trailing slash if not empty
	prefix := s3Key
	if prefix != "" && !strings.HasSuffix(prefix, "/") {
		prefix += "/"
	}

	// We only want files directly under this prefix, not recursively
	// minio-go uses an empty string separator by default if not set? We should set it if we only want immediate children.
	opts := minio.ListObjectsOptions{
		Prefix:    prefix,
		Recursive: false, // Not recursive, we just want immediate children
	}

	var entries []string
	objectCh := p.client.ListObjects(ctx, p.bucketName, opts)
	for object := range objectCh {
		if object.Err != nil {
			return nil, object.Err
		}

		// Remove the prefix to get the relative name
		name := strings.TrimPrefix(object.Key, prefix)
		if name != "" {
			// Trim trailing slash for directories returned by minio if any
			name = strings.TrimSuffix(name, "/")
			entries = append(entries, name)
		}
	}

	return entries, nil
}

// HybridFSMCP implements the MCP interface for filesystem operations.
type HybridFSMCP struct {
	provider FileSystemProvider
	proxy    *proxy.McpSyncProxy
}

// NewHybridFSMCP creates a new HybridFSMCP instance.
func NewHybridFSMCP(provider FileSystemProvider, mcpProxy *proxy.McpSyncProxy) *HybridFSMCP {
	return &HybridFSMCP{
		provider: provider,
		proxy:    mcpProxy,
	}
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// ListTools returns the list of available tools.
func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the content of a file.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`),
		},
		{
			Name:        "list_directory",
			Description: "Lists the contents of a directory.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		data, err := m.provider.ReadFile(ctx, path)
		if err != nil {
			return nil, err
		}
		if m.proxy != nil {
			m.proxy.Buffer(ctx, toolName, arguments)
		}
		return map[string]interface{}{"content": string(data)}, nil
	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		content, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content' argument")
		}
		err := m.provider.WriteFile(ctx, path, []byte(content))
		if err != nil {
			return nil, err
		}
		if m.proxy != nil {
			m.proxy.Buffer(ctx, toolName, arguments)
		}
		return map[string]interface{}{"status": "success"}, nil
	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		entries, err := m.provider.ListDir(ctx, path)
		if err != nil {
			return nil, err
		}
		if m.proxy != nil {
			m.proxy.Buffer(ctx, toolName, arguments)
		}
		return map[string]interface{}{"entries": entries}, nil
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

// RealS3ClientWrapper wraps a real *minio.Client to implement S3ClientInterface
type RealS3ClientWrapper struct {
	client *minio.Client
}

func (w *RealS3ClientWrapper) GetObject(ctx context.Context, bucketName, objectName string, opts minio.GetObjectOptions) (io.ReadCloser, error) {
	obj, err := w.client.GetObject(ctx, bucketName, objectName, opts)
	if err != nil {
		return nil, err
	}
	return obj, nil
}

func (w *RealS3ClientWrapper) PutObject(ctx context.Context, bucketName, objectName string, reader io.Reader, objectSize int64, opts minio.PutObjectOptions) (minio.UploadInfo, error) {
	return w.client.PutObject(ctx, bucketName, objectName, reader, objectSize, opts)
}

func (w *RealS3ClientWrapper) ListObjects(ctx context.Context, bucketName string, opts minio.ListObjectsOptions) <-chan minio.ObjectInfo {
	return w.client.ListObjects(ctx, bucketName, opts)
}

// NewProviderFactory returns a FileSystemProvider based on environment configuration.
func NewProviderFactory(baseDir string) (FileSystemProvider, error) {
	if envBoolDefault("OHC_MULTITENANT", false) {
		endpoint := os.Getenv("S3_ENDPOINT")
		accessKey := os.Getenv("S3_ACCESS_KEY")
		secretKey := os.Getenv("S3_SECRET_KEY")
		bucketName := os.Getenv("S3_BUCKET_NAME")
		if bucketName == "" {
			bucketName = "ohc-tenant-data"
		}

		if endpoint == "" {
			return nil, errors.New("S3_ENDPOINT is required in Cloud Mode")
		}

		// Initialize minio client object.
		minioClient, err := minio.New(endpoint, &minio.Options{
			Creds:  credentials.NewStaticV4(accessKey, secretKey, ""),
			Secure: envBoolDefault("S3_SECURE", true),
		})
		if err != nil {
			return nil, err
		}

		wrapper := &RealS3ClientWrapper{client: minioClient}
		return NewCloudFSProvider(wrapper, bucketName), nil
	}
	return NewLocalFSProvider(baseDir), nil
}

func envBoolDefault(key string, fallback bool) bool {
	val := os.Getenv(key)
	if val == "" {
		return fallback
	}
	return strings.ToLower(val) == "true" || val == "1"
}

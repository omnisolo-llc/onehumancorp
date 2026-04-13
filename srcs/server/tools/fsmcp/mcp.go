package fsmcp

import (
	"bytes"
	"context"
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
	fullPath := filepath.Join(p.baseDir, cleanPath)

    if !strings.HasPrefix(fullPath, filepath.Clean(p.baseDir)+string(filepath.Separator)) && fullPath != filepath.Clean(p.baseDir) {
        return "", errors.New("directory traversal not allowed")
    }

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
	return os.WriteFile(fullPath, data, 0644)
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

// CloudFSProvider implements FileSystemProvider for an S3-compatible environment.
type CloudFSProvider struct {
	client     *minio.Client
	bucketName string
}

func NewCloudFSProvider(endpoint, accessKeyID, secretAccessKey, bucketName string, useSSL bool) (*CloudFSProvider, error) {
	minioClient, err := minio.New(endpoint, &minio.Options{
		Creds:  credentials.NewStaticV4(accessKeyID, secretAccessKey, ""),
		Secure: useSSL,
	})
	if err != nil {
		return nil, err
	}
	return &CloudFSProvider{
		client:     minioClient,
		bucketName: bucketName,
	}, nil
}

func (p *CloudFSProvider) getTenantPrefix(ctx context.Context) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing claims or organization ID")
	}
	return fmt.Sprintf("tenant/%s/fs/", claims.OrganizationID), nil
}

func (p *CloudFSProvider) sanitizePath(ctx context.Context, path string) (string, error) {
	if strings.Contains(path, "..") {
		return "", errors.New("directory traversal not allowed")
	}
	tenantPrefix, err := p.getTenantPrefix(ctx)
	if err != nil {
		return "", err
	}
	cleanPath := filepath.Clean("/" + path)
	cleanPath = strings.TrimPrefix(cleanPath, "/")
	return tenantPrefix + cleanPath, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	objectKey, err := p.sanitizePath(ctx, path)
	if err != nil {
		return nil, err
	}
	object, err := p.client.GetObject(ctx, p.bucketName, objectKey, minio.GetObjectOptions{})
	if err != nil {
		return nil, err
	}
	defer object.Close()

	var buf bytes.Buffer
	if _, err := io.Copy(&buf, object); err != nil {
		return nil, err
	}
	return buf.Bytes(), nil
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	objectKey, err := p.sanitizePath(ctx, path)
	if err != nil {
		return err
	}
	reader := bytes.NewReader(data)
	_, err = p.client.PutObject(ctx, p.bucketName, objectKey, reader, int64(len(data)), minio.PutObjectOptions{})
	return err
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	objectPrefix, err := p.sanitizePath(ctx, path)
	if err != nil {
		return nil, err
	}
	if objectPrefix != "" && !strings.HasSuffix(objectPrefix, "/") {
		objectPrefix += "/"
	}

	opts := minio.ListObjectsOptions{
		Prefix:    objectPrefix,
		Recursive: false,
	}

	var names []string
	for object := range p.client.ListObjects(ctx, p.bucketName, opts) {
		if object.Err != nil {
			return nil, object.Err
		}

		name := strings.TrimPrefix(object.Key, objectPrefix)
        if name != "" {
		    names = append(names, name)
        }
	}
	return names, nil
}

// FSMCP implements the MCP interface for filesystem operations.
type FSMCP struct {
	provider FileSystemProvider
}

// NewFSMCP creates a new FSMCP instance.
func NewFSMCP(provider FileSystemProvider) *FSMCP {
	return &FSMCP{
		provider: provider,
	}
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// ListTools returns the list of available tools.
func (m *FSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "fs_read",
			Description: "Reads the content of a file.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
		{
			Name:        "fs_write",
			Description: "Writes content to a file.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`),
		},
		{
			Name:        "fs_list",
			Description: "Lists the contents of a directory.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
	}
}

// CallTool executes a tool by name.
func (m *FSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "fs_read":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		data, err := m.provider.ReadFile(ctx, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"content": string(data)}, nil
	case "fs_write":
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
		return map[string]interface{}{"status": "success"}, nil
	case "fs_list":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		entries, err := m.provider.ListDir(ctx, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"entries": entries}, nil
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

// NewProviderFactory returns a FileSystemProvider based on environment configuration.
func NewProviderFactory() (FileSystemProvider, error) {
	if envBoolDefault("OHC_STANDALONE", false) {
        baseDir := os.Getenv("OHC_FS_BASE_DIR")
        if baseDir == "" {
            homeDir, err := os.UserHomeDir()
            if err != nil {
                return nil, fmt.Errorf("could not get home dir: %w", err)
            }
            baseDir = filepath.Join(homeDir, ".ohc-local-data", "fs")
        }
		return NewLocalFSProvider(baseDir), nil
	}

    endpoint := os.Getenv("S3_ENDPOINT")
    accessKey := os.Getenv("S3_ACCESS_KEY")
    secretKey := os.Getenv("S3_SECRET_KEY")
    bucketName := os.Getenv("S3_BUCKET_NAME")

    if endpoint == "" || accessKey == "" || secretKey == "" || bucketName == "" {
        return nil, errors.New("missing S3 configuration environment variables")
    }

    useSSL := envBoolDefault("S3_USE_SSL", true)

	return NewCloudFSProvider(endpoint, accessKey, secretKey, bucketName, useSSL)
}

func envBoolDefault(key string, fallback bool) bool {
	val := os.Getenv(key)
	if val == "" {
		return fallback
	}
	return strings.ToLower(val) == "true" || val == "1"
}

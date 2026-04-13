package hybridfsmcp

import (
	"bytes"
	"context"
	"io"
	"strings"
	"testing"

	"github.com/minio/minio-go/v7"
	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Test write and read
	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if string(data) != "hello" {
		t.Fatalf("expected 'hello', got '%s'", string(data))
	}

	// Test list dir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Fatalf("expected ['test.txt'], got %v", entries)
	}

	// Test directory traversal prevention
	_, err = provider.ReadFile(ctx, "../../../etc/passwd")
	if err == nil {
		t.Fatalf("expected error on traversal, got nil")
	}
}

type MockReadCloser struct {
	io.Reader
}

func (m MockReadCloser) Close() error { return nil }

type MockS3Client struct {
	objects map[string][]byte
}

func NewMockS3Client() *MockS3Client {
	return &MockS3Client{
		objects: make(map[string][]byte),
	}
}

func (m *MockS3Client) GetObject(ctx context.Context, bucketName, objectName string, opts minio.GetObjectOptions) (io.ReadCloser, error) {
	data, ok := m.objects[objectName]
	if !ok {
		// Mock minio error
		return nil, minio.ErrorResponse{Code: "NoSuchKey"}
	}
	return MockReadCloser{bytes.NewReader(data)}, nil
}

func (m *MockS3Client) PutObject(ctx context.Context, bucketName, objectName string, reader io.Reader, objectSize int64, opts minio.PutObjectOptions) (minio.UploadInfo, error) {
	data, err := io.ReadAll(reader)
	if err != nil {
		return minio.UploadInfo{}, err
	}
	m.objects[objectName] = data
	return minio.UploadInfo{}, nil
}

func (m *MockS3Client) ListObjects(ctx context.Context, bucketName string, opts minio.ListObjectsOptions) <-chan minio.ObjectInfo {
	ch := make(chan minio.ObjectInfo)
	go func() {
		defer close(ch)
		for k := range m.objects {
			if strings.HasPrefix(k, opts.Prefix) {
				ch <- minio.ObjectInfo{Key: k}
			}
		}
	}()
	return ch
}

func TestCloudFSProvider(t *testing.T) {
	mockClient := NewMockS3Client()
	provider := NewCloudFSProvider(mockClient, "test-bucket")

	claims := &auth.Claims{
		OrganizationID: "tenant1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test write and read
	err := provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if string(data) != "hello cloud" {
		t.Fatalf("expected 'hello cloud', got '%s'", string(data))
	}

	// Verify tenant isolation in S3 key
	if _, ok := mockClient.objects["tenant/tenant1/fs/test.txt"]; !ok {
		t.Fatalf("expected file to be created with tenant prefix in S3")
	}

	// Test with no claims
	ctxNoClaims := context.Background()
	err = provider.WriteFile(ctxNoClaims, "test.txt", []byte("fail"))
	if err == nil {
		t.Fatalf("expected error without claims")
	}
}

func TestHybridFSMCP(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	// List tools
	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}

	// Write file
	_, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "mcp.txt",
		"content": "mcp content",
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Read file
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "mcp.txt",
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["content"] != "mcp content" {
		t.Fatalf("expected 'mcp content', got '%v'", resMap["content"])
	}

	// List dir
	listRes, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	listResMap := listRes.(map[string]interface{})
	entries := listResMap["entries"].([]string)
	if len(entries) != 1 || entries[0] != "mcp.txt" {
		t.Fatalf("expected ['mcp.txt'], got %v", entries)
	}
}

func TestNewProviderFactory(t *testing.T) {
	t.Setenv("OHC_MULTITENANT", "true")
	t.Setenv("S3_ENDPOINT", "localhost:9000")
	t.Setenv("S3_ACCESS_KEY", "minio")
	t.Setenv("S3_SECRET_KEY", "minio123")
	provider, err := NewProviderFactory("/tmp")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if _, ok := provider.(*CloudFSProvider); !ok {
		t.Fatalf("expected CloudFSProvider")
	}

	t.Setenv("OHC_MULTITENANT", "false")
	provider, err = NewProviderFactory("/tmp")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if _, ok := provider.(*LocalFSProvider); !ok {
		t.Fatalf("expected LocalFSProvider")
	}

	t.Setenv("OHC_MULTITENANT", "true")
	t.Setenv("S3_ENDPOINT", "")
	_, err = NewProviderFactory("/tmp")
	if err == nil {
		t.Fatalf("expected error when S3_ENDPOINT is missing")
	}
}

func TestLocalFSProvider_Errors(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Test read non-existent file
	_, err := provider.ReadFile(ctx, "nonexistent.txt")
	if err == nil {
		t.Fatalf("expected error reading nonexistent file")
	}

	// Test write directory traversal
	err = provider.WriteFile(ctx, "../../../etc/passwd", []byte("hacked"))
	if err == nil {
		t.Fatalf("expected error writing with traversal")
	}

	// Test list dir traversal
	_, err = provider.ListDir(ctx, "../../../etc")
	if err == nil {
		t.Fatalf("expected error listing with traversal")
	}

	// Test list non-existent dir
	_, err = provider.ListDir(ctx, "nonexistent")
	if err == nil {
		t.Fatalf("expected error listing nonexistent dir")
	}
}

func TestCloudFSProvider_Errors(t *testing.T) {
	mockClient := NewMockS3Client()
	provider := NewCloudFSProvider(mockClient, "test-bucket")

	claims := &auth.Claims{
		OrganizationID: "tenant1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test read non-existent file
	_, err := provider.ReadFile(ctx, "nonexistent.txt")
	if err == nil {
		t.Fatalf("expected error reading nonexistent file")
	}

	// Test read traversal
	_, err = provider.ReadFile(ctx, "../../../etc/passwd")
	if err == nil {
		t.Fatalf("expected error reading with traversal")
	}

	// Test write traversal
	err = provider.WriteFile(ctx, "../../../etc/passwd", []byte("hacked"))
	if err == nil {
		t.Fatalf("expected error writing with traversal")
	}

	// Test list dir traversal
	_, err = provider.ListDir(ctx, "../../../etc")
	if err == nil {
		t.Fatalf("expected error listing with traversal")
	}

	// Test missing claims
	ctxNoClaims := context.Background()
	_, err = provider.ReadFile(ctxNoClaims, "test.txt")
	if err == nil {
		t.Fatalf("expected error without claims")
	}

	_, err = provider.ListDir(ctxNoClaims, ".")
	if err == nil {
		t.Fatalf("expected error without claims")
	}
}

func TestHybridFSMCP_Errors(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	// Test unknown tool
	_, err := mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Fatalf("expected error for unknown tool")
	}

	// Test read_file missing path
	_, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{})
	if err == nil {
		t.Fatalf("expected error for missing path in read_file")
	}

	// Test read_file provider error
	_, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "nonexistent.txt",
	})
	if err == nil {
		t.Fatalf("expected error for provider error in read_file")
	}

	// Test write_file missing path
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"content": "test",
	})
	if err == nil {
		t.Fatalf("expected error for missing path in write_file")
	}

	// Test write_file missing content
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err == nil {
		t.Fatalf("expected error for missing content in write_file")
	}

	// Test write_file traversal error
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "../test.txt",
		"content": "test",
	})
	if err == nil {
		t.Fatalf("expected error for provider error in write_file")
	}

	// Test list_directory missing path
	_, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{})
	if err == nil {
		t.Fatalf("expected error for missing path in list_directory")
	}

	// Test list_directory provider error
	_, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": "nonexistent_dir",
	})
	if err == nil {
		t.Fatalf("expected error for provider error in list_directory")
	}
}

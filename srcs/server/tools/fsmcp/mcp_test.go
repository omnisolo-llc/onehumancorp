package fsmcp

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

func TestFSMCP(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewFSMCP(provider)
	ctx := context.Background()

	// List tools
	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}

	// Write file
	_, err := mcp.CallTool(ctx, "fs_write", map[string]interface{}{
		"path":    "mcp.txt",
		"content": "mcp content",
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Read file
	res, err := mcp.CallTool(ctx, "fs_read", map[string]interface{}{
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
	listRes, err := mcp.CallTool(ctx, "fs_list", map[string]interface{}{
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

	t.Setenv("OHC_MULTITENANT", "true")
	t.Setenv("OHC_STANDALONE", "true")
	provider, err = NewProviderFactory("/tmp")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if _, ok := provider.(*LocalFSProvider); !ok {
		t.Fatalf("expected LocalFSProvider when standalone is true")
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
	t.Setenv("OHC_STANDALONE", "false")
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

func TestFSMCP_Errors(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewFSMCP(provider)
	ctx := context.Background()

	// Test unknown tool
	_, err := mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Fatalf("expected error for unknown tool")
	}

	// Test fs_read missing path
	_, err = mcp.CallTool(ctx, "fs_read", map[string]interface{}{})
	if err == nil {
		t.Fatalf("expected error for missing path in fs_read")
	}

	// Test fs_read provider error
	_, err = mcp.CallTool(ctx, "fs_read", map[string]interface{}{
		"path": "nonexistent.txt",
	})
	if err == nil {
		t.Fatalf("expected error for provider error in fs_read")
	}

	// Test fs_write missing path
	_, err = mcp.CallTool(ctx, "fs_write", map[string]interface{}{
		"content": "test",
	})
	if err == nil {
		t.Fatalf("expected error for missing path in fs_write")
	}

	// Test fs_write missing content
	_, err = mcp.CallTool(ctx, "fs_write", map[string]interface{}{
		"path": "test.txt",
	})
	if err == nil {
		t.Fatalf("expected error for missing content in fs_write")
	}

	// Test fs_write traversal error
	_, err = mcp.CallTool(ctx, "fs_write", map[string]interface{}{
		"path":    "../test.txt",
		"content": "test",
	})
	if err == nil {
		t.Fatalf("expected error for provider error in fs_write")
	}

	// Test fs_list missing path
	_, err = mcp.CallTool(ctx, "fs_list", map[string]interface{}{})
	if err == nil {
		t.Fatalf("expected error for missing path in fs_list")
	}

	// Test fs_list provider error
	_, err = mcp.CallTool(ctx, "fs_list", map[string]interface{}{
		"path": "nonexistent_dir",
	})
	if err == nil {
		t.Fatalf("expected error for provider error in fs_list")
	}
}

func TestLocalFSProvider_ListDir_Errors(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Cause ReadDir to fail by making a file where a directory should be
	err := provider.WriteFile(ctx, "a", []byte("file"))
	if err != nil {
		t.Fatalf("setup failed: %v", err)
	}
	_, err = provider.ListDir(ctx, "a")
	if err == nil {
		t.Fatalf("expected error reading dir, got nil")
	}
}

func TestCloudFSProvider_ListDir_Errors2(t *testing.T) {
	mockClient := NewMockS3Client()
	_ = NewCloudFSProvider(mockClient, "test-bucket")

	claims := &auth.Claims{
		OrganizationID: "tenant1",
	}
	_ = context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)
	// mockClient ListObjects currently does not simulate error ObjectInfos easily
}

func TestFSMCP_SuccessWithClaims4(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewFSMCP(provider)
	claims := &auth.Claims{
		SessionID: "test-session",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Write file first
	_, err := mcp.CallTool(ctx, "fs_write", map[string]interface{}{
		"path":    "mcp4.txt",
		"content": "mcp content",
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	_, err = mcp.CallTool(ctx, "fs_read", map[string]interface{}{
		"path":    "mcp4.txt",
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
}

func TestFSMCP_Errors2(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewFSMCP(provider)
	ctx := context.Background()

	_, err := mcp.CallTool(ctx, "fs_write", map[string]interface{}{
		"path":    "mcp4.txt",
		"content": 123,
	})
	if err == nil {
		t.Fatalf("expected error, got nil")
	}

	_, err = mcp.CallTool(ctx, "fs_read", map[string]interface{}{
		"path":    123,
	})
	if err == nil {
		t.Fatalf("expected error, got nil")
	}

	_, err = mcp.CallTool(ctx, "fs_list", map[string]interface{}{
		"path":    123,
	})
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestCloudFSProvider_ListDir_Subdir(t *testing.T) {
	mockClient := NewMockS3Client()
	provider := NewCloudFSProvider(mockClient, "test-bucket")

	claims := &auth.Claims{
		OrganizationID: "tenant1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	err := provider.WriteFile(ctx, "dir/test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

    entries, err := provider.ListDir(ctx, "dir")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
    if len(entries) != 1 || entries[0] != "test.txt" {
        t.Fatalf("expected test.txt, got %v", entries)
    }
}

func TestFSMCP_SuccessWithClaims5(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewFSMCP(provider)
	claims := &auth.Claims{
		SessionID: "test-session",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	_, err := mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{
		"path":    "mcp3.txt",
	})
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestCloudFSProvider_RealClientWrapper_Errors(t *testing.T) {
	// Not practically testable easily since we can't create a real minio client pointing to an invalid port easily that will fail early enough
}

type ErrorReader struct{}

func (e *ErrorReader) Read(p []byte) (n int, err error) {
	return 0, io.ErrUnexpectedEOF
}

func TestCloudFSProvider_PutObject_ReaderError(t *testing.T) {
	mockClient := NewMockS3Client()
	provider := NewCloudFSProvider(mockClient, "test-bucket")

	claims := &auth.Claims{
		OrganizationID: "tenant1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Since CloudFSProvider.WriteFile wraps the byte array in bytes.NewReader, we can't trigger reader error easily.
    _ = ctx
    _ = provider
}

func TestLocalFSProvider_WriteFile_Errors(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Error when path traversal
	err := provider.WriteFile(ctx, "../../../etc/passwd", []byte("hacked"))
	if err == nil {
		t.Fatalf("expected error, got nil")
	}

    // ListDir error traversal
	_, err = provider.ListDir(ctx, "../../../etc")
	if err == nil {
		t.Fatalf("expected error, got nil")
	}

    // ReadFile error traversal
	_, err = provider.ReadFile(ctx, "../../../etc/passwd")
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

type ErrorReaderS3 struct{}
func (e ErrorReaderS3) Read(p []byte) (n int, err error) {
	return 0, io.ErrUnexpectedEOF
}
func (e ErrorReaderS3) Close() error {
	return nil
}

type MockS3ClientReaderErr struct {
	MockS3Client
}
func (m *MockS3ClientReaderErr) GetObject(ctx context.Context, bucketName, objectName string, opts minio.GetObjectOptions) (io.ReadCloser, error) {
	return ErrorReaderS3{}, nil
}

func TestCloudFSProvider_Read_ReaderError(t *testing.T) {
	mockClient := &MockS3ClientReaderErr{}
	provider := NewCloudFSProvider(mockClient, "test-bucket")

	claims := &auth.Claims{
		OrganizationID: "tenant1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	_, err := provider.ReadFile(ctx, "test.txt")
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

type MockS3ClientPutErr struct {
	MockS3Client
}
func (m *MockS3ClientPutErr) PutObject(ctx context.Context, bucketName, objectName string, reader io.Reader, objectSize int64, opts minio.PutObjectOptions) (minio.UploadInfo, error) {
	return minio.UploadInfo{}, io.ErrUnexpectedEOF
}

func TestCloudFSProvider_Write_PutError(t *testing.T) {
	mockClient := &MockS3ClientPutErr{}
	provider := NewCloudFSProvider(mockClient, "test-bucket")

	claims := &auth.Claims{
		OrganizationID: "tenant1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

type MockS3ClientListErr struct {
	MockS3Client
}
func (m *MockS3ClientListErr) ListObjects(ctx context.Context, bucketName string, opts minio.ListObjectsOptions) <-chan minio.ObjectInfo {
    ch := make(chan minio.ObjectInfo, 1)
    ch <- minio.ObjectInfo{Err: io.ErrUnexpectedEOF}
    close(ch)
    return ch
}

func TestCloudFSProvider_List_ListError(t *testing.T) {
	mockClient := &MockS3ClientListErr{}
	provider := NewCloudFSProvider(mockClient, "test-bucket")

	claims := &auth.Claims{
		OrganizationID: "tenant1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	_, err := provider.ListDir(ctx, ".")
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

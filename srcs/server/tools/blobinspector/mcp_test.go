package blobinspector

import (
	"context"
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/storage"
)

// MockS3Provider is a mock for testing the S3 Cloud provider.
type MockS3Provider struct {
	blobs map[string]storage.BlobMetadata
}

func (m *MockS3Provider) IsLocal() bool {
	return false
}

func (m *MockS3Provider) ListBlobs(ctx context.Context, prefix string) ([]storage.BlobMetadata, error) {
	var result []storage.BlobMetadata
	for k, v := range m.blobs {
		if prefix == "" || filepath.HasPrefix(k, prefix) {
			result = append(result, v)
		}
	}
	return result, nil
}

func (m *MockS3Provider) ReadBlobMetadata(ctx context.Context, key string) (storage.BlobMetadata, error) {
	meta, ok := m.blobs[key]
	if !ok {
		return storage.BlobMetadata{}, os.ErrNotExist
	}
	return meta, nil
}

func (m *MockS3Provider) GetBlobURL(ctx context.Context, key string) (string, error) {
	if _, ok := m.blobs[key]; !ok {
		return "", os.ErrNotExist
	}
	return "s3://bucket/" + key, nil
}

func TestBlobInspectorMCP_ListTools(t *testing.T) {
	provider, err := storage.NewLocalProvider(t.TempDir())
	if err != nil {
		t.Fatal(err)
	}
	inspector := NewBlobInspectorMCP(provider)
	tools := inspector.ListTools()
	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}
}

func TestBlobInspectorMCP_LocalMode(t *testing.T) {
	// Create a temp directory and a file
	tmpDir, err := os.MkdirTemp("", "blobtest")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	testFilePath := filepath.Join(tmpDir, "test.txt")
	err = os.WriteFile(testFilePath, []byte("hello"), 0644)
	if err != nil {
		t.Fatal(err)
	}

	provider, err := storage.NewLocalProvider(tmpDir)
	if err != nil {
		t.Fatal(err)
	}
	inspector := NewBlobInspectorMCP(provider)

	ctx := context.Background()

	// Test list_blobs
	res, err := inspector.CallTool(ctx, "list_blobs", map[string]interface{}{"prefix": ""})
	if err != nil {
		t.Fatal(err)
	}
	blobsMap := res.(map[string]interface{})
	blobs := blobsMap["results"].([]map[string]interface{})
	if len(blobs) != 1 || blobs[0]["key"] != "test.txt" {
		t.Fatalf("unexpected list result: %v", blobs)
	}

	// Test read_blob_metadata
	res, err = inspector.CallTool(ctx, "read_blob_metadata", map[string]interface{}{"key": "test.txt"})
	if err != nil {
		t.Fatal(err)
	}
	meta := res.(map[string]interface{})
	if meta["size"] != int64(5) {
		t.Fatalf("expected size 5, got %v", meta["size"])
	}

	// Test get_blob_url
	res, err = inspector.CallTool(ctx, "get_blob_url", map[string]interface{}{"key": "test.txt"})
	if err != nil {
		t.Fatal(err)
	}
	url := res.(map[string]interface{})["url"].(string)
	if url != "file://"+testFilePath {
		t.Fatalf("unexpected url: %s", url)
	}
}

func TestBlobInspectorMCP_CloudMode_TenantIsolation(t *testing.T) {
	provider := &MockS3Provider{
		blobs: map[string]storage.BlobMetadata{
			"tenant1/image.png": {Key: "tenant1/image.png", Size: int64(100), LastModified: time.Now()},
			"tenant2/data.txt":  {Key: "tenant2/data.txt", Size: int64(200), LastModified: time.Now()},
		},
	}
	inspector := NewBlobInspectorMCP(provider)

	// No claims should fail
	ctx := context.Background()
	_, err := inspector.CallTool(ctx, "list_blobs", map[string]interface{}{})
	if err == nil {
		t.Fatal("expected error due to missing claims")
	}

	// With claims
	claims := &auth.Claims{OrganizationID: "tenant1"}
	ctxWithAuth := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	// list_blobs
	res, err := inspector.CallTool(ctxWithAuth, "list_blobs", map[string]interface{}{})
	if err != nil {
		t.Fatal(err)
	}
	blobsMap := res.(map[string]interface{})
	blobs := blobsMap["results"].([]map[string]interface{})
	if len(blobs) != 1 || blobs[0]["key"] != "image.png" {
		t.Fatalf("unexpected list result: %v", blobs)
	}

	// read_blob_metadata
	res, err = inspector.CallTool(ctxWithAuth, "read_blob_metadata", map[string]interface{}{"key": "image.png"})
	if err != nil {
		t.Fatal(err)
	}
	meta := res.(map[string]interface{})
	if meta["size"] != int64(100) {
		t.Fatalf("expected size 100, got %v", meta["size"])
	}

	// get_blob_url
	res, err = inspector.CallTool(ctxWithAuth, "get_blob_url", map[string]interface{}{"key": "image.png"})
	if err != nil {
		t.Fatal(err)
	}
	url := res.(map[string]interface{})["url"].(string)
	if url != "s3://bucket/tenant1/image.png" {
		t.Fatalf("unexpected url: %s", url)
	}

	// Unauthorized access to another tenant's blob
	_, err = inspector.CallTool(ctxWithAuth, "read_blob_metadata", map[string]interface{}{"key": "tenant2/data.txt"})
	if err == nil {
		t.Fatal("expected error when reading other tenant's blob") // Actually it prepends tenant1, so tenant1/tenant2/data.txt which won't exist. This is the desired isolation.
	}
}

func TestBlobInspectorMCP_MissingArguments(t *testing.T) {
	provider, err := storage.NewLocalProvider(t.TempDir())
	if err != nil {
		t.Fatal(err)
	}
	inspector := NewBlobInspectorMCP(provider)
	ctx := context.Background()

	_, err = inspector.CallTool(ctx, "read_blob_metadata", map[string]interface{}{})
	if err == nil {
		t.Fatal("expected error due to missing path")
	}

	_, err = inspector.CallTool(ctx, "get_blob_url", map[string]interface{}{})
	if err == nil {
		t.Fatal("expected error due to missing path")
	}

	_, err = inspector.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Fatal("expected error due to unknown tool")
	}
}

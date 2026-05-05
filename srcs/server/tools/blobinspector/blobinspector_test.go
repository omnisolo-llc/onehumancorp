package blobinspector

import (
	"context"
	"errors"
	"testing"
	"time"
)

type MockStorageProvider struct {
	isLocal          bool
	blobs            map[string]BlobMetadata
	listErr          error
	readMetadataErr  error
	getURLErr        error
}

func (m *MockStorageProvider) IsLocal() bool {
	return m.isLocal
}

func (m *MockStorageProvider) ListBlobs(ctx context.Context, prefix string) ([]BlobMetadata, error) {
	if m.listErr != nil {
		return nil, m.listErr
	}
	var res []BlobMetadata
	for k, v := range m.blobs {
		// basic prefix match
		if len(k) >= len(prefix) && k[:len(prefix)] == prefix {
			res = append(res, v)
		}
	}
	return res, nil
}

func (m *MockStorageProvider) ReadBlobMetadata(ctx context.Context, key string) (*BlobMetadata, error) {
	if m.readMetadataErr != nil {
		return nil, m.readMetadataErr
	}
	if b, ok := m.blobs[key]; ok {
		return &b, nil
	}
	return nil, errors.New("not found")
}

func (m *MockStorageProvider) GetBlobURL(ctx context.Context, key string) (string, error) {
	if m.getURLErr != nil {
		return "", m.getURLErr
	}
	if _, ok := m.blobs[key]; ok {
		return "http://example.com/" + key, nil
	}
	return "", errors.New("not found")
}

type MockHub struct {
	storage StorageProvider
}

func (m *MockHub) Storage() StorageProvider {
	return m.storage
}

func TestBlobInspector_ListTools(t *testing.T) {
	inspector := NewBlobInspector(&MockHub{})
	tools := inspector.ListTools()
	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}
}

func TestBlobInspector_CallTool_ListBlobs_Cloud(t *testing.T) {
	storage := &MockStorageProvider{
		isLocal: false,
		blobs: map[string]BlobMetadata{
			"org-123/images/logo.png": {Key: "org-123/images/logo.png", Size: 100, LastModified: time.Now(), ContentType: "image/png"},
			"org-123/docs/readme.txt": {Key: "org-123/docs/readme.txt", Size: 50, LastModified: time.Now(), ContentType: "text/plain"},
			"org-456/images/logo.png": {Key: "org-456/images/logo.png", Size: 100, LastModified: time.Now(), ContentType: "image/png"},
		},
	}
	inspector := NewBlobInspector(&MockHub{storage: storage})
	ctx := context.Background()
	claims := &Claims{OrganizationID: "org-123"}

	res, err := inspector.CallTool(ctx, "list_blobs", map[string]interface{}{"prefix": "images/"}, claims)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	blobs := res.([]BlobMetadata)
	if len(blobs) != 1 {
		t.Fatalf("expected 1 blob, got %d", len(blobs))
	}

	if blobs[0].Key != "images/logo.png" {
		t.Errorf("expected key 'images/logo.png', got '%s'", blobs[0].Key)
	}
}

func TestBlobInspector_CallTool_ListBlobs_Local(t *testing.T) {
	storage := &MockStorageProvider{
		isLocal: true,
		blobs: map[string]BlobMetadata{
			"images/logo.png": {Key: "images/logo.png", Size: 100, LastModified: time.Now(), ContentType: "image/png"},
			"docs/readme.txt": {Key: "docs/readme.txt", Size: 50, LastModified: time.Now(), ContentType: "text/plain"},
		},
	}
	inspector := NewBlobInspector(&MockHub{storage: storage})
	ctx := context.Background()
	claims := &Claims{OrganizationID: "org-123"}

	res, err := inspector.CallTool(ctx, "list_blobs", map[string]interface{}{"prefix": "images/"}, claims)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	blobs := res.([]BlobMetadata)
	if len(blobs) != 1 {
		t.Fatalf("expected 1 blob, got %d", len(blobs))
	}

	if blobs[0].Key != "images/logo.png" {
		t.Errorf("expected key 'images/logo.png', got '%s'", blobs[0].Key)
	}
}

func TestBlobInspector_CallTool_ReadBlobMetadata_Cloud(t *testing.T) {
	storage := &MockStorageProvider{
		isLocal: false,
		blobs: map[string]BlobMetadata{
			"org-123/images/logo.png": {Key: "org-123/images/logo.png", Size: 100, LastModified: time.Now(), ContentType: "image/png"},
		},
	}
	inspector := NewBlobInspector(&MockHub{storage: storage})
	ctx := context.Background()
	claims := &Claims{OrganizationID: "org-123"}

	res, err := inspector.CallTool(ctx, "read_blob_metadata", map[string]interface{}{"key": "images/logo.png"}, claims)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	meta := res.(*BlobMetadata)
	if meta.Key != "images/logo.png" {
		t.Errorf("expected key 'images/logo.png', got '%s'", meta.Key)
	}
}

func TestBlobInspector_CallTool_GetBlobURL_Cloud(t *testing.T) {
	storage := &MockStorageProvider{
		isLocal: false,
		blobs: map[string]BlobMetadata{
			"org-123/images/logo.png": {Key: "org-123/images/logo.png", Size: 100, LastModified: time.Now(), ContentType: "image/png"},
		},
	}
	inspector := NewBlobInspector(&MockHub{storage: storage})
	ctx := context.Background()
	claims := &Claims{OrganizationID: "org-123"}

	res, err := inspector.CallTool(ctx, "get_blob_url", map[string]interface{}{"key": "images/logo.png"}, claims)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	urlMap := res.(map[string]interface{})
	if urlMap["url"] != "http://example.com/org-123/images/logo.png" {
		t.Errorf("unexpected URL: %v", urlMap["url"])
	}
}

func TestBlobInspector_CallTool_Unauthorized(t *testing.T) {
	inspector := NewBlobInspector(&MockHub{})
	ctx := context.Background()

	_, err := inspector.CallTool(ctx, "list_blobs", map[string]interface{}{"prefix": "images/"}, nil)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}

	if err.Error() != "unauthorized: claims are missing" {
		t.Errorf("unexpected error message: %v", err)
	}
}

func TestBlobInspector_CallTool_ListBlobs_Cloud_MissingOrg(t *testing.T) {
	storage := &MockStorageProvider{isLocal: false}
	inspector := NewBlobInspector(&MockHub{storage: storage})
	ctx := context.Background()
	claims := &Claims{OrganizationID: ""}

	_, err := inspector.CallTool(ctx, "list_blobs", map[string]interface{}{"prefix": "images/"}, claims)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
	if err.Error() != "organization ID is required for cloud storage" {
		t.Errorf("unexpected error message: %v", err)
	}
}

func TestBlobInspector_CallTool_ListBlobs_StorageError(t *testing.T) {
	storage := &MockStorageProvider{isLocal: true, listErr: errors.New("list error")}
	inspector := NewBlobInspector(&MockHub{storage: storage})
	ctx := context.Background()
	claims := &Claims{OrganizationID: "org-123"}

	_, err := inspector.CallTool(ctx, "list_blobs", map[string]interface{}{"prefix": "images/"}, claims)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
	if err.Error() != "failed to list blobs: list error" {
		t.Errorf("unexpected error message: %v", err)
	}
}

func TestBlobInspector_CallTool_ReadBlobMetadata_MissingKey(t *testing.T) {
	storage := &MockStorageProvider{isLocal: true}
	inspector := NewBlobInspector(&MockHub{storage: storage})
	ctx := context.Background()
	claims := &Claims{OrganizationID: "org-123"}

	_, err := inspector.CallTool(ctx, "read_blob_metadata", map[string]interface{}{}, claims)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
	if err.Error() != "invalid or missing 'key' argument" {
		t.Errorf("unexpected error message: %v", err)
	}
}

func TestBlobInspector_CallTool_ReadBlobMetadata_Cloud_MissingOrg(t *testing.T) {
	storage := &MockStorageProvider{isLocal: false}
	inspector := NewBlobInspector(&MockHub{storage: storage})
	ctx := context.Background()
	claims := &Claims{OrganizationID: ""}

	_, err := inspector.CallTool(ctx, "read_blob_metadata", map[string]interface{}{"key": "test.txt"}, claims)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
	if err.Error() != "organization ID is required for cloud storage" {
		t.Errorf("unexpected error message: %v", err)
	}
}

func TestBlobInspector_CallTool_ReadBlobMetadata_StorageError(t *testing.T) {
	storage := &MockStorageProvider{isLocal: true, readMetadataErr: errors.New("read error")}
	inspector := NewBlobInspector(&MockHub{storage: storage})
	ctx := context.Background()
	claims := &Claims{OrganizationID: "org-123"}

	_, err := inspector.CallTool(ctx, "read_blob_metadata", map[string]interface{}{"key": "test.txt"}, claims)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
	if err.Error() != "failed to read blob metadata: read error" {
		t.Errorf("unexpected error message: %v", err)
	}
}

func TestBlobInspector_CallTool_GetBlobURL_MissingKey(t *testing.T) {
	storage := &MockStorageProvider{isLocal: true}
	inspector := NewBlobInspector(&MockHub{storage: storage})
	ctx := context.Background()
	claims := &Claims{OrganizationID: "org-123"}

	_, err := inspector.CallTool(ctx, "get_blob_url", map[string]interface{}{}, claims)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
	if err.Error() != "invalid or missing 'key' argument" {
		t.Errorf("unexpected error message: %v", err)
	}
}

func TestBlobInspector_CallTool_GetBlobURL_Cloud_MissingOrg(t *testing.T) {
	storage := &MockStorageProvider{isLocal: false}
	inspector := NewBlobInspector(&MockHub{storage: storage})
	ctx := context.Background()
	claims := &Claims{OrganizationID: ""}

	_, err := inspector.CallTool(ctx, "get_blob_url", map[string]interface{}{"key": "test.txt"}, claims)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
	if err.Error() != "organization ID is required for cloud storage" {
		t.Errorf("unexpected error message: %v", err)
	}
}

func TestBlobInspector_CallTool_GetBlobURL_StorageError(t *testing.T) {
	storage := &MockStorageProvider{isLocal: true, getURLErr: errors.New("url error")}
	inspector := NewBlobInspector(&MockHub{storage: storage})
	ctx := context.Background()
	claims := &Claims{OrganizationID: "org-123"}

	_, err := inspector.CallTool(ctx, "get_blob_url", map[string]interface{}{"key": "test.txt"}, claims)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
	if err.Error() != "failed to get blob url: url error" {
		t.Errorf("unexpected error message: %v", err)
	}
}

func TestBlobInspector_CallTool_MissingStorage(t *testing.T) {
	inspector := NewBlobInspector(&MockHub{storage: nil})
	ctx := context.Background()
	claims := &Claims{OrganizationID: "org-123"}

	_, err := inspector.CallTool(ctx, "list_blobs", map[string]interface{}{}, claims)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
	if err.Error() != "storage provider not configured" {
		t.Errorf("unexpected error message: %v", err)
	}
}

func TestBlobInspector_CallTool_UnknownTool(t *testing.T) {
	storage := &MockStorageProvider{isLocal: true}
	inspector := NewBlobInspector(&MockHub{storage: storage})
	ctx := context.Background()
	claims := &Claims{OrganizationID: "org-123"}

	_, err := inspector.CallTool(ctx, "unknown_tool", map[string]interface{}{}, claims)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
	if err.Error() != "unknown tool: unknown_tool" {
		t.Errorf("unexpected error message: %v", err)
	}
}

func TestBlobInspector_CallTool_PathTraversal(t *testing.T) {
	storage := &MockStorageProvider{isLocal: false}
	inspector := NewBlobInspector(&MockHub{storage: storage})
	ctx := context.Background()
	claims := &Claims{OrganizationID: "org-123"}

	_, err := inspector.CallTool(ctx, "read_blob_metadata", map[string]interface{}{"key": "../org-456/secret.txt"}, claims)
	if err == nil {
		t.Fatalf("expected error for path traversal, got nil")
	}
	if err.Error() != "invalid key: path traversal not allowed" {
		t.Errorf("unexpected error message: %v", err)
	}

	_, err = inspector.CallTool(ctx, "list_blobs", map[string]interface{}{"prefix": "../../"}, claims)
	if err == nil {
		t.Fatalf("expected error for path traversal, got nil")
	}
	if err.Error() != "invalid key: path traversal not allowed" {
		t.Errorf("unexpected error message: %v", err)
	}
}

func TestBlobInspector_CallTool_ListBlobs_Cloud_TrailingSlash(t *testing.T) {
	storage := &MockStorageProvider{
		isLocal: false,
		blobs: map[string]BlobMetadata{
			"org-123/images/logo.png": {Key: "org-123/images/logo.png", Size: 100, LastModified: time.Now(), ContentType: "image/png"},
			"org-123/images_backup/logo.png": {Key: "org-123/images_backup/logo.png", Size: 100, LastModified: time.Now(), ContentType: "image/png"},
		},
	}
	inspector := NewBlobInspector(&MockHub{storage: storage})
	ctx := context.Background()
	claims := &Claims{OrganizationID: "org-123"}

	res, err := inspector.CallTool(ctx, "list_blobs", map[string]interface{}{"prefix": "images/"}, claims)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	blobs := res.([]BlobMetadata)
	if len(blobs) != 1 {
		t.Fatalf("expected 1 blob, got %d", len(blobs))
	}

	if blobs[0].Key != "images/logo.png" {
		t.Errorf("expected key 'images/logo.png', got '%s'", blobs[0].Key)
	}
}

func TestBlobInspector_CallTool_ListBlobs_Cloud_NoTrailingSlash(t *testing.T) {
	storage := &MockStorageProvider{
		isLocal: false,
		blobs: map[string]BlobMetadata{
			"org-123/images/logo.png": {Key: "org-123/images/logo.png", Size: 100, LastModified: time.Now(), ContentType: "image/png"},
			"org-123/images_backup/logo.png": {Key: "org-123/images_backup/logo.png", Size: 100, LastModified: time.Now(), ContentType: "image/png"},
		},
	}
	inspector := NewBlobInspector(&MockHub{storage: storage})
	ctx := context.Background()
	claims := &Claims{OrganizationID: "org-123"}

	res, err := inspector.CallTool(ctx, "list_blobs", map[string]interface{}{"prefix": "images"}, claims)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	blobs := res.([]BlobMetadata)
	// Expecting both because 'images' is a prefix of both 'images/' and 'images_backup/'
	if len(blobs) != 2 {
		t.Fatalf("expected 2 blobs, got %d", len(blobs))
	}
}

func TestBlobInspector_CallTool_ListBlobs_Cloud_EmptyPrefix(t *testing.T) {
	storage := &MockStorageProvider{
		isLocal: false,
		blobs: map[string]BlobMetadata{
			"org-123/images/logo.png": {Key: "org-123/images/logo.png", Size: 100, LastModified: time.Now(), ContentType: "image/png"},
		},
	}
	inspector := NewBlobInspector(&MockHub{storage: storage})
	ctx := context.Background()
	claims := &Claims{OrganizationID: "org-123"}

	res, err := inspector.CallTool(ctx, "list_blobs", map[string]interface{}{"prefix": ""}, claims)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	blobs := res.([]BlobMetadata)
	if len(blobs) != 1 {
		t.Fatalf("expected 1 blob, got %d", len(blobs))
	}

	if blobs[0].Key != "images/logo.png" {
		t.Errorf("expected key 'images/logo.png', got '%s'", blobs[0].Key)
	}
}

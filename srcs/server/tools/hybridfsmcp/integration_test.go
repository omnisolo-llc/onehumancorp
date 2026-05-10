package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"
)

// TestComprehensiveFeature tests the full lifecycle of the hybridfsmcp tools
// to prove the feature is fully implemented and works correctly.
func TestComprehensiveFeature_Local(t *testing.T) {
	// Setup
	tmpDir := t.TempDir()

	provider, err := NewLocalFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	server := NewServer(provider)
	ctx := context.Background()
	claims := &Claims{OrganizationID: "test-org"}

	// Test Write
	writeRes, err := server.CallTool(ctx, "write_file", map[string]interface{}{
		"path": "test_comprehensive.txt",
		"content": "Hello World",
	}, claims)
	if err != nil {
		t.Fatalf("Write failed: %v", err)
	}
	if status, ok := writeRes.(map[string]interface{})["status"]; !ok || status != "success" {
		t.Errorf("Expected success, got %v", writeRes)
	}

	// Test Read
	readRes, err := server.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test_comprehensive.txt",
	}, claims)
	if err != nil {
		t.Fatalf("Read failed: %v", err)
	}
	if content, ok := readRes.(map[string]interface{})["content"]; !ok || content != "Hello World" {
		t.Errorf("Expected 'Hello World', got %v", readRes)
	}

	// Test List
	listRes, err := server.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	}, claims)
	if err != nil {
		t.Fatalf("List failed: %v", err)
	}
	list := listRes.([]string)
	if len(list) != 1 || list[0] != "test_comprehensive.txt" {
		t.Errorf("Expected ['test_comprehensive.txt'], got %v", list)
	}

	// Test Search
	searchRes, err := server.CallTool(ctx, "search_files", map[string]interface{}{
		"path": ".",
		"query": "test",
	}, claims)
	if err != nil {
		t.Fatalf("Search failed: %v", err)
	}
	searchList := searchRes.([]string)
	if len(searchList) != 1 || searchList[0] != "test_comprehensive.txt" {
		t.Errorf("Expected ['test_comprehensive.txt'], got %v", searchList)
	}
}

func TestComprehensiveFeature_Cloud(t *testing.T) {
	// Setup
	tmpDir := t.TempDir()

	provider, err := NewCloudFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	server := NewServer(provider)
	ctx := context.Background()
	claims := &Claims{OrganizationID: "test-org"}

	// Test Write
	writeRes, err := server.CallTool(ctx, "write_file", map[string]interface{}{
		"path": "test_cloud.txt",
		"content": "Hello Cloud",
	}, claims)
	if err != nil {
		t.Fatalf("Write failed: %v", err)
	}
	if status, ok := writeRes.(map[string]interface{})["status"]; !ok || status != "success" {
		t.Errorf("Expected success, got %v", writeRes)
	}

	// Verify it was written to the tenant subfolder
	b, err := os.ReadFile(filepath.Join(tmpDir, "test-org", "test_cloud.txt"))
	if err != nil {
		t.Fatalf("Failed to read raw file: %v", err)
	}
	if string(b) != "Hello Cloud" {
		t.Errorf("Expected 'Hello Cloud', got %s", string(b))
	}
}

package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "local_provider_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Test WriteFile
	testFile := "test.txt"
	testData := []byte("hello local")
	err = provider.WriteFile(ctx, testFile, testData)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	readData, err := provider.ReadFile(ctx, testFile)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(readData) != string(testData) {
		t.Errorf("Expected '%s', got '%s'", testData, readData)
	}

	// Test ListDir
	files, err := provider.ListDir(ctx, "")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(files) != 1 || files[0] != testFile {
		t.Errorf("Expected ['%s'], got %v", testFile, files)
	}

	// Test SearchFiles
	searchFiles, err := provider.SearchFiles(ctx, "", "test")
	if err != nil {
		t.Fatalf("SearchFiles failed: %v", err)
	}
	if len(searchFiles) != 1 || searchFiles[0] != testFile {
		t.Errorf("Expected ['%s'], got %v", testFile, searchFiles)
	}

	// Test Path Traversal Protection
	_, err = provider.ReadFile(ctx, "../../../etc/passwd")
	if err == nil {
		t.Errorf("Expected error on path traversal attempt")
	}

	// Test sibling directory overlap
	siblingDir := tempDir + "sibling"
	os.Mkdir(siblingDir, 0755)
	defer os.RemoveAll(siblingDir)
	err = os.WriteFile(filepath.Join(siblingDir, "sibling.txt"), []byte("sibling"), 0644)
	if err == nil {
		// Attempt to read from sibling matching prefix
		relPath, _ := filepath.Rel(tempDir, filepath.Join(siblingDir, "sibling.txt"))
		_, err = provider.ReadFile(ctx, relPath)
		if err == nil {
			t.Errorf("Expected error on sibling directory overlap traversal")
		}
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloud_provider_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	localProvider := NewLocalFSProvider(tempDir)
	provider := NewCloudFSProvider(tempDir, localProvider)

	// Set up context with org ID
	claims := &auth.Claims{OrganizationID: "tenant1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	testFile := "test.txt"
	testData := []byte("hello cloud")
	err = provider.WriteFile(ctx, testFile, testData)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify file is actually in the tenant subfolder
	tenantPath := filepath.Join(tempDir, "tenant1", testFile)
	if _, err := os.Stat(tenantPath); os.IsNotExist(err) {
		t.Errorf("File was not written to tenant specific subfolder: %s", tenantPath)
	}

	// Test ReadFile
	readData, err := provider.ReadFile(ctx, testFile)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(readData) != string(testData) {
		t.Errorf("Expected '%s', got '%s'", testData, readData)
	}

	// Test ListDir
	files, err := provider.ListDir(ctx, "")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(files) != 1 || files[0] != testFile {
		t.Errorf("Expected ['%s'], got %v", testFile, files)
	}

	// Test SearchFiles
	searchFiles, err := provider.SearchFiles(ctx, "", "test")
	if err != nil {
		t.Fatalf("SearchFiles failed: %v", err)
	}
	if len(searchFiles) != 1 || searchFiles[0] != testFile {
		t.Errorf("Expected ['%s'], got %v", testFile, searchFiles)
	}

	// Test context without org ID
	ctxNoOrg := context.Background()
	_, err = provider.ReadFile(ctxNoOrg, testFile)
	if err == nil {
		t.Errorf("Expected error when no org ID in context")
	}
}

func TestFactory(t *testing.T) {
	tempDir, _ := os.MkdirTemp("", "factory_test")
	defer os.RemoveAll(tempDir)

	ctx := context.Background()

	// Test Standalone Mode
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	p1 := NewProvider(ctx, tempDir)
	if _, ok := p1.(*LocalFSProvider); !ok {
		t.Errorf("Expected LocalFSProvider in standalone mode, got %T", p1)
	}

	// Test Cloud Mode
	os.Unsetenv("OHC_STANDALONE")

	p2 := NewProvider(ctx, tempDir)
	if _, ok := p2.(*CloudFSProvider); !ok {
		t.Errorf("Expected CloudFSProvider in cloud mode, got %T", p2)
	}
}

package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider := &LocalFSProvider{BaseDir: tmpDir}
	ctx := context.Background()

	// Test writing and reading
	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello" {
		t.Fatalf("Expected 'hello', got %q", string(data))
	}

	// Test list dir
	files, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(files) != 1 || files[0] != "test.txt" {
		t.Fatalf("Expected ['test.txt'], got %v", files)
	}

	// Test bounds checking
	err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
	if err == nil {
		t.Fatal("Expected error when escaping base dir, got nil")
	}

	_, err = provider.ReadFile(ctx, "../escape.txt")
	if err == nil {
		t.Fatal("Expected error when escaping base dir, got nil")
	}

	_, err = provider.ListDir(ctx, "../")
	if err == nil {
		t.Fatal("Expected error when escaping base dir, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider := &CloudFSProvider{BaseDir: tmpDir}

	// Missing claims context
	ctxNoClaims := context.Background()
	err := provider.WriteFile(ctxNoClaims, "test.txt", []byte("hello"))
	if err == nil {
		t.Fatal("Expected error for missing org ID, got nil")
	}

	// With claims context
	claims := &auth.Claims{OrganizationID: "org-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	err = provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Fatalf("Expected 'hello cloud', got %q", string(data))
	}

	// Ensure it created the correct subfolder
	b, err := os.ReadFile(filepath.Join(tmpDir, "org-123", "test.txt"))
	if err != nil {
		t.Fatalf("Failed to read underlying file: %v", err)
	}
	if string(b) != "hello cloud" {
		t.Fatalf("Expected 'hello cloud' in underlying file, got %q", string(b))
	}

	// Test bounds checking
	err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
	if err == nil {
		t.Fatal("Expected error when escaping tenant dir, got nil")
	}

	_, err = provider.ReadFile(ctx, "../escape.txt")
	if err == nil {
		t.Fatal("Expected error when escaping tenant dir, got nil")
	}

	_, err = provider.ListDir(ctx, "../")
	if err == nil {
		t.Fatal("Expected error when escaping tenant dir, got nil")
	}

	// Test ListDir
	files, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(files) != 1 || files[0] != "test.txt" {
		t.Fatalf("Expected ['test.txt'], got %v", files)
	}
}

func TestNewProvider(t *testing.T) {
	ctx := context.Background()

	os.Setenv("OHC_STANDALONE", "true")
	p1 := NewProvider(ctx, "/tmp")
	if _, ok := p1.(*LocalFSProvider); !ok {
		t.Fatalf("Expected LocalFSProvider, got %T", p1)
	}

	os.Setenv("OHC_STANDALONE", "false")
	p2 := NewProvider(ctx, "/tmp")
	if _, ok := p2.(*CloudFSProvider); !ok {
		t.Fatalf("Expected CloudFSProvider, got %T", p2)
	}
}

func TestServer(t *testing.T) {
	tmpDir := t.TempDir()
	provider := &LocalFSProvider{BaseDir: tmpDir}
	server := NewServer(provider)
	ctx := context.Background()

	// Write
	resW := server.HandleWriteFile(ctx, "test.txt", []byte("hello"))
	if resW.Status != "success" {
		t.Fatalf("WriteFile failed: %s", resW.ResultData)
	}

	// Read
	resR := server.HandleReadFile(ctx, "test.txt")
	if resR.Status != "success" {
		t.Fatalf("ReadFile failed: %s", resR.ResultData)
	}

	// List
	resL := server.HandleListDirectory(ctx, ".")
	if resL.Status != "success" {
		t.Fatalf("ListDir failed: %s", resL.ResultData)
	}

	// Write error (escape)
	resWE := server.HandleWriteFile(ctx, "../bad.txt", []byte("bad"))
	if resWE.Status != "error" {
		t.Fatalf("Expected error for escaping path, got success")
	}

	// Read error (escape)
	resRE := server.HandleReadFile(ctx, "../bad.txt")
	if resRE.Status != "error" {
		t.Fatalf("Expected error for escaping path, got success")
	}

	// List error (escape)
	resLE := server.HandleListDirectory(ctx, "../")
	if resLE.Status != "error" {
		t.Fatalf("Expected error for escaping path, got success")
	}
}

func TestLocalFSProviderPrefixCollision(t *testing.T) {
	tmpDir := t.TempDir()

	dir1 := filepath.Join(tmpDir, "tenant1")
	dir2 := filepath.Join(tmpDir, "tenant10")

	os.MkdirAll(dir1, 0755)
	os.MkdirAll(dir2, 0755)

	os.WriteFile(filepath.Join(dir2, "secret.txt"), []byte("secret"), 0644)

	provider := &LocalFSProvider{BaseDir: dir1}
	ctx := context.Background()

	_, err := provider.ReadFile(ctx, "../tenant10/secret.txt")
	if err == nil {
		t.Fatal("Expected error when reading from tenant10 while in tenant1, got nil")
	}
}

func TestCloudFSProviderPrefixCollision(t *testing.T) {
	tmpDir := t.TempDir()

	dir1 := filepath.Join(tmpDir, "tenant1")
	dir2 := filepath.Join(tmpDir, "tenant10")

	os.MkdirAll(dir1, 0755)
	os.MkdirAll(dir2, 0755)

	os.WriteFile(filepath.Join(dir2, "secret.txt"), []byte("secret"), 0644)

	provider := &CloudFSProvider{BaseDir: tmpDir}

	claims := &auth.Claims{OrganizationID: "tenant1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	_, err := provider.ReadFile(ctx, "../tenant10/secret.txt")
	if err == nil {
		t.Fatal("Expected error when reading from tenant10 while in tenant1, got nil")
	}
}

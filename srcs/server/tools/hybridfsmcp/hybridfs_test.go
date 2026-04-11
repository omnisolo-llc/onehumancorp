package hybridfsmcp

import (
	"context"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider, err := NewLocalFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()

	// Test write
	err = provider.WriteFile(ctx, "test.txt", []byte("hello world"))
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	// Test read
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
	if string(data) != "hello world" {
		t.Errorf("expected 'hello world', got %s", string(data))
	}

	// Test bounds checking
	err = provider.WriteFile(ctx, "../escaped.txt", []byte("bad"))
	if err != ErrAccessDenied {
		t.Errorf("expected ErrAccessDenied for ../escaped.txt, got %v", err)
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider, err := NewCloudFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctxWithClaims := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "tenant1"})
	ctxWithoutClaims := context.Background()

	// Test unauthorized
	err = provider.WriteFile(ctxWithoutClaims, "test.txt", []byte("hello"))
	if err != ErrUnauthorized {
		t.Errorf("expected ErrUnauthorized, got %v", err)
	}

	// Test write
	err = provider.WriteFile(ctxWithClaims, "test.txt", []byte("hello world"))
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	// Test read
	data, err := provider.ReadFile(ctxWithClaims, "test.txt")
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
	if string(data) != "hello world" {
		t.Errorf("expected 'hello world', got %s", string(data))
	}

	// Verify tenant isolation
	ctxOtherTenant := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "tenant2"})

	_, err = provider.ReadFile(ctxOtherTenant, "test.txt")
	if err == nil {
		t.Errorf("expected error when reading other tenant's file")
	}

	// Verify traversal blocks on Org ID
	ctxBadClaims := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "../tenant1"})
	err = provider.WriteFile(ctxBadClaims, "test.txt", []byte("bad"))
	if err != ErrUnauthorized {
		t.Errorf("expected ErrUnauthorized, got %v", err)
	}
}

func TestFactory(t *testing.T) {
	tmpDir := t.TempDir()

	t.Run("Cloud Mode", func(t *testing.T) {
		t.Setenv("OHC_STANDALONE", "false")
		p, err := NewProviderFactory(tmpDir)
		if err != nil {
			t.Fatalf("factory failed: %v", err)
		}
		if _, ok := p.(*CloudFSProvider); !ok {
			t.Errorf("expected CloudFSProvider")
		}
	})

	t.Run("Standalone Mode", func(t *testing.T) {
		t.Setenv("OHC_STANDALONE", "true")
		p, err := NewProviderFactory(tmpDir)
		if err != nil {
			t.Fatalf("factory failed: %v", err)
		}
		if _, ok := p.(*LocalFSProvider); !ok {
			t.Errorf("expected LocalFSProvider")
		}
	})
}

func TestServerExecution(t *testing.T) {
	tmpDir := t.TempDir()
	provider, _ := NewLocalFSProvider(tmpDir)
	server := NewServer(provider)
	ctx := context.Background()

	// Write
	writeArgs := []byte(`{"path": "file1.txt", "content": "file1 content"}`)
	res := server.ExecuteTool(ctx, "write_file", writeArgs)
	if res.Status != "success" {
		t.Errorf("expected success, got %v (%s)", res.Status, string(res.ResultData))
	}

	// Read
	readArgs := []byte(`{"path": "file1.txt"}`)
	res = server.ExecuteTool(ctx, "read_file", readArgs)
	if res.Status != "success" {
		t.Errorf("expected success, got %v (%s)", res.Status, string(res.ResultData))
	}
	if !strings.Contains(string(res.ResultData), "file1 content") {
		t.Errorf("expected data in result: %s", string(res.ResultData))
	}

	// List Dir
	listArgs := []byte(`{"path": "."}`)
	res = server.ExecuteTool(ctx, "list_directory", listArgs)
	if res.Status != "success" {
		t.Errorf("expected success, got %v (%s)", res.Status, string(res.ResultData))
	}
	if !strings.Contains(string(res.ResultData), "file1.txt") {
		t.Errorf("expected file1.txt in result: %s", string(res.ResultData))
	}

	// Search
	searchArgs := []byte(`{"path": ".", "pattern": "file1"}`)
	res = server.ExecuteTool(ctx, "search_files", searchArgs)
	if res.Status != "success" {
		t.Errorf("expected success, got %v (%s)", res.Status, string(res.ResultData))
	}
	if !strings.Contains(string(res.ResultData), "file1.txt") {
		t.Errorf("expected file1.txt in result: %s", string(res.ResultData))
	}
}

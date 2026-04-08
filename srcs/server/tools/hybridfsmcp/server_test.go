package hybridfsmcp

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestServer_CloudMode_RequiresAuth(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "server_cloud_test_*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	server := NewServer(tmpDir)

	ctx := context.Background()
	_, err = server.CallTool(ctx, "read_file", map[string]interface{}{"filepath": "test.txt"})
	if err == nil {
		t.Errorf("expected error when claims are missing in cloud mode, got nil")
	}
}

func TestServer_StandaloneMode_NoAuthRequired(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "server_standalone_test_*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	server := NewServer(tmpDir)

	ctx := context.Background()

	// Should be able to write and read a file
	res, err := server.CallTool(ctx, "write_file", map[string]interface{}{
		"filepath": "test.txt",
		"content":  "standalone content",
	})
	if err != nil {
		t.Fatalf("failed to write file: %v", err)
	}
	if res.(string) != "File written successfully" {
		t.Errorf("unexpected write response: %v", res)
	}

	content, err := server.CallTool(ctx, "read_file", map[string]interface{}{
		"filepath": "test.txt",
	})
	if err != nil {
		t.Fatalf("failed to read file: %v", err)
	}
	if content.(string) != "standalone content" {
		t.Errorf("expected 'standalone content', got %v", content)
	}
}

func TestServer_CloudMode_WithAuth(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "server_cloud_auth_test_*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	server := NewServer(tmpDir)

	claims := &auth.Claims{
		OrganizationID: "test-org",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Should be able to write and read a file
	res, err := server.CallTool(ctx, "write_file", map[string]interface{}{
		"filepath": "test.txt",
		"content":  "cloud content",
	})
	if err != nil {
		t.Fatalf("failed to write file: %v", err)
	}
	if res.(string) != "File written successfully" {
		t.Errorf("unexpected write response: %v", res)
	}

	content, err := server.CallTool(ctx, "read_file", map[string]interface{}{
		"filepath": "test.txt",
	})
	if err != nil {
		t.Fatalf("failed to read file: %v", err)
	}
	if content.(string) != "cloud content" {
		t.Errorf("expected 'cloud content', got %v", content)
	}
}

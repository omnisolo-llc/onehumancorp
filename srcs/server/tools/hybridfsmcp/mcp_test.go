package hybridfsmcp

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfsmcp")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)

	ctx := context.Background()

	// Test write_file
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "test.txt",
		"content": "hello world",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Test read_file
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if res.(string) != "hello world" {
		t.Fatalf("expected 'hello world', got '%v'", res)
	}

	// Test list_directory
	resList, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": "",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	files := resList.([]FileInfo)
	if len(files) != 1 || files[0].Name != "test.txt" {
		t.Fatalf("expected 1 file 'test.txt', got %v", files)
	}

    // Test out of bounds
	_, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "../test.txt",
	})
	if err == nil {
		t.Fatalf("expected error for path out of bounds")
	}
}

func TestCloudFSProvider(t *testing.T) {
	provider := NewCloudFSProvider()
	mcp := NewHybridFSMCP(provider)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "tenant1",
	})

	// Test write_file
	_, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "test.txt",
		"content": "hello cloud",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Test read_file
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if res.(string) != "hello cloud" {
		t.Fatalf("expected 'hello cloud', got '%v'", res)
	}

	// Test list_directory
	resList, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": "",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	files := resList.([]FileInfo)
	if len(files) != 1 || files[0].Name != "test.txt" {
		t.Fatalf("expected 1 file 'test.txt', got %v", files)
	}

	// Test unauthorized
	ctxUnauth := context.Background()
	_, err = mcp.CallTool(ctxUnauth, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err == nil {
		t.Fatalf("expected error for unauthorized access")
	}
}

func TestSearchFilesLocal(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfsmcp_search")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	// Write a file
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "hello_search.txt",
		"content": "hello",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	res, err := mcp.CallTool(ctx, "search_files", map[string]interface{}{
		"path":    "",
		"pattern": "search",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	files := res.([]FileInfo)
	if len(files) != 1 || files[0].Name != "hello_search.txt" {
		t.Fatalf("expected 1 file 'hello_search.txt', got %v", files)
	}
}

func TestSearchFilesCloud(t *testing.T) {
	provider := NewCloudFSProvider()
	mcp := NewHybridFSMCP(provider)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "tenant1",
	})

	// Write a file
	_, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "hello_search.txt",
		"content": "hello cloud",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	res, err := mcp.CallTool(ctx, "search_files", map[string]interface{}{
		"path":    "",
		"pattern": "search",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	files := res.([]FileInfo)
	if len(files) != 1 || files[0].Name != "hello_search.txt" {
		t.Fatalf("expected 1 file 'hello_search.txt', got %v", files)
	}
}

func TestFactory(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	p1 := NewProvider("/tmp")
	if _, ok := p1.(*CloudFSProvider); !ok {
		t.Fatalf("expected CloudFSProvider")
	}

	os.Setenv("OHC_MULTITENANT", "false")
	p2 := NewProvider("/tmp")
	if _, ok := p2.(*LocalFSProvider); !ok {
		t.Fatalf("expected LocalFSProvider")
	}
}

func TestListTools(t *testing.T) {
	provider := NewCloudFSProvider()
	mcp := NewHybridFSMCP(provider)
	tools := mcp.ListTools()
	if len(tools) != 4 {
		t.Fatalf("expected 4 tools, got %d", len(tools))
	}
}

func TestErrorCases(t *testing.T) {
    provider := NewCloudFSProvider()
	mcp := NewHybridFSMCP(provider)
    ctx := context.Background()

    // Missing path for read_file
    _, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{})
    if err == nil {
        t.Fatal("expected error")
    }

    // Missing path for write_file
    _, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{
        "content": "hi",
    })
    if err == nil {
        t.Fatal("expected error")
    }

    // Missing content for write_file
    _, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{
        "path": "test.txt",
    })
    if err == nil {
        t.Fatal("expected error")
    }

    // Missing path for list_directory
    _, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{})
    if err == nil {
        t.Fatal("expected error")
    }

    // Missing path for search_files
    _, err = mcp.CallTool(ctx, "search_files", map[string]interface{}{
        "pattern": "test",
    })
    if err == nil {
        t.Fatal("expected error")
    }

    // Missing pattern for search_files
    _, err = mcp.CallTool(ctx, "search_files", map[string]interface{}{
        "path": "",
    })
    if err == nil {
        t.Fatal("expected error")
    }

    // Unknown tool
    _, err = mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{})
    if err == nil {
        t.Fatal("expected error")
    }
}

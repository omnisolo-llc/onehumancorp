package fsmcp

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestStandaloneReadWriteList(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	tool := NewFsMcpTool(nil, "")
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "org1"}

	err := tool.Write(ctx, claims, "test.txt", "hello world")
	if err != nil {
		t.Fatalf("write failed: %v", err)
	}

	content, err := tool.Read(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("read failed: %v", err)
	}
	if content != "hello world" {
		t.Errorf("expected 'hello world', got '%s'", content)
	}

	entries, err := tool.List(ctx, claims, ".")
	if err != nil {
		t.Fatalf("list failed: %v", err)
	}
	found := false
	for _, e := range entries {
		if e == "test.txt" {
			found = true
			break
		}
	}
	if !found {
		t.Errorf("expected test.txt in list results")
	}
}

func TestPathTraversal(t *testing.T) {
	tool := NewFsMcpTool(nil, "")
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "org1"}

	err := tool.Write(ctx, claims, "../test.txt", "hello")
	if err == nil {
		t.Errorf("expected error for path traversal")
	}

	_, err = tool.Read(ctx, claims, "../test.txt")
	if err == nil {
		t.Errorf("expected error for path traversal")
	}

	_, err = tool.List(ctx, claims, "../")
	if err == nil {
		t.Errorf("expected error for path traversal")
	}
}

func TestCloudModeMissingS3(t *testing.T) {
    os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	tool := NewFsMcpTool(nil, "")
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "org1"}

    err := tool.Write(ctx, claims, "test.txt", "hello world")
	if err == nil {
		t.Errorf("expected error for missing s3 client")
	}

    _, err = tool.Read(ctx, claims, "test.txt")
	if err == nil {
		t.Errorf("expected error for missing s3 client")
	}

    _, err = tool.List(ctx, claims, ".")
	if err == nil {
		t.Errorf("expected error for missing s3 client")
	}
}

func TestCloudModeMissingClaims(t *testing.T) {
    os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	tool := NewFsMcpTool(nil, "")
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: ""}

    err := tool.Write(ctx, claims, "test.txt", "hello world")
	if err == nil {
		t.Errorf("expected error for missing claims")
	}

    _, err = tool.Read(ctx, claims, "test.txt")
	if err == nil {
		t.Errorf("expected error for missing claims")
	}

    _, err = tool.List(ctx, claims, ".")
	if err == nil {
		t.Errorf("expected error for missing claims")
	}
}

func TestCloudModeCleanPath(t *testing.T) {
    os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	tool := NewFsMcpTool(nil, "")
	claims := &auth.Claims{OrganizationID: "org1"}

    p, err := tool.cleanPath(claims, "test.txt")
    if err != nil || p != "tenant/org1/fs/test.txt" {
       t.Errorf("cleanPath failed, got: %s, err: %v", p, err)
    }
}

func TestStandaloneOutsideBase(t *testing.T) {
    os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	tool := NewFsMcpTool(nil, "")
    // Modify tool.localBaseDir to force error
    tool.localBaseDir = "/tmp/fake_dir_that_does_not_exist_at_all"

    claims := &auth.Claims{OrganizationID: "org1"}

    // We already check for ../ but what if the request path was absolute /etc/passwd
    _, err := tool.cleanPath(claims, "/etc/passwd")
    if err == nil {
         t.Errorf("expected error for absolute path traversal")
    }
}

func TestListMissingDir(t *testing.T) {
    os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	tool := NewFsMcpTool(nil, "")
    claims := &auth.Claims{OrganizationID: "org1"}

    entries, err := tool.List(context.Background(), claims, "non_existent_folder_xyz_123")
    if err != nil {
         t.Errorf("list on missing dir should return empty array, got err: %v", err)
    }
    if len(entries) != 0 {
         t.Errorf("expected 0 entries")
    }
}

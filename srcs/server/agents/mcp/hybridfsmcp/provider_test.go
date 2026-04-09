package hybridfsmcp

import (
    "testing"
)

func TestLocalFSProvider(t *testing.T) {
    workspace := t.TempDir()
    provider := &LocalFSProvider{WorkspaceDir: workspace}

    err := provider.WriteFile("test.txt", []byte("hello"))
    if err != nil { t.Fatalf("expected no error, got %v", err) }

    data, err := provider.ReadFile("test.txt")
    if err != nil { t.Fatalf("expected no error, got %v", err) }
    if string(data) != "hello" { t.Fatalf("expected hello, got %s", string(data)) }

    _, err = provider.ReadFile("../out.txt")
    if err == nil { t.Fatalf("expected error for path outside workspace") }

    entries, err := provider.ListDir(".")
    if err != nil { t.Fatalf("expected no error, got %v", err) }
    if len(entries) != 1 || entries[0].Name() != "test.txt" {
        t.Fatalf("unexpected dir entries")
    }
}

func TestCloudFSProvider(t *testing.T) {
    baseDir := t.TempDir()
    provider := &CloudFSProvider{TenantID: "tenant-123", BaseDir: baseDir}

    // Tenant dir should be created by write
    err := provider.WriteFile("test.txt", []byte("hello cloud"))
    if err != nil { t.Fatalf("expected no error, got %v", err) }

    data, err := provider.ReadFile("test.txt")
    if err != nil { t.Fatalf("expected no error, got %v", err) }
    if string(data) != "hello cloud" { t.Fatalf("expected hello cloud, got %s", string(data)) }

    _, err = provider.ReadFile("../tenant-456/out.txt")
    if err == nil { t.Fatalf("expected error for path outside tenant scope") }

    entries, err := provider.ListDir(".")
    if err != nil { t.Fatalf("expected no error, got %v", err) }
    if len(entries) != 1 || entries[0].Name() != "test.txt" {
        t.Fatalf("unexpected dir entries")
    }
}

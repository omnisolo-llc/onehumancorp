package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()

	tests := []struct {
		name    string
		op      string
		path    string
		data    string
		pattern string
		wantErr bool
	}{
		{"Write Valid File", "write", "test.txt", "hello", "", false},
		{"Read Valid File", "read", "test.txt", "hello", "", false},
		{"Write Escape Attempt", "write", "../escape.txt", "bad", "", true},
		{"Read Escape Attempt", "read", "../escape.txt", "", "", true},
		{"List Dir", "list", ".", "", "", false},
		{"List Escape", "list", "../", "", "", true},
		{"Search Pattern", "search", ".", "test", "test", false},
		{"Search Escape", "search", "../", "test", "test", true},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			switch tc.op {
			case "write":
				err := provider.WriteFile(ctx, tc.path, []byte(tc.data))
				if (err != nil) != tc.wantErr {
					t.Errorf("WriteFile() error = %v, wantErr %v", err, tc.wantErr)
				}
			case "read":
				data, err := provider.ReadFile(ctx, tc.path)
				if (err != nil) != tc.wantErr {
					t.Errorf("ReadFile() error = %v, wantErr %v", err, tc.wantErr)
				}
				if !tc.wantErr && string(data) != tc.data {
					t.Errorf("ReadFile() got = %v, want %v", string(data), tc.data)
				}
			case "list":
				_, err := provider.ListDir(ctx, tc.path)
				if (err != nil) != tc.wantErr {
					t.Errorf("ListDir() error = %v, wantErr %v", err, tc.wantErr)
				}
			case "search":
				_, err := provider.SearchFiles(ctx, tc.path, tc.pattern)
				if (err != nil) != tc.wantErr {
					t.Errorf("SearchFiles() error = %v, wantErr %v", err, tc.wantErr)
				}
			}
		})
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	tenantCtx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "tenant1"})
	noTenantCtx := context.Background()

	// Pre-create tenant dir to avoid WalkDir not exist issues if needed
	os.MkdirAll(filepath.Join(tempDir, "tenant1"), 0755)

	tests := []struct {
		name    string
		ctx     context.Context
		op      string
		path    string
		data    string
		pattern string
		wantErr bool
	}{
		{"Write Valid File", tenantCtx, "write", "test.txt", "hello", "", false},
		{"Read Valid File", tenantCtx, "read", "test.txt", "hello", "", false},
		{"Write Missing Tenant", noTenantCtx, "write", "test.txt", "hello", "", true},
		{"Read Missing Tenant", noTenantCtx, "read", "test.txt", "", "", true},
		{"Write Escape", tenantCtx, "write", "../tenant2/test.txt", "bad", "", true},
		{"Read Escape", tenantCtx, "read", "../tenant2/test.txt", "", "", true},
		{"List Dir Valid", tenantCtx, "list", ".", "", "", false},
		{"List Dir Escape", tenantCtx, "list", "../tenant2", "", "", true},
		{"Search Valid", tenantCtx, "search", ".", "test", "test", false},
		{"Search Escape", tenantCtx, "search", "../tenant2", "test", "test", true},
		{"Search Missing Tenant", noTenantCtx, "search", ".", "test", "test", true},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			switch tc.op {
			case "write":
				err := provider.WriteFile(tc.ctx, tc.path, []byte(tc.data))
				if (err != nil) != tc.wantErr {
					t.Errorf("WriteFile() error = %v, wantErr %v", err, tc.wantErr)
				}
			case "read":
				data, err := provider.ReadFile(tc.ctx, tc.path)
				if (err != nil) != tc.wantErr {
					t.Errorf("ReadFile() error = %v, wantErr %v", err, tc.wantErr)
				}
				if !tc.wantErr && string(data) != tc.data {
					t.Errorf("ReadFile() got = %v, want %v", string(data), tc.data)
				}
			case "list":
				_, err := provider.ListDir(tc.ctx, tc.path)
				if (err != nil) != tc.wantErr {
					t.Errorf("ListDir() error = %v, wantErr %v", err, tc.wantErr)
				}
			case "search":
				_, err := provider.SearchFiles(tc.ctx, tc.path, tc.pattern)
				if (err != nil) != tc.wantErr {
					t.Errorf("SearchFiles() error = %v, wantErr %v", err, tc.wantErr)
				}
			}
		})
	}
}

func TestNewProviderFactory(t *testing.T) {
	tempDir := t.TempDir()

	// Test Standalone
	t.Setenv("OHC_STANDALONE", "true")
	p1, err := NewProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}
	if _, ok := p1.(*LocalFSProvider); !ok {
		t.Errorf("expected LocalFSProvider when OHC_STANDALONE is true")
	}

	// Test Cloud
	t.Setenv("OHC_STANDALONE", "false")
	p2, err := NewProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}
	if _, ok := p2.(*CloudFSProvider); !ok {
		t.Errorf("expected CloudFSProvider when OHC_STANDALONE is false")
	}
}

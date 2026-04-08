package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs-test-*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := &LocalFSProvider{workspaceDir: tempDir}
	ctx := context.Background()

	tests := []struct {
		name    string
		path    string
		content string
		wantErr bool
	}{
		{
			name:    "Happy Path Write",
			path:    "test.txt",
			content: "hello local",
			wantErr: false,
		},
		{
			name:    "Directory traversal",
			path:    "../test.txt",
			content: "hello escape",
			wantErr: true,
		},
		{
			name:    "Absolute path",
			path:    "/etc/passwd",
			content: "hello passwd",
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := provider.WriteFile(ctx, tt.path, []byte(tt.content))
			if (err != nil) != tt.wantErr {
				t.Errorf("WriteFile() error = %v, wantErr %v", err, tt.wantErr)
			}
			if !tt.wantErr {
				readContent, err := provider.ReadFile(ctx, tt.path)
				if err != nil {
					t.Errorf("ReadFile() unexpected error: %v", err)
				}
				if string(readContent) != tt.content {
					t.Errorf("ReadFile() got = %v, want %v", string(readContent), tt.content)
				}

				files, err := provider.ListDir(ctx, ".")
				if err != nil {
					t.Errorf("ListDir() unexpected error: %v", err)
				}
				found := false
				for _, f := range files {
					if f == filepath.Base(tt.path) {
						found = true
						break
					}
				}
				if !found {
					t.Errorf("ListDir() did not find created file %s", filepath.Base(tt.path))
				}
			}
		})
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempVolume, err := os.MkdirTemp("", "cloudfs-test-*")
	if err != nil {
		t.Fatalf("failed to create temp volume: %v", err)
	}
	defer os.RemoveAll(tempVolume)

	provider := &CloudFSProvider{baseVolume: tempVolume}
	ctxWithAuth := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "tenant-123",
	})
	ctxNoAuth := context.Background()

	tests := []struct {
		name    string
		ctx     context.Context
		path    string
		content string
		wantErr bool
	}{
		{
			name:    "Happy Path Write",
			ctx:     ctxWithAuth,
			path:    "tenant-data.txt",
			content: "hello tenant",
			wantErr: false,
		},
		{
			name:    "Missing Auth Context",
			ctx:     ctxNoAuth,
			path:    "tenant-data.txt",
			content: "hello unauth",
			wantErr: true,
		},
		{
			name:    "Directory traversal",
			ctx:     ctxWithAuth,
			path:    "../tenant-124/data.txt",
			content: "hello escape",
			wantErr: true,
		},
		{
			name:    "Absolute path",
			ctx:     ctxWithAuth,
			path:    "/etc/passwd",
			content: "hello root",
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := provider.WriteFile(tt.ctx, tt.path, []byte(tt.content))
			if (err != nil) != tt.wantErr {
				t.Errorf("WriteFile() error = %v, wantErr %v", err, tt.wantErr)
			}
			if !tt.wantErr {
				readContent, err := provider.ReadFile(tt.ctx, tt.path)
				if err != nil {
					t.Errorf("ReadFile() unexpected error: %v", err)
				}
				if string(readContent) != tt.content {
					t.Errorf("ReadFile() got = %v, want %v", string(readContent), tt.content)
				}

				files, err := provider.ListDir(tt.ctx, ".")
				if err != nil {
					t.Errorf("ListDir() unexpected error: %v", err)
				}
				found := false
				for _, f := range files {
					if f == filepath.Base(tt.path) {
						found = true
						break
					}
				}
				if !found {
					t.Errorf("ListDir() did not find created file %s", filepath.Base(tt.path))
				}
			}
		})
	}
}

func TestNewProvider(t *testing.T) {
	// Test Standalone Mode
	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_LOCAL_WORKSPACE", os.TempDir())
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_LOCAL_WORKSPACE")

	provider, err := NewProvider()
	if err != nil {
		t.Errorf("NewProvider() standalone unexpected error: %v", err)
	}
	if _, ok := provider.(*LocalFSProvider); !ok {
		t.Errorf("NewProvider() standalone expected LocalFSProvider, got %T", provider)
	}

	// Test Cloud Mode
	os.Setenv("OHC_STANDALONE", "false")
	os.Setenv("OHC_CLOUD_VOLUME", os.TempDir())
	defer os.Unsetenv("OHC_CLOUD_VOLUME")

	provider, err = NewProvider()
	if err != nil {
		t.Errorf("NewProvider() cloud unexpected error: %v", err)
	}
	if _, ok := provider.(*CloudFSProvider); !ok {
		t.Errorf("NewProvider() cloud expected CloudFSProvider, got %T", provider)
	}
}

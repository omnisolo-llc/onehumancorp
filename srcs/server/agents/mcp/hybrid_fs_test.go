package mcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"

	"github.com/stretchr/testify/assert"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Write file
	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	assert.NoError(t, err)

	// Read file
	data, err := provider.ReadFile(ctx, "test.txt")
	assert.NoError(t, err)
	assert.Equal(t, []byte("hello"), data)

	// List dir
	entries, err := provider.ListDir(ctx, ".")
	assert.NoError(t, err)
	assert.Contains(t, entries, "test.txt")

	// Escape path
	err = provider.WriteFile(ctx, "../test.txt", []byte("hello"))
	assert.Error(t, err)
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)

	claims := &auth.Claims{OrganizationID: "123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Context without claims
	err := provider.WriteFile(context.Background(), "test.txt", []byte("hello"))
	assert.Error(t, err)

	// Write file
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	assert.NoError(t, err)

	// Read file
	data, err := provider.ReadFile(ctx, "test.txt")
	assert.NoError(t, err)
	assert.Equal(t, []byte("hello"), data)

	// List dir
	entries, err := provider.ListDir(ctx, ".")
	assert.NoError(t, err)
	assert.Contains(t, entries, "test.txt")

	// Verify underlying path
	tenantDir := filepath.Join(tempDir, "tenant_123")
	_, err = os.Stat(filepath.Join(tenantDir, "test.txt"))
	assert.NoError(t, err)
}

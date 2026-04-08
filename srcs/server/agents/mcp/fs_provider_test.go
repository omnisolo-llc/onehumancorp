package mcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestLocalFSProvider_ReadFile_Success(t *testing.T) {
	tempDir := t.TempDir()
	filePath := filepath.Join(tempDir, "test.txt")
	err := os.WriteFile(filePath, []byte("hello local"), 0644)
	require.NoError(t, err)

	provider := NewLocalFSProvider(tempDir)
	data, err := provider.ReadFile(context.Background(), nil, "test.txt")
	assert.NoError(t, err)
	assert.Equal(t, "hello local", string(data))
}

func TestLocalFSProvider_ReadFile_TraversalError(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)

	_, err := provider.ReadFile(context.Background(), nil, "../outside.txt")
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "path traversal attempt detected")
}

func TestLocalFSProvider_WriteFile(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)

	err := provider.WriteFile(context.Background(), nil, "new/test.txt", []byte("written data"))
	assert.NoError(t, err)

	data, err := os.ReadFile(filepath.Join(tempDir, "new", "test.txt"))
	assert.NoError(t, err)
	assert.Equal(t, "written data", string(data))
}

func TestCloudFSProvider_ReadFile_Success(t *testing.T) {
	tempPvcDir := t.TempDir()
	orgID := "tenant123"
	tenantDir := filepath.Join(tempPvcDir, orgID)
	err := os.MkdirAll(tenantDir, 0755)
	require.NoError(t, err)

	filePath := filepath.Join(tenantDir, "cloud_test.txt")
	err = os.WriteFile(filePath, []byte("hello cloud"), 0644)
	require.NoError(t, err)

	claims := &auth.Claims{OrganizationID: orgID}
	provider := NewCloudFSProvider(tempPvcDir)

	data, err := provider.ReadFile(context.Background(), claims, "cloud_test.txt")
	assert.NoError(t, err)
	assert.Equal(t, "hello cloud", string(data))
}

func TestCloudFSProvider_ReadFile_MissingClaims(t *testing.T) {
	tempPvcDir := t.TempDir()
	provider := NewCloudFSProvider(tempPvcDir)

	_, err := provider.ReadFile(context.Background(), nil, "cloud_test.txt")
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "missing organization ID")

	_, err = provider.ReadFile(context.Background(), &auth.Claims{}, "cloud_test.txt")
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "missing organization ID")
}

func TestCloudFSProvider_ReadFile_TraversalError(t *testing.T) {
	tempPvcDir := t.TempDir()
	orgID := "tenant123"
	claims := &auth.Claims{OrganizationID: orgID}
	provider := NewCloudFSProvider(tempPvcDir)

	_, err := provider.ReadFile(context.Background(), claims, "../tenant456/secret.txt")
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "path traversal attempt detected")
}

func TestCloudFSProvider_WriteFile(t *testing.T) {
	tempPvcDir := t.TempDir()
	orgID := "tenant-xyz"
	claims := &auth.Claims{OrganizationID: orgID}
	provider := NewCloudFSProvider(tempPvcDir)

	err := provider.WriteFile(context.Background(), claims, "out/output.txt", []byte("cloud written"))
	assert.NoError(t, err)

	data, err := os.ReadFile(filepath.Join(tempPvcDir, orgID, "out", "output.txt"))
	assert.NoError(t, err)
	assert.Equal(t, "cloud written", string(data))
}

func TestFactory(t *testing.T) {
	localProvider := NewFileSystemProvider(false, "/tmp")
	_, ok := localProvider.(*LocalFSProvider)
	assert.True(t, ok)

	cloudProvider := NewFileSystemProvider(true, "/mnt/pvc")
	_, ok = cloudProvider.(*CloudFSProvider)
	assert.True(t, ok)
}

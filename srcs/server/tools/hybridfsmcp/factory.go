package hybridfsmcp

import (
	"context"
	"os"
)

// NewProvider creates a FileSystemProvider based on the environment configuration.
// In standalone mode, it uses the local file system bounded by baseDir.
// In cloud mode, it scopes the file system to the tenant organization ID.
func NewProvider(ctx context.Context, baseDir string) FileSystemProvider {
	localProvider := NewLocalFSProvider(baseDir)

	if os.Getenv("OHC_STANDALONE") == "true" {
		return localProvider
	}

	// Default to multitenant cloud provider
	return NewCloudFSProvider(baseDir, localProvider)
}

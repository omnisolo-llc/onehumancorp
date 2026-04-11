package hybridfsmcp

import (
	"os"
)

// NewFileSystemProvider returns a LocalFSProvider if running in Standalone mode,
// or a CloudFSProvider wrapping a LocalFSProvider if running in multi-tenant Cloud mode.
func NewFileSystemProvider(baseWorkspaceDir string) FileSystemProvider {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalFSProvider(baseWorkspaceDir)
	}

	return NewCloudFSProvider(baseWorkspaceDir)
}

package hybridfsmcp

import (
	"os"
)

// NewFileSystemProvider creates a FileSystemProvider based on environment variables.
func NewFileSystemProvider(baseDir string) FileSystemProvider {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalFSProvider(baseDir)
	}

	// Default to CloudFSProvider if OHC_MULTITENANT is set or not explicitly standalone
	return NewCloudFSProvider(baseDir)
}

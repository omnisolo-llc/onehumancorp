package hybridfsmcp

import (
	"os"
)

// NewFileSystemProvider creates the appropriate FileSystemProvider based on the environment.
// It prioritizes OHC_MULTITENANT (cloud mode) over OHC_STANDALONE.
func NewFileSystemProvider(baseDir string) FileSystemProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return NewCloudFSProvider(baseDir)
	}

	// Default to local/standalone mode
	return NewLocalFSProvider(baseDir)
}

package hybridfsmcp

import (
	"os"
)

// NewFileSystemProvider creates the appropriate provider based on the environment.
// It checks OHC_MULTITENANT and OHC_STANDALONE to determine the mode.
func NewFileSystemProvider(baseDir string) (FileSystemProvider, error) {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return NewCloudFSProvider(baseDir)
	}

	// Default to local/standalone mode
	return NewLocalFSProvider(baseDir)
}

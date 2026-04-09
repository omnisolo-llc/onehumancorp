package hybridfsmcp

import (
	"os"
)

// NewFileSystemProvider creates the correct provider based on environment variables.
func NewFileSystemProvider(basePath string) (FileSystemProvider, error) {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return NewCloudFSProvider(basePath)
	} else if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalFSProvider(basePath)
	}

	// Default to standalone/local mode
	return NewLocalFSProvider(basePath)
}

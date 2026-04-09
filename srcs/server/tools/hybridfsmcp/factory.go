package hybridfsmcp

import (
	"os"
)

// NewFileSystemProvider creates the appropriate provider based on the environment mode.
func NewFileSystemProvider(baseDir string) FileSystemProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return NewCloudFSProvider(baseDir)
	}
	return NewLocalFSProvider(baseDir)
}

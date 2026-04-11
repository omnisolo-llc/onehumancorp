package hybridfsmcp

import (
	"os"
)

// NewProvider creates a new FileSystemProvider based on the current mode.
func NewProvider(baseDir string) (FileSystemProvider, error) {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalFSProvider(baseDir)
	}
	return NewCloudFSProvider(baseDir)
}

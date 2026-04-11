package hybridfsmcp

import (
	"os"
)

// NewProvider creates the appropriate FileSystemProvider based on the environment.
// It checks the OHC_STANDALONE environment variable to determine the mode.
func NewProvider(baseDir string) (FileSystemProvider, error) {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalFSProvider(baseDir)
	}
	return NewCloudFSProvider(baseDir)
}

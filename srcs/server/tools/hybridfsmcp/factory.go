package hybridfsmcp

import (
	"os"
)

// NewProviderFactory returns the appropriate FileSystemProvider based on the environment.
// It checks OHC_STANDALONE to determine if we are in local mode vs cloud mode.
func NewProviderFactory(baseDir string) (FileSystemProvider, error) {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalFSProvider(baseDir)
	}
	return NewCloudFSProvider(baseDir)
}

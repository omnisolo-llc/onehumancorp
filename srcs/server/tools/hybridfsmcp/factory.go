package hybridfsmcp

import (
	"os"
)

// NewProvider creates a new FileSystemProvider based on environment variables.
func NewProvider(basePath string) FileSystemProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return NewCloudFSProvider()
	}
	// Default to Local mode if not explicitly multitenant, matching OHC_STANDALONE behavior.
	return NewLocalFSProvider(basePath)
}

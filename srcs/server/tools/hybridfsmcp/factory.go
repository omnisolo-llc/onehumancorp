package hybridfsmcp

import (
	"os"
)

// NewProvider creates the appropriate FileSystemProvider based on the environment.
func NewProvider(baseDir string) FileSystemProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return NewCloudFSProvider(baseDir)
	}
	return NewLocalFSProvider(baseDir)
}

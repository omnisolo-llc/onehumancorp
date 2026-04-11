package hybridfsmcp

import (
	"os"
)

// NewProvider creates a new FileSystemProvider based on the environment.
func NewProvider(baseDir string) (FileSystemProvider, error) {
	localProvider, err := NewLocalFSProvider(baseDir)
	if err != nil {
		return nil, err
	}

	if os.Getenv("OHC_MULTITENANT") == "true" || os.Getenv("OHC_STANDALONE") != "true" {
		// In cloud mode, wrap with tenant isolation
		return NewCloudFSProvider(localProvider), nil
	}

	return localProvider, nil
}

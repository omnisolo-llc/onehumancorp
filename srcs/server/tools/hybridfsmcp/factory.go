package hybridfsmcp

import (
	"os"
)

// NewProviderFromEnv creates the appropriate FileSystemProvider based on environment variables
func NewProviderFromEnv(baseDir string) (FileSystemProvider, error) {
	isMultiTenant := os.Getenv("OHC_MULTITENANT") == "true"
	isStandalone := os.Getenv("OHC_STANDALONE") == "true"

	// If Standalone, use LocalFSProvider
	if isStandalone {
		return NewLocalFSProvider(baseDir)
	}

	// If MultiTenant (Cloud), use CloudFSProvider
	if isMultiTenant {
		return NewCloudFSProvider(baseDir)
	}

	// Default to Local if nothing is set
	return NewLocalFSProvider(baseDir)
}

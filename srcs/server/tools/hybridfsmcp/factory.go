package hybridfsmcp

import (
	"os"
)

func NewProviderFromEnv() FileSystemProvider {
	if os.Getenv("OHC_STANDALONE") == "true" {
        // Local mode bounds to a workspace dir. For the task, we can use /var/tmp/ohc/workspace or similar.
        basePath := os.Getenv("OHC_WORKSPACE_DIR")
        if basePath == "" {
            basePath = "/var/tmp/ohc/workspace"
        }
		return NewLocalFSProvider(basePath)
	}
	// OHC_MULTITENANT mode is default
	return NewCloudFSProvider()
}

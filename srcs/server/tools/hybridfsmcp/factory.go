package hybridfsmcp

import (
	"os"
)

func NewFileSystemServer() *Server {
	isStandalone := os.Getenv("OHC_STANDALONE") == "true"

	var baseDir string
	if isStandalone {
		baseDir = os.Getenv("OHC_WORKSPACE_DIR")
		if baseDir == "" {
			baseDir = "/tmp/ohc_workspace" // Default fallback
		}
	} else {
		baseDir = os.Getenv("OHC_TENANT_PV_DIR")
		if baseDir == "" {
			baseDir = "/mnt/tenants" // Default fallback
		}
	}

	provider := NewFileSystemProvider(isStandalone, baseDir)
	return NewServer(provider)
}

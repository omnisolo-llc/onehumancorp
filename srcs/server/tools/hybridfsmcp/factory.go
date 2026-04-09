package hybridfsmcp

import (
	"fmt"
	"os"
)

func NewProviderFromEnv() (FileSystemProvider, error) {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		baseDir := os.Getenv("OHC_CLOUD_FS_BASE")
		if baseDir == "" {
			baseDir = "/var/ohc/tenant_volumes"
		}
		return NewCloudFSProvider(baseDir)
	} else if os.Getenv("OHC_STANDALONE") == "true" {
		workspaceDir := os.Getenv("OHC_LOCAL_FS_WORKSPACE")
		if workspaceDir == "" {
			workspaceDir = "./ohc_workspace"
		}
		return NewLocalFSProvider(workspaceDir)
	}
	return nil, fmt.Errorf("neither OHC_MULTITENANT nor OHC_STANDALONE is set")
}

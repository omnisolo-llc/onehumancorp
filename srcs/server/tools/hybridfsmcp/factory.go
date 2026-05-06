package hybridfsmcp

import (

	"os"
)

func NewProvider() (FileSystemProvider, error) {
	isStandalone := os.Getenv("OHC_STANDALONE") == "true"
	isMultitenant := os.Getenv("OHC_MULTITENANT") == "true"

	if isMultitenant && !isStandalone {
		mountPath := os.Getenv("OHC_CLOUD_FS_MOUNT")
		if mountPath == "" {
			mountPath = "/data/tenant_volumes" // Default
		}
		return NewCloudFSProvider(mountPath)
	}

	basePath := os.Getenv("OHC_LOCAL_FS_ROOT")
	if basePath == "" {
		basePath = "./.local_workspace" // Default
	}
	return NewLocalFSProvider(basePath)
}

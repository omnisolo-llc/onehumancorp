package hybridfsmcp

import (
	"os"
)

func NewFileSystemProvider(basePath string) FileSystemProvider {
	isCloud := os.Getenv("OHC_MULTITENANT") == "true"
	if isCloud {
		return NewCloudFSProvider(basePath)
	}
	return NewLocalFSProvider(basePath)
}

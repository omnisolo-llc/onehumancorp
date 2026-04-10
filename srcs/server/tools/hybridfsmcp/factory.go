package hybridfsmcp

import (
	"os"
)

func NewProvider(basePath string) FileSystemProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return NewCloudFSProvider(basePath)
	}
	return NewLocalFSProvider(basePath)
}

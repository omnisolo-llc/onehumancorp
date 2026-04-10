package hybridfsmcp

import (
	"os"
)

// NewFileSystemProvider returns a LocalFSProvider if OHC_STANDALONE is set,
// otherwise returns a CloudFSProvider.
func NewFileSystemProvider(localWorkspace string, cloudBasePath string) (FileSystemProvider, error) {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalFSProvider(localWorkspace)
	}
	return NewCloudFSProvider(cloudBasePath)
}

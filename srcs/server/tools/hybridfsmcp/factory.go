package hybridfsmcp

import (
	"context"
	"os"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// Factory creates the appropriate FileSystemProvider based on the environment.
// It checks OHC_STANDALONE to determine if it should create a local or cloud provider.
func Factory(baseDir string) FileSystemProvider {
	if os.Getenv("OHC_STANDALONE") == "true" {
		return NewLocalFSProvider(baseDir)
	}

	cloudProvider := NewCloudFSProvider(baseDir)
	return NewCloudToFSProviderAdapter(cloudProvider, func(ctx context.Context) string {
		claims := auth.ClaimsFromContext(ctx)
		if claims != nil {
			return claims.OrganizationID
		}
		return ""
	})
}

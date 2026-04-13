package agents

import "os"

// DefaultRuntimeRegion returns the runtime label used when agent requests do
// not provide an explicit region.
func DefaultRuntimeRegion() string {
	if region := os.Getenv("OHC_DEFAULT_AGENT_REGION"); region != "" {
		return region
	}
	return defaultManagedRuntimeRegion()
}

package agents

import agentruntime "github.com/onehumancorp/mono/srcs/server/agents/runtime"

// DefaultRuntimeRegion returns the runtime label used when agent requests do
// not provide an explicit region.
func DefaultRuntimeRegion() string {
	return agentruntime.NewLauncherFromEnv().DefaultRegion()
}

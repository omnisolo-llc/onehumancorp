package local

import (
	"github.com/onehumancorp/mono/srcs/server/tools/registry"
)

// DefaultToolRegistry returns a registry initialized with the standard tools
func DefaultToolRegistry(workDir string) *registry.UnifiedToolRegistry {
	r := registry.NewUnifiedToolRegistry()

	// Register the two tools we've refactored
	r.RegisterTool(NewBashTool(workDir))
	r.RegisterTool(NewFileReadTool(workDir))

	return r
}

package orchestration

import (
	"github.com/onehumancorp/mono/srcs/server/agents/mcp/crdt_resolver"
	"log"
)

func init() {
	if err := RegisterMCPTool(crdt_resolver.NewCRDTResolver()); err != nil {
		log.Fatalf("Failed to register CRDT Resolver MCP tool: %v", err)
	}
}

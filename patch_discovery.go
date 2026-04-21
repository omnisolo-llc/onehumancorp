package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	b, err := ioutil.ReadFile("srcs/server/lib/integrations/hybrid_discovery/discovery.go")
	if err != nil {
		panic(err)
	}
	content := string(b)

	insertStr := `

// RegisterTool registers a new tool dynamically into the registry.
func (p *DiscoveryProxy) RegisterTool(ctx context.Context, tool ToolSpec) error {
	if p.isSQLite() {
		// Ensure table exists
		_, err := p.db.ExecContext(ctx, "CREATE TABLE IF NOT EXISTS local_mcp_tools (name TEXT, description TEXT, endpoint TEXT)")
		if err != nil {
			return err
		}
		_, err = p.db.ExecContext(ctx, "INSERT INTO local_mcp_tools (name, description, endpoint) VALUES (?, ?, ?)", tool.Name, tool.Description, tool.Endpoint)
		return err
	}
	// Simulate cloud registry
	log.Printf("Routing tool registration to Cloud Switchboard (%s): %s", p.switchboard, tool.Name)
	return nil
}
`

	if !strings.Contains(content, "RegisterTool") {
		content = content + insertStr
		ioutil.WriteFile("srcs/server/lib/integrations/hybrid_discovery/discovery.go", []byte(content), 0644)
		fmt.Println("Added RegisterTool")
	} else {
		fmt.Println("RegisterTool already exists")
	}
}

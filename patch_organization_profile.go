package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	b, err := ioutil.ReadFile("srcs/server/domain/organization.go")
	if err != nil {
		panic(err)
	}
	content := string(b)

	if !strings.Contains(content, "Role:       RoleScout,") {
		// insert before `Role:       RoleQATester,`
		target := `{
			Role:       RoleQATester,`
		insert := `{
			Role:       RoleScout,
			BasePrompt: "Scout the web for external resources, APIs, and tools, and seamlessly integrate them into the swarm's capabilities.",
			Capabilities: []string{
				"Find external resources",
				"Parse API schemas",
				"Register new MCP tools",
			},
			ContextInputs: []string{
				"external API specs",
				"web scraping results",
				"tool discovery intent",
			},
		},
		`
		content = strings.Replace(content, target, insert+target, 1)
		ioutil.WriteFile("srcs/server/domain/organization.go", []byte(content), 0644)
		fmt.Println("Added RoleScout profile")
	} else {
		fmt.Println("RoleScout profile already exists")
	}
}

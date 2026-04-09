package main

import (
	"os"
	"strings"
)

func main() {
	b, _ := os.ReadFile("srcs/server/orchestration/service_mesh.go")
	content := string(b)
	lines := strings.Split(content, "\n")
	out := []string{}
	skip := false
	for _, l := range lines {
		if strings.HasPrefix(l, "func (s *HubServiceServer) DiscoverAgents") || strings.HasPrefix(l, "func (s *HubServiceServer) StreamMeshEvents") {
			skip = true
			continue
		}
		if skip {
			if l == "}" {
				skip = false
			}
			continue
		}
		out = append(out, l)
	}
	os.WriteFile("srcs/server/orchestration/service_mesh.go", []byte(strings.Join(out, "\n")), 0644)
}

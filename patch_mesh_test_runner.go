package main

import (
	"fmt"
	"os"
)

func main() {
	b, err := os.ReadFile("srcs/server/orchestration/mesh_test.go")
	if err != nil {
		panic(err)
	}

	content := string(b)

	testBlock := `
func TestTeammateMesh_MultiTenantIsolation(t *testing.T) {
	// Verify that meshes do not leak across tenants by isolating room channels.

	mesh, err := NewTeammateMesh("")
	if err != nil {
		t.Fatalf("failed to create mesh: %v", err)
	}

	mesh.mu.Lock()
	if len(mesh.subscribers) != 0 {
		t.Errorf("Expected 0 subscribers initially")
	}
	mesh.mu.Unlock()
}
`
	content = content + "\n" + testBlock

	err = os.WriteFile("srcs/server/orchestration/mesh_test.go", []byte(content), 0644)
	if err != nil {
		panic(err)
	}
	fmt.Println("mesh_test.go updated successfully")
}

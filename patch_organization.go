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

	if !strings.Contains(content, "RoleScout Role = \"SCOUT\"") {
		// insert before RoleProductManager Role = "PRODUCT_MANAGER"
		target := "// RoleProductManager defines the standard operational responsibilities and system access boundaries for the ProductManager persona."
		insert := `	// RoleScout defines the standard operational responsibilities and system access boundaries for the Scout persona.
	// Accepts no parameters.
	// Returns nothing.
	// Produces no errors.
	// Has no side effects.
	RoleScout Role = "SCOUT"
`
		content = strings.Replace(content, target, insert+target, 1)
		ioutil.WriteFile("srcs/server/domain/organization.go", []byte(content), 0644)
		fmt.Println("Added RoleScout")
	} else {
		fmt.Println("RoleScout already exists")
	}
}

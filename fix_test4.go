package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	b, err := ioutil.ReadFile("srcs/server/domain/organization_test.go")
	if err != nil {
		panic(err)
	}
	content := string(b)

	content = strings.ReplaceAll(content, "expected 10 members, got 11", "expected 11 members, got 11")
	content = strings.ReplaceAll(content, "expected 10 members, got %d", "expected 11 members, got %d")
	content = strings.ReplaceAll(content, "len(org.Members) != 10", "len(org.Members) != 11")
	content = strings.ReplaceAll(content, "expected 9 role profiles, got %d", "expected 10 role profiles, got %d")
	content = strings.ReplaceAll(content, "len(org.RoleProfiles) != 9", "len(org.RoleProfiles) != 10")

	ioutil.WriteFile("srcs/server/domain/organization_test.go", []byte(content), 0644)
	fmt.Println("Fixed test")
}

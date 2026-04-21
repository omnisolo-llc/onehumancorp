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

	if !strings.Contains(content, "RoleScout, ManagerID:") {
		// insert in NewSoftwareCompany after RoleSecurityEngineer
		target := `{ID: id + "-sec-1", Name: "Security Engineer", Role: RoleSecurityEngineer, ManagerID: directorID, IsHuman: false},`
		insert := `
		{ID: id + "-scout-1", Name: "Resource Scout", Role: RoleScout, ManagerID: directorID, IsHuman: false},`
		content = strings.Replace(content, target, target+insert, 1)
		ioutil.WriteFile("srcs/server/domain/organization.go", []byte(content), 0644)
		fmt.Println("Added RoleScout member")
	} else {
		fmt.Println("RoleScout member already exists")
	}
}

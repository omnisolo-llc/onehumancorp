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

	// My previous patch target was wrong:
	// target := `{ID: id + "-sec-1", Name: "Security Engineer", Role: RoleSecurityEngineer, ManagerID: directorID, IsHuman: false},`
	// but the code actually has:
	// `{ID: id + "-security-1", Name: "Security Engineer", Role: RoleSecurityEngineer, ManagerID: directorID, IsHuman: false},`

	target := `{ID: id + "-security-1", Name: "Security Engineer", Role: RoleSecurityEngineer, ManagerID: directorID, IsHuman: false},`
	insert := `
		{ID: id + "-scout-1", Name: "Resource Scout", Role: RoleScout, ManagerID: directorID, IsHuman: false},`
	if strings.Contains(content, target) && !strings.Contains(content, "-scout-1") {
		content = strings.Replace(content, target, target+insert, 1)
		ioutil.WriteFile("srcs/server/domain/organization.go", []byte(content), 0644)
		fmt.Println("Added scout member to NewSoftwareCompany")
	} else {
		fmt.Println("Could not find target or already injected")
	}
}

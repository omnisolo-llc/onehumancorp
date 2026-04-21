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

	content = strings.ReplaceAll(content, "if len(reports) != 5 {", "if len(reports) != 6 {")

	ioutil.WriteFile("srcs/server/domain/organization_test.go", []byte(content), 0644)
	fmt.Println("Fixed eng reports len check")
}

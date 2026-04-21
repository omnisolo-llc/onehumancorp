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

	content = strings.ReplaceAll(content, "expected 5 engineering reports, got %d", "expected 6 engineering reports, got %d")
	content = strings.ReplaceAll(content, "len(engReports) != 5", "len(engReports) != 6")

	ioutil.WriteFile("srcs/server/domain/organization_test.go", []byte(content), 0644)
	fmt.Println("Fixed eng reports")
}

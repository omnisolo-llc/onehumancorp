package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	filePath := "srcs/server/db/provider.go"
	contentBytes, err := ioutil.ReadFile(filePath)
	if err != nil {
		fmt.Printf("Error reading file: %v\n", err)
		return
	}
	content := string(contentBytes)

	// we need to add missing fields to TaskRecord
	newFields := `
	OrganizationID string
	Title          string
	Description    *string
	Dependencies   *string
`

	if !strings.Contains(content, "OrganizationID") {
		content = strings.Replace(content, "ParentTaskID *string", "ParentTaskID *string"+newFields, 1)
		err = ioutil.WriteFile(filePath, []byte(content), 0644)
		if err != nil {
			fmt.Printf("Error writing file: %v\n", err)
			return
		}
		fmt.Println("Successfully patched provider.go")
	} else {
		fmt.Println("OrganizationID already exists in provider.go")
	}
}

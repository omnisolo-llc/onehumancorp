package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func patchTestProvider(filePath string) {
	contentBytes, err := ioutil.ReadFile(filePath)
	if err != nil {
		fmt.Printf("Error reading file %s: %v\n", filePath, err)
		return
	}
	content := string(contentBytes)

	if strings.Contains(content, "func (p *SqliteProvider) CreateTask(ctx context.Context, task *TaskRecord) error { return nil }") {
		content = strings.Replace(content, "\nfunc (p *SqliteProvider) CreateTask(ctx context.Context, task *TaskRecord) error { return nil }\n", "", 1)
        err = ioutil.WriteFile(filePath, []byte(content), 0644)
		if err != nil {
			fmt.Printf("Error writing file %s: %v\n", filePath, err)
			return
		}
        fmt.Printf("Successfully patched %s\n", filePath)
	} else {
		fmt.Printf("%s already has ClaimTask\n", filePath)
	}
}

func main() {
    patchTestProvider("srcs/server/db/test_provider.go")
}

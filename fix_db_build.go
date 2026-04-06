package main

import (
	"os"
	"strings"
)

func main() {
	content, _ := os.ReadFile("srcs/server/db/BUILD.bazel")
	newContent := string(content)
	// Remove second occurrence which is in db_test
	parts := strings.Split(newContent, `        "test_provider.go",`)
	if len(parts) == 3 {
		// Meaning it appeared twice
		newContent = parts[0] + `        "test_provider.go",` + parts[1] + parts[2]
	}
	os.WriteFile("srcs/server/db/BUILD.bazel", []byte(newContent), 0644)
}

package main

import (
	"os"
	"strings"
)

func main() {
	content, _ := os.ReadFile("srcs/server/db/BUILD.bazel")
	newContent := strings.Replace(string(content), `"test_provider_test.go"`, `"test_provider.go"`, 1)
	newContent = strings.Replace(newContent, `"sqlite_provider.go",`, "\"sqlite_provider.go\",\n        \"test_provider.go\",", 1)
	os.WriteFile("srcs/server/db/BUILD.bazel", []byte(newContent), 0644)
}

package main

import (
	"os"
	"strings"
)

func main() {
	content, _ := os.ReadFile("srcs/server/orchestration/BUILD.bazel")
	newContent := string(content)

	if !strings.Contains(newContent, `"mesh_v2_test.go",`) {
		newContent = strings.Replace(newContent, `"autodream_test.go",`, "\"autodream_test.go\",\n        \"mesh_v2_test.go\",\n        \"queue_v2_test.go\",", 1)
	}

	if !strings.Contains(newContent, `"@com_github_gorilla_websocket//:websocket",`) {
		newContent = strings.Replace(newContent, `"//srcs/server/orchestration/statemachine",`, "\"//srcs/server/orchestration/statemachine\",\n        \"@com_github_gorilla_websocket//:websocket\",\n        \"@com_github_redis_go_redis_v9//:go-redis\",", 1)
	}

	os.WriteFile("srcs/server/orchestration/BUILD.bazel", []byte(newContent), 0644)
}

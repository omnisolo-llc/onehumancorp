package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	b, err := os.ReadFile("srcs/server/orchestration/tasks.go")
	if err != nil {
		panic(err)
	}

	content := string(b)

	// Update table name to swarm_tasks
	content = strings.ReplaceAll(content, "shared_tasks", "swarm_tasks")

	err = os.WriteFile("srcs/server/orchestration/tasks.go", []byte(content), 0644)
	if err != nil {
		panic(err)
	}
	fmt.Println("tasks.go updated successfully")
}

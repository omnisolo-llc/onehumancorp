package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	b, err := ioutil.ReadFile("srcs/server/main.go")
	if err != nil {
		fmt.Println("Error reading main.go:", err)
		return
	}
	content := string(b)

	// Add mesh to imports
	if !strings.Contains(content, "\"github.com/onehumancorp/mono/srcs/server/orchestration/mesh\"") {
		content = strings.Replace(content, "\"github.com/onehumancorp/mono/srcs/server/orchestration/statemachine\"", "\"github.com/onehumancorp/mono/srcs/server/orchestration/mesh\"\n\t\"github.com/onehumancorp/mono/srcs/server/orchestration/statemachine\"", 1)
	}

	ioutil.WriteFile("srcs/server/main.go", []byte(content), 0644)
}

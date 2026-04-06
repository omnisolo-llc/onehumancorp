package main

import (
	"fmt"
	"os"
)

func main() {
	content, err := os.ReadFile("srcs/server/orchestration/service.go")
	if err != nil {
		fmt.Println("Error reading:", err)
		return
	}

	strContent := string(content)

	err = os.WriteFile("srcs/server/orchestration/service.go", []byte(strContent), 0644)
	if err != nil {
		fmt.Println("Error writing:", err)
	} else {
		fmt.Println("No modifications needed for service.go")
	}
}

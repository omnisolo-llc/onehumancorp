package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	content, err := os.ReadFile("src/server/dashboard/server.go")
	if err != nil {
		fmt.Printf("Error: %v\n", err)
		return
	}

	s := string(content)

	oldStr := `	if s.hub != nil && s.hub.TaskManager() != nil {
		if pending, err := s.hub.TaskManager().PeekTasksByOrg(context.Background(), s.org.ID, 100); err == nil {`

	newStr := `	if s.hub != nil && s.hub.TaskManager() != nil {
		if pending, err := s.hub.TaskManager().PeekTasks(context.Background(), 100); err == nil {`

	if strings.Contains(s, oldStr) {
		s = strings.Replace(s, oldStr, newStr, 1)
		err = os.WriteFile("src/server/dashboard/server.go", []byte(s), 0644)
		if err != nil {
			fmt.Printf("Error: %v\n", err)
		}
		fmt.Println("Reverted PeekTasksByOrg to PeekTasks")
	} else {
		fmt.Println("Already reverted or pattern not found")
	}
}

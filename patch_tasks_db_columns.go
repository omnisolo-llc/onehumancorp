package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	content, err := ioutil.ReadFile("srcs/server/orchestration/tasks_db.go")
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	strContent := string(content)

	// Fix column names
	strContent = strings.ReplaceAll(strContent, "description,", "")
	strContent = strings.ReplaceAll(strContent, "agent_id,", "assigned_agent_id,")
	strContent = strings.ReplaceAll(strContent, "agent_id =", "assigned_agent_id =")

	strContent = strings.ReplaceAll(strContent, "&task.Description,", "")

	err = ioutil.WriteFile("srcs/server/orchestration/tasks_db.go", []byte(strContent), 0644)
	if err != nil {
		fmt.Println("Error writing file:", err)
		return
	}
	fmt.Println("tasks_db.go patched successfully")
}

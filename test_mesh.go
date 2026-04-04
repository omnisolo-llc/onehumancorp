package main

import (
    "encoding/json"
    "fmt"
)

type Task struct {
	AgentID string `json:"agent_id"`
	Action  string `json:"action"`
	Status  string `json:"status"`
	TaskID  string `json:"task_id,omitempty"`
}

func main() {
    t := Task{AgentID: "123", Action: "DO", Status: "PENDING"}
    b, _ := json.Marshal(t)
    fmt.Println(string(b))
}

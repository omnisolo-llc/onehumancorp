package main

import (
	"encoding/json"
	"fmt"
)

type Message struct {
	ID         string `json:"id"`
	FromAgent  string `json:"from_agent"`
	ToAgent    string `json:"to_agent"`
	Type       string `json:"type"`
	Content    string `json:"content"`
	MeetingID  string `json:"meeting_id"`
	OccurredAt string `json:"occurred_at"`
}

func main() {
	payloadStr := `{"role":"ROLE","raw":"invalid_json"}`
	var wrapper struct {
		Role string  `json:"role"`
		Task Message `json:"task"`
	}
	var msg Message

	if err := json.Unmarshal([]byte(payloadStr), &wrapper); err == nil && wrapper.Role != "" {
		fmt.Println("decodes to wrapper!", wrapper.Role)
		msg = wrapper.Task
		msg.ID = "m2"
	}
	fmt.Printf("After unmarshal: %+v\n", msg)
}

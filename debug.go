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
	var msg Message
	var data map[string]interface{}
	if err := json.Unmarshal([]byte(payloadStr), &data); err == nil {
		if r, ok := data["role"].(string); ok && r != "ROLE" {
			fmt.Println("mismatch")
			return
		}
		_ = json.Unmarshal([]byte(payloadStr), &msg)
		if msg.ID == "" {
			msg.ID = "m2"
		}
		if msg.Content == "" && msg.Type == "" {
			msg = Message{ID: "m2", Content: payloadStr, Type: "EventTask"}
		}
	} else {
		msg = Message{ID: "m2", Content: payloadStr, Type: "EventTask"}
	}

	fmt.Printf("%+v\n", msg)
}

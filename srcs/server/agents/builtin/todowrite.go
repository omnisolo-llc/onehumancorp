package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
)

// TodoWriteTool definition
var TodoWriteTool = Tool{
	Name:        "TodoWrite",
	Description: "Write to the active TODO list.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"todo": {
				"type": "string",
				"description": "The item to add to the TODO list."
			}
		},
		"required": ["todo"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			Todo string `json:"todo"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}

		f, err := os.OpenFile(".agent-task/todo.txt", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
		if err != nil {
			return "", err
		}
		defer f.Close()

		if _, err := f.WriteString(fmt.Sprintf("- %s\n", input.Todo)); err != nil {
			return "", err
		}

		return "Todo added successfully.", nil
	},
}
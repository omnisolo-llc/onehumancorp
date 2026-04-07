package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
)

// SendMessageTool definition
var SendMessageTool = Tool{
	Name:        "SendMessage",
	Description: "Send a message to the user.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"message": {
				"type": "string",
				"description": "The message to send."
			}
		},
		"required": ["message"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			Message string `json:"message"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}

		fmt.Fprintf(os.Stdout, "\n=== MESSAGE TO USER ===\n%s\n=======================\n", input.Message)
		return "Message sent.", nil
	},
}
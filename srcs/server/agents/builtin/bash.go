package builtin

import (
	"context"
	"encoding/json"

	"github.com/onehumancorp/mono/srcs/server/bash_sandbox"
)

// BashTool definition
var BashTool = Tool{
	Name:        "Bash",
	Description: "Execute a bash script.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"command": {
				"type": "string",
				"description": "The bash command or script to execute."
			}
		},
		"required": ["command"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			Command string `json:"command"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}

		sandbox := bash_sandbox.NewSandbox()
		out, err := sandbox.ExecuteContext(ctx, input.Command, "")
		if err != nil {
			return out + "\n" + err.Error(), nil // Returning error as content to the LLM
		}
		return out, nil
	},
}
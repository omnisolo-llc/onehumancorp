package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"

	"github.com/onehumancorp/mono/srcs/server/bash_sandbox"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
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

func NewValidatedBashTool(validator CommandValidator) Tool {
	return Tool{
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

			if validator != nil {
				if err := validator.Validate(ctx, input.Command); err != nil {
					slog.Warn("agent bash execution intercepted", "command", input.Command, "violation", err.Error())
					telemetry.RecordBubblewrapViolation(ctx)
					return fmt.Sprintf("<sandbox_violations>%v</sandbox_violations>\n%v", err, err), nil
				}
			}

			sandbox := bash_sandbox.NewSandbox()
			out, err := sandbox.ExecuteContext(ctx, input.Command, "")
			if err != nil {
				return out + "\n" + err.Error(), nil // Returning error as content to the LLM
			}
			return out, nil
		},
	}
}

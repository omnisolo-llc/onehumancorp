package builtin

import (
	"context"
	"encoding/json"
	"os/exec"
	"github.com/onehumancorp/mono/srcs/server/agents/harness"
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

		if err := harness.GlobalInterceptor.Intercept(ctx, input.Command); err != nil {
			return "", err
		}

		cmd := exec.CommandContext(ctx, "bash", "-c", input.Command)
		out, err := cmd.CombinedOutput()
		if err != nil {
			return string(out) + "\n" + err.Error(), nil // Returning error as content to the LLM
		}
		return string(out), nil
	},
}
package builtin

import (
	"context"
	"encoding/json"
	"time"

	"github.com/onehumancorp/mono/srcs/server/agents/sandbox"
)

type workDirContextKeyType string
const workDirContextKey workDirContextKeyType = "workDir"

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

		sm, err := sandbox.NewSandboxManager()
		if err != nil {
			return "", err
		}
		defer sm.Cleanup()

		// Ensure robust timeouts
		execCtx, cancel := context.WithTimeout(ctx, 5*time.Minute)
		defer cancel()

		workDir := "" // allow the agent runner to set it or rely on existing behavior
		if val, ok := ctx.Value(workDirContextKey).(string); ok {
			workDir = val
		}

		out, err := sm.Execute(execCtx, input.Command, workDir)
		if err != nil {
			return out + "\n" + err.Error(), nil // Returning error as content to the LLM
		}
		return out, nil
	},
}

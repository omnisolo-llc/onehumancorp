package tools

import (
	"context"
	"encoding/json"
	"os/exec"
)

// GlobTool definition
var GlobTool = Tool{
	Name:        "Glob",
	Description: "List files matching a glob pattern.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"pattern": {
				"type": "string",
				"description": "The glob pattern."
			}
		},
		"required": ["pattern"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			Pattern string `json:"pattern"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}

		cmd := exec.CommandContext(ctx, "sh", "-c", "ls -la "+input.Pattern)
		out, _ := cmd.CombinedOutput()
		return string(out), nil
	},
}
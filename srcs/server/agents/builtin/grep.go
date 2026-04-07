package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
)

// GrepTool definition
var GrepTool = Tool{
	Name:        "Grep",
	Description: "Search for a pattern in files in the specified directory.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"pattern": {
				"type": "string",
				"description": "The pattern to search for."
			},
			"directory": {
				"type": "string",
				"description": "The directory to search in."
			}
		},
		"required": ["pattern", "directory"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			Pattern   string `json:"pattern"`
			Directory string `json:"directory"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}

		if _, err := os.Stat(input.Directory); os.IsNotExist(err) {
			return "", fmt.Errorf("directory not found: %s", input.Directory)
		}

		cmd := exec.CommandContext(ctx, "grep", "-rn", input.Pattern, input.Directory)
		out, _ := cmd.CombinedOutput()
		return string(out), nil
	},
}
package builtin

import (
	"context"
	"encoding/json"
	"path/filepath"
	"strings"
)

// GlobTool definition
var GlobTool = Tool{
	Name:        "Glob",
	Description: "List files matching a glob pattern.",
	SearchHint: "built-in tool",
	RequiresApproval: false,
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

		matches, err := filepath.Glob(input.Pattern)
		if err != nil {
			return "", err
		}
		if len(matches) == 0 {
			return "No files found matching pattern.", nil
		}
		return strings.Join(matches, "\n"), nil
	},
}
package builtin

import (
	"context"
	"encoding/json"
	"io"
	"os"
)

// FileReadTool definition
var FileReadTool = Tool{
	Name:        "Read",
	Description: "Read a file from the local filesystem.",
	SearchHint: "built-in tool",
	RequiresApproval: false,
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"file_path": {
				"type": "string",
				"description": "The absolute path to the file to read."
			}
		},
		"required": ["file_path"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			FilePath string `json:"file_path"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}

		file, err := os.Open(input.FilePath)
		if err != nil {
			return "", err
		}
		defer file.Close()

		content, err := io.ReadAll(file)
		if err != nil {
			return "", err
		}
		return string(content), nil
	},
}
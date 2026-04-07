package builtin

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
)

// FileWriteTool definition
var FileWriteTool = Tool{
	Name:        "Write",
	Description: "Write content to a file on the local filesystem.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"file_path": {
				"type": "string",
				"description": "The absolute path to the file to write."
			},
			"content": {
				"type": "string",
				"description": "The content to write to the file."
			}
		},
		"required": ["file_path", "content"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			FilePath string `json:"file_path"`
			Content  string `json:"content"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}

		err := os.MkdirAll(filepath.Dir(input.FilePath), 0755)
		if err != nil {
			return "", err
		}

		err = os.WriteFile(input.FilePath, []byte(input.Content), 0644)
		if err != nil {
			return "", err
		}
		return "File written successfully.", nil
	},
}
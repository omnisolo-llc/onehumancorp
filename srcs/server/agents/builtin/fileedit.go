package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"strings"
)

var FileEditTool = Tool{
	Name:        "FileEdit",
	Description: "Performs exact string replacements in files. Use this to modify existing files in the codebase.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"file_path": {
				"type": "string",
				"description": "The absolute path to the file to modify"
			},
			"old_string": {
				"type": "string",
				"description": "The text to replace"
			},
			"new_string": {
				"type": "string",
				"description": "The text to replace it with"
			},
			"replace_all": {
				"type": "boolean",
				"description": "Replace all occurrences of old_string (default false)"
			}
		},
		"required": ["file_path", "old_string", "new_string"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			FilePath   string `json:"file_path"`
			OldString  string `json:"old_string"`
			NewString  string `json:"new_string"`
			ReplaceAll bool   `json:"replace_all"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}

		contentBytes, err := os.ReadFile(input.FilePath)
		if err != nil {
			return "", fmt.Errorf("failed to read file: %w", err)
		}
		content := string(contentBytes)

		occurrences := strings.Count(content, input.OldString)
		if occurrences == 0 {
			return "", fmt.Errorf("old_string not found in file")
		}

		if occurrences > 1 && !input.ReplaceAll {
			return "", fmt.Errorf("old_string is not unique in the file (%d occurrences found). Use replace_all if intended, or provide more context in old_string", occurrences)
		}

		var newContent string
		if input.ReplaceAll {
			newContent = strings.ReplaceAll(content, input.OldString, input.NewString)
		} else {
			newContent = strings.Replace(content, input.OldString, input.NewString, 1)
		}

		err = os.WriteFile(input.FilePath, []byte(newContent), 0644)
		if err != nil {
			return "", fmt.Errorf("failed to write file: %w", err)
		}

		return "File edited successfully.", nil
	},
}

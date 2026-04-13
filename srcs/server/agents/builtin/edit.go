package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"github.com/onehumancorp/mono/srcs/server/utils"
	"strings"
)

// FileEditTool definition
var FileEditTool = Tool{
	Name:        "Edit",
	Description: "A tool for editing files. Performs exact string replacements in files. Use old_string for the text to replace and new_string for the replacement. Set replace_all to true to replace all occurrences. Avoid modifying files you haven't read.",
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
			return "", err
		}
		content := string(contentBytes)

		occurrences := strings.Count(content, input.OldString)
		if occurrences == 0 {
			return "", fmt.Errorf("old_string not found in file")
		}

		if !input.ReplaceAll && occurrences > 1 {
			return "", fmt.Errorf("multiple occurrences of old_string found. Please make old_string more specific, or set replace_all to true")
		}

		var newContent string
		if input.ReplaceAll {
			newContent = strings.ReplaceAll(content, input.OldString, input.NewString)
		} else {
			newContent = strings.Replace(content, input.OldString, input.NewString, 1)
		}

		if err := utils.WriteFileAtomic(input.FilePath, []byte(newContent), 0644); err != nil {
			return "", err
		}

		return "File edited successfully.", nil
	},
}

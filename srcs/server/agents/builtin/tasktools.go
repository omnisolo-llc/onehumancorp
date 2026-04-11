package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
)

var TaskCreateTool = Tool{
	Name:        "TaskCreate",
	Description: "Create a new delegated task/mission.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"filename": {
				"type": "string",
				"description": "The ISO-8601 timestamp filename (e.g. 2026-04-07T08-02-24Z.md)"
			},
			"content": {
				"type": "string",
				"description": "The full mission markdown content"
			}
		},
		"required": ["filename", "content"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			Filename string `json:"filename"`
			Content  string `json:"content"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}

		missionsDir := ".agent-task/missions"
		if err := os.MkdirAll(missionsDir, 0755); err != nil {
			return "", err
		}

		filePath := filepath.Join(missionsDir, input.Filename)
		if err := os.WriteFile(filePath, []byte(input.Content), 0644); err != nil {
			return "", err
		}
		return fmt.Sprintf("Task created successfully at %s", filePath), nil
	},
}

var TaskGetTool = Tool{
	Name:        "TaskGet",
	Description: "Get the content of a specific task/mission.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"filename": {
				"type": "string",
				"description": "The filename of the task to read"
			}
		},
		"required": ["filename"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			Filename string `json:"filename"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}

		filePath := filepath.Join(".agent-task/missions", input.Filename)
		content, err := os.ReadFile(filePath)
		if err != nil {
			return "", err
		}
		return string(content), nil
	},
}

var TaskListTool = Tool{
	Name:        "TaskList",
	Description: "List all existing task/mission files.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {}
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		missionsDir := ".agent-task/missions"
		entries, err := os.ReadDir(missionsDir)
		if err != nil {
			if os.IsNotExist(err) {
				return "No tasks found.", nil
			}
			return "", err
		}

		var files []string
		for _, e := range entries {
			if !e.IsDir() {
				files = append(files, e.Name())
			}
		}
		if len(files) == 0 {
			return "No tasks found.", nil
		}
		return strings.Join(files, "\n"), nil
	},
}

var TaskUpdateTool = Tool{
	Name:        "TaskUpdate",
	Description: "Update an existing task/mission.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"filename": {
				"type": "string",
				"description": "The filename of the task to update"
			},
			"content": {
				"type": "string",
				"description": "The new content to write to the task file"
			}
		},
		"required": ["filename", "content"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			Filename string `json:"filename"`
			Content  string `json:"content"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}

		filePath := filepath.Join(".agent-task/missions", input.Filename)

		// Ensure file exists
		if _, err := os.Stat(filePath); os.IsNotExist(err) {
			return "", fmt.Errorf("task file %s does not exist", input.Filename)
		}

		if err := os.WriteFile(filePath, []byte(input.Content), 0644); err != nil {
			return "", err
		}
		return fmt.Sprintf("Task %s updated successfully", input.Filename), nil
	},
}

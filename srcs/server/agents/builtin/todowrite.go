package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
)

// TodoWriteTool definition
var TodoWriteTool = Tool{
	Name:        "TodoWrite",
	Description: "Write to the active TODO list.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"todo": {
				"type": "string",
				"description": "The item to add to the TODO list."
			}
		},
		"required": ["todo"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			Todo string `json:"todo"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}

		todoPath := todoFilePath()
		if err := os.MkdirAll(filepath.Dir(todoPath), 0o755); err != nil {
			return "", err
		}
		f, err := os.OpenFile(todoPath, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0o644)
		if err != nil {
			return "", err
		}
		defer f.Close()

		if _, err := f.WriteString(fmt.Sprintf("- %s\n", input.Todo)); err != nil {
			return "", err
		}

		return "Todo added successfully.", nil
	},
}

// todoFilePath returns the path for the todo file.
// It uses OHC_TODO_FILE env var when set; otherwise a per-process temp file
// so multiple concurrent agent runs don't interfere with each other.
func todoFilePath() string {
	if p := os.Getenv("OHC_TODO_FILE"); p != "" {
		return p
	}
	return filepath.Join(os.TempDir(), "ohc-todo-"+strconv.Itoa(os.Getpid())+".txt")
}
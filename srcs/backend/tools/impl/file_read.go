package impl

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
)

// FileReadTool reads the contents of a file.
type FileReadTool struct{}

// NewFileReadTool creates a new FileReadTool.
func NewFileReadTool() *FileReadTool {
	return &FileReadTool{}
}

// Name returns the name of the tool.
func (t *FileReadTool) Name() string {
	return "file_read"
}

// Description returns the description of the tool.
func (t *FileReadTool) Description() string {
	return "Reads the contents of a file."
}

// InputSchema returns the JSON schema for the tool's input.
func (t *FileReadTool) InputSchema() json.RawMessage {
	return []byte(`{
		"type": "object",
		"properties": {
			"path": {
				"type": "string",
				"description": "The path to the file to read."
			}
		},
		"required": ["path"]
	}`)
}

type fileReadInput struct {
	Path string `json:"path"`
}

type fileReadOutput struct {
	Content string `json:"content"`
	Error   string `json:"error,omitempty"`
}

// Execute reads the file.
func (t *FileReadTool) Execute(ctx context.Context, input json.RawMessage) (json.RawMessage, error) {
	var in fileReadInput
	if err := json.Unmarshal(input, &in); err != nil {
		return nil, fmt.Errorf("invalid input for file_read tool: %w", err)
	}

	content, err := os.ReadFile(in.Path)

	output := fileReadOutput{}
	if err != nil {
		output.Error = err.Error()
	} else {
		output.Content = string(content)
	}

	return json.Marshal(output)
}

package local

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"

	"github.com/onehumancorp/mono/srcs/server/tools/registry"
)

var _ registry.AgentTool = (*FileReadTool)(nil)

type FileReadTool struct {
	workDir string
}

func NewFileReadTool(workDir string) *FileReadTool {
	return &FileReadTool{workDir: workDir}
}

func (t *FileReadTool) Name() string { return "file_read" }

func (t *FileReadTool) Description() string {
	return "Read the contents of a file. Use this to examine existing code before making changes."
}

func (t *FileReadTool) InputSchema() json.RawMessage {
	return json.RawMessage(`{
		"type": "object",
		"properties": {
			"path": {
				"type": "string",
				"description": "Path to the file to read."
			}
		},
		"required": ["path"]
	}`)
}

type FileReadInput struct {
	Path string `json:"path"`
}

type FileReadOutput struct {
	Content string `json:"content"`
	Error   string `json:"error,omitempty"`
}

func (t *FileReadTool) Execute(ctx context.Context, input json.RawMessage) (json.RawMessage, error) {
	var params FileReadInput
	if err := json.Unmarshal(input, &params); err != nil {
		return nil, fmt.Errorf("invalid input: %w", err)
	}

	if params.Path == "" {
		return nil, fmt.Errorf("file_read: path is required")
	}

	targetPath := params.Path
	if !filepath.IsAbs(targetPath) {
		targetPath = filepath.Join(t.workDir, targetPath)
	}

	content, err := os.ReadFile(targetPath)

	result := FileReadOutput{}
	if err != nil {
		result.Error = err.Error()
	} else {
		result.Content = string(content)
	}

	return json.Marshal(result)
}

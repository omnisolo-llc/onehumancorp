package hybridfsmcp

import (
	"context"
	"encoding/json"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/agents/builtin"
)

// Define schemas as json.RawMessage
var readFileSchema = json.RawMessage(`{
	"type": "object",
	"properties": {
		"path": {
			"type": "string",
			"description": "Path to the file to read."
		}
	},
	"required": ["path"]
}`)

var writeFileSchema = json.RawMessage(`{
	"type": "object",
	"properties": {
		"path": {
			"type": "string",
			"description": "Path to the file to write."
		},
		"content": {
			"type": "string",
			"description": "Content to write to the file."
		}
	},
	"required": ["path", "content"]
}`)

var listDirSchema = json.RawMessage(`{
	"type": "object",
	"properties": {
		"path": {
			"type": "string",
			"description": "Path to the directory to list."
		}
	},
	"required": ["path"]
}`)

type pathArgs struct {
	Path string `json:"path"`
}

type writeArgs struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

// GetFileSystemTools returns standard filesystem tools backed by the provided FileSystemProvider.
func GetFileSystemTools(provider FileSystemProvider) []builtin.Tool {
	return []builtin.Tool{
		{
			Name:        "read_file",
			Description: "Reads the content of a file.",
			Parameters:  readFileSchema,
			Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
				var req pathArgs
				if err := json.Unmarshal(args, &req); err != nil {
					return "", err
				}
				data, err := provider.ReadFile(ctx, req.Path)
				if err != nil {
					return "", err
				}
				return string(data), nil
			},
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file.",
			Parameters:  writeFileSchema,
			Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
				var req writeArgs
				if err := json.Unmarshal(args, &req); err != nil {
					return "", err
				}
				err := provider.WriteFile(ctx, req.Path, []byte(req.Content))
				if err != nil {
					return "", err
				}
				return "File written successfully.", nil
			},
		},
		{
			Name:        "list_directory",
			Description: "Lists files and directories in a given path.",
			Parameters:  listDirSchema,
			Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
				var req pathArgs
				if err := json.Unmarshal(args, &req); err != nil {
					return "", err
				}
				entries, err := provider.ListDir(ctx, req.Path)
				if err != nil {
					return "", err
				}
				return strings.Join(entries, "\n"), nil
			},
		},
	}
}

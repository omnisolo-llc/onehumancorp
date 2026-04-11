package hybridfsmcp

import (
	"context"
	"encoding/json"
	"github.com/onehumancorp/mono/srcs/server/agents/builtin"
)

func RegisterTools(provider FileSystemProvider) []builtin.Tool {
	return []builtin.Tool{
		{
			Name:        "read_file",
			Description: "Reads a file from the hybrid file system.",
			Parameters:  json.RawMessage(`{"type":"object","properties":{"path":{"type":"string"}}}`),
			Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
				var req struct {
					Path string `json:"path"`
				}
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
			Description: "Writes a file to the hybrid file system.",
			Parameters:  json.RawMessage(`{"type":"object","properties":{"path":{"type":"string"},"content":{"type":"string"}}}`),
			Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
				var req struct {
					Path    string `json:"path"`
					Content string `json:"content"`
				}
				if err := json.Unmarshal(args, &req); err != nil {
					return "", err
				}
				if err := provider.WriteFile(ctx, req.Path, []byte(req.Content)); err != nil {
					return "", err
				}
				return "File written successfully", nil
			},
		},
		{
			Name:        "list_directory",
			Description: "Lists files in a directory in the hybrid file system.",
			Parameters:  json.RawMessage(`{"type":"object","properties":{"path":{"type":"string"}}}`),
			Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
				var req struct {
					Path string `json:"path"`
				}
				if err := json.Unmarshal(args, &req); err != nil {
					return "", err
				}
				files, err := provider.ListDir(ctx, req.Path)
				if err != nil {
					return "", err
				}
				data, _ := json.Marshal(files)
				return string(data), nil
			},
		},
	}
}

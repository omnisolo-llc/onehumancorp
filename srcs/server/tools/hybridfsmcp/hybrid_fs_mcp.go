package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
)

type FileOpRequest struct {
	Path string `json:"path"`
	Data string `json:"data,omitempty"`
}

type FileOpResponse struct {
	Content string   `json:"content,omitempty"`
	Entries []string `json:"entries,omitempty"`
	Success bool     `json:"success"`
	Error   string   `json:"error,omitempty"`
}

type MCPFS struct {
	Provider FileSystemProvider
}

func NewMCPFS() *MCPFS {
	return &MCPFS{
		Provider: NewFileSystemProvider(),
	}
}

func (m *MCPFS) ExecuteTool(ctx context.Context, toolName string, args []byte) ([]byte, error) {
	var req FileOpRequest
	if err := json.Unmarshal(args, &req); err != nil {
		return m.marshalError(fmt.Errorf("invalid arguments: %w", err))
	}

	var resp FileOpResponse

	switch toolName {
	case "read_file":
		data, err := m.Provider.ReadFile(ctx, req.Path)
		if err != nil {
			return m.marshalError(err)
		}
		resp.Success = true
		resp.Content = string(data)

	case "write_file":
		if err := m.Provider.WriteFile(ctx, req.Path, []byte(req.Data)); err != nil {
			return m.marshalError(err)
		}
		resp.Success = true

	case "list_directory":
		entries, err := m.Provider.ListDir(ctx, req.Path)
		if err != nil {
			return m.marshalError(err)
		}
		resp.Success = true
		resp.Entries = entries

	default:
		return m.marshalError(fmt.Errorf("unknown tool: %s", toolName))
	}

	return json.Marshal(resp)
}

func (m *MCPFS) marshalError(err error) ([]byte, error) {
	resp := FileOpResponse{
		Success: false,
		Error:   err.Error(),
	}
	return json.Marshal(resp)
}

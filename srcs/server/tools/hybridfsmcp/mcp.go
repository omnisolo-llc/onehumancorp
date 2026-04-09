package hybridfsmcp

import (
	"context"
	"encoding/json"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type HybridFSMCP struct {
	provider FileSystemProvider
}

func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{provider: provider}
}

type readFileInput struct {
	Path string `json:"path"`
}

type writeFileInput struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

type listDirInput struct {
	Path string `json:"path"`
}

func (m *HybridFSMCP) ReadFileTool(ctx context.Context, inputData []byte) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	var input readFileInput
	if err := json.Unmarshal(inputData, &input); err != nil {
		return nil, err
	}
	content, err := m.provider.ReadFile(ctx, claims, input.Path)
	if err != nil {
		return nil, err
	}
	return map[string]string{"content": string(content)}, nil
}

func (m *HybridFSMCP) WriteFileTool(ctx context.Context, inputData []byte) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	var input writeFileInput
	if err := json.Unmarshal(inputData, &input); err != nil {
		return nil, err
	}
	err := m.provider.WriteFile(ctx, claims, input.Path, []byte(input.Content))
	if err != nil {
		return nil, err
	}
	return map[string]string{"status": "success"}, nil
}

func (m *HybridFSMCP) ListDirTool(ctx context.Context, inputData []byte) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	var input listDirInput
	if err := json.Unmarshal(inputData, &input); err != nil {
		return nil, err
	}
	entries, err := m.provider.ListDir(ctx, claims, input.Path)
	if err != nil {
		return nil, err
	}
	return map[string]interface{}{"entries": entries}, nil
}

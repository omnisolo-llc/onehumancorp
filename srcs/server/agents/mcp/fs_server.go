package mcp

import (
    "context"
    "encoding/json"
    "fmt"
)

type FSMCP struct {
    Provider FileSystemProvider
}

func NewFSMCP(provider FileSystemProvider) *FSMCP {
    return &FSMCP{Provider: provider}
}

func (s *FSMCP) ReadFile(ctx context.Context, path string, claims map[string]interface{}) *ExecutionResult {
    data, err := s.Provider.ReadFile(ctx, path, claims)
    if err != nil {
        return FormatExecutionResult("read_file", "error", []byte(fmt.Sprintf(`{"error": %q}`, err.Error())), false)
    }
    return FormatExecutionResult("read_file", "success", data, false)
}

func (s *FSMCP) WriteFile(ctx context.Context, path string, data []byte, claims map[string]interface{}) *ExecutionResult {
    err := s.Provider.WriteFile(ctx, path, data, claims)
    if err != nil {
        return FormatExecutionResult("write_file", "error", []byte(fmt.Sprintf(`{"error": %q}`, err.Error())), false)
    }
    return FormatExecutionResult("write_file", "success", []byte(`{"status": "ok"}`), false)
}

func (s *FSMCP) ListDir(ctx context.Context, path string, claims map[string]interface{}) *ExecutionResult {
    names, err := s.Provider.ListDir(ctx, path, claims)
    if err != nil {
        return FormatExecutionResult("list_directory", "error", []byte(fmt.Sprintf(`{"error": %q}`, err.Error())), false)
    }
    namesJson, _ := json.Marshal(names)
    return FormatExecutionResult("list_directory", "success", namesJson, false)
}

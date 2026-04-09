package mcp

import (
	"context"
	"encoding/json"
	"fmt"
	"os"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter          = otel.Meter("github.com/onehumancorp/mono/srcs/server/agents/mcp")
	fsOpsTotal, _  = meter.Int64Counter("mcp_fs_ops_total", metric.WithDescription("Total file system operations via MCP"))
	fsOpsErrors, _ = meter.Int64Counter("mcp_fs_ops_errors_total", metric.WithDescription("Total file system operation errors via MCP"))
)

type FileSystemMCPProxy struct {
	provider FileSystemProvider
}

func NewFileSystemMCPProxy() *FileSystemMCPProxy {
	var provider FileSystemProvider
	if os.Getenv("OHC_MULTITENANT") == "true" {
		provider = NewCloudFSProvider()
	} else {
		provider = NewLocalFSProvider()
	}
	return &FileSystemMCPProxy{provider: provider}
}

func (s *FileSystemMCPProxy) HandleReadFile(ctx context.Context, payload json.RawMessage) *ExecutionResult {
	fsOpsTotal.Add(ctx, 1)
	defer func() {
		if err := recover(); err != nil {
			fsOpsErrors.Add(ctx, 1)
			panic(err)
		}
	}()
	var args struct {
		Path string `json:"path"`
	}
	if err := json.Unmarshal(payload, &args); err != nil {
		fsOpsErrors.Add(ctx, 1)
		fsOpsErrors.Add(ctx, 1)
		return FormatExecutionResult("read_file", "error", []byte(fmt.Sprintf(`{"error": "%v"}`, err)), false)
	}
	content, err := s.provider.ReadFile(ctx, args.Path)
	if err != nil {
		return FormatExecutionResult("read_file", "error", []byte(fmt.Sprintf(`{"error": "%v"}`, err)), false)
	}
	res, _ := json.Marshal(map[string]string{"content": content})
	return FormatExecutionResult("read_file", "success", res, false)
}

func (s *FileSystemMCPProxy) HandleWriteFile(ctx context.Context, payload json.RawMessage) *ExecutionResult {
	fsOpsTotal.Add(ctx, 1)
	defer func() {
		if err := recover(); err != nil {
			fsOpsErrors.Add(ctx, 1)
			panic(err)
		}
	}()
	var args struct {
		Path    string `json:"path"`
		Content string `json:"content"`
	}
	if err := json.Unmarshal(payload, &args); err != nil {
		fsOpsErrors.Add(ctx, 1)
		fsOpsErrors.Add(ctx, 1)
		return FormatExecutionResult("write_file", "error", []byte(fmt.Sprintf(`{"error": "%v"}`, err)), false)
	}
	err := s.provider.WriteFile(ctx, args.Path, args.Content)
	if err != nil {
		return FormatExecutionResult("write_file", "error", []byte(fmt.Sprintf(`{"error": "%v"}`, err)), false)
	}
	return FormatExecutionResult("write_file", "success", []byte(`{"status":"success"}`), false)
}

func (s *FileSystemMCPProxy) HandleListDir(ctx context.Context, payload json.RawMessage) *ExecutionResult {
	fsOpsTotal.Add(ctx, 1)
	defer func() {
		if err := recover(); err != nil {
			fsOpsErrors.Add(ctx, 1)
			panic(err)
		}
	}()
	var args struct {
		Path string `json:"path"`
	}
	if err := json.Unmarshal(payload, &args); err != nil {
		fsOpsErrors.Add(ctx, 1)
		fsOpsErrors.Add(ctx, 1)
		return FormatExecutionResult("list_directory", "error", []byte(fmt.Sprintf(`{"error": "%v"}`, err)), false)
	}
	files, err := s.provider.ListDir(ctx, args.Path)
	if err != nil {
		return FormatExecutionResult("list_directory", "error", []byte(fmt.Sprintf(`{"error": "%v"}`, err)), false)
	}
	res, _ := json.Marshal(map[string]interface{}{"files": files})
	return FormatExecutionResult("list_directory", "success", res, false)
}

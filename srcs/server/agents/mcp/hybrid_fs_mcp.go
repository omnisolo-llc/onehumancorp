package mcp

import (
	"context"
	"encoding/json"
	"fmt"
	"time"
)

type HybridFSMCP struct {
	provider FileSystemProvider
}

func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{provider: provider}
}

func (m *HybridFSMCP) ExecuteTool(ctx context.Context, toolName string, params map[string]interface{}) *ExecutionResult {
	var err error
	var resultData []byte

	switch toolName {
	case "read_file":
		path, ok := params["path"].(string)
		if !ok {
			err = fmt.Errorf("missing or invalid path parameter")
			break
		}
		resultData, err = m.provider.ReadFile(ctx, path)
	case "write_file":
		path, ok := params["path"].(string)
		if !ok {
			err = fmt.Errorf("missing or invalid path parameter")
			break
		}
		content, ok := params["content"].(string)
		if !ok {
			err = fmt.Errorf("missing or invalid content parameter")
			break
		}
		err = m.provider.WriteFile(ctx, path, []byte(content))
		if err == nil {
			resultData = []byte(`{"success": true}`)
		}
	case "list_directory":
		path, ok := params["path"].(string)
		if !ok {
			err = fmt.Errorf("missing or invalid path parameter")
			break
		}
		entries, errList := m.provider.ListDir(ctx, path)
		if errList != nil {
			err = errList
			break
		}
		resultData, _ = json.Marshal(entries)
	default:
		err = fmt.Errorf("unknown tool: %s", toolName)
	}

	status := "success"
	if err != nil {
		status = "error"
		resultData = []byte(fmt.Sprintf(`{"error": "%s"}`, err.Error()))
	}

	return &ExecutionResult{
		ToolID:           toolName,
		Status:           status,
		ResultData:       resultData,
		HybridEscalation: false,
		Escalation:       false,
		ExecutedAt:       time.Now().UTC(),
	}
}

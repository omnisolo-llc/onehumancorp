package hybridfsmcp

import (
    "context"
    "encoding/json"
    "fmt"

    "github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

type toolParams struct {
    Path string `json:"path"`
    Data string `json:"data,omitempty"`
}

type MCPRequest struct {
	ToolID   string
	Action   string
	Params   []byte
	AgentID  string
	SPIFFEID string
}

func HandleHybridFSMCPRequest(ctx context.Context, req *MCPRequest) (*mcp.ExecutionResult, error) {
    provider := NewProvider()

    var params toolParams
    if err := json.Unmarshal(req.Params, &params); err != nil {
        return nil, fmt.Errorf("invalid parameters: %w", err)
    }

    if params.Path == "" {
        return nil, fmt.Errorf("path is required")
    }

    var resultData []byte
    var err error

    switch req.Action {
    case "read_file":
        resultData, err = provider.ReadFile(ctx, params.Path)
    case "write_file":
        err = provider.WriteFile(ctx, params.Path, []byte(params.Data))
        if err == nil {
            resultData = []byte(`{"status":"success"}`)
        }
    case "list_directory":
        var infos []FileInfo
        infos, err = provider.ListDir(ctx, params.Path)
        if err == nil {
            resultData, err = json.Marshal(infos)
        }
    default:
        return nil, fmt.Errorf("unsupported action: %s", req.Action)
    }

    if err != nil {
        return nil, err
    }

    return mcp.FormatExecutionResult(req.ToolID, "success", resultData, false), nil
}

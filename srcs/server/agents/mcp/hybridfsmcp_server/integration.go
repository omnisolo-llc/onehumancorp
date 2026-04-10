package hybridfsmcp_server

import (
	"context"
	"encoding/json"

	"github.com/onehumancorp/mono/srcs/server/tools/hybridfsmcp"
	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
)

type Provider struct {
	server *hybridfsmcp.Server
}

func NewProvider(basePath string) *Provider {
	fsProvider := hybridfsmcp.NewProvider(basePath)
	return &Provider{
		server: hybridfsmcp.NewServer(fsProvider),
	}
}

func (p *Provider) GetTools() []map[string]interface{} {
	return hybridfsmcp.GetMCPTools()
}

func (p *Provider) Execute(ctx context.Context, toolID string, args []byte) (*mcp.ExecutionResult, error) {
	res, err := p.server.CallTool(ctx, toolID, args)
	if err != nil {
		return mcp.FormatExecutionResult(toolID, "error", []byte(err.Error()), false), err
	}

	resBytes, err := json.Marshal(res)
	if err != nil {
		return mcp.FormatExecutionResult(toolID, "error", []byte(err.Error()), false), err
	}

	return mcp.FormatExecutionResult(toolID, "success", resBytes, false), nil
}

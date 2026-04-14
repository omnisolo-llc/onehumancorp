package agentgrpc

import (
	"context"
	"encoding/json"
	"fmt"
	"os/exec"
	"strings"

	mcpmcp "github.com/modelcontextprotocol/go-sdk/mcp"

	agentservicepb "github.com/onehumancorp/mono/srcs/proto/agentservice"
	"github.com/onehumancorp/mono/srcs/server/agents/builtin"
	"google.golang.org/adk/agent"
	"google.golang.org/adk/tool"
	"google.golang.org/adk/tool/functiontool"
	"google.golang.org/adk/tool/mcptoolset"
)

// builtinToolRegistry maps canonical tool names to their Tool definitions.
var builtinToolRegistry = map[string]builtin.Tool{
	"Bash":       builtin.BashTool,
	"Read":       builtin.FileReadTool,
	"Write":      builtin.FileWriteTool,
	"Edit":       builtin.FileEditTool,
	"Glob":       builtin.GlobTool,
	"Grep":       builtin.GrepTool,
	"WebFetch":   builtin.WebFetchTool,
	"WebSearch":  builtin.WebSearchTool,
	"SendMessage": builtin.SendMessageTool,
	"TodoWrite":  builtin.TodoWriteTool,
	"ToolSearch": builtin.ToolSearchTool,
	"TaskCreate": builtin.TaskCreateTool,
	"TaskGet":    builtin.TaskGetTool,
	"TaskList":   builtin.TaskListTool,
	"TaskUpdate": builtin.TaskUpdateTool,
}

// BuildToolsets constructs the list of adk tool.Toolset from a ToolsetConfig proto.
//
// When cfg is nil or cfg.BuiltinTools is empty all registered built-in tools
// are enabled.  MCP server connections are established lazily by mcptoolset.New.
func BuildToolsets(ctx context.Context, cfg *agentservicepb.ToolsetConfig) ([]tool.Toolset, error) {
	var sets []tool.Toolset

	// 1. Built-in tools wrapped as adk functiontool.Tool objects.
	builtinSet, err := buildBuiltinToolset(cfg)
	if err != nil {
		return nil, fmt.Errorf("builtin toolset: %w", err)
	}
	if builtinSet != nil {
		sets = append(sets, builtinSet)
	}

	if cfg == nil {
		return sets, nil
	}

	// 2. MCP servers configured via proto.
	for _, mcpCfg := range cfg.McpServers {
		ts, err := buildMCPToolset(mcpCfg)
		if err != nil {
			return nil, fmt.Errorf("mcp server %q: %w", mcpCfg.Name, err)
		}
		// Optionally filter to allowed tools only.
		if len(mcpCfg.AllowedTools) > 0 {
			ts = tool.FilterToolset(ts, tool.AllowedToolsPredicate(mcpCfg.AllowedTools))
		}
		sets = append(sets, ts)
	}

	return sets, nil
}

// buildBuiltinToolset wraps the selected built-in tools as an adk staticToolset.
func buildBuiltinToolset(cfg *agentservicepb.ToolsetConfig) (tool.Toolset, error) {
	var selected []builtin.Tool

	if cfg == nil || len(cfg.BuiltinTools) == 0 {
		// Default: all built-ins.
		selected = builtin.AllTools()
	} else {
		for _, name := range cfg.BuiltinTools {
			t, ok := builtinToolRegistry[name]
			if !ok {
				return nil, fmt.Errorf("unknown built-in tool %q (valid: %s)",
					name, strings.Join(builtinToolNames(), ", "))
			}
			selected = append(selected, t)
		}
	}

	var adkTools []tool.Tool
	for _, bt := range selected {
		at, err := wrapBuiltinTool(bt)
		if err != nil {
			return nil, fmt.Errorf("wrap tool %q: %w", bt.Name, err)
		}
		adkTools = append(adkTools, at)
	}

	return &staticToolset{name: "builtin", tools: adkTools}, nil
}

// wrapBuiltinTool adapts a builtin.Tool to an adk functiontool.Tool.
//
// The function signature required by functiontool.New is:
//
//	func(tool.Context, TArgs) (TResult, error)
//
// We use a generic map-based args/result type that round-trips through JSON so
// we can bridge any builtin tool without generating per-tool type definitions.
type genericArgs map[string]any
type genericResult struct {
	Output string `json:"output"`
}

func wrapBuiltinTool(bt builtin.Tool) (tool.Tool, error) {
	type btCapture struct {
		bt builtin.Tool
	}
	cap := btCapture{bt: bt}

	fn := func(tctx tool.Context, args genericArgs) (genericResult, error) {
		raw, err := json.Marshal(args)
		if err != nil {
			return genericResult{}, fmt.Errorf("marshal args: %w", err)
		}
		out, err := cap.bt.Execute(tctx, json.RawMessage(raw))
		if err != nil {
			return genericResult{}, err
		}
		return genericResult{Output: out}, nil
	}

	return functiontool.New(functiontool.Config{
		Name:        bt.Name,
		Description: bt.Description,
	}, fn)
}

// buildMCPToolset creates an mcptoolset.Toolset for the given server config.
// The MCP connection is established lazily on first use.
func buildMCPToolset(cfg *agentservicepb.MCPServerConfig) (tool.Toolset, error) {
	transport, err := mcpTransport(cfg)
	if err != nil {
		return nil, err
	}
	return mcptoolset.New(mcptoolset.Config{
		Transport: transport,
	})
}

// mcpTransport creates the appropriate mcp.Transport for the given config.
func mcpTransport(cfg *agentservicepb.MCPServerConfig) (mcpmcp.Transport, error) {
	switch cfg.Transport {
	case agentservicepb.MCPTransportType_MCP_TRANSPORT_STDIO, agentservicepb.MCPTransportType_MCP_TRANSPORT_UNSPECIFIED:
		if len(cfg.Command) == 0 {
			return nil, fmt.Errorf("STDIO transport requires at least one command element")
		}
		cmd := exec.Command(cfg.Command[0], cfg.Command[1:]...) //nolint:gosec
		for k, v := range cfg.Env {
			cmd.Env = append(cmd.Env, k+"="+v)
		}
		return &mcpmcp.CommandTransport{Command: cmd}, nil

	case agentservicepb.MCPTransportType_MCP_TRANSPORT_SSE:
		if cfg.Endpoint == "" {
			return nil, fmt.Errorf("SSE transport requires an endpoint URL")
		}
		return &mcpmcp.StreamableClientTransport{Endpoint: cfg.Endpoint}, nil

	default:
		return nil, fmt.Errorf("unsupported MCP transport type %v", cfg.Transport)
	}
}

// builtinToolNames returns a sorted list of registered built-in tool names.
func builtinToolNames() []string {
	names := make([]string, 0, len(builtinToolRegistry))
	for n := range builtinToolRegistry {
		names = append(names, n)
	}
	return names
}

// ── staticToolset ─────────────────────────────────────────────────────────────

// staticToolset is a simple tool.Toolset backed by a fixed slice.
type staticToolset struct {
	name  string
	tools []tool.Tool
}

func (s *staticToolset) Name() string { return s.name }

func (s *staticToolset) Tools(_ agent.ReadonlyContext) ([]tool.Tool, error) {
	return s.tools, nil
}

// Compile-time assertion that staticToolset satisfies tool.Toolset.
// We use the concrete method signature required by the interface.
var _ tool.Toolset = (*staticToolset)(nil)

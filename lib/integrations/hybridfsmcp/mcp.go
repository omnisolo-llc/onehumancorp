package hybridfsmcp

import (
    "context"
    "encoding/base64"
    "errors"
    "fmt"

    "github.com/onehumancorp/mono/srcs/server/auth"
)

type Tool struct {
    Name        string `json:"name"`
    Description string `json:"description"`
    InputSchema string `json:"inputSchema"`
}

type HybridFSProxyMCP struct {
    provider FileSystemProvider
}

func NewHybridFSProxyMCP(provider FileSystemProvider) *HybridFSProxyMCP {
    return &HybridFSProxyMCP{provider: provider}
}

func (m *HybridFSProxyMCP) ListTools() []Tool {
    return []Tool{
        {
            Name:        "read_file",
            Description: "Reads the content of a file.",
            InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
        },
        {
            Name:        "write_file",
            Description: "Writes content to a file. Content should be base64 encoded.",
            InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content_base64": {"type": "string"}}, "required": ["path", "content_base64"]}`,
        },
        {
            Name:        "list_directory",
            Description: "Lists files in a directory.",
            InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
        },
    }
}

func (m *HybridFSProxyMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
    claims := auth.ClaimsFromContext(ctx)
    if claims == nil && !m.provider.IsLocal() {
        return nil, errors.New("unauthorized: missing claims")
    }

    pathRaw, ok := arguments["path"]
    if !ok {
        return nil, errors.New("missing path argument")
    }
    path, ok := pathRaw.(string)
    if !ok {
        return nil, errors.New("path must be a string")
    }

    switch toolName {
    case "read_file":
        data, err := m.provider.ReadFile(ctx, claims, path)
        if err != nil {
            return nil, err
        }
        return map[string]interface{}{"content_base64": base64.StdEncoding.EncodeToString(data)}, nil
    case "write_file":
        contentB64Raw, ok := arguments["content_base64"]
        if !ok {
            return nil, errors.New("missing content_base64 argument")
        }
        contentB64, ok := contentB64Raw.(string)
        if !ok {
            return nil, errors.New("content_base64 must be a string")
        }
        content, err := base64.StdEncoding.DecodeString(contentB64)
        if err != nil {
            return nil, fmt.Errorf("invalid base64 content: %v", err)
        }
        err = m.provider.WriteFile(ctx, claims, path, content)
        if err != nil {
            return nil, err
        }
        return map[string]interface{}{"status": "success"}, nil
    case "list_directory":
        files, err := m.provider.ListDir(ctx, claims, path)
        if err != nil {
            return nil, err
        }
        return map[string]interface{}{"files": files}, nil
    default:
        return nil, fmt.Errorf("unknown tool: %s", toolName)
    }
}

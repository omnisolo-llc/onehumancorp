package hybridfsmcp

import (
    "context"
    "errors"
    "fmt"
    "path/filepath"
    "strings"

    "github.com/onehumancorp/mono/srcs/server/auth"
)

type Tool struct {
    Name        string `json:"name"`
    Description string `json:"description"`
    InputSchema string `json:"inputSchema"`
}

type HybridFSMCPServer struct {
    provider FileSystemProvider
}

func NewHybridFSMCPServer(provider FileSystemProvider) *HybridFSMCPServer {
    return &HybridFSMCPServer{provider: provider}
}

func (m *HybridFSMCPServer) ListTools() []Tool {
    return []Tool{
        {
            Name:        "read_file",
            Description: "Reads a file from the file system.",
            InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
        },
        {
            Name:        "write_file",
            Description: "Writes data to a file in the file system.",
            InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`,
        },
        {
            Name:        "list_directory",
            Description: "Lists contents of a directory in the file system.",
            InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
        },
    }
}

func (m *HybridFSMCPServer) resolvePathWithClaims(claims *auth.Claims, target string) string {
    if m.provider.IsLocal() || claims == nil {
        return target
    }
    cleanTarget := filepath.Clean("/" + target)
    cleanTarget = strings.TrimPrefix(cleanTarget, "/")
    return filepath.Join(claims.OrganizationID, cleanTarget)
}

func (m *HybridFSMCPServer) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
    claims := auth.ClaimsFromContext(ctx)
    if claims == nil && !m.provider.IsLocal() {
        return nil, errors.New("unauthorized: missing claims")
    }

    switch toolName {
    case "read_file":
        path, ok := arguments["path"].(string)
        if !ok {
            return nil, errors.New("missing or invalid 'path' argument")
        }
        scopedPath := m.resolvePathWithClaims(claims, path)
        data, err := m.provider.ReadFile(ctx, scopedPath)
        if err != nil {
            return nil, err
        }
        return map[string]interface{}{"status": "success", "content": string(data)}, nil
    case "write_file":
        path, ok := arguments["path"].(string)
        if !ok {
            return nil, errors.New("missing or invalid 'path' argument")
        }
        content, ok := arguments["content"].(string)
        if !ok {
            return nil, errors.New("missing or invalid 'content' argument")
        }
        scopedPath := m.resolvePathWithClaims(claims, path)
        err := m.provider.WriteFile(ctx, scopedPath, []byte(content))
        if err != nil {
            return nil, err
        }
        return map[string]interface{}{"status": "success"}, nil
    case "list_directory":
        path, ok := arguments["path"].(string)
        if !ok {
            return nil, errors.New("missing or invalid 'path' argument")
        }
        scopedPath := m.resolvePathWithClaims(claims, path)
        entries, err := m.provider.ListDir(ctx, scopedPath)
        if err != nil {
            return nil, err
        }
        return map[string]interface{}{"status": "success", "entries": entries}, nil
    default:
        return nil, fmt.Errorf("unknown tool: %s", toolName)
    }
}

package hybridfsmcp

import (
    "context"
    "errors"
    "fmt"
    "os"
)

// Tool represents an MCP tool definition.
type Tool struct {
    Name        string `json:"name"`
    Description string `json:"description"`
    InputSchema string `json:"inputSchema"`
}

type HybridFSMCP struct {
    provider FileSystemProvider
}

func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
    return &HybridFSMCP{provider: provider}
}

func Factory() *HybridFSMCP {
    isMultiTenant := os.Getenv("OHC_MULTITENANT") == "true"
    basePath := os.Getenv("OHC_FS_ROOT")
    if basePath == "" {
        basePath = os.TempDir()
    }
    var provider FileSystemProvider
    if isMultiTenant {
        provider = NewCloudFSProvider(basePath)
    } else {
        provider = NewLocalFSProvider(basePath)
    }
    return NewHybridFSMCP(provider)
}

func (m *HybridFSMCP) ListTools() []Tool {
    return []Tool{
        {
            Name:        "read_file",
            Description: "Reads the contents of a file.",
            InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
        },
        {
            Name:        "write_file",
            Description: "Writes contents to a file.",
            InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`,
        },
        {
            Name:        "list_directory",
            Description: "Lists files and directories in a given path.",
            InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
        },
    }
}

func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
    switch toolName {
    case "read_file":
        path, ok := arguments["path"].(string)
        if !ok {
            return nil, errors.New("missing or invalid 'path' argument")
        }
        data, err := m.provider.ReadFile(ctx, path)
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
        if err := m.provider.WriteFile(ctx, path, []byte(content)); err != nil {
            return nil, err
        }
        return map[string]interface{}{"status": "success"}, nil
    case "list_directory":
        path, ok := arguments["path"].(string)
        if !ok {
            return nil, errors.New("missing or invalid 'path' argument")
        }
        entries, err := m.provider.ListDir(ctx, path)
        if err != nil {
            return nil, err
        }
        var results []map[string]interface{}
        for _, e := range entries {
            results = append(results, map[string]interface{}{
                "name":   e.Name,
                "is_dir": e.IsDir,
                "size":   e.Size,
            })
        }
        return map[string]interface{}{"status": "success", "entries": results}, nil
    default:
        return nil, fmt.Errorf("unknown tool: %s", toolName)
    }
}

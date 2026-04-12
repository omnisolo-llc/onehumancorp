package hybridfsmcp

import (
    "context"
    "encoding/json"
    "errors"
    "fmt"
)

type Tool struct {
    Name        string          `json:"name"`
    Description string          `json:"description"`
    InputSchema json.RawMessage `json:"inputSchema"`
}

type HybridFSMCP struct {
    provider FileSystemProvider
}

func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
    return &HybridFSMCP{provider: provider}
}

func (m *HybridFSMCP) ListTools() []Tool {
    return []Tool{
        {
            Name:        "read_file",
            Description: "Reads the content of a file.",
            InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string"}},"required":["path"]}`),
        },
        {
            Name:        "write_file",
            Description: "Writes content to a file.",
            InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string"},"content":{"type":"string"}},"required":["path","content"]}`),
        },
        {
            Name:        "list_directory",
            Description: "Lists files and directories in a given path.",
            InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string"}},"required":["path"]}`),
        },
        {
            Name:        "search_files",
            Description: "Searches for files matching a pattern.",
            InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string"},"pattern":{"type":"string"}},"required":["path","pattern"]}`),
        },
    }
}

func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
    switch toolName {
    case "read_file":
        path, ok := arguments["path"].(string)
        if !ok {
            return nil, errors.New("missing or invalid 'path'")
        }
        data, err := m.provider.ReadFile(ctx, path)
        if err != nil {
            return nil, err
        }
        return map[string]interface{}{"status": "success", "content": string(data)}, nil
    case "write_file":
        path, ok := arguments["path"].(string)
        if !ok {
            return nil, errors.New("missing or invalid 'path'")
        }
        content, ok := arguments["content"].(string)
        if !ok {
            return nil, errors.New("missing or invalid 'content'")
        }
        err := m.provider.WriteFile(ctx, path, []byte(content))
        if err != nil {
            return nil, err
        }
        return map[string]interface{}{"status": "success"}, nil
    case "list_directory":
        path, ok := arguments["path"].(string)
        if !ok {
            return nil, errors.New("missing or invalid 'path'")
        }
        entries, err := m.provider.ListDir(ctx, path)
        if err != nil {
            return nil, err
        }
        return map[string]interface{}{"status": "success", "entries": entries}, nil
    case "search_files":
        path, ok := arguments["path"].(string)
        if !ok {
            return nil, errors.New("missing or invalid 'path'")
        }
        pattern, ok := arguments["pattern"].(string)
        if !ok {
            return nil, errors.New("missing or invalid 'pattern'")
        }
        // Assume provider will have SearchFiles method
        entries, err := m.provider.SearchFiles(ctx, path, pattern)
        if err != nil {
            return nil, err
        }
        return map[string]interface{}{"status": "success", "entries": entries}, nil
    default:
        return nil, fmt.Errorf("unknown tool: %s", toolName)
    }
}

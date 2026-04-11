package hybridfsmcp

import (
    "context"
    "errors"
    "fmt"
)

type Tool struct {
    Name        string `json:"name"`
    Description string `json:"description"`
    InputSchema string `json:"inputSchema"`
}

type FSMCP struct {
    provider FileSystemProvider
}

func NewFSMCP(provider FileSystemProvider) *FSMCP {
    return &FSMCP{provider: provider}
}

func (m *FSMCP) ListTools() []Tool {
    return []Tool{
        {
            Name:        "read_file",
            Description: "Reads the contents of a file.",
            InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
        },
        {
            Name:        "write_file",
            Description: "Writes data to a file.",
            InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`,
        },
        {
            Name:        "list_directory",
            Description: "Lists files in a directory.",
            InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
        },
    }
}

func (m *FSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
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
        return map[string]interface{}{"status": "success", "entries": entries}, nil
    default:
        return nil, fmt.Errorf("unknown tool: %s", toolName)
    }
}

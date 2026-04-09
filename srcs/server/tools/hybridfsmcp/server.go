package hybridfsmcp

import (
    "context"
    "fmt"
)

type Server struct {
    provider FileSystemProvider
}

func NewServer(provider FileSystemProvider) *Server {
    return &Server{provider: provider}
}

type Tool struct {
    Name        string `json:"name"`
    Description string `json:"description"`
    InputSchema string `json:"inputSchema"`
}

func (s *Server) ListTools() []Tool {
    return []Tool{
        {
            Name:        "read_file",
            Description: "Reads a file from the hybrid file system.",
            InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
        },
        {
            Name:        "write_file",
            Description: "Writes a file to the hybrid file system.",
            InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`,
        },
        {
            Name:        "list_directory",
            Description: "Lists files in a directory.",
            InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
        },
        {
            Name:        "search_files",
            Description: "Searches files matching a pattern.",
            InputSchema: `{"type": "object", "properties": {"dir": {"type": "string"}, "pattern": {"type": "string"}}, "required": ["dir", "pattern"]}`,
        },
    }
}

func (s *Server) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
    switch toolName {
    case "read_file":
        path, ok := arguments["path"].(string)
        if !ok {
            return nil, fmt.Errorf("missing or invalid path")
        }
        data, err := s.provider.ReadFile(ctx, path)
        if err != nil {
            return nil, err
        }
        return string(data), nil
    case "write_file":
        path, ok := arguments["path"].(string)
        if !ok {
            return nil, fmt.Errorf("missing or invalid path")
        }
        content, ok := arguments["content"].(string)
        if !ok {
            return nil, fmt.Errorf("missing or invalid content")
        }
        err := s.provider.WriteFile(ctx, path, []byte(content))
        if err != nil {
            return nil, err
        }
        return map[string]string{"status": "success"}, nil
    case "list_directory":
        path, ok := arguments["path"].(string)
        if !ok {
            return nil, fmt.Errorf("missing or invalid path")
        }
        infos, err := s.provider.ListDir(ctx, path)
        if err != nil {
            return nil, err
        }
        var names []string
        for _, info := range infos {
            names = append(names, info.Name())
        }
        return names, nil
    case "search_files":
        dir, ok := arguments["dir"].(string)
        if !ok {
            return nil, fmt.Errorf("missing or invalid dir")
        }
        pattern, ok := arguments["pattern"].(string)
        if !ok {
            return nil, fmt.Errorf("missing or invalid pattern")
        }
        matches, err := s.provider.SearchFiles(ctx, dir, pattern)
        if err != nil {
            return nil, err
        }
        return matches, nil
    default:
        return nil, fmt.Errorf("unknown tool: %s", toolName)
    }
}

package hybridfsmcp

import (
    "context"
    "encoding/json"
    "fmt"
    "os"
)

type Tool struct {
    Name        string          `json:"name"`
    Description string          `json:"description"`
    InputSchema json.RawMessage `json:"inputSchema"`
}

type Server struct {
    Provider FileSystemProvider
}

func NewServer() *Server {
    var provider FileSystemProvider
    if os.Getenv("OHC_MULTITENANT") == "true" {
        provider = &CloudFSProvider{BaseDir: "/var/tmp/ohc/cloud_fs"}
    } else if os.Getenv("OHC_STANDALONE") == "true" {
        provider = &LocalFSProvider{WorkspaceDir: "/var/tmp/ohc/local_fs"}
    } else {
        // Default to local for safety if neither is set
        provider = &LocalFSProvider{WorkspaceDir: "/var/tmp/ohc/local_fs"}
    }
    return &Server{Provider: provider}
}

func (s *Server) ListTools() []Tool {
    return []Tool{
        {
            Name:        "read_file",
            Description: "Reads a file from the hybrid file system.",
            InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
        },
        {
            Name:        "write_file",
            Description: "Writes a file to the hybrid file system.",
            InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`),
        },
        {
            Name:        "list_directory",
            Description: "Lists a directory in the hybrid file system.",
            InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
        },
    }
}

func (s *Server) CallTool(ctx context.Context, toolName string, args map[string]interface{}) (interface{}, error) {
    switch toolName {
    case "read_file":
        path, ok := args["path"].(string)
        if !ok {
            return nil, fmt.Errorf("missing string argument: path")
        }
        content, err := s.Provider.ReadFile(ctx, path)
        if err != nil {
            return nil, err
        }
        return string(content), nil
    case "write_file":
        path, ok := args["path"].(string)
        if !ok {
            return nil, fmt.Errorf("missing string argument: path")
        }
        content, ok := args["content"].(string)
        if !ok {
            return nil, fmt.Errorf("missing string argument: content")
        }
        err := s.Provider.WriteFile(ctx, path, []byte(content))
        if err != nil {
            return nil, err
        }
        return "success", nil
    case "list_directory":
        path, ok := args["path"].(string)
        if !ok {
            return nil, fmt.Errorf("missing string argument: path")
        }
        return s.Provider.ListDir(ctx, path)
    default:
        return nil, fmt.Errorf("unknown tool: %s", toolName)
    }
}

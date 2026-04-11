package hybridfsmcp

import (
    "context"
    "encoding/json"
    "fmt"
)

type ReadFileArgs struct {
    Path string `json:"path"`
}

type WriteFileArgs struct {
    Path string `json:"path"`
    Data []byte `json:"data"`
}

type ListDirArgs struct {
    Path string `json:"path"`
}

type HybridFSMCPServer struct {
    provider FileSystemProvider
}

func NewHybridFSMCP(isCloud bool, baseDir string, tenantID string) *HybridFSMCPServer {
    var provider FileSystemProvider
    if isCloud {
        provider = NewCloudFSProvider(baseDir, tenantID)
    } else {
        provider = NewLocalFSProvider(baseDir)
    }
    return &HybridFSMCPServer{provider: provider}
}

func (s *HybridFSMCPServer) ExecuteTool(ctx context.Context, toolName string, argsRaw json.RawMessage) (interface{}, error) {
    switch toolName {
    case "read_file":
        var args ReadFileArgs
        if err := json.Unmarshal(argsRaw, &args); err != nil {
            return nil, err
        }
        data, err := s.provider.ReadFile(ctx, args.Path)
        if err != nil {
            return nil, err
        }
        return string(data), nil
    case "write_file":
        var args WriteFileArgs
        if err := json.Unmarshal(argsRaw, &args); err != nil {
            return nil, err
        }
        if err := s.provider.WriteFile(ctx, args.Path, args.Data); err != nil {
            return nil, err
        }
        return "success", nil
    case "list_directory":
        var args ListDirArgs
        if err := json.Unmarshal(argsRaw, &args); err != nil {
            return nil, err
        }
        entries, err := s.provider.ListDir(ctx, args.Path)
        if err != nil {
            return nil, err
        }
        var names []string
        for _, e := range entries {
            names = append(names, e.Name())
        }
        return names, nil
    default:
        return nil, fmt.Errorf("unknown tool: %s", toolName)
    }
}

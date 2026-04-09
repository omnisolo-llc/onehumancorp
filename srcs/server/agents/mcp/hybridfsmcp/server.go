package hybridfsmcp

import (
    "encoding/json"
    "errors"
)

type HybridFSServer struct {
    Provider FileSystemProvider
}

type ToolParams struct {
    Operation string `json:"operation"`
    Path      string `json:"path"`
    Content   string `json:"content,omitempty"`
}

func (s *HybridFSServer) HandleToolCall(params []byte) (map[string]any, error) {
    var p ToolParams
    if err := json.Unmarshal(params, &p); err != nil {
        return nil, errors.New("invalid file system tool parameters")
    }

    switch p.Operation {
    case "read_file":
        data, err := s.Provider.ReadFile(p.Path)
        if err != nil { return nil, err }
        return map[string]any{"content": string(data)}, nil
    case "write_file":
        err := s.Provider.WriteFile(p.Path, []byte(p.Content))
        if err != nil { return nil, err }
        return map[string]any{"status": "success"}, nil
    case "list_directory":
        entries, err := s.Provider.ListDir(p.Path)
        if err != nil { return nil, err }
        names := make([]string, 0, len(entries))
        for _, e := range entries {
            names = append(names, e.Name())
        }
        return map[string]any{"files": names}, nil
    default:
        return nil, errors.New("unknown operation")
    }
}

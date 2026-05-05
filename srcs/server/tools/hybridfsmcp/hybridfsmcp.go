package hybridfsmcp

import (
	"context"
	"fmt"
)

type HybridFSMCP struct {
	Provider FileSystemProvider
}

func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	if provider == nil {
		provider = NewProvider()
	}
	return &HybridFSMCP{
		Provider: provider,
	}
}

func (h *HybridFSMCP) CallTool(ctx context.Context, name string, args map[string]interface{}, claims *Claims) (interface{}, error) {
	switch name {
	case "read_file":
		path, ok := args["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid argument: path")
		}
		data, err := h.Provider.ReadFile(ctx, claims, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"content": string(data)}, nil

	case "write_file":
		path, ok := args["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid argument: path")
		}

		var data []byte
		switch v := args["content"].(type) {
		case string:
			data = []byte(v)
		case []byte:
			data = v
		default:
			return nil, fmt.Errorf("missing or invalid argument: content")
		}

		err := h.Provider.WriteFile(ctx, claims, path, data)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success"}, nil

	case "list_directory":
		path, ok := args["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid argument: path")
		}

		infos, err := h.Provider.ListDir(ctx, claims, path)
		if err != nil {
			return nil, err
		}

		var result []map[string]interface{}
		for _, info := range infos {
			result = append(result, map[string]interface{}{
				"name":  info.Name(),
				"size":  info.Size(),
				"isDir": info.IsDir(),
			})
		}
		return map[string]interface{}{"files": result}, nil

	case "search_files":
		query, ok := args["query"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid argument: query")
		}

		results, err := h.Provider.SearchFiles(ctx, claims, query)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"files": results}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", name)
	}
}

package hybridfsmcp

import (
	"context"
	"fmt"
	"strings"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

// Server implements an MCP server for file operations using a FileSystemProvider.
type Server struct {
	provider FileSystemProvider
	meter    metric.Meter
	counter  metric.Int64Counter
	duration metric.Float64Histogram
}

func NewServer(provider FileSystemProvider) *Server {
	meter := otel.Meter("github.com/onehumancorp/mono/srcs/server/tools/hybridfsmcp")
	counter, _ := meter.Int64Counter("fsmcp_tool_calls_total", metric.WithDescription("Total number of FS MCP tool calls"))
	duration, _ := meter.Float64Histogram("fsmcp_tool_duration_seconds", metric.WithDescription("Duration of FS MCP tool calls"))

	return &Server{
		provider: provider,
		meter:    meter,
		counter:  counter,
		duration: duration,
	}
}


func (s *Server) ExecuteTool(ctx context.Context, tool string, args map[string]interface{}) (interface{}, error) {
	start := time.Now()
	attrs := metric.WithAttributes(attribute.String("tool", tool))
	s.counter.Add(ctx, 1, attrs)

	defer func() {
		s.duration.Record(ctx, time.Since(start).Seconds(), attrs)
	}()

	switch tool {
	case "read_file":
		path, ok := args["path"].(string)
		if !ok {
			return nil, fmt.Errorf("read_file requires string 'path'")
		}
		data, err := s.provider.ReadFile(ctx, path)
		if err != nil {
			return nil, err
		}
		return string(data), nil
	case "write_file":
		path, ok := args["path"].(string)
		if !ok {
			return nil, fmt.Errorf("write_file requires string 'path'")
		}
		dataStr, ok := args["data"].(string)
		if !ok {
			return nil, fmt.Errorf("write_file requires string 'data'")
		}
		err := s.provider.WriteFile(ctx, path, []byte(dataStr))
		if err != nil {
			return nil, err
		}
		return "ok", nil
	case "list_directory":
		path, ok := args["path"].(string)
		if !ok {
			return nil, fmt.Errorf("list_directory requires string 'path'")
		}
		list, err := s.provider.ListDir(ctx, path)
		if err != nil {
			return nil, err
		}
		return list, nil
	case "search_files":
		// Example simplistic search implementation using ListDir
		path, ok := args["path"].(string)
		if !ok {
			return nil, fmt.Errorf("search_files requires string 'path'")
		}
		pattern, ok := args["pattern"].(string)
		if !ok {
			return nil, fmt.Errorf("search_files requires string 'pattern'")
		}
		list, err := s.provider.ListDir(ctx, path)
		if err != nil {
			return nil, err
		}
		var results []string
		for _, f := range list {
			if strings.Contains(f, pattern) {
				results = append(results, f)
			}
		}
		return results, nil
	default:
		return nil, fmt.Errorf("unknown tool: %s", tool)
	}
}

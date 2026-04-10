package hybridfsmcp

import (
	"context"
	"io/fs"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// SearchFiles returns a list of paths matching the search term.
func (m *HybridFSMCP) SearchFiles(ctx context.Context, claims *auth.Claims, startPath string, term string) (interface{}, error) {
	// First ensure startPath exists and is accessible
	_, err := m.provider.ListDir(ctx, claims, startPath)
	if err != nil {
		return nil, err
	}

	var matches []string

	err = m.provider.Walk(ctx, claims, startPath, func(path string, info fs.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if path != "." && path != "" && strings.Contains(info.Name(), term) {
			matches = append(matches, path)
		}
		return nil
	})

	if err != nil {
		return nil, err
	}

	mode := "cloud"
	if m.provider.IsLocal() {
		mode = "standalone"
	}

	return map[string]interface{}{
		"status":  "success",
		"mode":    mode,
		"matches": matches,
	}, nil
}

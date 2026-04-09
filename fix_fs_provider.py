import re

with open('srcs/server/agents/mcp/fs_provider.go', 'r') as f:
    content = f.read()

# Fix the path traversal vulnerability
old_resolve = """func (p *LocalFSProvider) resolvePath(reqPath string) (string, error) {
	fullPath := filepath.Join(p.baseDir, reqPath)
	cleanPath := filepath.Clean(fullPath)
	if !strings.HasPrefix(cleanPath, p.baseDir) {
		return "", errors.New("path escapes base directory")
	}
	return cleanPath, nil
}"""

new_resolve = """func (p *LocalFSProvider) resolvePath(reqPath string) (string, error) {
	fullPath := filepath.Join(p.baseDir, reqPath)
	cleanPath := filepath.Clean(fullPath)

	baseDirWithSep := p.baseDir
	if !strings.HasSuffix(baseDirWithSep, string(filepath.Separator)) {
		baseDirWithSep += string(filepath.Separator)
	}

	if !strings.HasPrefix(cleanPath+string(filepath.Separator), baseDirWithSep) {
		return "", errors.New("path escapes base directory")
	}
	return cleanPath, nil
}"""

content = content.replace(old_resolve, new_resolve)

with open('srcs/server/agents/mcp/fs_provider.go', 'w') as f:
    f.write(content)

with open('srcs/server/agents/mcp/hybrid_fs_server.go', 'r') as f:
    content = f.read()

# Add observability
old_imports = """import (
	"context"
)"""

new_imports = """import (
	"context"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
)"""

content = content.replace(old_imports, new_imports)

old_read = """func (m *HybridFSMCP) ReadFile(ctx context.Context, path string) ([]byte, error) {
	return m.provider.ReadFile(ctx, path)
}"""

new_read = """func (m *HybridFSMCP) ReadFile(ctx context.Context, path string) ([]byte, error) {
	ctx, span := otel.Tracer("mcp").Start(ctx, "HybridFSMCP.ReadFile")
	span.SetAttributes(attribute.String("path", path))
	defer span.End()

	data, err := m.provider.ReadFile(ctx, path)
	if err != nil {
		span.RecordError(err)
	}
	return data, err
}"""

old_write = """func (m *HybridFSMCP) WriteFile(ctx context.Context, path string, data []byte) error {
	return m.provider.WriteFile(ctx, path, data)
}"""

new_write = """func (m *HybridFSMCP) WriteFile(ctx context.Context, path string, data []byte) error {
	ctx, span := otel.Tracer("mcp").Start(ctx, "HybridFSMCP.WriteFile")
	span.SetAttributes(attribute.String("path", path))
	defer span.End()

	err := m.provider.WriteFile(ctx, path, data)
	if err != nil {
		span.RecordError(err)
	}
	return err
}"""

old_list = """func (m *HybridFSMCP) ListDir(ctx context.Context, path string) ([]string, error) {
	return m.provider.ListDir(ctx, path)
}"""

new_list = """func (m *HybridFSMCP) ListDir(ctx context.Context, path string) ([]string, error) {
	ctx, span := otel.Tracer("mcp").Start(ctx, "HybridFSMCP.ListDir")
	span.SetAttributes(attribute.String("path", path))
	defer span.End()

	dirs, err := m.provider.ListDir(ctx, path)
	if err != nil {
		span.RecordError(err)
	}
	return dirs, err
}"""

old_search = """func (m *HybridFSMCP) SearchFiles(ctx context.Context, dir string, pattern string) ([]string, error) {
	return m.provider.SearchFiles(ctx, dir, pattern)
}"""

new_search = """func (m *HybridFSMCP) SearchFiles(ctx context.Context, dir string, pattern string) ([]string, error) {
	ctx, span := otel.Tracer("mcp").Start(ctx, "HybridFSMCP.SearchFiles")
	span.SetAttributes(attribute.String("dir", dir), attribute.String("pattern", pattern))
	defer span.End()

	files, err := m.provider.SearchFiles(ctx, dir, pattern)
	if err != nil {
		span.RecordError(err)
	}
	return files, err
}"""

content = content.replace(old_read, new_read)
content = content.replace(old_write, new_write)
content = content.replace(old_list, new_list)
content = content.replace(old_search, new_search)

with open('srcs/server/agents/mcp/hybrid_fs_server.go', 'w') as f:
    f.write(content)

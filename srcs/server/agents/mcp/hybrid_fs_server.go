package mcp

import (
	"context"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
)

type HybridFSMCP struct {
	provider FileSystemProvider
}

func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{provider: provider}
}

func (m *HybridFSMCP) ReadFile(ctx context.Context, path string) ([]byte, error) {
	ctx, span := otel.Tracer("mcp").Start(ctx, "HybridFSMCP.ReadFile")
	span.SetAttributes(attribute.String("path", path))
	defer span.End()

	data, err := m.provider.ReadFile(ctx, path)
	if err != nil {
		span.RecordError(err)
	}
	return data, err
}

func (m *HybridFSMCP) WriteFile(ctx context.Context, path string, data []byte) error {
	ctx, span := otel.Tracer("mcp").Start(ctx, "HybridFSMCP.WriteFile")
	span.SetAttributes(attribute.String("path", path))
	defer span.End()

	err := m.provider.WriteFile(ctx, path, data)
	if err != nil {
		span.RecordError(err)
	}
	return err
}

func (m *HybridFSMCP) ListDir(ctx context.Context, path string) ([]string, error) {
	ctx, span := otel.Tracer("mcp").Start(ctx, "HybridFSMCP.ListDir")
	span.SetAttributes(attribute.String("path", path))
	defer span.End()

	dirs, err := m.provider.ListDir(ctx, path)
	if err != nil {
		span.RecordError(err)
	}
	return dirs, err
}

func (m *HybridFSMCP) SearchFiles(ctx context.Context, dir string, pattern string) ([]string, error) {
	ctx, span := otel.Tracer("mcp").Start(ctx, "HybridFSMCP.SearchFiles")
	span.SetAttributes(attribute.String("dir", dir), attribute.String("pattern", pattern))
	defer span.End()

	files, err := m.provider.SearchFiles(ctx, dir, pattern)
	if err != nil {
		span.RecordError(err)
	}
	return files, err
}

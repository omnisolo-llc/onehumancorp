package hybridfsmcp

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

// FileSystemProvider defines the interface for underlying file operations.
type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, content []byte) error
	ListDir(ctx context.Context, path string) ([]string, error)
	IsLocal() bool
}

// LocalFSProvider implements FileSystemProvider for the local file system.
type LocalFSProvider struct {
	workspaceDir string
}

func NewLocalFSProvider(workspaceDir string) *LocalFSProvider {
	return &LocalFSProvider{workspaceDir: filepath.Clean(workspaceDir)}
}

func (p *LocalFSProvider) IsLocal() bool { return true }

func (p *LocalFSProvider) securePath(targetPath string) (string, error) {
	cleanWs := p.workspaceDir
	cleanTarget := filepath.Clean(filepath.Join(cleanWs, targetPath))
	if !strings.HasPrefix(cleanTarget, cleanWs+string(os.PathSeparator)) && cleanTarget != cleanWs {
		return "", errors.New("path traversal detected")
	}
	return cleanTarget, nil
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	secure, err := p.securePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(secure)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	secure, err := p.securePath(path)
	if err != nil {
		return err
	}
	// Ensure directory exists
	if err := os.MkdirAll(filepath.Dir(secure), 0755); err != nil {
		return err
	}
	return os.WriteFile(secure, content, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	secure, err := p.securePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(secure)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

// CloudFSProvider implements FileSystemProvider for tenant-scoped cloud environments.
type CloudFSProvider struct {
	baseDir string // E.g., a mounted PV for tenant workspaces
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{baseDir: filepath.Clean(baseDir)}
}

func (p *CloudFSProvider) IsLocal() bool { return false }

func (p *CloudFSProvider) securePath(claims *auth.Claims, targetPath string) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing claims or organization ID")
	}
	cleanBase := p.baseDir
	tenantDir := filepath.Join(cleanBase, claims.OrganizationID)
	cleanTarget := filepath.Clean(filepath.Join(tenantDir, targetPath))

	if !strings.HasPrefix(cleanTarget, tenantDir+string(os.PathSeparator)) && cleanTarget != tenantDir {
		return "", errors.New("path traversal detected out of tenant scope")
	}
	return cleanTarget, nil
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	claims := auth.ClaimsFromContext(ctx)
	secure, err := p.securePath(claims, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(secure)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, content []byte) error {
	claims := auth.ClaimsFromContext(ctx)
	secure, err := p.securePath(claims, path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(secure), 0755); err != nil {
		return err
	}
	return os.WriteFile(secure, content, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	claims := auth.ClaimsFromContext(ctx)
	secure, err := p.securePath(claims, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(secure)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, entry := range entries {
		names = append(names, entry.Name())
	}
	return names, nil
}

// HybridFSMCP implements the MCP interface for hybrid file system access.
type HybridFSMCP struct {
	provider FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance.
func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{provider: provider}
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// ListTools returns the list of available tools.
func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the content of a file.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`),
		},
		{
			Name:        "list_directory",
			Description: "Lists files and folders in a directory.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil && !m.provider.IsLocal() {
		return nil, errors.New("unauthorized: missing claims")
	}

	start := time.Now()
	tracer := otel.Tracer("hybridfsmcp")
	ctx, span := tracer.Start(ctx, fmt.Sprintf("CallTool_%s", toolName))
	defer span.End()

	recordTelemetry := func(err error) {
		status := "success"
		if err != nil {
			status = "error"
		}

		span.SetAttributes(
			attribute.String("mcp.tool", toolName),
			attribute.String("mcp.status", status),
			attribute.Bool("mcp.is_local", m.provider.IsLocal()),
		)

		if telemetry.MCPOperationsCounter != nil {
			telemetry.MCPOperationsCounter.Add(ctx, 1,
				metric.WithAttributes(
					attribute.String("tool", toolName),
					attribute.String("status", status),
				),
			)
		}
		if telemetry.MCPOperationDuration != nil {
			telemetry.MCPOperationDuration.Record(ctx, time.Since(start).Seconds(),
				metric.WithAttributes(
					attribute.String("tool", toolName),
				),
			)
		}
	}

	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			err := errors.New("missing or invalid 'path' argument")
			recordTelemetry(err)
			return nil, err
		}
		content, err := m.provider.ReadFile(ctx, path)
		recordTelemetry(err)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success", "content": string(content)}, nil

	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			err := errors.New("missing or invalid 'path' argument")
			recordTelemetry(err)
			return nil, err
		}
		contentStr, ok := arguments["content"].(string)
		if !ok {
			err := errors.New("missing or invalid 'content' argument")
			recordTelemetry(err)
			return nil, err
		}
		err := m.provider.WriteFile(ctx, path, []byte(contentStr))
		recordTelemetry(err)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success"}, nil

	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			err := errors.New("missing or invalid 'path' argument")
			recordTelemetry(err)
			return nil, err
		}
		entries, err := m.provider.ListDir(ctx, path)
		recordTelemetry(err)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success", "entries": entries}, nil

	default:
		err := fmt.Errorf("unknown tool: %s", toolName)
		recordTelemetry(err)
		return nil, err
	}
}

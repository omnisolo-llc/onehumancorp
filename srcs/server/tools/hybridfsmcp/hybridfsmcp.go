package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
	"go.opentelemetry.io/otel/attribute"
)


var (
	meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/tools/hybridfsmcp")
	mcpOpsCounter metric.Int64Counter
)

func init() {
	var err error
	mcpOpsCounter, err = meter.Int64Counter("mcp_fs_operations_total", metric.WithDescription("Total number of MCP filesystem operations"))
	if err != nil {
		// Ignore error in init
	}
}

type FileSystemProvider interface {
	ReadFile(ctx context.Context, path string) ([]byte, error)
	WriteFile(ctx context.Context, path string, data []byte) error
	ListDir(ctx context.Context, path string) ([]os.FileInfo, error)
}

type LocalFSProvider struct {
	BaseDir string
}

func NewLocalFSProvider(baseDir string) *LocalFSProvider {
	return &LocalFSProvider{BaseDir: filepath.Clean(baseDir)}
}

func (p *LocalFSProvider) resolvePath(target string) (string, error) {
	cleanTarget := filepath.Clean(filepath.Join(p.BaseDir, target))
	if cleanTarget == p.BaseDir || strings.HasPrefix(cleanTarget, p.BaseDir+string(filepath.Separator)) {
		return cleanTarget, nil
	}
	return "", fmt.Errorf("path traversal violation")
}

func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return err
	}
	// ensure dir exists
	if err := os.MkdirAll(filepath.Dir(resolved), 0755); err != nil {
		return err
	}
	return os.WriteFile(resolved, data, 0644)
}

func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]os.FileInfo, error) {
	resolved, err := p.resolvePath(path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(resolved)
	if err != nil {
		return nil, err
	}
	var infos []os.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err == nil {
			infos = append(infos, info)
		}
	}
	return infos, nil
}

type CloudFSProvider struct {
	BaseDir string
}

func NewCloudFSProvider(baseDir string) *CloudFSProvider {
	return &CloudFSProvider{BaseDir: filepath.Clean(baseDir)}
}

func (p *CloudFSProvider) resolvePath(ctx context.Context, target string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("missing organization id in context")
	}

	tenantDir := filepath.Join(p.BaseDir, claims.OrganizationID)
	cleanTarget := filepath.Clean(filepath.Join(tenantDir, target))

	if cleanTarget == tenantDir || strings.HasPrefix(cleanTarget, tenantDir+string(filepath.Separator)) {
		return cleanTarget, nil
	}
	return "", fmt.Errorf("path traversal violation")
}

func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	return os.ReadFile(resolved)
}

func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(resolved), 0755); err != nil {
		return err
	}
	return os.WriteFile(resolved, data, 0644)
}

func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]os.FileInfo, error) {
	resolved, err := p.resolvePath(ctx, path)
	if err != nil {
		return nil, err
	}
	entries, err := os.ReadDir(resolved)
	if err != nil {
		return nil, err
	}
	var infos []os.FileInfo
	for _, entry := range entries {
		info, err := entry.Info()
		if err == nil {
			infos = append(infos, info)
		}
	}
	return infos, nil
}

func NewProvider() FileSystemProvider {
	isMultiTenant := os.Getenv("OHC_MULTITENANT") == "true"

	baseDir := os.Getenv("OHC_FS_ROOT")
	if baseDir == "" {
		baseDir = os.TempDir()
	}

	if isMultiTenant {
		return NewCloudFSProvider(baseDir)
	}
	return NewLocalFSProvider(baseDir)
}

// ... implement mcp tool wrapper next

type MCPServer struct {
	provider FileSystemProvider
}

func NewMCPServer(provider FileSystemProvider) *MCPServer {
	return &MCPServer{provider: provider}
}

type ReadFileInput struct {
	Path string `json:"path"`
}

type WriteFileInput struct {
	Path string `json:"path"`
	Data string `json:"data"` // base64 encoded or raw string depending on design, let's assume string for simplicity for now
}

type ListDirInput struct {
	Path string `json:"path"`
}

func (s *MCPServer) HandleToolCall(ctx context.Context, toolName string, inputRaw json.RawMessage) (json.RawMessage, error) {
	switch toolName {
	case "read_file":
		var input ReadFileInput
		if err := json.Unmarshal(inputRaw, &input); err != nil {
			return nil, err
		}
		data, err := s.provider.ReadFile(ctx, input.Path)
		if err != nil {
			return nil, err
		}
		if mcpOpsCounter != nil { mcpOpsCounter.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "read_file"))) }
		return json.Marshal(map[string]string{"content": string(data)})

	case "write_file":
		var input WriteFileInput
		if err := json.Unmarshal(inputRaw, &input); err != nil {
			return nil, err
		}
		err := s.provider.WriteFile(ctx, input.Path, []byte(input.Data))
		if err != nil {
			return nil, err
		}
		if mcpOpsCounter != nil { mcpOpsCounter.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "write_file"))) }
		return json.Marshal(map[string]bool{"success": true})

	case "list_directory":
		var input ListDirInput
		if err := json.Unmarshal(inputRaw, &input); err != nil {
			return nil, err
		}
		infos, err := s.provider.ListDir(ctx, input.Path)
		if err != nil {
			return nil, err
		}
		names := []string{}
		for _, info := range infos {
			if info.IsDir() {
				names = append(names, info.Name()+"/")
			} else {
				names = append(names, info.Name())
			}
		}
		if mcpOpsCounter != nil { mcpOpsCounter.Add(ctx, 1, metric.WithAttributes(attribute.String("operation", "list_directory"))) }
		return json.Marshal(map[string][]string{"files": names})

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

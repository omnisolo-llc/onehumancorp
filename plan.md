1. **Create `srcs/server/tools/hybridfsmcp/provider.go`:**
   ```bash
   cat << 'EOF' > srcs/server/tools/hybridfsmcp/provider.go
   package hybridfsmcp

   import (
       "context"
       "fmt"
       "os"
       "path/filepath"
       "strings"

       "github.com/onehumancorp/mono/srcs/server/auth"
   )

   type FileSystemProvider interface {
       ReadFile(ctx context.Context, path string) ([]byte, error)
       WriteFile(ctx context.Context, path string, data []byte) error
       ListDir(ctx context.Context, path string) ([]string, error)
   }

   type LocalFSProvider struct {
       basePath string
   }

   func NewLocalFSProvider(basePath string) *LocalFSProvider {
       return &LocalFSProvider{basePath: basePath}
   }

   func (p *LocalFSProvider) securePath(targetPath string) (string, error) {
       cleanPath := filepath.Clean(targetPath)
       if filepath.IsAbs(cleanPath) {
           return "", fmt.Errorf("absolute paths are not allowed")
       }

       fullPath := filepath.Join(p.basePath, cleanPath)
       rel, err := filepath.Rel(p.basePath, fullPath)
       if err != nil {
           return "", err
       }
       if strings.HasPrefix(rel, "..") {
           return "", fmt.Errorf("path traversal not allowed")
       }
       return fullPath, nil
   }

   func (p *LocalFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
       fullPath, err := p.securePath(path)
       if err != nil {
           return nil, err
       }
       return os.ReadFile(fullPath)
   }

   func (p *LocalFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
       fullPath, err := p.securePath(path)
       if err != nil {
           return err
       }
       dir := filepath.Dir(fullPath)
       if err := os.MkdirAll(dir, 0755); err != nil {
           return err
       }
       return os.WriteFile(fullPath, data, 0644)
   }

   func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
       fullPath, err := p.securePath(path)
       if err != nil {
           return nil, err
       }
       entries, err := os.ReadDir(fullPath)
       if err != nil {
           return nil, err
       }
       var names []string
       for _, entry := range entries {
           names = append(names, entry.Name())
       }
       return names, nil
   }

   type CloudFSProvider struct {
       basePath string
   }

   func NewCloudFSProvider(basePath string) *CloudFSProvider {
       return &CloudFSProvider{basePath: basePath}
   }

   func (p *CloudFSProvider) securePath(ctx context.Context, targetPath string) (string, error) {
       claims := auth.ClaimsFromContext(ctx)
       if claims == nil || claims.OrganizationID == "" {
           return "", fmt.Errorf("unauthorized: missing organization ID")
       }

       cleanPath := filepath.Clean(targetPath)
       if filepath.IsAbs(cleanPath) {
           return "", fmt.Errorf("absolute paths are not allowed")
       }

       tenantBasePath := filepath.Join(p.basePath, claims.OrganizationID)

       fullPath := filepath.Join(tenantBasePath, cleanPath)
       rel, err := filepath.Rel(tenantBasePath, fullPath)
       if err != nil {
           return "", err
       }
       if strings.HasPrefix(rel, "..") {
           return "", fmt.Errorf("path traversal not allowed")
       }
       return fullPath, nil
   }

   func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
       fullPath, err := p.securePath(ctx, path)
       if err != nil {
           return nil, err
       }
       return os.ReadFile(fullPath)
   }

   func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
       fullPath, err := p.securePath(ctx, path)
       if err != nil {
           return err
       }
       dir := filepath.Dir(fullPath)
       if err := os.MkdirAll(dir, 0755); err != nil {
           return err
       }
       return os.WriteFile(fullPath, data, 0644)
   }

   func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
       fullPath, err := p.securePath(ctx, path)
       if err != nil {
           return nil, err
       }
       entries, err := os.ReadDir(fullPath)
       if err != nil {
           return nil, err
       }
       var names []string
       for _, entry := range entries {
           names = append(names, entry.Name())
       }
       return names, nil
   }
   EOF
   cat srcs/server/tools/hybridfsmcp/provider.go
   ```

2. **Create `srcs/server/tools/hybridfsmcp/mcp.go`:**
   ```bash
   cat << 'EOF' > srcs/server/tools/hybridfsmcp/mcp.go
   package hybridfsmcp

   import (
       "context"
       "encoding/json"
       "errors"
       "fmt"
   )

   type Tool struct {
       Name        string          `json:"name"`
       Description string          `json:"description"`
       InputSchema json.RawMessage `json:"inputSchema"`
   }

   type HybridFSMCP struct {
       provider FileSystemProvider
   }

   func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
       return &HybridFSMCP{provider: provider}
   }

   func (m *HybridFSMCP) ListTools() []Tool {
       return []Tool{
           {
               Name:        "read_file",
               Description: "Reads the content of a file.",
               InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string"}},"required":["path"]}`),
           },
           {
               Name:        "write_file",
               Description: "Writes content to a file.",
               InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string"},"content":{"type":"string"}},"required":["path","content"]}`),
           },
           {
               Name:        "list_directory",
               Description: "Lists files and directories in a given path.",
               InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string"}},"required":["path"]}`),
           },
       }
   }

   func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
       switch toolName {
       case "read_file":
           path, ok := arguments["path"].(string)
           if !ok {
               return nil, errors.New("missing or invalid 'path'")
           }
           data, err := m.provider.ReadFile(ctx, path)
           if err != nil {
               return nil, err
           }
           return map[string]interface{}{"status": "success", "content": string(data)}, nil
       case "write_file":
           path, ok := arguments["path"].(string)
           if !ok {
               return nil, errors.New("missing or invalid 'path'")
           }
           content, ok := arguments["content"].(string)
           if !ok {
               return nil, errors.New("missing or invalid 'content'")
           }
           err := m.provider.WriteFile(ctx, path, []byte(content))
           if err != nil {
               return nil, err
           }
           return map[string]interface{}{"status": "success"}, nil
       case "list_directory":
           path, ok := arguments["path"].(string)
           if !ok {
               return nil, errors.New("missing or invalid 'path'")
           }
           entries, err := m.provider.ListDir(ctx, path)
           if err != nil {
               return nil, err
           }
           return map[string]interface{}{"status": "success", "entries": entries}, nil
       default:
           return nil, fmt.Errorf("unknown tool: %s", toolName)
       }
   }
   EOF
   cat srcs/server/tools/hybridfsmcp/mcp.go
   ```

3. **Create tests:**
   ```bash
   cat << 'EOF' > srcs/server/tools/hybridfsmcp/provider_test.go
   package hybridfsmcp

   import (
       "context"
       "os"
       "path/filepath"
       "testing"

       "github.com/onehumancorp/mono/srcs/server/auth"
   )

   func TestLocalFSProvider(t *testing.T) {
       tmpDir := t.TempDir()
       provider := NewLocalFSProvider(tmpDir)

       ctx := context.Background()

       // Write
       err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
       if err != nil {
           t.Fatalf("unexpected error: %v", err)
       }

       // Read
       data, err := provider.ReadFile(ctx, "test.txt")
       if err != nil {
           t.Fatalf("unexpected error: %v", err)
       }
       if string(data) != "hello" {
           t.Errorf("expected hello, got %s", string(data))
       }

       // List
       entries, err := provider.ListDir(ctx, ".")
       if err != nil {
           t.Fatalf("unexpected error: %v", err)
       }
       if len(entries) != 1 || entries[0] != "test.txt" {
           t.Errorf("expected [test.txt], got %v", entries)
       }

       // Absolute path
       err = provider.WriteFile(ctx, "/etc/passwd", []byte("hack"))
       if err == nil {
           t.Errorf("expected error for absolute path")
       }

       // Traversal
       err = provider.WriteFile(ctx, "../hack.txt", []byte("hack"))
       if err == nil {
           t.Errorf("expected error for traversal")
       }
   }

   func TestCloudFSProvider(t *testing.T) {
       tmpDir := t.TempDir()
       provider := NewCloudFSProvider(tmpDir)

       claims := &auth.Claims{OrganizationID: "org-123"}
       ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

       // Write
       err := provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
       if err != nil {
           t.Fatalf("unexpected error: %v", err)
       }

       // Read
       data, err := provider.ReadFile(ctx, "test.txt")
       if err != nil {
           t.Fatalf("unexpected error: %v", err)
       }
       if string(data) != "hello cloud" {
           t.Errorf("expected hello cloud, got %s", string(data))
       }

       // Tenant separation
       claims2 := &auth.Claims{OrganizationID: "org-456"}
       ctx2 := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims2)

       _, err = provider.ReadFile(ctx2, "test.txt")
       if err == nil {
           t.Errorf("expected error reading other tenant file")
       }
   }
   EOF
   cat srcs/server/tools/hybridfsmcp/provider_test.go

   cat << 'EOF' > srcs/server/tools/hybridfsmcp/mcp_test.go
   package hybridfsmcp

   import (
       "context"
       "testing"
   )

   type MockProvider struct{}

   func (m *MockProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
       return []byte("mock data"), nil
   }
   func (m *MockProvider) WriteFile(ctx context.Context, path string, data []byte) error {
       return nil
   }
   func (m *MockProvider) ListDir(ctx context.Context, path string) ([]string, error) {
       return []string{"mock.txt"}, nil
   }

   func TestHybridFSMCP(t *testing.T) {
       mcp := NewHybridFSMCP(&MockProvider{})
       tools := mcp.ListTools()
       if len(tools) != 3 {
           t.Errorf("expected 3 tools, got %d", len(tools))
       }

       ctx := context.Background()

       // Read
       res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "test.txt"})
       if err != nil {
           t.Fatalf("unexpected error: %v", err)
       }
       resMap := res.(map[string]interface{})
       if resMap["content"] != "mock data" {
           t.Errorf("expected mock data, got %v", resMap["content"])
       }

       // Write
       res, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "test.txt", "content": "mock data"})
       if err != nil {
           t.Fatalf("unexpected error: %v", err)
       }
       resMap = res.(map[string]interface{})
       if resMap["status"] != "success" {
           t.Errorf("expected success, got %v", resMap["status"])
       }

       // List
       res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{"path": "."})
       if err != nil {
           t.Fatalf("unexpected error: %v", err)
       }
       resMap = res.(map[string]interface{})
       entries := resMap["entries"].([]string)
       if entries[0] != "mock.txt" {
           t.Errorf("expected mock.txt, got %v", entries)
       }
   }
   EOF
   cat srcs/server/tools/hybridfsmcp/mcp_test.go
   ```

4. **Add Bazel build configuration (`BUILD.bazel`):**
   ```bash
   cat << 'EOF' > srcs/server/tools/hybridfsmcp/BUILD.bazel
   load("@rules_go//go:def.bzl", "go_library", "go_test")

   go_library(
       name = "hybridfsmcp",
       srcs = [
           "mcp.go",
           "provider.go",
       ],
       importpath = "github.com/onehumancorp/mono/srcs/server/tools/hybridfsmcp",
       visibility = ["//visibility:public"],
       deps = [
           "//srcs/server/auth",
       ],
   )

   go_test(
       name = "hybridfsmcp_test",
       srcs = [
           "mcp_test.go",
           "provider_test.go",
       ],
       embed = [":hybridfsmcp"],
       deps = [
           "//srcs/server/auth",
       ],
   )
   EOF
   cat srcs/server/tools/hybridfsmcp/BUILD.bazel
   ```

5. **Run test suite**:
   ```bash
   export PATH="$PATH:$HOME/go/bin" && bazelisk test //... --test_output=errors --jobs=4 --local_test_jobs=1 > bazel_test.log 2>&1 &
   ```

6. **Mark Mission Done**:
   Update `.agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` status to DONE and agent to Miser. Wait, the instructions said: "When claiming a pending mission file from `.agent-task/missions/`, you must update the frontmatter to `status: IN_PROGRESS` and set the `agent:` field to your assigned role or name (e.g., `agent: Taskmaster`). Once the task is fully completed, update the mission frontmatter to `status: DONE`." Wait, my agent name is "Miser". Wait, actually I am an Implementer. Let me check the persona context. "Role: Principal Cost Engineer & Miser (L7)", "Swarm Category: IMPLEMENTER". I will set `agent: Miser`.

7. **Pre-commit**:
   Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

1. **Implement LocalFSProvider**
   - Create `srcs/server/tools/hybridfsmcp/local_provider.go` using a full bash command:
     ```bash
     cat << 'EOF' > srcs/server/tools/hybridfsmcp/local_provider.go
     package hybridfsmcp

     import (
         "context"
         "fmt"
         "os"
         "path/filepath"
         "strings"

         "github.com/onehumancorp/mono/srcs/server/auth"
     )

     type LocalFSProvider struct {
         workspaceDir string
     }

     func NewLocalFSProvider(workspaceDir string) (*LocalFSProvider, error) {
         abs, err := filepath.Abs(workspaceDir)
         if err != nil {
             return nil, err
         }
         return &LocalFSProvider{workspaceDir: abs}, nil
     }

     func (p *LocalFSProvider) resolvePath(target string) (string, error) {
         cleanTarget := filepath.Clean(target)
         if filepath.IsAbs(cleanTarget) {
             return "", fmt.Errorf("absolute paths not allowed")
         }
         fullPath := filepath.Join(p.workspaceDir, cleanTarget)

         if !strings.HasPrefix(fullPath, p.workspaceDir) || (len(fullPath) > len(p.workspaceDir) && fullPath[len(p.workspaceDir)] != filepath.Separator) {
             if fullPath != p.workspaceDir {
                 return "", fmt.Errorf("path escapes workspace boundary")
             }
         }
         return fullPath, nil
     }

     func (p *LocalFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
         fullPath, err := p.resolvePath(path)
         if err != nil {
             return nil, err
         }
         return os.ReadFile(fullPath)
     }

     func (p *LocalFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
         fullPath, err := p.resolvePath(path)
         if err != nil {
             return err
         }
         dir := filepath.Dir(fullPath)
         if err := os.MkdirAll(dir, 0700); err != nil {
             return err
         }
         return os.WriteFile(fullPath, content, 0600)
     }

     func (p *LocalFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
         fullPath, err := p.resolvePath(path)
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
     gofmt -w srcs/server/tools/hybridfsmcp/local_provider.go
     ```
   - Verify the file creation: `cat srcs/server/tools/hybridfsmcp/local_provider.go`.

2. **Implement CloudFSProvider**
   - Create `srcs/server/tools/hybridfsmcp/cloud_provider.go` using a full bash command:
     ```bash
     cat << 'EOF' > srcs/server/tools/hybridfsmcp/cloud_provider.go
     package hybridfsmcp

     import (
         "context"
         "fmt"
         "os"
         "path/filepath"
         "strings"

         "github.com/onehumancorp/mono/srcs/server/auth"
     )

     type CloudFSProvider struct {
         baseVolumeDir string
     }

     func NewCloudFSProvider(baseVolumeDir string) (*CloudFSProvider, error) {
         abs, err := filepath.Abs(baseVolumeDir)
         if err != nil {
             return nil, err
         }
         return &CloudFSProvider{baseVolumeDir: abs}, nil
     }

     func (p *CloudFSProvider) resolvePath(claims *auth.Claims, target string) (string, error) {
         if claims == nil || claims.OrganizationID == "" {
             return "", fmt.Errorf("missing organization ID in claims")
         }
         cleanTarget := filepath.Clean(target)
         if filepath.IsAbs(cleanTarget) {
             return "", fmt.Errorf("absolute paths not allowed")
         }
         tenantDir := filepath.Join(p.baseVolumeDir, claims.OrganizationID)
         fullPath := filepath.Join(tenantDir, cleanTarget)

         if !strings.HasPrefix(fullPath, tenantDir) || (len(fullPath) > len(tenantDir) && fullPath[len(tenantDir)] != filepath.Separator) {
             if fullPath != tenantDir {
                 return "", fmt.Errorf("path escapes tenant boundary")
             }
         }
         return fullPath, nil
     }

     func (p *CloudFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
         fullPath, err := p.resolvePath(claims, path)
         if err != nil {
             return nil, err
         }
         return os.ReadFile(fullPath)
     }

     func (p *CloudFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
         fullPath, err := p.resolvePath(claims, path)
         if err != nil {
             return err
         }
         dir := filepath.Dir(fullPath)
         if err := os.MkdirAll(dir, 0700); err != nil {
             return err
         }
         return os.WriteFile(fullPath, content, 0600)
     }

     func (p *CloudFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
         fullPath, err := p.resolvePath(claims, path)
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
     gofmt -w srcs/server/tools/hybridfsmcp/cloud_provider.go
     ```
   - Verify the file creation: `cat srcs/server/tools/hybridfsmcp/cloud_provider.go`.

3. **Create Hybrid FS MCP Server**
   - Create `srcs/server/tools/hybridfsmcp/mcp.go` using a full bash command:
     ```bash
     cat << 'EOF' > srcs/server/tools/hybridfsmcp/mcp.go
     package hybridfsmcp

     import (
         "context"
         "encoding/json"

         "github.com/onehumancorp/mono/srcs/server/auth"
     )

     type HybridFSMCP struct {
         provider FileSystemProvider
     }

     func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
         return &HybridFSMCP{provider: provider}
     }

     type readFileInput struct {
         Path string `json:"path"`
     }

     type writeFileInput struct {
         Path    string `json:"path"`
         Content string `json:"content"`
     }

     type listDirInput struct {
         Path string `json:"path"`
     }

     func (m *HybridFSMCP) ReadFileTool(ctx context.Context, inputData []byte) (interface{}, error) {
         claims := auth.ClaimsFromContext(ctx)
         var input readFileInput
         if err := json.Unmarshal(inputData, &input); err != nil {
             return nil, err
         }
         content, err := m.provider.ReadFile(ctx, claims, input.Path)
         if err != nil {
             return nil, err
         }
         return map[string]string{"content": string(content)}, nil
     }

     func (m *HybridFSMCP) WriteFileTool(ctx context.Context, inputData []byte) (interface{}, error) {
         claims := auth.ClaimsFromContext(ctx)
         var input writeFileInput
         if err := json.Unmarshal(inputData, &input); err != nil {
             return nil, err
         }
         err := m.provider.WriteFile(ctx, claims, input.Path, []byte(input.Content))
         if err != nil {
             return nil, err
         }
         return map[string]string{"status": "success"}, nil
     }

     func (m *HybridFSMCP) ListDirTool(ctx context.Context, inputData []byte) (interface{}, error) {
         claims := auth.ClaimsFromContext(ctx)
         var input listDirInput
         if err := json.Unmarshal(inputData, &input); err != nil {
             return nil, err
         }
         entries, err := m.provider.ListDir(ctx, claims, input.Path)
         if err != nil {
             return nil, err
         }
         return map[string]interface{}{"entries": entries}, nil
     }
     EOF
     gofmt -w srcs/server/tools/hybridfsmcp/mcp.go
     ```
   - Verify the file creation: `cat srcs/server/tools/hybridfsmcp/mcp.go`.

4. **Add Factory for Environment Instantiation**
   - Create `srcs/server/tools/hybridfsmcp/factory.go` using a full bash command:
     ```bash
     cat << 'EOF' > srcs/server/tools/hybridfsmcp/factory.go
     package hybridfsmcp

     import (
         "fmt"
         "os"
     )

     func NewProviderFromEnv() (FileSystemProvider, error) {
         if os.Getenv("OHC_MULTITENANT") == "true" {
             baseDir := os.Getenv("OHC_CLOUD_FS_BASE")
             if baseDir == "" {
                 baseDir = "/var/ohc/tenant_volumes"
             }
             return NewCloudFSProvider(baseDir)
         } else if os.Getenv("OHC_STANDALONE") == "true" {
             workspaceDir := os.Getenv("OHC_LOCAL_FS_WORKSPACE")
             if workspaceDir == "" {
                 workspaceDir = "./ohc_workspace"
             }
             return NewLocalFSProvider(workspaceDir)
         }
         return nil, fmt.Errorf("neither OHC_MULTITENANT nor OHC_STANDALONE is set")
     }
     EOF
     gofmt -w srcs/server/tools/hybridfsmcp/factory.go
     ```
   - Verify the file creation: `cat srcs/server/tools/hybridfsmcp/factory.go`.

5. **Create Unit Tests for LocalFSProvider**
   - Create `srcs/server/tools/hybridfsmcp/local_provider_test.go` using a full bash command:
     ```bash
     cat << 'EOF' > srcs/server/tools/hybridfsmcp/local_provider_test.go
     package hybridfsmcp

     import (
         "context"
         "testing"
     )

     func TestLocalFSProvider(t *testing.T) {
         tempDir := t.TempDir()
         provider, err := NewLocalFSProvider(tempDir)
         if err != nil {
             t.Fatalf("Failed to create provider: %v", err)
         }

         ctx := context.Background()

         t.Run("Write and Read File", func(t *testing.T) {
             err := provider.WriteFile(ctx, nil, "test.txt", []byte("hello"))
             if err != nil {
                 t.Fatalf("WriteFile failed: %v", err)
             }

             content, err := provider.ReadFile(ctx, nil, "test.txt")
             if err != nil {
                 t.Fatalf("ReadFile failed: %v", err)
             }
             if string(content) != "hello" {
                 t.Errorf("Expected 'hello', got '%s'", string(content))
             }
         })

         t.Run("Path Traversal Blocked", func(t *testing.T) {
             err := provider.WriteFile(ctx, nil, "../outside.txt", []byte("bad"))
             if err == nil {
                 t.Error("Expected error for path traversal, got nil")
             }
         })

         t.Run("List Dir", func(t *testing.T) {
             provider.WriteFile(ctx, nil, "dir/file1.txt", []byte("1"))
             provider.WriteFile(ctx, nil, "dir/file2.txt", []byte("2"))
             entries, err := provider.ListDir(ctx, nil, "dir")
             if err != nil {
                 t.Fatalf("ListDir failed: %v", err)
             }
             if len(entries) != 2 {
                 t.Errorf("Expected 2 entries, got %d", len(entries))
             }
         })
     }
     EOF
     gofmt -w srcs/server/tools/hybridfsmcp/local_provider_test.go
     ```
   - Verify the file creation: `cat srcs/server/tools/hybridfsmcp/local_provider_test.go`.

6. **Create Unit Tests for CloudFSProvider**
   - Create `srcs/server/tools/hybridfsmcp/cloud_provider_test.go` using a full bash command:
     ```bash
     cat << 'EOF' > srcs/server/tools/hybridfsmcp/cloud_provider_test.go
     package hybridfsmcp

     import (
         "context"
         "testing"

         "github.com/onehumancorp/mono/srcs/server/auth"
     )

     func TestCloudFSProvider(t *testing.T) {
         tempDir := t.TempDir()
         provider, err := NewCloudFSProvider(tempDir)
         if err != nil {
             t.Fatalf("Failed to create provider: %v", err)
         }

         ctx := context.Background()
         claims := &auth.Claims{OrganizationID: "org-123"}
         otherClaims := &auth.Claims{OrganizationID: "org-456"}

         t.Run("Write and Read File Tenant Scoped", func(t *testing.T) {
             err := provider.WriteFile(ctx, claims, "data.txt", []byte("tenant data"))
             if err != nil {
                 t.Fatalf("WriteFile failed: %v", err)
             }

             content, err := provider.ReadFile(ctx, claims, "data.txt")
             if err != nil {
                 t.Fatalf("ReadFile failed: %v", err)
             }
             if string(content) != "tenant data" {
                 t.Errorf("Expected 'tenant data', got '%s'", string(content))
             }

             // Other tenant should not read it
             _, err = provider.ReadFile(ctx, otherClaims, "data.txt")
             if err == nil {
                 t.Error("Expected error when other tenant reads file, got nil")
             }
         })

         t.Run("Path Traversal Blocked", func(t *testing.T) {
             err := provider.WriteFile(ctx, claims, "../org-456/data.txt", []byte("hack"))
             if err == nil {
                 t.Error("Expected error for cross-tenant path traversal, got nil")
             }
         })

         t.Run("List Dir", func(t *testing.T) {
             provider.WriteFile(ctx, claims, "docs/doc1.txt", []byte("1"))
             entries, err := provider.ListDir(ctx, claims, "docs")
             if err != nil {
                 t.Fatalf("ListDir failed: %v", err)
             }
             if len(entries) != 1 {
                 t.Errorf("Expected 1 entry, got %d", len(entries))
             }
         })
     }
     EOF
     gofmt -w srcs/server/tools/hybridfsmcp/cloud_provider_test.go
     ```
   - Verify the file creation: `cat srcs/server/tools/hybridfsmcp/cloud_provider_test.go`.

7. **Create Unit Tests for MCP Server**
   - Create `srcs/server/tools/hybridfsmcp/mcp_test.go` using a full bash command:
     ```bash
     cat << 'EOF' > srcs/server/tools/hybridfsmcp/mcp_test.go
     package hybridfsmcp

     import (
         "context"
         "encoding/json"
         "testing"

         "github.com/onehumancorp/mono/srcs/server/auth"
     )

     type mockProvider struct {
         data map[string]string
     }

     func (m *mockProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
         return []byte(m.data[path]), nil
     }

     func (m *mockProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
         m.data[path] = string(content)
         return nil
     }

     func (m *mockProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
         return []string{"file1.txt"}, nil
     }

     func TestHybridFSMCP(t *testing.T) {
         mock := &mockProvider{data: make(map[string]string)}
         mcp := NewHybridFSMCP(mock)

         ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-1"})

         t.Run("Write File Tool", func(t *testing.T) {
             input := writeFileInput{Path: "test.txt", Content: "hello mcp"}
             data, _ := json.Marshal(input)
             _, err := mcp.WriteFileTool(ctx, data)
             if err != nil {
                 t.Fatalf("WriteFileTool failed: %v", err)
             }
             if mock.data["test.txt"] != "hello mcp" {
                 t.Errorf("Expected 'hello mcp', got '%s'", mock.data["test.txt"])
             }
         })

         t.Run("Read File Tool", func(t *testing.T) {
             input := readFileInput{Path: "test.txt"}
             data, _ := json.Marshal(input)
             res, err := mcp.ReadFileTool(ctx, data)
             if err != nil {
                 t.Fatalf("ReadFileTool failed: %v", err)
             }
             resMap := res.(map[string]string)
             if resMap["content"] != "hello mcp" {
                 t.Errorf("Expected 'hello mcp', got '%s'", resMap["content"])
             }
         })

         t.Run("List Dir Tool", func(t *testing.T) {
             input := listDirInput{Path: "."}
             data, _ := json.Marshal(input)
             res, err := mcp.ListDirTool(ctx, data)
             if err != nil {
                 t.Fatalf("ListDirTool failed: %v", err)
             }
             resMap := res.(map[string]interface{})
             entries := resMap["entries"].([]string)
             if len(entries) != 1 || entries[0] != "file1.txt" {
                 t.Errorf("Unexpected entries: %v", entries)
             }
         })
     }
     EOF
     gofmt -w srcs/server/tools/hybridfsmcp/mcp_test.go
     ```
   - Verify the file creation: `cat srcs/server/tools/hybridfsmcp/mcp_test.go`.

8. **Create BUILD.bazel**
   - Create `srcs/server/tools/hybridfsmcp/BUILD.bazel` using a full bash command:
     ```bash
     cat << 'EOF' > srcs/server/tools/hybridfsmcp/BUILD.bazel
     load("@rules_go//go:def.bzl", "go_library", "go_test")

     go_library(
         name = "hybridfsmcp",
         srcs = [
             "cloud_provider.go",
             "factory.go",
             "interface.go",
             "local_provider.go",
             "mcp.go",
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
             "cloud_provider_test.go",
             "local_provider_test.go",
             "mcp_test.go",
         ],
         embed = [":hybridfsmcp"],
         deps = [
             "//srcs/server/auth",
         ],
     )
     EOF
     ```
   - Verify creation: `cat srcs/server/tools/hybridfsmcp/BUILD.bazel`.

9. **Update mission status**
    - Claim mission: `sed -i 's/agent: Researcher/agent: Link/' .agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md`
    - Verify with: `head -n 5 .agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md`.
    - Update `.agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md` to `status: DONE` using `sed -i 's/status: PENDING/status: DONE/' .agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md`.
    - Verify with: `head -n 5 .agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md`.

10. **Run Tests**
   - Run tests: `~/go/bin/bazelisk test //srcs/server/tools/hybridfsmcp/... --test_output=errors --jobs=4 --local_test_jobs=1`.

11. **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.**

12. **Submit PR**
   - Submit the PR using the `submit` tool.

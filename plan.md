# Execution Plan

1. **Claim the Mission**:
   - Rename file: `mv .agent-task/missions/2026-04-07T08-05-00Z_research_hybrid_fs_mcp.md .agent-task/missions/2026-04-07T08-05-00Z.md`
   - Update file content: `sed -i 's/status: PENDING/status: IN_PROGRESS/g' .agent-task/missions/2026-04-07T08-05-00Z.md`
   - Update agent content: `sed -i 's/agent: Researcher/agent: Jules/g' .agent-task/missions/2026-04-07T08-05-00Z.md`
   - Verify: `cat .agent-task/missions/2026-04-07T08-05-00Z.md | head -n 5`

2. **Implement File System Provider Interface and Local/Cloud implementations**:
   - Run the following command:
     ```bash
     cat << 'EOF' > srcs/server/tools/hybridfsmcp/provider.go
     package hybridfsmcp

     import (
         "context"
         "fmt"
         "io/fs"
         "os"
         "path/filepath"
         "strings"
     )

     type FileSystemProvider interface {
         ReadFile(ctx context.Context, path string) ([]byte, error)
         WriteFile(ctx context.Context, path string, data []byte) error
         ListDir(ctx context.Context, path string) ([]fs.DirEntry, error)
     }

     type LocalFSProvider struct {
         baseDir string
     }

     func NewLocalFSProvider(baseDir string) *LocalFSProvider {
         return &LocalFSProvider{baseDir: baseDir}
     }

     func (p *LocalFSProvider) securePath(path string) (string, error) {
         cleanPath := filepath.Clean(path)
         fullPath := filepath.Join(p.baseDir, cleanPath)
         if !strings.HasPrefix(fullPath, p.baseDir) {
             return "", fmt.Errorf("access denied: %s is outside base directory", path)
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

     func (p *LocalFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
         fullPath, err := p.securePath(path)
         if err != nil {
             return nil, err
         }
         return os.ReadDir(fullPath)
     }

     type CloudFSProvider struct {
         baseDir  string
         tenantID string
     }

     func NewCloudFSProvider(baseDir, tenantID string) *CloudFSProvider {
         return &CloudFSProvider{
             baseDir:  baseDir,
             tenantID: tenantID,
         }
     }

     func (p *CloudFSProvider) securePath(path string) (string, error) {
         if p.tenantID == "" {
             return "", fmt.Errorf("tenant ID is required")
         }
         tenantDir := filepath.Join(p.baseDir, p.tenantID)
         cleanPath := filepath.Clean(path)
         fullPath := filepath.Join(tenantDir, cleanPath)
         if !strings.HasPrefix(fullPath, tenantDir) {
             return "", fmt.Errorf("access denied: %s is outside tenant directory", path)
         }
         return fullPath, nil
     }

     func (p *CloudFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
         fullPath, err := p.securePath(path)
         if err != nil {
             return nil, err
         }
         return os.ReadFile(fullPath)
     }

     func (p *CloudFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
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

     func (p *CloudFSProvider) ListDir(ctx context.Context, path string) ([]fs.DirEntry, error) {
         fullPath, err := p.securePath(path)
         if err != nil {
             return nil, err
         }
         return os.ReadDir(fullPath)
     }
     EOF
     ```
   - Verify file exists: `cat srcs/server/tools/hybridfsmcp/provider.go && ls -la srcs/server/tools/hybridfsmcp/provider.go`

3. **Implement MCP Server & Factory Logic**:
   - Run the following command:
     ```bash
     cat << 'EOF' > srcs/server/tools/hybridfsmcp/server.go
     package hybridfsmcp

     import (
         "context"
         "encoding/json"
         "fmt"
     )

     type ReadFileArgs struct {
         Path string `json:"path"`
     }

     type WriteFileArgs struct {
         Path string `json:"path"`
         Data []byte `json:"data"`
     }

     type ListDirArgs struct {
         Path string `json:"path"`
     }

     type HybridFSMCPServer struct {
         provider FileSystemProvider
     }

     func NewHybridFSMCP(isCloud bool, baseDir string, tenantID string) *HybridFSMCPServer {
         var provider FileSystemProvider
         if isCloud {
             provider = NewCloudFSProvider(baseDir, tenantID)
         } else {
             provider = NewLocalFSProvider(baseDir)
         }
         return &HybridFSMCPServer{provider: provider}
     }

     func (s *HybridFSMCPServer) ExecuteTool(ctx context.Context, toolName string, argsRaw json.RawMessage) (interface{}, error) {
         switch toolName {
         case "read_file":
             var args ReadFileArgs
             if err := json.Unmarshal(argsRaw, &args); err != nil {
                 return nil, err
             }
             data, err := s.provider.ReadFile(ctx, args.Path)
             if err != nil {
                 return nil, err
             }
             return string(data), nil
         case "write_file":
             var args WriteFileArgs
             if err := json.Unmarshal(argsRaw, &args); err != nil {
                 return nil, err
             }
             if err := s.provider.WriteFile(ctx, args.Path, args.Data); err != nil {
                 return nil, err
             }
             return "success", nil
         case "list_directory":
             var args ListDirArgs
             if err := json.Unmarshal(argsRaw, &args); err != nil {
                 return nil, err
             }
             entries, err := s.provider.ListDir(ctx, args.Path)
             if err != nil {
                 return nil, err
             }
             var names []string
             for _, e := range entries {
                 names = append(names, e.Name())
             }
             return names, nil
         default:
             return nil, fmt.Errorf("unknown tool: %s", toolName)
         }
     }
     EOF
     ```
   - Verify file exists: `cat srcs/server/tools/hybridfsmcp/server.go && ls -la srcs/server/tools/hybridfsmcp/server.go`

4. **Write Unit Tests**:
   - Run the following command:
     ```bash
     cat << 'EOF' > srcs/server/tools/hybridfsmcp/provider_test.go
     package hybridfsmcp

     import (
         "context"
         "encoding/json"
         "os"
         "path/filepath"
         "testing"
     )

     func TestLocalFSProvider(t *testing.T) {
         tempDir := t.TempDir()
         provider := NewLocalFSProvider(tempDir)
         ctx := context.Background()

         // Test WriteFile
         err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
         if err != nil {
             t.Fatalf("WriteFile failed: %v", err)
         }

         // Test ReadFile
         data, err := provider.ReadFile(ctx, "test.txt")
         if err != nil {
             t.Fatalf("ReadFile failed: %v", err)
         }
         if string(data) != "hello" {
             t.Fatalf("expected 'hello', got '%s'", string(data))
         }

         // Test Path Escape (should fail)
         err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
         if err == nil {
             t.Fatalf("expected path escape to fail")
         }

         // Test ListDir
         entries, err := provider.ListDir(ctx, ".")
         if err != nil {
            t.Fatalf("ListDir failed: %v", err)
         }
         if len(entries) != 1 || entries[0].Name() != "test.txt" {
            t.Fatalf("ListDir unexpected output")
         }
     }

     func TestCloudFSProvider(t *testing.T) {
         tempDir := t.TempDir()
         provider := NewCloudFSProvider(tempDir, "tenant1")
         ctx := context.Background()

         // Test WriteFile
         err := provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
         if err != nil {
             t.Fatalf("WriteFile failed: %v", err)
         }

         // Verify path is actually under tenant
         _, err = os.Stat(filepath.Join(tempDir, "tenant1", "test.txt"))
         if err != nil {
             t.Fatalf("File was not created in tenant dir: %v", err)
         }

         // Test ReadFile
         data, err := provider.ReadFile(ctx, "test.txt")
         if err != nil {
             t.Fatalf("ReadFile failed: %v", err)
         }
         if string(data) != "hello cloud" {
             t.Fatalf("expected 'hello cloud', got '%s'", string(data))
         }

         // Test Path Escape (should fail)
         err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
         if err == nil {
             t.Fatalf("expected path escape to fail")
         }
     }

     func TestHybridFSMCP(t *testing.T) {
        tempDir := t.TempDir()
        server := NewHybridFSMCP(false, tempDir, "")
        ctx := context.Background()

        args1, _ := json.Marshal(WriteFileArgs{Path: "mcp.txt", Data: []byte("mcp")})
        _, err := server.ExecuteTool(ctx, "write_file", args1)
        if err != nil {
             t.Fatalf("WriteFile failed: %v", err)
        }

        args2, _ := json.Marshal(ReadFileArgs{Path: "mcp.txt"})
        res, err := server.ExecuteTool(ctx, "read_file", args2)
        if err != nil {
             t.Fatalf("ReadFile failed: %v", err)
        }
        if res.(string) != "mcp" {
            t.Fatalf("Expected 'mcp' got %v", res)
        }

        args3, _ := json.Marshal(ListDirArgs{Path: "."})
        resList, err := server.ExecuteTool(ctx, "list_directory", args3)
        if err != nil {
             t.Fatalf("ListDir failed: %v", err)
        }
        list := resList.([]string)
        if len(list) != 1 || list[0] != "mcp.txt" {
            t.Fatalf("ListDir unexpected output")
        }
     }
     EOF
     ```
   - Verify file exists: `cat srcs/server/tools/hybridfsmcp/provider_test.go && ls -la srcs/server/tools/hybridfsmcp/provider_test.go`

5. **Update BUILD files**:
   - Run: `bazelisk run //:gazelle -- update srcs/server/tools/hybridfsmcp`

6. **Run Tests**:
   - Run tests: `~/go/bin/bazelisk test //srcs/server/tools/hybridfsmcp/...`

* Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

7. **Complete the Mission**:
   - Update file status: `sed -i 's/status: IN_PROGRESS/status: DONE/g' .agent-task/missions/2026-04-07T08-05-00Z.md`
   - Run the following command to create status file:
     ```bash
     cat << 'EOF' > .agent-task/status/$(date -Iseconds).yml
     agent: Jules
     mission: "2026-04-07T08-05-00Z"
     status: DONE
     EOF
     ```
   - Run the following command to create memory file:
     ```bash
     cat << 'EOF' > .agent-task/memory/$(date -Iseconds).yml
     agent: Jules
     summary: "Implemented Hybrid File System MCP Server bridging Local and Cloud execution via provider interface."
     EOF
     ```

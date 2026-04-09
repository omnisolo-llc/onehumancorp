package hybridfsmcp

import (
    "encoding/json"
    "testing"
)

func TestHybridFSServer(t *testing.T) {
    workspace := t.TempDir()
    provider := &LocalFSProvider{WorkspaceDir: workspace}
    server := &HybridFSServer{Provider: provider}

    writeParams := ToolParams{Operation: "write_file", Path: "test.txt", Content: "hello server"}
    data, _ := json.Marshal(writeParams)
    _, err := server.HandleToolCall(data)
    if err != nil { t.Fatalf("expected no error, got %v", err) }

    readParams := ToolParams{Operation: "read_file", Path: "test.txt"}
    data, _ = json.Marshal(readParams)
    res, err := server.HandleToolCall(data)
    if err != nil { t.Fatalf("expected no error, got %v", err) }
    if res["content"] != "hello server" { t.Fatalf("expected hello server, got %v", res["content"]) }
}

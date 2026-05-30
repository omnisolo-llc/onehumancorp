package mcp

import (
    "context"
    "io/ioutil"
    "os"
    "strings"
    "testing"
)

func TestLogAnalyzerTool(t *testing.T) {
    tmpFile, err := ioutil.TempFile("", "agent_harness.log")
    if err != nil {
        t.Fatalf("failed to create temp file: %v", err)
    }
    defer os.Remove(tmpFile.Name())

    logData := "2023-10-27T10:00:00Z INFO something\n2023-10-27T10:01:00Z ERROR error 1\n2023-10-27T10:02:00Z ERROR error 2\n"
    if _, err := tmpFile.Write([]byte(logData)); err != nil {
        t.Fatalf("failed to write to temp file: %v", err)
    }
    tmpFile.Close()

    tool := &LogAnalyzerTool{
        LogPath: tmpFile.Name(),
    }

    result, err := tool.Execute(context.Background(), "ERROR", 60)
    if err != nil {
        t.Fatalf("Execute failed: %v", err)
    }

    if !strings.Contains(result, "Found 2 logs") {
        t.Errorf("expected 2 logs found, got: %s", result)
    }
    if !strings.Contains(result, "error 1") || !strings.Contains(result, "error 2") {
        t.Errorf("expected log lines not found in result: %s", result)
    }
}

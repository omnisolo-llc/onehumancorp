package mcp

import (
    "context"
    "fmt"
    "io/ioutil"
    "strings"
)

type LogAnalyzerTool struct {
    LogPath string
}

func (l *LogAnalyzerTool) Execute(ctx context.Context, level string, minutes int) (string, error) {
    path := l.LogPath
    if path == "" {
        path = "logs/agent_harness.log"
    }

    content, err := ioutil.ReadFile(path)
    if err != nil {
        return "", fmt.Errorf("failed to read log file: %w", err)
    }

    lines := strings.Split(string(content), "\n")

    var filtered []string
    for _, line := range lines {
        if strings.Contains(line, level) {
            filtered = append(filtered, line)
        }
    }

    // Top 50 lines (taking the last 50)
    var result []string
    startIdx := len(filtered) - 50
    if startIdx < 0 {
        startIdx = 0
    }

    for i := startIdx; i < len(filtered); i++ {
        result = append(result, filtered[i])
    }

    summary := fmt.Sprintf("Found %d logs. Showing latest up to 50.\n", len(filtered))
    summary += strings.Join(result, "\n")

    return summary, nil
}

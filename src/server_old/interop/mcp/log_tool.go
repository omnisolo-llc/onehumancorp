package mcp

import (
	"bufio"
	"context"
	"encoding/json"
	"fmt"
	"os"
	"strings"
	"time"
)

type LogAnalyzerTool struct {
	logFilePath string
}

func NewLogAnalyzerTool(logFilePath string) *LogAnalyzerTool {
	return &LogAnalyzerTool{logFilePath: logFilePath}
}

type LogEntry struct {
	Time  string `json:"time"`
	Level string `json:"level"`
	Msg   string `json:"msg"`
}

func (t *LogAnalyzerTool) Execute(ctx context.Context, level string, minutes int) (string, error) {
	file, err := os.Open(t.logFilePath)
	if err != nil {
		if os.IsNotExist(err) {
			return "No logs found.", nil
		}
		return "", fmt.Errorf("failed to open log file: %w", err)
	}
	defer file.Close()

	var results []string
	cutoffTime := time.Now().Add(-time.Duration(minutes) * time.Minute)

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()

		// Attempt to parse JSON log line (slog format)
		var entry LogEntry
		parsedJSON := false
		if err := json.Unmarshal([]byte(line), &entry); err == nil {
			logTime, err := time.Parse(time.RFC3339Nano, entry.Time)
			if err == nil {
				parsedJSON = true
				if logTime.After(cutoffTime) && (level == "" || strings.ToUpper(entry.Level) == strings.ToUpper(level)) {
					results = append(results, line)
				}
			}
		}

		if !parsedJSON {
            // Fallback for plain text lines for testing or simple logs
            if level == "" || strings.Contains(strings.ToUpper(line), strings.ToUpper(level)) {
                results = append(results, line)
            }
        }
	}

	if err := scanner.Err(); err != nil {
		return "", fmt.Errorf("error reading log file: %w", err)
	}

    // Keep only the last 50 lines if there are too many
    if len(results) > 50 {
        results = results[len(results)-50:]
    }

	if len(results) == 0 {
		return "No logs found matching criteria.", nil
	}

	return strings.Join(results, "\n"), nil
}

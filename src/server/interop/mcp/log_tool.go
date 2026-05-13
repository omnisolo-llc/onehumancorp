package mcp

import (
	"bufio"
	"context"
	"fmt"
	"os"
	"strings"
	"time"
)

type LogAnalyzerTool struct {
	LogFilePath string
}

func NewLogAnalyzerTool(path string) *LogAnalyzerTool {
	return &LogAnalyzerTool{LogFilePath: path}
}

func (t *LogAnalyzerTool) Execute(ctx context.Context, level string, minutes int) (string, error) {
	file, err := os.Open(t.LogFilePath)
	if err != nil {
		return "", fmt.Errorf("failed to open log file: %w", err)
	}
	defer file.Close()

	cutoff := time.Now().Add(-time.Duration(minutes) * time.Minute)

	// Use a circular buffer to only keep the last 50 lines in memory
	const maxLines = 50
	buffer := make([]string, 0, maxLines)

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()

		// Attempt to parse a timestamp if it's the first token (common log format)
		// E.g., "2023-10-27T10:00:00Z INFO Server started"
		parts := strings.SplitN(line, " ", 2)
		if len(parts) > 0 {
			if logTime, err := time.Parse(time.RFC3339, parts[0]); err == nil {
				if logTime.Before(cutoff) {
					continue // Skip old logs
				}
			}
		}

		if strings.Contains(line, level) {
			if len(buffer) < maxLines {
				buffer = append(buffer, line)
			} else {
				// Shift buffer and append (circular approach for simplicity on small n)
				copy(buffer, buffer[1:])
				buffer[len(buffer)-1] = line
			}
		}
	}

	if err := scanner.Err(); err != nil {
		return "", fmt.Errorf("error reading log file: %w", err)
	}

	if len(buffer) == 0 {
		return fmt.Sprintf("No logs found for level %s in the last %d minutes.", level, minutes), nil
	}

	return strings.Join(buffer, "\n"), nil
}

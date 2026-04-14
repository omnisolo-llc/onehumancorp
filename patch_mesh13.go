package main

import (
    "fmt"
    "io/ioutil"
    "strings"
)

func main() {
    content, err := ioutil.ReadFile("srcs/server/orchestration/mesh.go")
    if err != nil {
        fmt.Println("Error reading file:", err)
        return
    }

    strContent := string(content)

    badCode := `	err := meshWithRetry(ctx, 3, func() error {
		select {
		case lm.capsBroadcast[shardIdx] <- caps:
			return nil
		default:
			return fmt.Errorf("LocalTeammateMesh caps broadcast channel full")
	if err != nil {
		slog.Warn("LocalTeammateMesh caps broadcast channel full, dropping message after retries")
	}
	return nil
}`
    goodCode := `	err := meshWithRetry(ctx, 3, func() error {
		select {
		case lm.capsBroadcast[shardIdx] <- caps:
			return nil
		default:
			return fmt.Errorf("LocalTeammateMesh caps broadcast channel full")
		}
	})

	if err != nil {
		slog.Warn("LocalTeammateMesh caps broadcast channel full, dropping message after retries")
	}
	return nil
}`

    strContent = strings.Replace(strContent, badCode, goodCode, 1)

    err = ioutil.WriteFile("srcs/server/orchestration/mesh.go", []byte(strContent), 0644)
    if err != nil {
        fmt.Println("Error writing file:", err)
    }
}

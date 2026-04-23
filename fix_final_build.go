package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	content, err := ioutil.ReadFile("srcs/server/orchestration/autodream.go")
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	strContent := string(content)
	strContent = strings.Replace(strContent, `	rows, err := tx.Query(ctx, query, threshold)
	if errQuery != nil {
		slog.Error("AutoDream: failed to fetch stale sessions", "error", errQuery)
		return
	}`, `	rows, err := tx.Query(ctx, query, threshold)
	if err != nil {
		slog.Error("AutoDream: failed to fetch stale sessions", "error", err)
		return
	}`, 1)

	err = ioutil.WriteFile("srcs/server/orchestration/autodream.go", []byte(strContent), 0644)
	if err != nil {
		fmt.Println("Error writing file:", err)
		return
	}


	content2, err := ioutil.ReadFile("srcs/server/orchestration/autodream_worker.go")
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	strContent2 := string(content2)
	strContent2 = strings.Replace(strContent2, "autodream.BatchProcessingDuration", "autodream_metrics.BatchProcessingDuration", -1)
	strContent2 = strings.Replace(strContent2, "autodream.ConsolidationErrorsTotal", "autodream_metrics.ConsolidationErrorsTotal", -1)
	strContent2 = strings.Replace(strContent2, "autodream.MemoriesProcessedTotal", "autodream_metrics.MemoriesProcessedTotal", -1)

	strContent2 = strings.Replace(strContent2, "\"github.com/onehumancorp/mono/srcs/server/telemetry\"\n\t\"gopkg.in/yaml.v3\"", "autodream_metrics \"github.com/onehumancorp/mono/srcs/server/orchestration/autodream\"\n\t\"github.com/onehumancorp/mono/srcs/server/telemetry\"\n\t\"gopkg.in/yaml.v3\"", 1)

	err = ioutil.WriteFile("srcs/server/orchestration/autodream_worker.go", []byte(strContent2), 0644)
	if err != nil {
		fmt.Println("Error writing file:", err)
		return
	}

	fmt.Println("Files successfully patched")
}

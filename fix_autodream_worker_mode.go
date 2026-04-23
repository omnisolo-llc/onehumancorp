package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	content, err := ioutil.ReadFile("srcs/server/orchestration/autodream_worker.go")
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	strContent := string(content)

	// In ProcessMemories, add defer after mode and start are declared
	strContent = strings.Replace(strContent, `	mode := kairos.GetMode()
	start := time.Now()
	_ = mode
	_ = start`, `	mode := kairos.GetMode()
	start := time.Now()
	_ = mode
	_ = start

	defer func() {
		autodream.BatchProcessingDuration.WithLabelValues(mode, "ProcessMemories").Observe(time.Since(start).Seconds())
	}()`, 1)

	// Since we earlier tried to add the defer at the beginning where `mode` was undefined
	// we should remove the bad defer if it exists there
	strContent = strings.Replace(strContent, `func (w *AutoDreamWorker) ProcessMemories(ctx context.Context) error {
	defer func() {
		autodream.BatchProcessingDuration.WithLabelValues(mode, "ProcessMemories").Observe(time.Since(start).Seconds())
	}()`, `func (w *AutoDreamWorker) ProcessMemories(ctx context.Context) error {`, 1)


	err = ioutil.WriteFile("srcs/server/orchestration/autodream_worker.go", []byte(strContent), 0644)
	if err != nil {
		fmt.Println("Error writing file:", err)
		return
	}
	fmt.Println("File successfully patched")
}

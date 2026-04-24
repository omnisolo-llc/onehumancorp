package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
	b, err := ioutil.ReadFile("srcs/server/pipeline/autodream_pipeline.go")
	if err != nil {
		panic(err)
	}

	content := string(b)
	content = strings.ReplaceAll(content, "telemetry.RecordAutoDreamIngestionError(ctx, \"system\"", "telemetry.RecordAutoDreamIngestionError(ctx, \"sys\"")
	content = strings.ReplaceAll(content, "telemetry.RecordAutoDreamMemoryIngested(ctx, \"system\")", "telemetry.RecordAutoDreamMemoryIngested(ctx, \"sys\")")
	content = strings.ReplaceAll(content, "_, err = tx.Exec(ctx, insertQuery, id, \"system\", s.AgentID, summary, embeddingStr, \"session_compression\")", "_, err = tx.Exec(ctx, insertQuery, id, \"sys\", s.AgentID, summary, embeddingStr, \"session_compression\")")

	err = ioutil.WriteFile("srcs/server/pipeline/autodream_pipeline.go", []byte(content), 0644)
	if err != nil {
		panic(err)
	}
	fmt.Println("done")
}

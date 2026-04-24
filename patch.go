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
	content = strings.ReplaceAll(content, "_, err = p.pool.Exec(ctx, insertQuery, id, \"system\", \"system\", summary, embeddingStr, \"file_ingestion\")", "_, err = p.pool.Exec(ctx, insertQuery, id, \"sys\", \"sys\", summary, embeddingStr, \"file_ingestion\")")

	err = ioutil.WriteFile("srcs/server/pipeline/autodream_pipeline.go", []byte(content), 0644)
	if err != nil {
		panic(err)
	}
	fmt.Println("done")
}

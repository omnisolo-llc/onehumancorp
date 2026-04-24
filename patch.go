package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
    data, err := os.ReadFile("srcs/server/telemetry/buffer_pii_linter_test.go")
    if err != nil {
        panic(err)
    }
    content := string(data)

    // Check if the file is correctly updated.
    fmt.Println(strings.Contains(content, "hasRedact := false"))
}

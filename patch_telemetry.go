package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
    content, err := ioutil.ReadFile("srcs/server/telemetry/telemetry.go")
    if err != nil {
        panic(err)
    }

    contentStr := string(content)

    contentStr = strings.Replace(contentStr,
`	if telemetryEnabled && meshThroughput != nil {`,
`	if meshThroughput != nil {`, 1)

    ioutil.WriteFile("srcs/server/telemetry/telemetry.go", []byte(contentStr), 0644)
    fmt.Println("Patched telemetry.go again")
}

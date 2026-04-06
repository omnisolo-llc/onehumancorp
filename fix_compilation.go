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

    // Remove the bad code snippet
    badCode := `
var (
	meshThroughput, _ = meter.Int64Counter("ohc.mesh.throughput",
		metric.WithDescription("Total bytes transmitted through the mesh"),
		metric.WithUnit("By"),
	)
)

func RecordMeshThroughput(ctx context.Context, bytes int64) {
	if meshThroughput != nil {
		meshThroughput.Add(ctx, bytes)
	}
}
`
    contentStr = strings.Replace(contentStr, badCode, "", 1)

    // Add the correct variables
    varCode := `	latencyHistogram metric.Float64Histogram
	MeshLatencyRecorder metric.Float64Histogram`
    contentStr = strings.Replace(contentStr, varCode, varCode + "\n\tMeshThroughputRecorder metric.Int64Counter", 1)

    // Find where the recorders are initialized
    initCode := `	MeshLatencyRecorder, err = meter.Float64Histogram("ohc.mesh.latency.seconds",`
    contentStr = strings.Replace(contentStr, initCode, "	MeshThroughputRecorder, err = meter.Int64Counter(\"ohc.mesh.throughput\", metric.WithDescription(\"Total bytes transmitted through the mesh\"), metric.WithUnit(\"By\"))\n\tif err != nil {\n\t\tlog.Printf(\"Failed to create MeshThroughputRecorder: %v\", err)\n\t}\n" + initCode, 1)

    // Add the function back
    funcCode := `
func RecordMeshThroughput(ctx context.Context, bytes int64) {
	if MeshThroughputRecorder != nil {
		MeshThroughputRecorder.Add(ctx, bytes)
	}
}
`
    contentStr += funcCode

    ioutil.WriteFile("srcs/server/telemetry/telemetry.go", []byte(contentStr), 0644)
    fmt.Println("Fixed telemetry.go")
}

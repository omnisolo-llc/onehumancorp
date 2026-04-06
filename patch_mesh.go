package main

import (
	"fmt"
	"io/ioutil"
	"strings"
)

func main() {
    content, err := ioutil.ReadFile("srcs/server/orchestration/mesh.go")
    if err != nil {
        panic(err)
    }

    contentStr := string(content)

    if !strings.Contains(contentStr, "telemetry.RecordMeshLatency") {
        newContent := strings.ReplaceAll(contentStr,
`func (rm *RedisMeshTransport) BroadcastMeshEvent(ctx context.Context, topic string, payload []byte) error {
	cmd := rm.client.B().Publish().Channel("mesh:events:" + topic).Message(string(payload)).Build()`,
`func (rm *RedisMeshTransport) BroadcastMeshEvent(ctx context.Context, topic string, payload []byte) error {
	start := time.Now()
	defer func() {
		telemetry.RecordMeshLatency(ctx, "broadcast", time.Since(start))
		telemetry.RecordMeshThroughput(ctx, int64(len(payload)))
	}()
	cmd := rm.client.B().Publish().Channel("mesh:events:" + topic).Message(string(payload)).Build()`)
        ioutil.WriteFile("srcs/server/orchestration/mesh.go", []byte(newContent), 0644)
        fmt.Println("Patched mesh.go")
    } else {
        fmt.Println("Already patched")
    }
}

package main

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"flag"
	"fmt"
	"os"
	"time"

	"github.com/onehumancorp/mono/srcs/server/agents/local"
	agentruntime "github.com/onehumancorp/mono/srcs/server/agents/runtime"
)

func main() {
	var encoded string
	flag.StringVar(&encoded, "task-json-base64", os.Getenv("OHC_AGENT_TASK_JSON_BASE64"), "base64-encoded task request")
	flag.Parse()

	if encoded == "" {
		fatalf("missing --task-json-base64 or OHC_AGENT_TASK_JSON_BASE64")
	}

	request, err := decodeTaskRequest(encoded)
	if err != nil {
		fatalf("decode task request: %v", err)
	}

	state, err := local.SpawnTask(context.Background(), request.Description, request.Prompt, request.WorkDir, local.AgentConfig{})
	if err != nil {
		fatalf("spawn task: %v", err)
	}

	ticker := time.NewTicker(200 * time.Millisecond)
	defer ticker.Stop()

	for range ticker.C {
		status := state.Status()
		if !status.IsTerminal() {
			continue
		}

		fmt.Fprintf(os.Stdout, "agent_id=%s status=%s output=%s\n", request.AgentID, status, state.OutputFile)
		switch status {
		case local.TaskStatusCompleted:
			if result := state.Result(); result != "" {
				fmt.Fprintln(os.Stdout, result)
			}
			return
		case local.TaskStatusFailed:
			if errText := state.Err(); errText != "" {
				fmt.Fprintln(os.Stderr, errText)
			}
			os.Exit(1)
		case local.TaskStatusKilled:
			os.Exit(130)
		}
	}
}

func decodeTaskRequest(encoded string) (agentruntime.TaskRequest, error) {
	var request agentruntime.TaskRequest
	raw, err := base64.StdEncoding.DecodeString(encoded)
	if err != nil {
		return request, err
	}
	if err := json.Unmarshal(raw, &request); err != nil {
		return request, err
	}
	return request, nil
}

func fatalf(format string, args ...interface{}) {
	fmt.Fprintf(os.Stderr, format+"\n", args...)
	os.Exit(1)
}

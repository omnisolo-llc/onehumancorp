package main

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"flag"
	"fmt"
	"os"
	"os/signal"
	"syscall"

	"github.com/onehumancorp/mono/srcs/server/agents/builtin"
	"github.com/onehumancorp/mono/srcs/server/agents/builtinclient"
	agentruntime "github.com/onehumancorp/mono/srcs/server/agents/runtime"
)

func main() {
	ctx, stop := signal.NotifyContext(context.Background(), syscall.SIGINT, syscall.SIGTERM)
	defer stop()

	var encoded string
	var workerEncoded string
	flag.StringVar(&encoded, "task-json-base64", os.Getenv("OHC_AGENT_TASK_JSON_BASE64"), "base64-encoded task request")
	flag.StringVar(&workerEncoded, "worker-config-base64", os.Getenv("OHC_AGENT_WORKER_CONFIG_BASE64"), "base64-encoded worker config")
	flag.Parse()

	if workerEncoded != "" {
		cfg, err := builtin.DecodeWorkerConfig(workerEncoded)
		if err != nil {
			fatalf("decode worker config: %v", err)
		}
		adapter, err := builtin.NewGRPCHubAdapter(ctx, cfg.GetHubAddress())
		if err != nil {
			fatalf("connect hub: %v", err)
		}
		defer adapter.Close()

		runner := builtin.NewRunner(adapter, builtin.HubAgent{
			ID:             cfg.GetAgentId(),
			Name:           cfg.GetAgentName(),
			Role:           cfg.GetRole(),
			OrganizationID: cfg.GetOrganizationId(),
			ProviderType:   cfg.GetProviderType(),
			Region:         cfg.GetRegion(),
			Managed:        true,
		}, cfg.GetBuiltinAddress())
		runner.Start(ctx)
		return
	}

	if encoded == "" {
		fatalf("missing --task-json-base64 or --worker-config-base64")
	}

	request, err := decodeTaskRequest(encoded)
	if err != nil {
		fatalf("decode task request: %v", err)
	}

	client, err := builtinclient.DialContext(ctx, builtinclient.AddressFromEnv())
	if err != nil {
		fatalf("connect builtin agent: %v", err)
	}
	defer client.Close()

	taskText := request.Prompt
	if taskText == "" {
		taskText = request.Description
	}

	result, err := client.RunTask(ctx, builtinclient.RunTaskRequest{Task: taskText}, nil)
	if err != nil {
		if ctx.Err() != nil {
			os.Exit(130)
		}
		fatalf("run task: %v", err)
	}

	fmt.Fprintf(os.Stdout, "agent_id=%s status=completed\n", request.AgentID)
	if result != "" {
		fmt.Fprintln(os.Stdout, result)
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

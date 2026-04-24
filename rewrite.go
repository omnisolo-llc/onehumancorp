package main

import (
	"io/ioutil"
	"strings"
)

func main() {
	content, _ := ioutil.ReadFile("src/server/agents/agent_task_worker.go")
	text := string(content)

	// Add imports
	importBlock := `
	agentservicepb "github.com/onehumancorp/mono/src/proto/agentservice"
	agentgrpc "github.com/onehumancorp/mono/src/server/agents/grpc"
	"github.com/onehumancorp/mono/src/server/integrations/plane"
	"github.com/onehumancorp/mono/src/server/orchestration"
)`

	newImportBlock := `
	agentservicepb "github.com/onehumancorp/mono/src/proto/agentservice"
	"github.com/onehumancorp/mono/src/server/integrations/plane"
	"github.com/onehumancorp/mono/src/server/orchestration"
	"github.com/redis/rueidis"
	"github.com/google/uuid"
	"google.golang.org/protobuf/proto"
)`
	text = strings.Replace(text, importBlock, newImportBlock, 1)

	oldFunc := `// dispatchToBuiltinAgent sends a task to the builtin Rust agent gRPC service.
// The Rust binary must be running and reachable at OHC_AGENT_ADDRESS
// (default: 127.0.0.1:50051). It exposes the AgentService gRPC interface.
func dispatchToBuiltinAgent(payload, description, role string) error {
	address := os.Getenv("OHC_AGENT_ADDRESS")
	if address == "" {
		address = "127.0.0.1:50051"
	}
	client, err := agentgrpc.NewClient(address, agentgrpc.ClientOptionsFromEnv())
	if err != nil {
		return fmt.Errorf("connect to builtin agent at %s: %w", address, err)
	}
	defer client.Close() //nolint:errcheck

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Minute)
	defer cancel()

	var lastContent string
	err = client.RunTask(ctx, &agentservicepb.RunTaskRequest{
		Task:       payload,
		Department: role,
	}, func(evt *agentservicepb.RunTaskEvent) {
		if evt.Content != "" {
			lastContent = evt.Content
		}
	})
	if err != nil {
		return fmt.Errorf("builtin agent RunTask: %w", err)
	}
	slog.Info("builtin agent task completed", "description", description, "result_len", len(lastContent))
	return nil
}`

	newFunc := `// dispatchToBuiltinAgent sends a task to the builtin Rust agent asynchronously via Redis pub/sub.
func dispatchToBuiltinAgent(payload, description, role string) error {
	redisURL := os.Getenv("OHC_REDIS_URL")
	if redisURL == "" {
		redisURL = os.Getenv("REDIS_URL")
	}
	if redisURL == "" {
		redisURL = "redis://127.0.0.1:6379"
	}

	opts, err := rueidis.ParseURL(redisURL)
	if err != nil {
		return fmt.Errorf("parse redis url: %w", err)
	}

	client, err := rueidis.NewClient(opts)
	if err != nil {
		return fmt.Errorf("connect to redis: %w", err)
	}
	defer client.Close()

	req := &agentservicepb.RunTaskRequest{
		TaskId:     uuid.New().String(),
		Task:       payload,
		Department: role,
	}

	b, err := proto.Marshal(req)
	if err != nil {
		return fmt.Errorf("marshal RunTaskRequest: %w", err)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	cmd := client.B().Publish().Channel("agent_jobs").Message(rueidis.BinaryString(b)).Build()
	if err := client.Do(ctx, cmd).Error(); err != nil {
		return fmt.Errorf("publish to agent_jobs: %w", err)
	}

	slog.Info("builtin agent task dispatched to redis", "description", description, "task_id", req.TaskId)
	return nil
}`

	text = strings.Replace(text, oldFunc, newFunc, 1)

	ioutil.WriteFile("src/server/agents/agent_task_worker.go", []byte(text), 0644)
}

package harness

import (
	"context"
	"fmt"
)

type Executor interface {
	Execute(ctx context.Context, cmd string, customEnv []string) (string, error)
}

type Orchestrator struct {
	ProxyPort int
	Executor  Executor
	Telemetry SandboxTelemetryEmitter
}

func (o *Orchestrator) SpawnSubAgent(ctx context.Context, cmd string) error {
	proxyEnvStr := fmt.Sprintf("http://127.0.0.1:%d", o.ProxyPort)

	var executor Executor = o.Executor
	if executor == nil {
		executor = NewBwrapExecutor(o.Telemetry)
	}

	customEnv := []string{
		fmt.Sprintf("HTTP_PROXY=%s", proxyEnvStr),
		fmt.Sprintf("HTTPS_PROXY=%s", proxyEnvStr),
	}

	output, err := executor.Execute(ctx, cmd, customEnv)
	if err != nil {
		return fmt.Errorf("failed to execute sub-agent: %w", err)
	}

	fmt.Printf("Sub-agent execution output: %s\n", output)
	return nil
}

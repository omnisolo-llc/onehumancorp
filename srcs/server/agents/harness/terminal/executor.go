package terminal


import (
	"context"
	"fmt"
	"os/exec"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/agents/harness"
	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
)

type Executor struct {
	harness   harness.IsolationHarness
	validator CommandValidator
}

func NewExecutor(h harness.IsolationHarness) *Executor {
	return &Executor{
		harness:   h,
		validator: NewDefaultCommandValidator(),
	}
}

func NewExecutorWithValidator(h harness.IsolationHarness, v CommandValidator) *Executor {
	return &Executor{
		harness:   h,
		validator: v,
	}
}

func (e *Executor) ExecuteCommand(ctx context.Context, cmd string) ([]byte, error) {
	if err := e.validator.Validate(cmd); err != nil {
		return nil, err
	}


	// Get AgentContext to retrieve the proxy configuration
	var networkProxy string
	if ac, ok := orchestration.GetAgentContext(ctx); ok {
		if proxy, exists := ac.Env["HTTP_PROXY"]; exists {
			networkProxy = proxy
		}
	}

	tenantID := auth.OrganizationIDFromContext(ctx)
	if tenantID == "" {
		tenantID = "unknown"
	}

	// Extract a short prefix for the command (e.g. first word) for metrics
	cmdFields := strings.Fields(cmd)
	commandPrefix := "unknown"
	if len(cmdFields) > 0 {
		commandPrefix = cmdFields[0]
	}

	tracer := otel.Tracer("harness")
	ctx, span := tracer.Start(ctx, "ExecuteCommand")
	defer span.End()

	span.SetAttributes(
		attribute.String("tenant_id", tenantID),
		attribute.String("command_prefix", commandPrefix),
	)

	start := time.Now()
	stdout, stderr, err := e.harness.Execute(ctx, harness.ExecutionContext{
		Command: []string{"/bin/sh", "-c", cmd},
		NetworkProxy: networkProxy,
	})
	duration := time.Since(start).Seconds()

	exitCodeStr := "0"
	if err != nil {
		if exitErr, ok := err.(*exec.ExitError); ok {
			exitCodeStr = fmt.Sprintf("%d", exitErr.ExitCode())
		} else {
			exitCodeStr = "-1"
		}
	}

	span.SetAttributes(attribute.String("exit_code", exitCodeStr))

	telemetry.RecordHarnessCommandDuration(ctx, duration, tenantID, commandPrefix, exitCodeStr)

	if len(stdout) > 0 {
		telemetry.RecordHarnessIOBytes(ctx, int64(len(stdout)), tenantID, "stdout")
	}
	if len(stderr) > 0 {
		telemetry.RecordHarnessIOBytes(ctx, int64(len(stderr)), tenantID, "stderr")
	}

	var combined []byte
	combined = append(combined, stdout...)
	combined = append(combined, stderr...)

	return combined, err
}

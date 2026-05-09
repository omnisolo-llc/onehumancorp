package harness

import (
	"bytes"
	"context"
	"os/exec"
	"strings"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/trace"
	"onehumancorp/srcs/server/telemetry"
)

type AttemptResult struct {
	Stdout    string
	Stderr    string
	ExitCode  int
	Compacted bool
}

type AgentHarness interface {
	RunAttempt(ctx context.Context, cmd string) (*AttemptResult, error)
	Compact() error
	Reset() error
}

type AssistantAgentHarness struct {
	manager *AssistantSandboxManager
}

func NewAssistantAgentHarness(policyJSON string) (AgentHarness, error) {
	manager := NewAssistantSandboxManager()
	if policyJSON != "" {
		if err := manager.UpdateConfig(policyJSON); err != nil {
			return nil, err
		}
	}
	return &AssistantAgentHarness{manager: manager}, nil
}

func (h *AssistantAgentHarness) RunAttempt(ctx context.Context, cmd string) (*AttemptResult, error) {
	tracer := otel.Tracer("harness")
	ctx, span := tracer.Start(ctx, "LocalShellTask", trace.WithAttributes(
		attribute.String("command", cmd),
	))
	defer span.End()

	wrappedCmd, err := h.manager.WrapCommand(cmd)
	if err != nil {
		span.RecordError(err)
		return nil, err
	}

	c := exec.Command("sh", "-c", wrappedCmd)
	var out bytes.Buffer
	var stderr bytes.Buffer
	c.Stdout = &out
	c.Stderr = &stderr

	start := time.Now()
	err = c.Run()
	exitCode := 0
	if err != nil {
		span.RecordError(err)
		if exitErr, ok := err.(*exec.ExitError); ok {
			exitCode = exitErr.ExitCode()
		} else {
			exitCode = -1
		}
	}
	durationSecs := time.Since(start).Seconds()

	commandPrefix := ""
	parts := strings.Split(strings.TrimSpace(cmd), " ")
	if len(parts) > 0 {
		commandPrefix = parts[0]
	}

	_ = telemetry.RecordHarnessCommandDuration(ctx, durationSecs, "default", commandPrefix, exitCode)
	_ = telemetry.RecordHarnessIOBytes(ctx, int64(out.Len()), "default", "stdout")
	_ = telemetry.RecordHarnessIOBytes(ctx, int64(stderr.Len()), "default", "stderr")

	span.SetAttributes(
		attribute.Int("exit_code", exitCode),
		attribute.Int64("stdout_bytes", int64(out.Len())),
		attribute.Int64("stderr_bytes", int64(stderr.Len())),
	)

	return &AttemptResult{
		Stdout:   out.String(),
		Stderr:   stderr.String(),
		ExitCode: exitCode,
	}, nil
}

func (h *AssistantAgentHarness) Compact() error {
	return nil
}

func (h *AssistantAgentHarness) Reset() error {
	return nil
}

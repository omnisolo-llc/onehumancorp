package harness

import (
	"bytes"
	"context"
	"os/exec"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

type SandboxConfig struct {
	AllowedDomains []string
	DeniedDomains  []string
	ReadPaths      []string
	WritePaths     []string
	EnableSeccomp  bool
}

type Harness struct {
	config *SandboxConfig
	meter  metric.Meter
	execs  metric.Int64Counter
}

func NewHarness(config *SandboxConfig) *Harness {
	meter := otel.GetMeterProvider().Meter("harness")
	execs, _ := meter.Int64Counter("ohc_harness_executions_total", metric.WithDescription("Total number of harness executions"))

	return &Harness{
		config: config,
		meter:  meter,
		execs:  execs,
	}
}

func (h *Harness) Run(ctx context.Context, cmd string, args ...string) (string, string, error) {
	tracer := otel.Tracer("harness")
	ctx, span := tracer.Start(ctx, "Harness.Run")
	defer span.End()

	span.SetAttributes(attribute.String("cmd", cmd))
	span.SetAttributes(attribute.StringSlice("args", args))

	// Track execution
	h.execs.Add(ctx, 1)

	bwrapArgs := []string{}

	for _, rp := range h.config.ReadPaths {
		bwrapArgs = append(bwrapArgs, "--ro-bind", rp, rp)
	}
	for _, wp := range h.config.WritePaths {
		bwrapArgs = append(bwrapArgs, "--bind", wp, wp)
	}

	bwrapArgs = append(bwrapArgs, "--unshare-all", "--share-net", "--die-with-parent", "--dir", "/tmp")

	if h.config.EnableSeccomp {
		// Mock implementation - in a real scenario we'd bind a seccomp bpf file here
		bwrapArgs = append(bwrapArgs, "--unshare-user")
	}

	bwrapArgs = append(bwrapArgs, cmd)
	bwrapArgs = append(bwrapArgs, args...)

	execCmd := exec.CommandContext(ctx, "bwrap", bwrapArgs...)

	var stdout, stderr bytes.Buffer
	execCmd.Stdout = &stdout
	execCmd.Stderr = &stderr

	err := execCmd.Run()

	exitCode := 0
	if err != nil {
		if exitErr, ok := err.(*exec.ExitError); ok {
			exitCode = exitErr.ExitCode()
		} else {
			exitCode = -1
		}
	}
	span.SetAttributes(attribute.Int("exit_code", exitCode))

	return stdout.String(), stderr.String(), err
}

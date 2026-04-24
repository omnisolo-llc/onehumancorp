package harness

import (
	"context"
	"fmt"
	"os/exec"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/codes"
	"go.opentelemetry.io/otel/metric"
	"go.opentelemetry.io/otel/trace"
)

var (
	tracer = otel.Tracer("ohc/backend/harness")
	meter  = otel.Meter("ohc/backend/harness")

	executionsTotal metric.Int64Counter
)

func init() {
	var err error
	executionsTotal, err = meter.Int64Counter(
		"ohc_harness_executions_total",
		metric.WithDescription("Total number of harness executions"),
	)
	if err != nil {
		panic(fmt.Sprintf("failed to initialize metric: %v", err))
	}
}

// SandboxConfig defines the configuration for the bwrap sandbox.
type SandboxConfig struct {
	AllowedDomains []string
	DeniedDomains  []string
	ReadPaths      []string
	WritePaths     []string
	EnableSeccomp  bool
}

// Harness wraps the execution using bwrap.
type Harness struct {
	config *SandboxConfig
}

// NewHarness creates a new Harness instance.
func NewHarness(config *SandboxConfig) *Harness {
	return &Harness{
		config: config,
	}
}

// Run executes the command via bwrap, enforcing the read/write paths defined in the config.
func (h *Harness) Run(cmd string, args []string) {
	proxy := NewProxyServer(h.config.DeniedDomains)

	started := make(chan struct{})

	// Start proxy on a local port dynamically
	go func() {
		// Use port 0 to get an ephemeral port
		err := proxy.Start("127.0.0.1:0", started)
		if err != nil {
			// ProxyStart already closes started channel on success,
			// but we need to ensure it's closed on failure.
			// However we shouldn't close it twice. proxy.Start won't close if Listen fails.
			// Listen fails -> err != nil and channel is NOT closed.
			// Let's rely on proxy.Start logic to close it.
			select {
			case <-started:
			default:
				close(started)
			}
		}
	}()

	// Wait for the proxy to bind
	<-started

	defer proxy.Stop(context.Background())

	ctx := context.Background()
	ctx, span := tracer.Start(ctx, "Harness.Run", trace.WithAttributes(
		attribute.String("command", cmd),
		attribute.StringSlice("args", args),
	))
	defer span.End()

	executionsTotal.Add(ctx, 1)

	var bwrapArgs []string

	// Add read-only bind mounts
	for _, rp := range h.config.ReadPaths {
		bwrapArgs = append(bwrapArgs, "--ro-bind", rp, rp)
	}

	// Add write bind mounts
	for _, wp := range h.config.WritePaths {
		bwrapArgs = append(bwrapArgs, "--bind", wp, wp)
	}

	// Unshare all namespaces except net (need it to reach proxy)
	bwrapArgs = append(bwrapArgs, "--unshare-user", "--unshare-ipc", "--unshare-pid", "--unshare-uts", "--unshare-cgroup")

	if h.config.EnableSeccomp {
		// Mock applying seccomp rules for demonstration.
		// In a real harness, this would map to a generated BPF program.
		bwrapArgs = append(bwrapArgs, "--seccomp", "9") // fd 9 is a placeholder
	}

	// Setup a new dev
	bwrapArgs = append(bwrapArgs, "--dev", "/dev")

	// Inject proxy settings
	bwrapArgs = append(bwrapArgs, "--setenv", "HTTP_PROXY", "http://"+proxy.GetAddr())
	bwrapArgs = append(bwrapArgs, "--setenv", "HTTPS_PROXY", "http://"+proxy.GetAddr())

	bwrapArgs = append(bwrapArgs, cmd)
	bwrapArgs = append(bwrapArgs, args...)

	execCmd := exec.CommandContext(ctx, "bwrap", bwrapArgs...)

	err := execCmd.Run()

	if err != nil {
		span.RecordError(err)
		span.SetStatus(codes.Error, err.Error())
		if exitError, ok := err.(*exec.ExitError); ok {
			span.SetAttributes(attribute.Int("exit_code", exitError.ExitCode()))
		} else {
			span.SetAttributes(attribute.Int("exit_code", -1))
		}
	} else {
		span.SetStatus(codes.Ok, "success")
		span.SetAttributes(attribute.Int("exit_code", 0))
	}
}

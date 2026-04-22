package sandbox

import (
	"context"
	"errors"
	"os"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/harness"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter               = otel.Meter("github.com/onehumancorp/mono/srcs/server/harness/sandbox")
	violationsMetric    metric.Int64Counter
	wrappedExecsMetric  metric.Int64Counter
)

func init() {
	var err error
	violationsMetric, err = meter.Int64Counter("harness.sandbox.violations", metric.WithDescription("Number of sandbox violations"))
	if err != nil {
		panic(err)
	}
	wrappedExecsMetric, err = meter.Int64Counter("harness.sandbox.wrapped_executions", metric.WithDescription("Number of wrapped executions"))
	if err != nil {
		panic(err)
	}
}

type SandboxManager struct {
	config              harness.Config
	permissionEvaluator *PermissionEvaluator
}

func NewSandboxManager(config harness.Config, permissionEvaluator *PermissionEvaluator) *SandboxManager {
	if permissionEvaluator == nil {
		// Read from env if not provided
		disabledEnv := os.Getenv("OOS_SANDBOX_DISABLED_COMMANDS")
		var disabledCmds []string
		if disabledEnv != "" {
			disabledCmds = strings.Split(disabledEnv, ",")
		}

		policyEnv := os.Getenv("OOS_SANDBOX_POLICY")
		var allowedRegexes []string
		if policyEnv != "" {
			allowedRegexes = strings.Split(policyEnv, ",")
		}

		permissionEvaluator = NewPermissionEvaluator(disabledCmds, allowedRegexes)
	}
	return &SandboxManager{
		config:              config,
		permissionEvaluator: permissionEvaluator,
	}
}

func (s *SandboxManager) Execute(ctx context.Context, command string) (harness.Result, error) {
	// ... we will implement this according to interface
	return harness.Result{}, nil
}

func (s *SandboxManager) WrapCommand(ctx context.Context, cmd string) (string, error) {
	if !s.permissionEvaluator.IsAllowed(cmd) {
		violationsMetric.Add(ctx, 1)
		return "", errors.New("command not allowed by sandbox policy")
	}

	wrappedExecsMetric.Add(ctx, 1)
	return WrapCommand(cmd), nil
}

func (s *SandboxManager) AnnotateError(err error, stdout string) string {
	if err != nil {
		return "Sandbox Violation: " + err.Error() + "\nStdout: " + stdout
	}
	return stdout
}

package orchestration

import (
	"context"
	"fmt"
	"strings"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	harnessMeter   = otel.Meter("ohc_hybrid_harness")
	violationCount metric.Int64Counter
	execCount      metric.Int64Counter
)

func init() {
	var err error
	violationCount, err = harnessMeter.Int64Counter("ohc_sandbox_violation_total",
		metric.WithDescription("Total number of bash sandbox security violations via AST validator"))
	if err != nil {
		panic(err)
	}

	execCount, err = harnessMeter.Int64Counter("ohc_sandbox_exec_total",
		metric.WithDescription("Total number of bash sandbox execution attempts via harness"))
	if err != nil {
		panic(err)
	}
}

// SandboxManager intercepts agent tool invocations before execution.
type SandboxManager interface {
	ValidateContext(ctx context.Context, command string) error
	ExecuteContext(ctx context.Context, command string, workDir string) (string, error)
}

// SPIFFEContextKey is the context key for SPIFFE identity.
type SPIFFEContextKey struct{}

// BashASTValidator parses command strings and rejects prohibited patterns.
type BashASTValidator struct {
}

// NewBashASTValidator creates a new BashASTValidator.
func NewBashASTValidator() *BashASTValidator {
	return &BashASTValidator{}
}

// ValidateContext validates the bash command using simple parsing.
func (v *BashASTValidator) ValidateContext(ctx context.Context, command string) error {
	spiffeID := ctx.Value(SPIFFEContextKey{})
	if spiffeID == nil || spiffeID == "" {
		return fmt.Errorf("execution blocked: valid SPIFFE identity required")
	}

	cmd := strings.ToLower(command)
	if strings.Contains(cmd, "> /etc") || strings.Contains(cmd, "> /") {
		violationCount.Add(ctx, 1)
		return fmt.Errorf("prohibited redirect target")
	}

	if strings.Contains(cmd, "sudo ") || strings.HasPrefix(cmd, "sudo") {
		violationCount.Add(ctx, 1)
		return fmt.Errorf("prohibited command usage")
	}

	if strings.Contains(cmd, "chown ") || strings.HasPrefix(cmd, "chown") {
		violationCount.Add(ctx, 1)
		return fmt.Errorf("prohibited command usage")
	}

	if strings.Contains(cmd, "rm -rf /") {
		violationCount.Add(ctx, 1)
		return fmt.Errorf("prohibited rm target")
	}

	if strings.Contains(cmd, ">(") || strings.Contains(cmd, "<(") {
		violationCount.Add(ctx, 1)
		return fmt.Errorf("process substitution is prohibited")
	}

	return nil
}

// ExecuteContext executes the command if validation passes.
func (v *BashASTValidator) ExecuteContext(ctx context.Context, command string, workDir string) (string, error) {
	execCount.Add(ctx, 1)

	if err := v.ValidateContext(ctx, command); err != nil {
		return "", err
	}

	return "Execution Simulated", nil
}

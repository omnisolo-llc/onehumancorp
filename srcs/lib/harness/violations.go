package harness

import (
	"context"
	"fmt"
	"strings"
	"sync"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
	"go.opentelemetry.io/otel/attribute"
)

var (
	meter          = otel.Meter("ohc_agent_harness")
	violationCount metric.Int64Counter
)

func init() {
	var err error
	violationCount, err = meter.Int64Counter("harness.violation",
		metric.WithDescription("Total number of sandbox violations prevented by AST validation or bwrap policies"))
	if err != nil {
		panic(err)
	}
}

type Violation struct {
	Command string
	Error   string
	Time    time.Time
}

type ViolationStore struct {
	mu         sync.Mutex
	violations []Violation
}

func NewViolationStore() *ViolationStore {
	return &ViolationStore{
		violations: make([]Violation, 0),
	}
}

func (s *ViolationStore) RecordViolation(ctx context.Context, cmd, errStr string) {
	s.mu.Lock()
	defer s.mu.Unlock()
	s.violations = append(s.violations, Violation{
		Command: cmd,
		Error:   errStr,
		Time:    time.Now(),
	})
	violationCount.Add(ctx, 1, metric.WithAttributes(attribute.String("error", errStr)))
}

func (s *ViolationStore) GetViolations() []Violation {
	s.mu.Lock()
	defer s.mu.Unlock()
	res := make([]Violation, len(s.violations))
	copy(res, s.violations)
	return res
}

func AnnotateStderrWithSandboxFailures(stderr string, failures []Violation) string {
	if len(failures) == 0 {
		return stderr
	}
	var sb strings.Builder
	sb.WriteString(stderr)
	if len(stderr) > 0 && !strings.HasSuffix(stderr, "\n") {
		sb.WriteString("\n")
	}
	sb.WriteString("<sandbox_violations>\n")
	for _, f := range failures {
		sb.WriteString(fmt.Sprintf("[%s] %s: %s\n", f.Time.Format(time.RFC3339), f.Command, f.Error))
	}
	sb.WriteString("</sandbox_violations>\n")
	return sb.String()
}

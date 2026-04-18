1. **Add `HarnessExecutionDuration` metric to `srcs/server/telemetry/telemetry.go`:**
   - Use `replace_with_git_merge_diff` to modify `telemetry.go`.
   - SEARCH block:
     ```
<<<<<<< SEARCH
	autoDreamSyncDuration  metric.Float64Histogram
	autoDreamQueryDuration metric.Float64Histogram
	meshBroadcastTotal     metric.Int64Counter
=======
	autoDreamSyncDuration    metric.Float64Histogram
	autoDreamQueryDuration   metric.Float64Histogram
	HarnessExecutionDuration metric.Float64Histogram
	meshBroadcastTotal       metric.Int64Counter
>>>>>>> REPLACE
     ```
   - SEARCH block:
     ```
<<<<<<< SEARCH
	autoDreamQueryDuration, err = m.Float64Histogram(
		"ohc_autodream_query_duration_seconds",
		metric.WithDescription("Latency of AutoDream query operations in seconds"),
		metric.WithUnit("s"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	meshBroadcastTotal, err = m.Int64Counter(
=======
	autoDreamQueryDuration, err = m.Float64Histogram(
		"ohc_autodream_query_duration_seconds",
		metric.WithDescription("Latency of AutoDream query operations in seconds"),
		metric.WithUnit("s"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	HarnessExecutionDuration, err = m.Float64Histogram(
		"ohc_harness_execution_duration_seconds",
		metric.WithDescription("Latency of Harness execution in seconds"),
		metric.WithUnit("s"),
	)
	if err != nil {
		errs = append(errs, err)
	}

	meshBroadcastTotal, err = m.Int64Counter(
>>>>>>> REPLACE
     ```
   - SEARCH block:
     ```
<<<<<<< SEARCH
func RecordAutoDreamQueryLatency(ctx context.Context, latency float64, mode string) {
	if autoDreamQueryDuration != nil {
		autoDreamQueryDuration.Record(ctx, latency, metric.WithAttributes(
			attribute.String("deployment_mode", mode),
		))
	}
}

// RecordSIPSyncLatency records the latency of synchronization.
=======
func RecordAutoDreamQueryLatency(ctx context.Context, latency float64, mode string) {
	if autoDreamQueryDuration != nil {
		autoDreamQueryDuration.Record(ctx, latency, metric.WithAttributes(
			attribute.String("deployment_mode", mode),
		))
	}
}

// RecordHarnessExecutionDuration records the duration of harness execution.
func RecordHarnessExecutionDuration(ctx context.Context, duration float64, mode string, harnessType string) {
	if HarnessExecutionDuration != nil {
		HarnessExecutionDuration.Record(ctx, duration, metric.WithAttributes(
			attribute.String("deployment_mode", mode),
			attribute.String("harness_type", harnessType),
		))
	}
}

// RecordSIPSyncLatency records the latency of synchronization.
>>>>>>> REPLACE
     ```

2. **Add telemetry recording to Harness Implementations in `srcs/server/agent/harness/`:**
   - Use `replace_with_git_merge_diff` on `srcs/server/agent/harness/bwrap_linux.go`.
     ```
<<<<<<< SEARCH
import (
	"context"
	"os/exec"
)

type BwrapHarness struct{}
=======
import (
	"context"
	"os"
	"os/exec"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type BwrapHarness struct{}
>>>>>>> REPLACE
     ```
     ```
<<<<<<< SEARCH
func (h *BwrapHarness) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error) {
	args := []string{
=======
func (h *BwrapHarness) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error) {
	start := time.Now()
	defer func() {
		mode := "standalone"
		if os.Getenv("OHC_MULTITENANT") == "true" {
			mode = "cloud"
		}
		telemetry.RecordHarnessExecutionDuration(ctx, time.Since(start).Seconds(), mode, "bwrap")
	}()

	args := []string{
>>>>>>> REPLACE
     ```
   - Use `replace_with_git_merge_diff` on `srcs/server/agent/harness/sandbox_darwin.go`.
     ```
<<<<<<< SEARCH
import (
	"context"
	"os/exec"
)

type SandboxHarness struct{}
=======
import (
	"context"
	"os"
	"os/exec"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type SandboxHarness struct{}
>>>>>>> REPLACE
     ```
     ```
<<<<<<< SEARCH
func (h *SandboxHarness) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error) {
	profile := "(version 1)
(deny default)
(allow process-exec)
"
=======
func (h *SandboxHarness) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error) {
	start := time.Now()
	defer func() {
		mode := "standalone"
		if os.Getenv("OHC_MULTITENANT") == "true" {
			mode = "cloud"
		}
		telemetry.RecordHarnessExecutionDuration(ctx, time.Since(start).Seconds(), mode, "sandbox_darwin")
	}()

	profile := "(version 1)
(deny default)
(allow process-exec)
"
>>>>>>> REPLACE
     ```
   - Use `replace_with_git_merge_diff` on `srcs/server/agent/harness/fallback.go`.
     ```
<<<<<<< SEARCH
import (
	"context"
	"os/exec"
)

type FallbackHarness struct{}
=======
import (
	"context"
	"os"
	"os/exec"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type FallbackHarness struct{}
>>>>>>> REPLACE
     ```
     ```
<<<<<<< SEARCH
func (h *FallbackHarness) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error) {
	cmd := exec.CommandContext(ctx, execCtx.Command[0], execCtx.Command[1:]...)
	return cmd.CombinedOutput()
}
=======
func (h *FallbackHarness) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error) {
	start := time.Now()
	defer func() {
		mode := "standalone"
		if os.Getenv("OHC_MULTITENANT") == "true" {
			mode = "cloud"
		}
		telemetry.RecordHarnessExecutionDuration(ctx, time.Since(start).Seconds(), mode, "fallback")
	}()

	cmd := exec.CommandContext(ctx, execCtx.Command[0], execCtx.Command[1:]...)
	return cmd.CombinedOutput()
}
>>>>>>> REPLACE
     ```
   - Use `replace_with_git_merge_diff` on `srcs/server/agent/harness/BUILD.bazel`.
     ```
<<<<<<< SEARCH
    importpath = "github.com/onehumancorp/mono/srcs/server/agent/harness",
    visibility = ["//visibility:public"],
)
=======
    importpath = "github.com/onehumancorp/mono/srcs/server/agent/harness",
    visibility = ["//visibility:public"],
    deps = [
        "//srcs/server/telemetry",
    ],
)
>>>>>>> REPLACE
     ```

3. **Add tests for the new metric in telemetry:**
   - Use `replace_with_git_merge_diff` on `srcs/server/telemetry/telemetry_extra_test.go`.
     ```
<<<<<<< SEARCH
	origTaskFail := taskFailedCounter
	origCacheMiss := cacheMissesCounter

	// Nullify all
=======
	origTaskFail := taskFailedCounter
	origCacheMiss := cacheMissesCounter
	origHarnessExecutionDuration := HarnessExecutionDuration

	// Nullify all
>>>>>>> REPLACE
     ```
     ```
<<<<<<< SEARCH
	taskFailedCounter = nil
	cacheMissesCounter = nil

	defer func() {
=======
	taskFailedCounter = nil
	cacheMissesCounter = nil
	HarnessExecutionDuration = nil

	defer func() {
>>>>>>> REPLACE
     ```
     ```
<<<<<<< SEARCH
		taskFailedCounter = origTaskFail
		cacheMissesCounter = origCacheMiss
	}()

	RecordAgentApiError(ctx, "agent-1", "dev", "api-1")
=======
		taskFailedCounter = origTaskFail
		cacheMissesCounter = origCacheMiss
		HarnessExecutionDuration = origHarnessExecutionDuration
	}()

	RecordAgentApiError(ctx, "agent-1", "dev", "api-1")
>>>>>>> REPLACE
     ```
     ```
<<<<<<< SEARCH
	RecordTaskFailed(ctx, "task1", "err1")
	RecordCacheMiss(ctx, "op1", "type1")
}

func TestRecordOtherMetricsUninitialized(t *testing.T) {
=======
	RecordTaskFailed(ctx, "task1", "err1")
	RecordCacheMiss(ctx, "op1", "type1")
	RecordHarnessExecutionDuration(ctx, 1.5, "cloud", "bwrap")
}

func TestRecordOtherMetricsUninitialized(t *testing.T) {
>>>>>>> REPLACE
     ```
     ```
<<<<<<< SEARCH
	RecordTaskFailed(ctx, "task1", "err1")
	RecordCacheMiss(ctx, "op1", "type1")
}

func TestLogAgentExecutionFallback(t *testing.T) {
=======
	RecordTaskFailed(ctx, "task1", "err1")
	RecordCacheMiss(ctx, "op1", "type1")
	RecordHarnessExecutionDuration(ctx, 1.5, "cloud", "bwrap")
}

func TestLogAgentExecutionFallback(t *testing.T) {
>>>>>>> REPLACE
     ```

4. **Verify tests pass:**
   - Run `bazelisk test //srcs/server/telemetry/...` and `bazelisk test //srcs/server/agent/harness/...` using `run_in_bash_session` to ensure no regression and the new code paths are valid.

5. **Ensure proper testing, verification, review, and reflection are done:**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

6. **Submit the change:**
   - Submit the change with issue_id: 5457 tracking included.

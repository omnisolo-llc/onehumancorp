1. **Add `HarnessExecutionDuration` metric to `srcs/server/telemetry/telemetry.go`:**
   - Use `replace_with_git_merge_diff` to modify `telemetry.go`.
   - SEARCH block:
     ```
	autoDreamSyncDuration  metric.Float64Histogram
	autoDreamQueryDuration metric.Float64Histogram
     ```
   - REPLACE block:
     ```
	autoDreamSyncDuration  metric.Float64Histogram
	autoDreamQueryDuration metric.Float64Histogram
	HarnessExecutionDuration metric.Float64Histogram
     ```
   - SEARCH block:
     ```
	autoDreamQueryDuration, err = m.Float64Histogram(
		"ohc_autodream_query_duration_seconds",
		metric.WithDescription("Latency of AutoDream query operations in seconds"),
		metric.WithUnit("s"),
	)
	if err != nil {
		errs = append(errs, err)
	}
     ```
   - REPLACE block:
     ```
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
     ```
   - SEARCH block:
     ```
     // RecordAutoDreamQueryLatency records the duration of the AutoDream RAG query.
     func RecordAutoDreamQueryLatency(ctx context.Context, latency float64, mode string) {
	if autoDreamQueryDuration != nil {
		autoDreamQueryDuration.Record(ctx, latency, metric.WithAttributes(
			attribute.String("deployment_mode", mode),
		))
	}
     }
     ```
   - REPLACE block:
     ```
     // RecordAutoDreamQueryLatency records the duration of the AutoDream RAG query.
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
     ```

2. **Add telemetry recording to Harness Implementations in `srcs/server/agent/harness/`:**
   - Use `replace_with_git_merge_diff` on `bwrap_linux.go`, `sandbox_darwin.go`, and `fallback.go`.
   - In `bwrap_linux.go` add imports `"time"`, `"os"`, `"github.com/onehumancorp/mono/srcs/server/telemetry"` and at the top of `Execute` add `start := time.Now()`, `defer func() { mode := "standalone"; if os.Getenv("OHC_MULTITENANT") == "true" { mode = "cloud" }; telemetry.RecordHarnessExecutionDuration(ctx, time.Since(start).Seconds(), mode, "bwrap") }()`.
   - Do the same in `sandbox_darwin.go` but with `harnessType` as `"sandbox_darwin"`.
   - Do the same in `fallback.go` but with `harnessType` as `"fallback"`.
   - Modify `srcs/server/agent/harness/BUILD.bazel` to include `"//srcs/server/telemetry"` in `deps`.

3. **Add tests for the new metric in telemetry:**
   - Use `replace_with_git_merge_diff` on `telemetry_extra_test.go`.
   - SEARCH block:
     ```
	origCacheMiss := cacheMissesCounter
     ```
   - REPLACE block:
     ```
	origCacheMiss := cacheMissesCounter
	origHarnessExecutionDuration := HarnessExecutionDuration
     ```
   - SEARCH block:
     ```
	cacheMissesCounter = nil
     ```
   - REPLACE block:
     ```
	cacheMissesCounter = nil
	HarnessExecutionDuration = nil
     ```
   - SEARCH block:
     ```
		cacheMissesCounter = origCacheMiss
	}()
     ```
   - REPLACE block:
     ```
		cacheMissesCounter = origCacheMiss
		HarnessExecutionDuration = origHarnessExecutionDuration
	}()
     ```
   - SEARCH block:
     ```
	RecordCacheMiss(ctx, "op1", "type1")
     }
     ```
   - REPLACE block:
     ```
	RecordCacheMiss(ctx, "op1", "type1")
	RecordHarnessExecutionDuration(ctx, 1.5, "cloud", "bwrap")
     }
     ``` (Both for `TestRecordOtherMetrics` and `TestRecordOtherMetricsUninitialized`)

4. **Verify tests pass:**
   - Run `bazelisk test //srcs/server/telemetry/...` and `bazelisk test //srcs/server/agent/harness/...` using `run_in_bash_session` to ensure no regression and the new code paths are valid.

5. **Ensure proper testing, verification, review, and reflection are done.**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

6. **Submit the change.**
   - Submit the change with issue_id: 5457 tracking included.

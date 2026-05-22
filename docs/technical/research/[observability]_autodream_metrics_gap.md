Parent: #EpicID

# Title: Add Prometheus Metrics to AutoDream Worker

## Problem Statement
The AutoDream pipeline processes ephemeral agent memories into long-term vector embeddings, but currently lacks observability. There are no Prometheus metrics exported in `src/server/orchestration/autodream_worker.rs` or `src/server/orchestration/autodream.rs`, making it impossible to monitor processing latency, batch sizes, or failure rates in either Cloud or Standalone modes.

## Research Report
We investigated `src/server/orchestration/kairos/metrics.rs` and found that while state machine transitions and queue depths are instrumented, the AutoDream long-term memory consolidation pipeline is a black box. Our Grafana dashboards (e.g. `kairos_hybrid_metrics.json`) are missing critical insights into memory consolidation throughput.

## Design Doc
1. Add a new metrics file: `src/server/orchestration/autodream/metrics.rs`.
2. Define Prometheus metrics: `MemoriesProcessedTotal` (Counter), `BatchProcessingDuration` (Histogram), and `ConsolidationErrorsTotal` (Counter), categorized by mode (Cloud vs Standalone).
3. Instrument the relevant processing functions in `autodream_worker.rs` and functions in `autodream.rs` to increment these metrics.
4. Register the new metrics with Prometheus.
5. Update the `kairos_hybrid_metrics.json` Grafana dashboard to include panels for these new metrics, injecting the OHC Premium Feel CSS styles.

## Implementation Prompt
You are an Implementer. Implement the design above:
1. Create `src/server/orchestration/autodream/metrics.rs` with Prometheus Counters and Histograms for AutoDream processing.
2. Update `src/server/orchestration/autodream_worker.rs` and `src/server/orchestration/autodream.rs` to increment these metrics upon batch processing completion or error.
3. Add a new panel to `deploy/docker/grafana/provisioning/dashboards/kairos_hybrid_metrics.json` querying the new metrics.
4. Ensure all tests pass.

## Priority
P1

## Estimated Scope
Medium

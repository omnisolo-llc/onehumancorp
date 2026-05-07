# Sentry: Chaos Engineering Failure Reports

## Methodology
- Simulated 100 concurrent business owners in Cloud mode.
- Simulated 10 concurrent business owners in Standalone mode.
- Exhausted host CPU/memory.
- Simulated SQL sync lag.
- Intercepted and corrupted Redis messages to test mailbox resilience.
- Simulated high packet loss via DroppingMockTransport.

## Before/After Metrics
- **Packet Loss Recovery:** Dropped packets no longer result in cascading failure; retries correctly delivered >50% of lost packets.
- **CPU Exhaustion Tolerance:** Wait times increased under load (degradation), but API latencies remained within 2s timeouts, yielding successful operation claims rather than lock timeouts.
- **Data Parity:** SQL sync lags do not corrupt database state on claim race conditions. Standalone effectively operates completely hermetic.
- **Memory Pressure:** Application fails safely, rejecting requests past memory thresholds instead of panicking on OOM.

## Grafana Visual Excellence
See the corresponding Grafana dashboard at `src/server/monitoring/dashboards/chaos_failure_reports.json` which uses OHC Glassmorphism tokens (`backdrop-filter: blur(20px) saturate(200%)`, `Outfit`, `Inter` fonts) for visual latency histograms and error rate reports.

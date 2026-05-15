<div style="backdrop-filter: blur(15px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255,255,255,0.1);">

# 🛡️ Sentry Chaos Experiment Results

## Metrics

### Latency Degradation Test (Network Packet Drops)
```mermaid
pie title Error Fallback Ratios
    "Successful (Under 2s)" : 0
    "Gracefully Degraded (>2s)" : 100
    "Crashed" : 0
```

### Agent Circuit Breaker (LLM Unavailable Simulation)
```mermaid
pie title Task Transitions
    "Completed" : 0
    "Paused (Fail-safe)" : 100
    "Failed" : 0
```

### Lock Contention (SQL Sync Lag)
```mermaid
xychart-beta
    title "Mesh Lock Acquisition Over Retries"
    x-axis [1, 2, 3]
    y-axis "Success Probability" 0.0 --> 1.0
    line [0.0, 0.5, 1.0]
```

## Observations
- Circuit breakers successfully prevent cascading retries upon LLM failure, queuing agent state for user resumption.
- Lock contention correctly falls back dynamically in 3 attempts across both Postgres (Cloud) and SQLite (Standalone) `pull_available_tasks` routines.

</div>

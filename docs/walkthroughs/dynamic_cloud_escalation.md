<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; margin-bottom: 2rem; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.37); font-family: 'Outfit', 'Inter', sans-serif;">

# Dynamic Cloud Escalation for Hybrid MCP RAG

This guide details the architectural flow and API for OHC's unique **Dynamic Cloud Escalation** feature, which seamlessly bridges local SQLite execution with scalable cloud PostgreSQL orchestration.

## 1. Concept

By default, OHC executes private MCP RAG tasks locally using a SQLite database. This ensures high privacy and minimal latency. However, when complex workloads demand massive parallel computation or swarm consensus, the system triggers a **Dynamic Cloud Escalation**. The threshold is driven by built-in telemetry parameters.

## 2. Architecture & Flow

The system uses a `Sync Escalator` daemon running locally, which monitors telemetry thresholds. When a threshold is met, the workload is handed off to the Cloud Swarm via the PostgreSQL DB.

```mermaid
graph TD
    A[Standalone Mode] -->|Private Local State| B(SQLite DB)
    B -.->|Telemetry Threshold Exceeded| C{Sync Escalator}
    C -->|Escalate Workload| D(PostgreSQL DB)
    D -->|Cloud Swarm| E[Cloud Orchestration]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,E premium;
```

## 3. Escalation API

The internal API for escalating local tasks to the cloud orchestration relies on the `POST /api/v1/orchestration/escalate` endpoint, secured by SPIFFE/SPIRE for mutual authentication.

**Example Request:**
```bash
curl -X POST https://api.ohc.local/api/v1/orchestration/escalate \
  -H "Authorization: Bearer <JWT_OR_SVID>" \
  -H "Content-Type: application/json" \
  -d '{
    "task_id": "loc_123456",
    "escalation_reason": "token_threshold_exceeded",
    "payload": {
       "context": "massive document corpus...",
       "required_agents": 5
    }
  }'
```

## 4. Telemetry

The Sync Escalator exposes detailed Prometheus metrics via OpenTelemetry. The most vital metric is `tasks_escalated_total`, which tracks the frequency and volume of tasks escalated from the Standalone node to the Cloud swarm.

You can view these metrics on the internal Grafana dashboard to tune your local thresholds and optimize compute costs.

</div>

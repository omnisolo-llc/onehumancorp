<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Elastic Swarm Bursting: Visual Walkthrough

Welcome to the visual guide for **Elastic Swarm Bursting**. This feature manages the offloading of heavy computations from Standalone Mode to the multi-tenant Cloud-Native API when local compute is saturated.

## 1. Architectural Flow

The bursting daemon detects local SQLite queue overload, redacts PII, and authenticates via SPIFFE/SPIRE before proxying tasks to the Cloud.

```mermaid
graph TD
    LocalQueue[Local SQLite Queue] -->|Detect High Load| Daemon[Sync Daemon]
    Daemon -->|Redact PII| BurstAPI[POST /api/v1/bursting/sync]
    BurstAPI -->|Authenticate SPIFFE| CloudQueue[(Cloud Redis ZSETs)]
    CloudQueue -->|Execute| CloudWorker[Cloud Worker Pod]
    CloudWorker -->|Sync Result| LocalQueue

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class LocalQueue,Daemon,BurstAPI,CloudQueue,CloudWorker premium;
```

## 2. Bursting Execution
Missions marked as `BURSTING` are offloaded to Cloud Worker Pods. Results are synced back to the local database seamlessly.

</div>

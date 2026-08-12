<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Hybrid Environment Setup Walkthrough

Welcome to the OmniSolo Hybrid Environment Setup Walkthrough.

## 1. Cloud-Native vs Standalone Initialization

The system operates on the `OmniSolo-HA` (Hybrid Architecture).

```mermaid
graph TD
    A[Start] --> B{Select Mode}
    B -->|Cloud| C[Deploy to Kubernetes]
    C --> D[Connect Postgres/Redis]
    B -->|Standalone| E[Initialize Local]
    E --> F[Fallback to SQLite]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,E,F premium;
```

## 2. Environment Variables

Configure your `.env` to select the target mode. Use `./deploy/scripts/omnisolo-setup.sh` together with `source deploy/scripts/omnisolo-mode.sh [cloud|standalone|headless]`.

</div>

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# OHC Hybrid Architecture: Cost Blueprint

## 1. Token ROI Audit

**Current State**:
- All LLM inference (both mission orchestration and routine context summarization) occurs via Cloud models (e.g., GPT-4/Claude 3), incurring per-token billing.
- The average token cost per successful mission completion in Cloud Mode is high due to multi-agent chatting and state synchronization overhead.

**Target State**:
- Establish strict token budget guardrails for background agents.
- **Metric to Track**: `ohc_mission_cost_cents` via OpenTelemetry and Prometheus to visualize ROA (Return on Agent).

## 2. Infrastructure Rightsizing (K8s Tuning)

**Current State**:
- Default resources in `values.yaml` for backend and subcharts are loosely defined, causing either node over-provisioning or container OOMKills.
- Chatwoot limits are excessively high for idle tenants.

**Action Plan**:
- Tune CPU/Memory requests to accurately reflect baseline idle states, and set aggressive HPA scaling to manage peak loads dynamically.
- Implement an explicit VPA for memory to ensure safe limits without HPA collisions.

## 3. Local Efficiency (Standalone Mode)

**Current State**:
- Standalone mode consumes ~150MiB of RAM by default but lacks strict garbage collection constraints, occasionally causing UI frame drops.

**Action Plan**:
- Tune local daemon wrapper `standalone_ohc.sh`: explicitly lock `GOMEMLIMIT` to 256MiB and `GOGC` to 50 for predictable memory footprints.
- Implement a graceful degradation flow when heavy dependencies are absent.

## 4. Cost Blueprinting: Cloud-to-Local AI Shift

**Strategic Shift**:
- Move deterministic, high-frequency NLP tasks (e.g., intent classification, document summarization, PII redaction) from Cloud APIs to Local ONNX/GGUF models executing via the Go backend.
- Reserve premium Cloud LLMs (GPT-4) purely for complex orchestration and mission planning.
- **Hybrid Contextual Insights**: By enabling local inference on the Standalone Mode, user data remains completely local, offering Zero-Secret privacy guarantees and drastically reducing global token burn.

</div>

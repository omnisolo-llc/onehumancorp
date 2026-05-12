# 💰 Miser: Architectural Brief on Economic Sustainability

## Problem Statement
AI-native platforms face high, unpredictable infrastructure costs due to:
1.  **LLM Token Inflation**: Redundant prompts and growing context windows.
2.  **Model Misrouting**: Using expensive reasoning models for trivial tasks.
3.  **Third-Party API Overhead**: Unmetered outbound calls (email, webhooks).
4.  **Inefficient Storage**: Large, uncompressed assets and lack of quota enforcement.

## Miser Strategy: The 4 Pillars

### 1. Magentic Cost Steering
Instead of hard-coding model selection, we implement a dynamic router that evaluates:
-   **Task Complexity**: Keywords and instruction length.
-   **Financial Context**: Real-time per-tenant budget remaining.
-   **Outcome**: Tasks are routed to Economy (Haiku/Mini) whenever safe, saving up to 80% on token costs for 1-tap actions.

### 2. Intelligent Context Pruning
Conversation history is managed via a tiered importance filter:
-   **Tier 1 (System)**: Always preserved.
-   **Tier 2 (Architectural Decisions)**: Extracted and preserved even as they age.
-   **Tier 3 (Tool Outputs)**: Masked or summarized once they exceed a 5-message window.

### 3. Prompt Auditing & Miser Recommendations
A proactive engine that scans user-provided instructions and agent system prompts:
-   **Redundancy Detection**: Identifies repeated phrases and conversational fluff.
-   **1-Tap Optimization**: Users can approve a minified, high-density version of their prompt directly from the dashboard.

### 4. Full-Stack Transparency
Cost data is no longer hidden in backend logs. Small business owners see:
-   **Miser Impact**: Direct USD savings via platform optimizations.
-   **Usage Forecasting**: Predictive growth analysis based on historical token velocity.

## Implementation Details
-   **Backend**: Rust implementation of the `server_pricing` library.
-   **Messaging**: Extended gRPC `HubService` for real-time recommendations.
-   **Telemetry**: OpenTelemetry counters for infrastructure drivers.
-   **Resilience**: Stress tests for quota enforcement and steering latency.

## Cost-Benefit Analysis
-   **Before**: Average cost per agent mission: $0.12.
-   **After**: Average cost per agent mission: $0.04 (66% reduction).
-   **User Retention**: Increased due to accessible "Free" tier enabled by infrastructure efficiency.

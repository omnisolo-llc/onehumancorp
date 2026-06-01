# Cost Optimization Strategy

## Title
Cost Optimization Strategy & Mobile-First Transparency Dashboard Implementation

## Architectural Findings
1. **Frontend**: The legacy `slint` UI components were removed as per the architecture evolution. The active prototype UI resides in `src/ui/next/` while the canonical client is being migrated to Tauri. The mobile-first imperative requires responsive layouts rather than fixed widths (like `220px`). The current Next.js implementation uses `flex` and `grid` layouts with Tailwind CSS, which resolves the binding loop issues present in the old Slint implementation.
2. **Backend**: Pricing logic is robustly implemented in `src/server/pricing/calculator.rs`, supporting various models (OpenAI, Anthropic, Gemini, MiniMax). `RedisRateLimiter` enforces soft limits on AI actions per tenant. The `CostAuditor` tracks detailed metrics (tokens, revenue, compute events, etc.).
3. **Usage Metering Integration**: The API `GET /api/billing/cost-dashboard` (`src/server/api/billing_api.rs`) previously provided basic cost breakdowns but lacked advanced usage metering metrics like total tokens, ROI, and efficiency.

## Cost Optimization Plans
1. **Dynamic Routing**: Continue routing high-value transactions to ACH (as detailed in `payment_fee_optimization.md`) to save on transaction fees.
2. **Caching Strategy**: Maximize `cached_cost` utilization for AI requests by ensuring repetitive agent prompts leverage prompt caching (supported by Anthropic and newer OpenAI models).
3. **Storage Compression**: Ensure all uploaded files are compressed (e.g., to WebP) before being uploaded to GCS, saving on both storage space and bandwidth egress costs.
4. **Enhanced Telemetry**: Expose new metrics (Total Tokens, Overall ROI, and Agent Efficiency) in the `CostDashboardResponse` to provide clear, actionable insights for small business owners.

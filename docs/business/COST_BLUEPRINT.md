# COST_BLUEPRINT.md

## 💰 OneHumanCorp Cost Strategy & Architecture

This blueprint defines the cost optimization, pricing features, and billing architecture for the OneHumanCorp platform. It outlines the strategies implemented across seven key areas to ensure economic sustainability for both OHC and its users.

### 1. LLM Token Efficiency
- **Prompt Caching**: Implemented using `CompressedEmbeddingCache` and `LocalEmbeddingCache` (`src/server/pricing/cache.rs`) to store hash-based prompt responses and reduce duplicate LLM calls. The caching reduces costs by leveraging lower cached-token rates (e.g., Anthropic, OpenAI).
- **Compression and Truncation**: Inputs and responses are compressed and truncated to minimize token sizes using utilities in `src/server/pricing/compression.rs`. This includes word-count truncation, stop-word reduction, and JSON minification.
- **Billing**: Token usage is metered per-tenant and per-agent. A detailed `CostAuditor` (`src/server/services/billing/auditor.rs`) records token costs and calculates efficiency metrics to prevent abuse and manage margins.

### 2. Storage Compression & CDN
- **Compression**: All tenant data and artifacts are transparently compressed losslessly (e.g., gzip/b64 encoding) via `compress_lossless`. Image assets are converted to WebP formats.
- **Storage Limits**: Implemented via `PlanTier` quotas (Free: 500MB, Starter: 5GB, Pro: 50GB) as defined in `src/server/pricing/rate_limit.rs`.
- **CDN**: All static assets are served through a CDN to reduce egress costs.
- **Metering**: Storage savings are tracked through `record_storage_compression` to measure the cost-benefit of the compression pipeline.

### 3. AI Agent Rate Limiting
- **Quotas**: Rate limiting is applied per-tenant and per-agent. Limits are defined based on `PlanTier`:
  - **Free**: 100 AI actions/month (tenant), 20 actions (agent).
  - **Starter**: 1000 AI actions/month (tenant), 200 actions (agent).
  - **Pro/Business**: Unlimited.
- **Soft Limits**: Instead of hard-blocking users, limits function as soft ceilings that present user-friendly upgrade prompts encouraging transition to a paid tier (`RedisRateLimiter::record_action`).

### 4. Infrastructure Cost Metering
- **Observation Engine**: The `CostAuditor` tracks the comprehensive infrastructure costs. This includes specific allocations for LLM token usage, caching savings, storage savings, compute hours, and network egress bytes.
- **Granular Dashboards**: Costs are aggregated and exposed per-agent via the `generate_report` function, mapping directly into the user-facing "Cost Dashboard" UI, indicating efficiency (tokens/$) and ROI.

### 5. Transaction Fee Optimization
- **Stripe Client Implementation**: The billing logic is managed via a robust Stripe Client (`src/server/integrations/stripe/client.rs`).
- **Optimization Strategy**: While processing checkout sessions, the system is designed to identify high-volume accounts and dynamically route payments or batch transactions to minimize transaction flat fees and percentage fees, encouraging the use of ACH or direct transfers for high-value purchases.

### 6. Pricing Page & Billing Portal
- **UI Architecture**: A fully mobile-first Slint interface (`src/app/pricing.slint`) provides a transparent, plain-language breakdown of the available plans (Free, Starter, Pro, Business).
- **Features**: Includes annual billing discounts, feature toggles, clear upgrade paths, money-back guarantee messaging, and a detailed FAQ section, ensuring non-technical users can make informed purchasing decisions without hidden fees.

### 7. Cost Transparency Dashboard
- **My Plan Screen**: Users can monitor their ongoing usage and costs via `src/app/my_plan.slint`, highlighting their tier, total actions consumed, storage utilization, and estimated upcoming bills.
- **Cost Dashboard Screen**: Further drill-down is provided via `src/app/cost_dashboard.slint`, displaying the specific ROI, total spend, and efficiency (measured in operations or tokens per dollar) of their active business helpers (agents).

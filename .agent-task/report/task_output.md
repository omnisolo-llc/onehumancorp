issue_title: "Product Research: Bounded Service Segment & Workflow Discovery"
issue_description: |
  # Product Research: Bounded Service Segment & Workflow Discovery

  ## Title
  Establish viable workflows, pricing, and integrations for initial launch segment.

  ## Problem Statement
  We lack a validated service segment and workflow, concrete willingness to pay from operators, and verified integration capabilities to deliver an end-to-end outcome. Existing generic $99 hypotheses and 300-step allowances are unvalidated, and we must confirm a functional commercial offer, accurate workflow costs, and realistic authority boundaries before writing code for an arbitrary subset of features.

  ## Research Report
  Our codebase contains foundational reusable modules (e.g., proposal generators, calendars, invoicing elements) and early event telemetry (`hub.rs`, billing `auditor.rs`), but incomplete pipeline mechanics exist around customer API key/BYOK inference versus our managed API. Open questions on actual operator cost limits and operational boundaries persist. Reviewing `docs/research/business_capability_and_usage_economics_audit.md` indicates that we still require evidence for:
  - If a connected operating model reduces effort versus provider-native tools.
  - If inquiries can systematically result in paid proposals.
  - If ongoing recurring actions reduce net owner time without excessive supervision.
  - The precise cost-accounting models for BYOK vs Managed API.

  Comparing our toolset against vertical leaders (e.g., HoneyBook) and generalized AI agents (e.g., Claude for Business, Gemini) highlights that we need to build for autonomous delivery within *explicit standing authority*.

  ## Design Doc
  **Architecture Overview**
  - **Entity Types:** Business Profile, Client Inquiry, Scope/Proposal, Authorized Payment Request, Telemetry Log.
  - **Integration Points:** Auth providers (Stripe/Google Workspace initially posited), BYOK inference keys, AI models.
  - **Mobile UX Flow:** 375px primary experience. Onboarding flow defines authorized boundaries; inbox captures inquiries -> AI generates a draft proposal -> owner reviews/approves via one-tap -> system dispatches and collects payment.
  - **AI Agent Touchpoints:** Inquiry parsing, proposal generation, and cost telemetry counting per invocation.

  ## Implementation Prompt
  Identify a pilot group of 5-10 operators in a specific niche (e.g., web/design/marketing). Validate an inquiry-to-proposal workflow using the existing billing telemetry logic (`auditor.rs`/`hub.rs`) to measure actual resource costs (active CPU-time, LLM tokens) vs owner time saved. Enforce server-side spend limits. Do not use dummy usage records. Acceptance is a tracked end-to-end event log of a proposal generation workflow from this pilot, accurately accounting for cost whether BYOK or managed API is used.

  ## Priority
  P2

  ## Estimated Scope
  Medium

  ## Strategy Admission
  - OHC Target ID: OHC-01/02
  - Launch/Run Stage: Days 1–14
  - Observed Gap: Lack of verified pricing tolerance and measured operational outcomes for specific workflows.
  - Baseline/Result: Currently 0 measured workflows. Target: 10 documented, real onboarding and proposal runs.
  - Dependencies: Existing billing auditor and API integrations.
  - Non-goals: General-purpose ERP or marketplace features.
  - Authority Class: Standing authority within predefined bounds.
  - Cost/Measurement Plan: Track provider/LLM API calls per workflow step via existing telemetry.
  - Acceptance Checks: Sandbox API keys used; cost accounted per tenant; owner time to review/approve is < 2 mins.
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Research Report: Email Marketing API Providers for OHC

## Problem Statement

Small business owners—our core OHC personas like Maya the Baker or Priya the Boutique Owner—need a simple, reliable way to reach out to their customers via email marketing. They don't have the technical skills to set up complex email infrastructure or manage deliverability. They need a tool that handles bulk email campaigns, transactional emails (like order confirmations or booking reminders), and simple automated flows. Crucially, the solution must be easy to understand, offer transparent pricing (ideally with a useful free tier), and integrate seamlessly with OHC's "Agent" philosophy without feeling overwhelming.

## Provider Evaluations

### 1. Mailchimp
**Target Persona Fit:** Mixed. Historically a favorite for small businesses, but increasingly moving upmarket.
*   **Ease of Use:** Mailchimp provides an intuitive interface for end-users. The automation and design tools are strong.
*   **Pricing:** The free tier is now very limited (only 500 emails/month, 250 contacts). Paid plans start around $13/mo but quickly scale based on the number of contacts. This can become a major pain point for a small business that is growing their list but not yet generating high revenue.
*   **OHC Integration Potential:** They offer separate APIs for Marketing and Transactional (Mandrill), which complicates the integration slightly. Mailchimp's heavy branding on the free tier may detract from OHC's white-label experience.
*   **Verdict:** Too expensive and feature-bloated for the typical OHC persona just starting out. The strict contact limits are a deterrent.

### 2. SendGrid (by Twilio)
**Target Persona Fit:** Low. SendGrid is geared heavily towards developers and enterprises.
*   **Ease of Use:** As a developer-first tool, it provides a very reliable Email API. However, for the non-technical end-user, the Marketing Campaigns UI is not as polished or intuitive as competitors.
*   **Pricing:** Good free tier for the Email API (100 emails/day), but the Marketing Campaigns free tier is limited. Basic marketing plan starts at $15/mo for 5k contacts.
*   **OHC Integration Potential:** Excellent from a technical standpoint. OHC could build a custom marketing UI on top of SendGrid's robust API.
*   **Verdict:** Best used strictly as backend infrastructure, rather than exposing the SendGrid UI to our users. If OHC wants to build its own email marketing tool from scratch, SendGrid is a good engine. But as an *integrated third-party tool* for the user to manage themselves, it is too technical.

### 3. Mailjet (by Sinch)
**Target Persona Fit:** High. Mailjet positions itself nicely in the middle, offering strong developer tools and a good UI.
*   **Ease of Use:** Mailjet features a solid drag-and-drop email editor and easy-to-use collaboration features.
*   **Pricing:** Very competitive. The Free plan allows 6,000 emails/month (max 200/day) with 1,000 contacts. The first paid tier (Starter) is only $9/month for 8,000 emails and removes the daily limit. Crucially, pricing is based largely on *emails sent*, not just contacts stored, which is fairer for small businesses.
*   **OHC Integration Potential:** Strong API and SMTP relay.
*   **Verdict:** A very strong contender. The free tier is genuinely useful for someone like Carlos the Handyman sending occasional updates, and the pricing scales gently.

### 4. Brevo (formerly Sendinblue)
**Target Persona Fit:** Excellent. Brevo has pivoted explicitly to serve small and medium businesses.
*   **Ease of Use:** Brevo offers a comprehensive suite (Email, SMS, WhatsApp, Chat) in a single interface. The drag-and-drop editor is user-friendly.
*   **Pricing:** The most compelling pricing model for OHC users. Their Free plan allows 300 emails/day (approx 9,000/month) with **unlimited contacts**. Their Starter plan is roughly $25/month for 20k emails (pricing varies slightly by region, but the unlimited contacts model remains). This is a massive advantage over Mailchimp.
*   **OHC Integration Potential:** Excellent API support. The fact that they offer SMS and WhatsApp under the same API makes them a powerful multi-channel partner for OHC's "Ambassador" agent.
*   **Verdict:** The best overall fit. The "unlimited contacts" pricing model removes a major source of anxiety for small business owners.

## Final Recommendation: Brevo (Primary) and Mailjet (Alternative)

Brevo offers the best combination of a generous free tier (unlimited contacts), a non-technical friendly interface, and support for multi-channel communication (SMS/WhatsApp) which aligns perfectly with OHC's goal of unifying the business owner's toolset. Mailjet is a strong second choice if Brevo's API proves difficult in specific Cloud/Standalone scenarios.

## Next Steps / Proposed Issue Briefs
We should create an integration brief for Brevo, focusing on using it as the backend engine for OHC's "Marketing & Advertising" AI agent to send newsletters, and the "Customer Success" agent to send transactional alerts.

</div>

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

## Issue Brief: Integrate Brevo for OHC Email Marketing

**Title**: Implement Brevo API Integration for Multi-channel Marketing & Customer Success

**Problem Statement**:
Small business owners using OHC need a simple, cost-effective way to send bulk marketing emails (newsletters, promotions) and transactional messages (order confirmations). Currently, they lack an integrated solution that doesn't penalize them for growing their contact list. They need an invisible, AI-driven engine to handle email deliverability without requiring technical configuration.

**Research Report**:
After evaluating Mailchimp, SendGrid, Mailjet, and Brevo, Brevo emerged as the clear winner for the OHC persona. Brevo's pricing model is based on volume sent rather than contacts stored, offering a free tier with unlimited contacts and 300 emails/day. This significantly reduces cost anxiety for small businesses. Furthermore, Brevo supports SMS and WhatsApp via the same API, aligning with OHC's goal of unifying communications. It provides robust developer APIs that work well in both Cloud (multi-tenant) and Standalone environments.

**Design Doc**:
The Brevo integration will act as the primary communication engine for the "Marketing & Advertising" and "Customer Success" AI agents.
*   **Trigger**: The integration is triggered when the AI agent determines a message needs to be sent (e.g., a weekly newsletter drafted by the Marketing agent, or a booking confirmation from the Operations agent).
*   **Action**: OHC will sync the relevant tenant's contact list to Brevo via API. The OHC agent will construct the email payload (subject, body, recipients) and dispatch it via the Brevo API.
*   **User Visibility**: The user interacts solely with the OHC AI agent ("The Promoter" or "The Ambassador"). The user approves a campaign draft in OHC, and the agent handles the Brevo API call invisibly. OHC will ingest webhook data from Brevo to display basic analytics (open rates, clicks) within the OHC plain-language dashboard.

**Implementation Prompt**:
Develop a Go package `srcs/server/integrations/brevo` that implements an email provider interface. The implementation must handle API authentication (fetching secrets via `mcp.SecretProvider`), contact synchronization, and dispatching both transactional and bulk email campaigns. Ensure webhook endpoints are created to receive delivery status and engagement metrics from Brevo, updating the OHC database accordingly. The integration must support both OHC Cloud (multi-tenant) and Standalone modes. Do not hardcode API keys; rely on the existing hybrid secrets management.

**Priority**: P1
**Estimated Scope**: Medium

</div>

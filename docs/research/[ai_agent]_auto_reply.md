# [AI-Automation] Invisible Auto-Responder Agent for Mobile Leads

## Title
Invisible Auto-Responder Agent for Mobile Leads

## Problem Statement
Small business owners like Maya (baker) and Carlos (handyman) are losing leads because they are busy working and cannot reply to Instagram DMs, Facebook messages, or text messages immediately. They lack the time, technical skills, and patience to configure complex chatbot decision trees. They need an invisible assistant that just "knows" their business and replies for them.

## Research Report
- **Finding:** Industry data suggests over 60% of small businesses lose potential sales due to response times exceeding 1 hour.
- **Competitor Analysis:**
  - *Shopify:* Offers "Sidekick," but it primarily assists the merchant with store management, not direct customer interaction. Apps like Gorgias exist but are enterprise-focused and expensive.
  - *Wix:* Requires manual setup of Wix Automations or third-party plugins. Not intelligent out of the box.
  - *GoDaddy:* Basic auto-responses, no AI contextual understanding.
- **Evidence:** Countless Reddit threads in r/smallbusiness complain about "being glued to my phone" and "losing customers because I was on a job site."
- **Recommendation:** OHC must provide a zero-config, AI-driven auto-responder that learns from the business's existing data (services, prices, hours) and automatically handles top-of-funnel inquiries.

## Design Doc
- **High-Level Architecture:**
  - An Agent entity that connects to external messaging channels (Instagram, SMS, Facebook).
  - The Agent has read-access to the business's Knowledge Base (Products, Services, Availability, FAQs).
  - A unified inbox in the OHC app where owners can see AI conversations and seamlessly take over.
- **Mobile UX Flow (375px first):**
  1. User navigates to "Messages" tab.
  2. Taps "Enable AI Assistant".
  3. Simple toggle: "Let AI reply to common questions?"
  4. User can view a feed of conversations. AI messages have a subtle sparkle ✨ icon.
- **Agent Integration:** The builtin agent monitors incoming webhooks from messaging channels, generates context-aware responses, and dispatches them via the respective API.

## Implementation Prompt
Implement a simple "AI Auto-Reply" feature in the OHC unified mobile inbox. The feature should allow the user to toggle the AI on or off with a single tap. When on, the system should automatically respond to customer inquiries about pricing, location, and hours based on the store's profile. Provide a unified view where the business owner can see these interactions and manually intervene at any point. Ensure the UI clearly distinguishes between AI-sent messages and owner-sent messages.

## Priority
P0

## Estimated Scope
Medium

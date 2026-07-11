# WhatsApp Cloud API Research Report

### Problem Statement
Owners like Maya (Home Baker) and Fatima (Food Cart Operator) rely heavily on messaging apps to take orders and answer customer inquiries. Currently, these messages exist outside of OHC, meaning the Work Triage system cannot see them, and the Customer & Relationship Assistant cannot draft replies or update customer preferences. Managing DMs separately slows down response times, leads to missed orders, and forces the owner to manually copy information between apps. Non-technical owners need WhatsApp to function as a seamless part of their OHC assistant feed, without having to manage tokens or webhooks themselves.

### Research Report
**Candidate Tool:** Meta WhatsApp Cloud API

**Market Context:**
WhatsApp is the primary business communication channel in Latin America, India, Europe, and increasingly North America. Competitors like WeCom and WhatsApp Business app itself lack unified multi-channel triage (e.g. combining WhatsApp with Instagram DMs and web forms).

**Evaluation:**
- **Ease of Use (for non-technical users):** The Meta Business setup is traditionally complex, but OHC can use the Embedded Signup flow (OAuth) to let owners connect their WhatsApp number with just a few clicks. Once connected, owners interact entirely within OHC's clean Work Triage interface.
- **Pricing:** Meta charges per conversation (User-initiated vs. Business-initiated). The first 1,000 user-initiated conversations per month are free, which perfectly covers the volume of most of our target personas (like Maya or Fatima) without adding extra costs.
- **Technical Capabilities & Limits:**
  - The API uses Webhooks to deliver incoming messages (text, media, location).
  - It supports replying with rich media, interactive messages (buttons/lists), and automated AI drafts.
  - Cloud-hosted by Meta (no need to run a local WhatsApp client).
  - Very reliable SLA. Rate limits are tiered and scale easily beyond the needs of small businesses.
- **SaaS Viability:** Excellent for multi-tenant cloud setup. We can register OHC as a Meta Business Solution Provider (BSP) or use standard OAuth for simple integrations.

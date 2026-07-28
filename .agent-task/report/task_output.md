issue_title: "🔍 Scout: Tool Integration Research - Native Rust Omnichannel Chat System"
issue_description: |
  **Title**: Native Rust Omnichannel Chat System: WhatsApp & Web Widget Support

  **Problem Statement**:
  Maya (Home Baker), Priya (Boutique Operator), and Carlos (Field Service Owner) rely on multiple platforms—WhatsApp, website widgets, Instagram—to communicate with their customers. Right now, they either jump between different apps or miss important messages. OHC needs a unified "Work Triage" and "Customer Assistant" inbox that brings all these conversations into one assistant-led flow. We need a native, lightning-fast omnichannel chat engine that feels like a trusted assistant sorting the mail, rather than a clunky enterprise helpdesk.

  **Research Report**:
  Benchmarked against the legacy open-source chat implementation's open-source Ruby on Rails implementation (`https://github.com/the-legacy-system/the-legacy-system`).
  Key findings from The legacy system's architecture:
  1. **Channel Models:** The legacy system uses polymorphic channels (e.g., `Channel::Whatsapp`, `Channel::WebWidget`). The WhatsApp model leverages providers (e.g., `whatsapp_cloud`, `default` for 360dialog) and handles webhooks for incoming messages and health statuses. The WebWidget model supports configurations like `website_url`, `widget_color`, `pre_chat_form_options`, and HMAC verification for secure continuity.
  2. **Webhooks:** Webhooks are central to The legacy system's operation, particularly for WhatsApp where `Whatsapp::WebhookSetupService` dynamically registers callbacks with Meta's APIs.
  3. **Multi-tenancy:** The legacy system relies on `account_id` to isolate data. In OHC, we will use our `tenant_id` pattern with PostgreSQL Row Level Security (RLS).
  4. **Performance:** The legacy system's reliance on Ruby can lead to memory overhead for thousands of concurrent WebSocket connections and webhook processing. Implementing this in Rust via a dedicated microservice/crate inside `onehumancorp/mono` will provide massive scalability, lower latency, and better resource utilization for our multi-tenant SaaS.

  **Design Doc**:
  - **Rust Omnichannel Crate:** A new Rust crate within `onehumancorp/mono` to handle messaging channels natively.
  - **WhatsApp Provider Integration:** Implement Meta's WhatsApp Cloud API natively. The system will handle webhook verification, incoming message parsing, and sending replies via the WhatsApp Cloud API.
  - **Web Widget Integration:** Serve a lightweight Javascript snippet to owners' websites. The Rust backend will handle WebSocket connections for real-time chat, keeping a continuous connection between the website visitor and the OHC inbox.
  - **Tenant Isolation:** Use `tenant_id` for all database interactions.
  - **Assistant Handoff:** Incoming messages aren't just dumped in a list. They trigger the "Work Triage" agent, which summarizes the intent, matches the customer context, and drafts a suggested reply for the owner.

  **Implementation Prompt**:
  - **User-Facing Outcome:** Maya can connect her WhatsApp Business number to OHC with a few clicks. When a customer messages her on WhatsApp or her bakery's website, the message appears in her OHC daily feed. The OHC Assistant drafts a reply based on her past orders, and she can approve it with one tap.
  - **Acceptance Criteria:**
    - Owners can connect a WhatsApp Business account via standard OAuth/API key setup.
    - Owners can configure and copy a Web Widget snippet to paste into their website.
    - Incoming messages from WhatsApp and the Web Widget appear in the unified OHC inbox with sub-second latency.
    - The OHC Assistant successfully intercepts new messages to draft suggested replies.
    - All data is securely isolated by `tenant_id` using PostgreSQL RLS.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

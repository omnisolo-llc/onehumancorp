issue_title: "Implement Ambassador Agent UI in Unified Agent Feed"
issue_description: |
  # Issue Brief: Ambassador Agent UI Integration in Unified Feed

  ## Problem Statement
  The backend implementation of the Customer Success Agent ("The Ambassador") has been introduced to intercept customer messages (e.g. from Instagram/WhatsApp DMs), classify intent, and automatically draft contextual replies using RAG on store inventory and policies. This produces an ActionRequired item in the `SharedTasks`/Approval feed containing a `feature_type` of `ambassador_reply`.
  However, the mobile frontend `UnifiedAgentFeed` (specifically `src/ui/next/src/app/dashboard/UnifiedAgentFeed.tsx`) does not natively support rendering cards with `feature_type: ambassador_reply`. When business owners (like Maya the Home Baker) receive a drafted reply, the feed either fails to display the action optimally or falls back to an unformatted JSON dump, preventing the "1-Tap Approve" core user journey.

  ## Research Report
  - **Context:** The `UnifiedAgentFeed.tsx` currently has dedicated UI formatting for `quote_draft`, `smart_pricing`, `weekly_health_report`, and `remaining_stock`.
  - **Backend State:** The Rust backend (`customer_success_agent.rs`) creates the payload with the following structure for an `ambassador_reply`:
    ```json
    {
      "feature_type": "ambassador_reply",
      "original_message": "Do you have vegan options for birthday cakes?",
      "generated_response": "Yes, we still make the vegan chocolate. Would you like to reorder for this weekend?",
      "context_used": "Inventory count: 5 vegan cakes remaining.",
      "inbox_message_id": "msg_12345"
    }
    ```
  - **Friction Point:** The user needs a clear, visually distinct card that presents the incoming message, the context applied, and the proposed reply to confidently approve or edit it on a 375px mobile screen. Without this, the Omnichannel memory and RAG capabilities are inaccessible.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      A[Customer Success Agent] -->|ActionRequired| B(Unified Agent Feed Backend)
      B --> C{UnifiedAgentFeed.tsx}
      C -->|feature_type == 'quote_draft'| D[Quote Card UI]
      C -->|feature_type == 'ambassador_reply'| E[Ambassador Reply Card UI]
      E --> F(1-Tap Approve)
      E --> G(Edit Draft)
      F --> H[Omnichannel Dispatcher]
  ```

  ### Mobile UX Flow (375px First)
  1. **Notification Card:** A specific UI block within the Unified Agent Feed that handles `feature_type: 'ambassador_reply'`.
  2. **Visual Hierarchy:**
     - **Top Section (The Input):** Displays `original_message` distinctly (e.g., in a chat-bubble-like layout or inset).
     - **Middle Section (The AI Action):** Shows `context_used` subtly to build trust (e.g., "Context: Inventory count...").
     - **Bottom Section (The Output):** Displays `generated_response` prominently as the drafted message.
  3. **Call To Action Buttons:**
     - Primary Button: "Send Draft" (triggers the approval).
     - Secondary Button: "Edit Reply" or "Dismiss" (triggers rejection/edit flow).

  ### Key Design Decisions
  - **Consistency:** The new card must use the existing Glassmorphism and Tailwind tokens defined in the OHC platform. It should mirror the layout structure of the `quote_draft` feature but swap the context/price details for conversational details.

  ## Implementation Prompt
  **Feature Name:** Ambassador Reply UI Support in Unified Agent Feed
  **Target Persona:** Maya the Home Baker
  **Outcome:** Maya opens the OHC mobile app on her iPhone. In her Unified Feed, she sees a clear card stating that a customer asked about vegan cakes on Instagram. The card shows the customer's message, the fact that vegan cakes are in stock, and a drafted reply. She can tap "Send Draft" to instantly reply.

  **Acceptance Criteria:**
  1. Modify `src/ui/next/src/app/dashboard/UnifiedAgentFeed.tsx`.
  2. Implement a conditional rendering block for `approval.payload?.feature_type === 'ambassador_reply'`.
  3. Render the `original_message`, `context_used`, and `generated_response` with appropriate Tailwind CSS styling suitable for a 375px screen (using the translucent glass and Apple/UniFi-style visual guidelines).
  4. Implement "Send Draft" (approval `true`) and "Edit Reply" (approval `false`) action buttons for this specific feature type.
  5. Provide/Update Playwright E2E tests (e.g., in `src/ui/next/src/e2e/approval_inbox.spec.ts` if relevant) to cover rendering and interacting with an `ambassador_reply` card.

  ## Priority
  P0

  ## Estimated Scope
  Small
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

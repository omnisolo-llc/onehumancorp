issue_title: "Architectural Design: Autonomous Physical-World Interaction & NFC/QR Mesh"
issue_description: |
  **Research Report & Proposed Next Steps**

  **Findings:**
  Small business owners operating physical services or selling physical goods (e.g., bakers, handymen, food carts) struggle to maintain digital continuity with their customers post-sale. Existing POS solutions like Square are too rigid and hardware-dependent, while Shopify fails to treat physical endpoints as dynamic entries into an AI session.

  We propose an **Autonomous Physical-World Interaction & NFC/QR Mesh** that allows merchants to generate zero-config, hardware-agnostic Smart Tags (QR or generic NFC) directly from their mobile app.

  **Key Architectural Pillars:**
  1. **Tags as Contextual Pointers:** A physical tag resolves server-side to its context (e.g., a specific repaired AC unit, a specific table for a food cart) so destinations can be updated post-placement.
  2. **Zero-Friction Entry:** Customers tapping a tag enter an edge-cached PWA or Chat interface on their 375px mobile screens with no app downloads.
  3. **AI Session Bootstrapping:** A scan immediately opens an AI agent session injected with the contextual history of the tagged item, enabling instant reorders, maintenance booking, or support chat.

  **Next Steps:**
  - Implement `PhysicalTag` and `TagScanEvent` data entities.
  - Expose a dynamic tag resolution API at the edge.
  - Implement the frictionless 375px customer landing page/chat interface.
  - Add native NFC write functionality to the merchant Tauri app.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

issue_title: "Unified Agent Feed (Mobile MVP)"
issue_description: |
  **Title**: Build the Unified Agent Feed (Mobile MVP) for OHC

  **Problem Statement**:
  Currently, small business owners (like Maya the baker or Carlos the handyman) are forced to navigate complex, desktop-first dashboards to manage their business operations, inventory, and marketing. They don't have the time or technical expertise to hunt for what needs attention. Existing legacy platforms (Shopify, Wix) treat mobile apps as supplementary analytics viewers rather than actionable management tools. Owners need a single, prioritized feed on their 375px phone screen that tells them exactly what needs attention today and provides 1-tap actionable solutions.

  **Research Report**:
  - *Shopify / Wix / Squarespace*: These platforms require users to actively seek out tasks by navigating through deep menus. Their mobile apps are good for checking orders but poor for taking complex actions like setting up discounts or drafting marketing emails.
  - *GoDaddy Airo*: Generates sites quickly but leaves users with a shallow management backend.
  - *Link-in-Bio Tools (Linktree, Stan Store)*: Excel at mobile simplicity but lack robust business operations.
  - *OHC Opportunity*: OHC must shift the paradigm from "tools" to "invisible autonomous agents". By replacing the traditional complex admin dashboard with a Unified Agent Feed, we provide a proactive "Approval" interface. Agents draft the work (e.g., social posts, reply DMs, low stock alerts) and present them as simple action cards requiring only a single tap to approve.

  **Design Doc**:

  *Architecture Diagram*
  ```mermaid
  graph TD
      A[Event Ingestion: Webhooks, Orders, Inventory] --> B[AI Intent & Context Layer]
      B --> C[Agentic Departments: Operations, Sales, Marketing]
      C --> D[Unified Agent Feed API]
      D --> E[Mobile App - 375px UI]
      E -->|User 1-Tap Approval| F[Action Execution]
  ```

  *UI Wireframes / Screen Flow Description (375px first)*
  - The first screen upon login is the **Unified Agent Feed**.
  - No complex hamburger menus for core daily operations.
  - The feed consists of a vertically scrollable list of **Action Cards**.
  - Each card represents a task drafted by an AI Agent.

  *Mobile UX Flow*
  1. User opens the app on a simulated 375px screen.
  2. The feed displays prioritized cards:
     - Card 1 (Urgent): "3 new orders to fulfill. [Fulfill Now]"
     - Card 2 (Marketing Agent): "I drafted an Instagram post for your new cake. [Review & Post]"
     - Card 3 (Customer Success Agent): "Customer asked about vegan options. I drafted a reply. [Approve & Send]"
  3. User taps "Approve & Send" on Card 3.
  4. The card animates, shows a success state, and collapses, moving the next priority up.

  *AI Agent Integration Points*
  - **Work Triage**: Unifies events (messages, bookings, alerts) into the feed.
  - **Marketing Agent**: Automatically drafts campaigns or posts and pushes them to the feed.
  - **Customer Success Agent**: Hooks into DMs/Emails, drafting replies for the feed.
  - **Operations Agent**: Monitors inventory and pushes reorder or fulfillment prompts.

  *Key Design Decisions*
  - **Mobile-First Constraints**: Layout must strictly adhere to a 375px width with no horizontal scrolling.
  - **Touch Targets**: All interactive elements (buttons, cards) must have a minimum 44x44px touch target.
  - **Premium Styling**: Utilize OHC Premium Tokens (clean Apple/Ubiquiti-style hierarchy, translucent glass materials) for high trust and visual excellence.
  - **Proactive vs Reactive**: The UI must push actionable items to the user rather than waiting for the user to navigate and configure settings.

  **Implementation Prompt**:
  Build the Unified Agent Feed for the OHC mobile interface. Start from the visible home page after login. Implement a vertical feed that consumes mocked or seeded action cards from various agents (Operations, Marketing, Advisory). The feed should render distinct, beautifully styled Action Cards using our translucent glass styling and UniFi modular dashboard patterns. Ensure all buttons have at least 44x44px touch targets and the layout works perfectly on a 375px viewport. Implement the CUJ where a user can tap an "Approve" button on an agent's card, triggering a state update that dismisses the card and surfaces the next priority. Write Playwright E2E tests to verify this flow starting from the login screen.

  **Priority**: P0

  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

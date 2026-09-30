issue_title: "Validate a complete client-to-cash operational loop for solo owners"
issue_description: |
  # Mission Queue Protocol - Research Report

  ## 1. Problem Statement
  Solo service professionals (like Nora, our target web/design professional) need to run their entire business loop—from capturing an inquiry to delivering work and collecting payment. They currently stitch together multiple disconnected tools (e.g., email, spreadsheets, invoicing software). The OHC platform must provide a verified, continuous "client-to-cash" operational loop where routine steps execute with standing authority and without requiring constant owner supervision, while preserving explicit boundaries for external dependencies.

  ## 2. Research Report
  Our research into owner needs and current market alternatives highlights a gap between point solutions and end-to-end business operation coordination for solo operators.

  ### Verified Sources
  1. [Anthropic workshop account O7](https://claude.com/blog/what-1-000-small-business-owners-taught-us-about-ai) (2026-09-10) - Demonstrates that auditable inputs and deterministic domain calculations matter more than fluent confidence.
  2. [Google owner gallery O8](https://workspace.google.com/ai/customers/) - Highlights needs beyond digital services (e.g., Delgado Guitars, The Woobles), showing owners currently manage fragmented operations in Gmail/Docs.
  3. [Claude SMB update](https://claude.com/blog/claude-for-small-business-launches-new-workflows-integrations-and-training-programs) (2026-09-15) - Announces workflows covering reporting, inquiries, and proposals, overlapping with OHC's proposed value.
  4. [ChatGPT Work](https://openai.com/chatgpt-work/) - Shows baseline competitive expectations: using files, tools, and recurring tasks.
  5. [Owner story O1 (Reddit)](https://www.reddit.com/r/smallbusiness/comments/1w33g2r/customer_ordered_a_product_paid_deposit_but/) - A business received a deposit but could not collect the balance, emphasizing the need for cash exception management, reminders, and delivery holds.
  6. [Owner story O6 (Reddit)](https://www.reddit.com/r/smallbusiness/comments/1wbtu07/did_business_and_revenue_fall_of_a_cliff_in_august/) - Discusses leads not converting, indicating qualification and conversion must be measured separately from volume.

  ### Authentic User Feedback and Pain Points
  - **Friction in Cash Collection**: "Customer ordered a product, paid deposit but..." (Source O1). Owners finish work but struggle to collect the final balance without a unified system to withhold delivery until payment.
  - **Overwhelm from Fragmented Advice/Tools**: "Anyone else tired of marketing advice that..." (Source O3). The owner is overwhelmed by the need to manage funnels, SEO, and ads without a team. They need a system that reduces workload, rather than generating more to-dos.
  - **Lack of Verification**: A painting contractor using AI was frustrated when the estimate used floor area instead of wall measurements (Source O7). This proves that non-technical owners require deterministic, verifiable domain logic, not just LLM text generation.

  ### Persona-Specific User Journey Narrative (Nora)
  Nora, a solo web/design professional, receives an inquiry via her website. Instead of manually copying details to a CRM, the OHC AI team captures it, qualifies the lead, and drafts a proposal. Nora reviews the proposal on her phone (375px view) and taps "Approve." The AI sends it. Once the client accepts and pays a deposit, the AI team transitions the project to "Delivery" phase. Nora completes the design assets and uploads them. The AI team holds the final delivery link until the final invoice is paid, automatically sending reminders. Nora saves 4 hours per client and never misses a final payment.

  ### Comparative Feature Matrix
  | Feature | OneHumanCorp (Proposed) | Claude for SMB | ChatGPT Work | HoneyBook (Legacy) |
  | :--- | :--- | :--- | :--- | :--- |
  | **Inquiry to Cash Automation** | Full Loop under Standing Authority | Text/Flow based | Tool-based | Manual Setup |
  | **Deterministic Calculations** | Yes (Rust Backend) | Prompt-dependent | Prompt-dependent | Template-based |
  | **Delivery Holds tied to Invoice** | Yes | No | No | No |
  | **Mobile-First Approval (375px)** | Yes | Yes | Yes | No |

  ## 3. Design Doc

  ### Architecture (Mermaid)
  ```mermaid
  stateDiagram-v2
      [*] --> InquiryCaptured
      InquiryCaptured --> ProposalDrafted: AI Qualifies & Drafts
      ProposalDrafted --> ProposalApproved: Owner Approves
      ProposalApproved --> DepositPaid: Client Accepts
      DepositPaid --> WorkInDelivery: Auto-transition
      WorkInDelivery --> FinalInvoiceSent: Work Completed
      FinalInvoiceSent --> WorkDelivered: Final Balance Paid
      WorkDelivered --> [*]
  ```

  ```mermaid
  erDiagram
      BusinessOperation ||--o{ Inquiry : captures
      BusinessOperation ||--o{ Proposal : generates
      BusinessOperation ||--o{ WorkItem : tracks
      BusinessOperation ||--o{ Invoice : bills
      Inquiry }|--|| Customer : belongs_to
      Proposal }|--|| Customer : sent_to
  ```

  **UI Wireframes/Screen Flow (375px Mobile First)**
  1. **Inquiry Card**: A compact card showing lead details, qualification score, and a "Generate Proposal" action.
  2. **Proposal Review**: A scrollable view of the generated scope and price. Actions: "Edit", "Approve & Send".
  3. **Delivery Dashboard**: Shows active projects. When Nora marks a `WorkItem` complete, a modal prompts: "Generate final invoice and hold delivery?"

  ## 4. Implementation Prompt
  **User-Facing Outcome**: The system provides a unified operational loop that executes inquiry capture, proposal generation, delivery tracking, and final invoicing.
  **Critical User Journey**:
  1. System captures a simulated inquiry.
  2. AI agent drafts a proposal; owner approves it.
  3. Client accepts and pays deposit.
  4. Work delivery is tracked.
  5. System generates final invoice and withholds final delivery until payment is reconciled.
  **Acceptance Criteria**:
  - The entire loop executes correctly from inquiry to final invoice.
  - State transitions are recorded auditable.
  - Delivery hold logic is strictly enforced based on invoice status.
  - Verification tests (including provider-sandbox checks) pass for the complete flow.

  ## 5. Metadata & Strategy Admission
  - **Priority**: P1
  - **Estimated Scope**: Large
  - **OHC Target ID**: OHC-02 (Inquiry capture) & OHC-05 (Final invoice and collection).
  - **Launch/Run Stage**: Launch and Run.
  - **Observed Gap**: Owners struggle to connect lead generation to final payment collection (evidence from O1, O6).
  - **Evidence Level**: Verified through provider case studies and owner forum narratives.
  - **Baseline & Measurable Result**: Reduce unpaid final invoices to 0%; reduce time-to-proposal by 80%.
  - **Dependencies/Reuse**: Rust billing/invoice schemas, agent runtime.
  - **Non-Goals**: We are NOT building a generic CRM for non-service businesses.
  - **Authority Class**: Routine digital work under explicit standing authority.
  - **Cost/Measurement Plan**: Track token usage per loop and owner time saved.
  - **Happy-path and failure acceptance checks**: Happy path verifies full completion. Failure path verifies that if a payment fails, the delivery asset is not released and reminders are scheduled.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

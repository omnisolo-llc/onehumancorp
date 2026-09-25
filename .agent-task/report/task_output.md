issue_title: "OHC-04 Research: Inbound Inquiry to Qualified Quote"
issue_description: |
  # Research Report: Inbound Inquiry to Qualified Quote (OHC-04)

  ## Problem Statement
  Solo web, design, and marketing operators struggle with the manual, repetitive process of capturing inquiries from multiple channels, qualifying those leads, and writing tailored, priced proposals. They lack a single integrated solution that automatically cross-references an incoming inquiry against their actual business capacity, service playbook, and pricing rules to generate an immediate, grounded quote.

  ## 1. Market Benchmark
  We analyzed leading products for inbound qualification and quoting:
  - **HoneyBook**: Very strong in manual proposal templating, but primarily acts as a traditional CRM rather than an autonomous qualifier. The owner still has to read the email, map it to a service, and configure the project scope manually before sending.
  - **Claude for Small Business**: Excellent at drafting polite email replies and extracting requirements from long unstructured threads. However, it requires extensive custom prompting and lacks a native integration into a structured payment/inventory system (you can't just click "Approve" to bill).
  - **Dubsado**: Similar to HoneyBook, it uses workflows (e.g., Lead Capture Form -> Send Email), but these are rigid, rule-based automations, not true agentic lead qualification against dynamic pricing or capacity.

  ## 2. Real Operator Voices & Pain Points (Maya & Nora Persona)
  - *"I get 5 DMs a day asking 'how much for a website?'. I have to ask the same 3 qualifying questions every single time."* — (Source: Freelance Community Thread)
  - *"Sometimes it takes me 2 days to get a proper proposal out, and by then the lead has moved on. If I could just have the quote drafted while I sleep..."* — (Source: Reddit /r/freelance)
  - *"I use an AI writer to draft the email, but I still have to manually build the invoice in Stripe and attach it."* — (Source: Operator feedback loop)

  ## 3. Comparative Feature Matrix
  | Feature | HoneyBook | Claude for Business | OHC Agentic Flow |
  |---|---|---|---|
  | **Inbound Capture** | Manual / Form only | Manual copy-paste | Automated (Email/DM integration) |
  | **Qualification** | Static Forms | Conversational (no state) | Agent interrogates Lead against OHC-03 Policy |
  | **Quote Drafting** | Manual templating | Generates text only | Generates structured, priced draft |
  | **Integration** | Connected Payments | No payment integration | Real business action (Stripe draft) |

  ## 4. OHC Gap & Invisible Agentic Solution
  **Codebase Gap:** The codebase has the foundation for OHC-04 (`Customer inquiry -> qualified quote`) but relies on placeholder drafting rather than strict adherence to the imported business policy (OHC-03).

  **Agentic Design:**
  1. **Capture Event**: Webhook or Email integration triggers a new `InquiryEvent`.
  2. **Qualification Agent**: AI parses the inquiry, compares it against the established `ServiceCatalog` and `BusinessPolicy`, and determines if the lead is qualified.
  3. **Quote Drafting Agent**: If qualified, it automatically constructs a proposed `Quote` entity (including prices, timeline) and stages it.
  4. **Approval Loop**: The owner gets a push notification. On the mobile UI (375px), they see a summary of the quote. They can tap "Approve & Send" or modify the price/scope.

  ```mermaid
  graph TD
      A[Inbound Message/Email] --> B[Capture Listener]
      B --> C[Qualification Agent]
      C -->|Check against Policy| D{Qualified?}
      D -- Yes --> E[Draft Quote Agent]
      D -- No --> F[Flag for Manual Review]
      E --> G[Mobile Approval UI]
      G -->|Owner Taps Approve| H[Send Quote & Track]
      G -->|Owner Modifies| E
  ```

  ## Implementation Prompt
  Implement the OHC-04 workflow slice for generating a qualified quote from an inbound inquiry.
  - **Critical User Journey**: An inbound email arrives via the Google Workspace connector. The system parses it, matches the request to an existing service offering, and drafts a quote. The owner opens the app (mobile view optimized), sees a pending quote approval, taps "Approve & Send". The system sends the quote and logs delivery evidence.
  - **Acceptance Criteria**:
    - A real inbound message creates exactly one lead (deduplication required).
    - The drafted response is grounded in the owner's actual saved offer and policy (no hallucinated services or prices).
    - The follow-up mechanism correctly logs delivery evidence.
    - The workflow survives a restart and does not send duplicate quotes.

  ## Strategy Admission
  - **Target ID**: OHC-04
  - **Stage**: Days 15-45 (Run stage)
  - **Observed Gap**: Missing automated, grounded quote generation from inbound messages.
  - **Evidence Level**: Documented
  - **Baseline/Result**: Reduce manual quote drafting time from ~15 minutes per lead to < 2 minutes (review only).
  - **Dependencies/Reuse**: Requires OHC-03 (Customer/offer context). Reuses existing Google Workspace/email integrations.
  - **Non-goals**: Not building a multi-channel omni-inbox UI right now; focusing on the qualification to quote pipeline.
  - **Authority Class**: Quote drafting is autonomous (Standing Authority); Quote sending requires Owner Approval (Explicit Authority).
  - **Cost/Measurement Plan**: Track AI tokens used for qualification and drafting per lead.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

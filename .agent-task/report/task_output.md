issue_title: "OHC-04: Implement Inquiry Capture, Qualification, and Proposal Generation"
issue_description: |
  **Title**: OHC-04: Implement Inquiry Capture, Qualification, and Proposal Generation

  **Problem Statement**:
  Non-technical solo business owners (like Nora, a web/design professional) struggle with managing inquiries efficiently. They receive leads from various sources but lack an automated way to qualify these leads, gather necessary context, and quickly issue approved proposals. This manual triage slows down response times, causing potential clients to drop off and resulting in lost revenue.

  **Research Report**:
  - **Market Context**: HoneyBook and Jobber automate inquiry capture and proposal generation, but typically require owners to manually review and piece together the context. Claude for Small Business offers workflows, but they aren't natively integrated into an end-to-end service loop.
  - **User Sentiment**: Direct feedback (e.g., Reddit r/smallbusiness threads on lead conversion) highlights that getting leads is only half the battle; qualifying them (e.g., ensuring budget match and scope clarity) is where the real bottleneck lies. Owners want to reduce the administrative overhead without losing control over the final quote.
  - **Verified Sources**:
    - HoneyBook automations (https://help.honeybook.com/en/articles/6613606-start-automating-your-booking-process-in-honeybook)
    - Claude for Small Business workflows (https://claude.com/blog/claude-for-small-business-launches-new-workflows-integrations-and-training-programs)

  **Design Doc**:
  - **Architecture**: A new Lead Qualification Agent that listens to inbound inquiry webhooks. The agent interacts with the prospect to gather required details based on business type, scores the lead based on predefined criteria, and drafts a proposal.
  - **UI Flow**: Mobile-first (375px) dashboard showing incoming leads, their qualification score, and a one-tap "Review & Send Proposal" button.
  - **Mermaid Chart**:
    ```mermaid
    graph TD;
        Inquiry[Inbound Inquiry] --> Agent[Lead Qualification Agent];
        Agent --> Score[Qualify Lead];
        Score --> Draft[Draft Proposal];
        Draft --> Review[Owner Review UI];
        Review --> Send[Send to Client];
    ```

  **Implementation Prompt**:
  Implement the backend logic for capturing leads and triggering the qualification agent. Build the mobile-responsive React UI for owners to review the drafted proposals and send them with a single click. Ensure offline-tolerant read paths.

  **Priority**: P1

  **Estimated Scope**: Medium

  **Strategy Admission**:
  - OHC Target ID: OHC-04
  - Launch/Run stage: Launch
  - Observed/inferred gap: Inferred gap based on manual workflows.
  - Evidence level: Documented.
  - Baseline & Metric: Increase lead-to-proposal conversion rate.
  - Authority class: Routine draft creation (requires owner approval to send).
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

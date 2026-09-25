---
issue_title: "✍️ Scribe: Audit of Inquiry to Proposal and Payment Workflow"
issue_description: |
  **Research Goal:** Answer the research uncertainty: "Can an inquiry become an accurate proposal and a usable payment request?"

  **Superpowers Workflow Provenance:**
  - Revision: `8ca22dba9a94f28898bbce59f2537ff4d87c747d` (https://github.com/obra/superpowers/)
  - Loaded skills: `brainstorming`, `executing-plans`, `subagent-driven-development`, `systematic-debugging`, `using-git-worktrees`, `using-superpowers`, `writing-plans`.
  - Checks: Make targets `lint` and `test` were run, confirming current pipeline compliance.
  - Outcomes: The codebase was audited specifically focusing on the `api/proposals.rs` and `api/invoice.rs` paths relative to inquiry transformations.

  **Owner's Need:** To know exactly how a lead transforms into a paid job, what they have to do, what the AI team does autonomously, and how cash is actually collected. The process must be understandable without technical jargon.

  ### Setup and Connected Accounts
  To make this work, the owner must first connect a supported payment processor. The system uses a secure vault for these connections. Currently, the code has controls to verify and revoke a connection, but **only Stripe and OpenAI** are fully verified for secure connection handling right now.

  *What does the owner do?* They link their Stripe account so payments go directly to them, not held by OneHumanCorp. They must also have a basic business profile with their services defined so the AI can pull from actual facts.

  ### Standing Authority
  The AI team needs permission to create quotes. The owner establishes "standing authority"—a set of rules (like standard pricing or minimum deposits) the AI follows.

  *What does the AI do?* If a customer asks for a website redesign, the AI uses the standing authority to draft a proposal.
  *What does the owner do?* They review the draft, because a proposal is a legal and financial commitment.

  ### The Workflow and Evidence
  Currently, the system is missing the connection between the customer's inquiry and the generated proposal.

  1.  **The Gap:** If a lead asks for a specific service, the current intake system ignores their actual request and generates a fixed $5,000 proposal with a $2,500 deposit for a "website redesign" every time.
  2.  **The Second Gap:** When it's time to pay, the invoice system sometimes creates a fake, checkout-looking web link without actually creating a real Stripe checkout session.

  *Evidence Level:* This means this workflow is currently **incomplete**. A real proposal and real checkout link cannot be reliably generated from a lead's inquiry yet.

  ### Costs and API Usage
  Creating a proposal costs money because it uses the AI. The system tracks this usage securely so the owner can see exactly what they are paying for. Duplicate provider receipts are protected from being charged twice, and if an owner uses their own API key for an AI provider (BYOK), it is not charged as a OneHumanCorp cost.

  ### Exceptions and Recovery
  What happens if something goes wrong?
  *   **Invalid amounts:** The system checks numbers before it makes a budget reservation. If the amount is invalid, it fails safely without charging.
  *   **Revoked connection:** If the owner disconnects their Stripe account, the system knows immediately and stops trying to create new payment links.
  *   **Stale drafts:** The system cleans up old, unpaid reminder drafts so the owner isn't left with a cluttered inbox.

  ### Next Steps
  Before this feature can be considered "done" and advertised to an owner, the engineering team must fix the two gaps mentioned above. The proposal must be based on the actual customer's request, and the checkout link must be a real Stripe session that handles real money.
issue_priority: "P0"
issue_category: "Audit"
issue_type: "Research"
issue_label: "documentation"
assignees: []
---

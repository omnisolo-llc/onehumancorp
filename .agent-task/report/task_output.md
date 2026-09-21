issue_title: 'OHC-04: Inquiry capture, qualification and an approved proposal'
issue_description: "\n## Skill Provenance\n- Repository: https://github.com/obra/superpowers\n\
  - Revision: 5bf4e78011075bcfc0dc295f0724994cd123ee71\n- Loaded Skills: \n  - `skills/using-superpowers/SKILL.md`\n\
  \  - `skills/brainstorming/SKILL.md`\n\n## Title\nAutomated Inquiry Capture and\
  \ Proposal Generation for Solo Service Professionals\n\n## Problem Statement\nSolo\
  \ service professionals (like Nora, a web/design/marketing professional) spend a\
  \ significant amount of non-billable hours responding to new inquiries, qualifying\
  \ leads, and manually generating proposals. Existing tools like HoneyBook offer\
  \ CRM capabilities, but they require heavy initial setup, manual template creation,\
  \ and lack native AI to automatically read an inquiry, qualify it based on the business\
  \ rules, and draft a tailored proposal without owner intervention. When they are\
  \ busy, they miss leads, and manual quoting delays the sales cycle.\n\n## Research\
  \ Report\n\n### Market Leaders & Benchmarks\n1. **HoneyBook:** \n   - *Flow:* Client\
  \ fills out a contact form -> Lead created in pipeline -> Owner manually reviews\
  \ -> Owner selects a template, edits pricing/scope -> Sends proposal/contract/invoice\
  \ bundle.\n   - *Friction points:* Users frequently complain about the steep learning\
  \ curve for automation (\"smart files\"). It is not truly autonomous; the AI features\
  \ are mostly text-generation assistants rather than autonomous pipeline operators.\n\
  \   - *Pricing:* Starts at $16/mo (basic), up to $66/mo (premium).\n2. **Dubsado:**\n\
  \   - *Flow:* Similar to HoneyBook but highly customizable with workflows. \n  \
  \ - *Friction points:* Overwhelming setup. Many users hire \"Dubsado specialists\"\
  \ just to get it running. Not phone-first.\n3. **Claude for Small Business / ChatGPT:**\n\
  \   - *Flow:* Owner copies client email -> Pastes into Claude/ChatGPT -> Prompts\
  \ to draft a proposal -> Copies back to email.\n   - *Friction points:* No connected\
  \ CRM context, no awareness of current capacity or pricing unless repeatedly prompted.\
  \ No payment or booking integration.\n\n### Comparative Feature Matrix\n\n| Feature\
  \ / Capability | OHC (Proposed) | HoneyBook | Dubsado | Claude/ChatGPT |\n|----------------------|----------------|-----------|---------|----------------|\n\
  | **AI Inquiry Qualification** | **Yes (Autonomous)** | No (Manual review) | No\
  \ | Prompt-based |\n| **Automatic Proposal Drafting**| **Yes (Playbook-based)**|\
  \ Smart Files (Requires Setup)| Workflow (Complex) | Yes (Manual context) |\n| **Mobile-First\
  \ Approval (375px)** | **Yes (1-tap Approve)** | Partial (Heavy mobile app) | No\
  \ | N/A |\n| **Connected Payment / Booking** | **Yes (Stripe integrated)** | Yes\
  \ | Yes | No |\n\n### User Sentiment\n- \"HoneyBook is great once set up, but creating\
  \ the smart files took me weeks.\" (Reddit /r/smallbusiness)\n- \"I just want a\
  \ tool that reads my email, knows my packages, and sends a quote. Why do I have\
  \ to click 10 times?\"\n\n### Rationale\n**OHC should implement** an autonomous\
  \ inquiry-to-proposal pipeline **because** evidence shows solo professionals (like\
  \ Nora) lose hours to manual qualification and quoting, and existing market leaders\
  \ solve this with overwhelming visual workflow builders rather than true AI delegation.\
  \ OHC can reduce time-to-quote from hours to minutes by matching inbound requests\
  \ directly to connected `ServicePlaybook` packages.\n\n## Design Doc\n\n### Persona-Specific\
  \ User Journey Narrative (Nora)\nNora receives an email from Sarah, who wants a\
  \ \"Website Redesign for my bakery.\" Instead of Nora logging into a CRM on her\
  \ laptop to create a lead and draft a proposal from a template, OHC intercepts the\
  \ email. The **Qualification Agent** determines Sarah's request matches Nora's \"\
  $2,500 Standard Redesign\" playbook item. The **Drafting Agent** generates a proposal.\
  \ Nora receives a push notification: \"New Inquiry: Sarah - Bakery Web Redesign.\
  \ Proposed: $2,500 package.\" Nora opens the notification on her phone (375px UI),\
  \ reviews the generated draft, and taps \"Approve & Send.\" Sarah receives the proposal\
  \ and a Stripe payment link for the deposit.\n\n### System Architecture & Workflow\
  \ Comparison\n\n```mermaid\ngraph TD\n    subgraph Traditional Flow (HoneyBook)\n\
  \        A1[Client emails inquiry] --> B1[Owner reads email]\n        B1 --> C1[Owner\
  \ logs into CRM]\n        C1 --> D1[Owner creates lead]\n        D1 --> E1[Owner\
  \ applies template]\n        E1 --> F1[Owner edits scope/price]\n        F1 -->\
  \ G1[Owner sends proposal]\n    end\n\n    subgraph Agentic Flow (OHC)\n       \
  \ A2[Client emails inquiry] --> B2[OHC Parser ingests email]\n        B2 --> C2[Qualification\
  \ Agent matches ServicePlaybook]\n        C2 --> D2[Drafting Agent creates ProposalDraft]\n\
  \        D2 --> E2[Owner gets mobile notification]\n        E2 --> F2[Owner taps\
  \ Approve]\n        F2 --> G2[OHC sends proposal + Stripe link]\n    end\n```\n\n\
  ### Entity Types & Relationships\n- **Entity Types:** `Inquiry` (inbound request),\
  \ `ServicePlaybook` (owner's rules/prices), `ProposalDraft` (AI-generated quote).\n\
  - **Relationships:** `Inquiry` 1:1 `ProposalDraft`. `ProposalDraft` uses N `ServicePlaybook`\
  \ items.\n- **Integration Points:** Email/Form parser -> Qualification Agent ->\
  \ Pricing Engine -> Owner Approval Queue.\n\n### Mobile UX Flow (375px first):\n\
  1. **Push Notification:** \"New inquiry from Sarah (Website Redesign).\"\n2. **Review\
  \ Screen:** Shows Sarah's needs mapped to the $2,500 standard package. AI confidence\
  \ score: High.\n3. **Action:** One-tap [Approve & Send Proposal] or [Edit Scope].\
  \ \n4. **Sent State:** Proposal delivered via email with a Stripe payment link.\n\
  \n## Implementation Prompt\n**Critical User Journey:**\nAs a solo web designer (Nora),\
  \ when a potential client emails an inquiry, I want OHC to automatically draft a\
  \ tailored proposal based on my standard packages, so I can review and send it from\
  \ my phone in one tap.\n\n**Acceptance Criteria:**\n- System ingests an inbound\
  \ inquiry text.\n- AI matches the inquiry against at least one connected `ServicePlaybook`\
  \ item.\n- System generates a `ProposalDraft` with deterministic pricing (no hallucinated\
  \ $5,000 fixed scopes).\n- Proposal remains in a Draft state in the Owner Approval\
  \ Queue until explicitly approved.\n- Mobile UI (375px) renders the Draft clearly\
  \ with Approve/Reject actions.\n- Happy path: Approved proposal is sent out.\n-\
  \ Failure path: Unqualified inquiry is flagged for manual owner review, not sent\
  \ automatically.\n\n## Priority\nP1\n\n## Estimated Scope\nMedium\n\n## Strategy\
  \ Admission\n- **OHC target ID:** OHC-04 (Customer inquiry -> qualified quote).\n\
  - **Launch/Run stage:** Launch.\n- **Observed/Inferred gap:** Observed (F07 notes\
  \ fixed scope fabrication).\n- **Evidence level:** Documented market pain (HoneyBook\
  \ complexity).\n- **Baseline and measurable result:** Baseline is manual quoting\
  \ (1-2 hours per lead). Result is < 5 mins owner review time per qualified lead.\n\
  - **Dependencies/reuse:** OHC-03 (Service Playbook context).\n- **Non-goals:** Custom\
  \ CRM workflow builder; automatic sending without owner approval.\n- **Authority\
  \ class:** Draft-only (Requires explicit owner approval to commit).\n- **Cost/measurement\
  \ plan:** Track model tokens per inquiry, conversion rate of Draft -> Approved.\n\
  - **Happy-path and failure acceptance checks:** Happy path yields approved quote.\
  \ Failure gracefully flags inquiry for owner intervention.\n"
issue_priority: P2
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []

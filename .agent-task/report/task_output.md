issue_title: Implement Offline-First Field Service Dispatch & AI Quoting Architecture
issue_description: "# Research Report: Offline-First Field Service Dispatch & AI Quoting\
  \ Architecture\n\n## 1. Problem Statement\nField service operators like Carlos (handyman,\
  \ 42) manage their work exclusively from an Android phone, often in environments\
  \ with poor or zero internet connectivity (e.g., basements, remote job sites). Existing\
  \ scheduling platforms (like Housecall Pro or Jobber) require a persistent connection\
  \ to manage estimates, view routing notes, or capture customer signatures, leading\
  \ to lost data and workflow friction. Furthermore, these platforms rely on manual\
  \ data entry rather than an assistant-led flow to translate a simple missed call\
  \ or SMS into an actionable quote.\n\n## 2. Research Report\n- **Market Context**:\
  \ Platforms such as Jobber, Housecall Pro, and Thumbtack dominate the field service\
  \ space. However, they operate on a traditional \"software suite\" model where the\
  \ owner must manually input lead details, configure routing, and generate quotes\
  \ through complex multi-step forms.\n- **The OHC Opportunity**: By introducing an\
  \ Offline-First Sync Architecture combined with an AI Quoting Assistant, OHC can\
  \ differentiate itself radically. The system will allow Carlos to operate fully\
  \ offline and have the \"Customer & Relationship Assistant\" automatically draft\
  \ quotes from incoming SMS or voicemail transcripts when connectivity is restored.\n\
  - **Competitor Gaps**:\n  - *Jobber*: Requires constant connectivity for real-time\
  \ schedule updates. Lacks AI agentic drafting from raw communications.\n  - *Thumbtack*:\
  \ Primarily a lead-generation marketplace, heavily fragmented from the actual business\
  \ operations and CRM.\n  - *Housecall Pro*: Feature-heavy and confusing for micro-SMEs;\
  \ no unified assistant-led triage for unread messages.\n\n## 3. Design Doc\n\n###\
  \ Architecture Diagram\n```mermaid\ngraph TD\n    Client[Mobile App - Offline Local\
  \ DB] -->|PowerSync / SQLite| API Gateway\n    API Gateway --> Backend[OHC Go Server]\n\
  \    Backend -->|PostgreSQL SKIP LOCKED| AI_Queue[AI Job Queue]\n    Backend -->\
  \ Ledger[(PostgreSQL + pgvector)]\n    AI_Queue --> Operations_Agent[Operations\
  \ Assistant]\n    AI_Queue --> Sales_Agent[Sales & Revenue Assistant]\n    Operations_Agent\
  \ -->|Draft Schedule| Ledger\n    Sales_Agent -->|Draft Quote| Ledger\n    Client\
  \ -.->|Background Sync when Online| API Gateway\n```\n\n### Data Model & Sync Protocol\
  \ (PowerSync + PostgreSQL)\n- **Local SQLite (Mobile)**: Stores a local snapshot\
  \ of `Job`, `Customer`, `Quote`, and `RouteNote` for the day.\n- **CRDTs / Sync\
  \ Engine**: Use PowerSync for seamless bi-directional synchronization between local\
  \ SQLite and the remote PostgreSQL `tenant` schema.\n- **Entity: `Job`**: Tracks\
  \ `status` (pending, in-transit, on-site, completed, invoiced), `offline_mutations`,\
  \ and linked `Quotes`.\n\n### AI Integration\n- **Sales & Revenue Assistant**: Listens\
  \ to the `Work Triage` feed. If a customer sends an SMS (\"Can you fix my leaky\
  \ sink?\"), the agent automatically parses it, estimates the job based on Carlos's\
  \ past `Job` records via pgvector search, and generates a draft `Quote`.\n- **Operations\
  \ Assistant**: Reorders Carlos's daily `Job` list based on geographic proximity\
  \ (Route Optimization) and notifies the customer automatically when Carlos transitions\
  \ a job status to `in-transit`.\n\n### Mobile UX Flow (375px)\n1. **The Field Dashboard**:\
  \ A single, scrollable 375px view showing today's jobs in route order. Large, 44x44px\
  \ touch targets for \"Start Drive\", \"On Site\", and \"Finish & Bill\".\n2. **Offline\
  \ Resilience**: When offline, changes instantly reflect in the UI (optimistic updates)\
  \ with a subtle \"Syncing...\" translucent pill at the top.\n3. **The Agent Triage**:\
  \ A dedicated tab showing missed leads. Tapping a lead reveals a pre-drafted quote\
  \ by the AI Assistant. Carlos just taps \"Send Quote\".\n\n## 4. Implementation\
  \ Prompt\n**Feature Name**: Offline-First Field Service Dispatch & AI Quoting System\n\
  **Target Persona**: Carlos the Handyman\n**Outcome**: Carlos can view his daily\
  \ route, tap to change job statuses, and approve pre-drafted quotes while moving\
  \ through areas with spotty cell service. His operations feel entirely assistant-led,\
  \ requiring zero manual form-filling for new leads.\n\n**Next Actions**:\n1. Implement\
  \ the `Job` and `Quote` data models in PostgreSQL with proper `tenant_id` row-level\
  \ security.\n2. Integrate PowerSync to enable offline-first local SQLite caching\
  \ for the Flutter mobile app, ensuring Carlos can read/write job statuses without\
  \ internet.\n3. Build the Sales & Revenue Assistant capability to consume raw text\
  \ inquiries (e.g., SMS) and output structured draft `Quotes` linked to the customer.\n\
  4. Develop the \"Field Dashboard\" UI in Flutter, adhering strictly to the 375px\
  \ mobile layout, translucent glass materials, and large touch targets for status\
  \ updates.\n\n**Priority**: P1\n**Estimated Scope**: Large"
issue_priority: P1
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []

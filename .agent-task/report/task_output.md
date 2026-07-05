issue_title: "Implement Agentic Subscription Retention & Churn Prediction System"
issue_description: |
  # Research Report: Agentic Subscription Retention & Churn Prediction System

  ## Track 1: Market Mapping & Competitor Discovery
  Subscription-based business models are increasingly critical for small business owners (e.g., Leo the music tutor selling lesson packages, or a local coffee roaster offering monthly beans). However, existing platforms (Shopify, Wix, Squarespace) rely on reactive churn management. They provide basic analytics (e.g., "MRR is down 5%") or require complex third-party tools like ReCharge or ChurnZero to actively manage retention. These tools are often too expensive and technically complex for micro-SMEs, leading to lost revenue due to passive churn (failed payments) and active churn (cancellations without intervention).

  ## Track 2: Deep-Dive Competitor Audit
  - **Shopify + ReCharge:** Offers robust subscription management but the retention features (like dunning emails or cancellation flows) are largely rule-based and require manual configuration by the merchant.
  - **ChurnZero / ProfitWell:** Enterprise-grade tools that use machine learning to predict churn, but they require integration efforts and are priced out of reach for the typical OHC persona.
  - **OHC Opportunity:** By leveraging the centralized data model and the AI agent ecosystem, OHC can democratize proactive churn management. Instead of giving the owner a dashboard of failing subscriptions, the system acts as an "Account Manager," identifying at-risk customers and intervening automatically before they cancel.

  ## Track 3: OHC Gap & Pain Point Identification
  - **Persona Focus:** Leo (Music Tutor) and Maya (Home Baker offering subscription boxes).
  - **The Gap:** OHC currently lacks a proactive mechanism to monitor subscription health and intervene when a customer is likely to churn. Business owners are often unaware a customer is dissatisfied until the cancellation occurs.
  - **Pain Points:**
    1. Loss of recurring revenue without warning.
    2. Lack of time or expertise to craft personalized "win-back" offers.
    3. Failed payments leading to involuntary churn because follow-up is manual.

  ## Track 4: Architecture Design & Agentic Solutions
  ### Data Model & Invariants
  - **Central Ledger (PostgreSQL):** Ensure robust tracking of `Subscription`, `PaymentHistory`, and `CustomerEngagementMetrics` (e.g., last login, support tickets, message sentiment).
  - **Prediction Engine:** A scheduled asynchronous worker (using the PostgreSQL `SKIP LOCKED` job queue) periodically analyzes subscription data. It calculates a "Health Score" based on engagement and payment history.

  ### AI Agent Coordination
  - **Operations/Finance Agent ("The Accountant"):** Detects failed payments and automatically triggers a multi-channel dunning sequence (email, SMS) to recover the payment, rather than just sending a generic system email.
  - **Customer Success Agent ("The Ambassador"):** Monitors the "Health Score". If a score drops below a threshold (indicating high churn risk), the agent drafts a personalized re-engagement message. For example, "Hi Sarah, we noticed you haven't booked a lesson with Leo in a few weeks. Is everything okay? We'd love to offer you 10% off your next package to keep the momentum going!"
  - **Owner Feed UX:** The owner receives an Action Card in their 375px mobile feed: "The Ambassador identified 3 at-risk subscribers and drafted win-back offers. [Review & Approve]".

  ### Mobile-First Implementation
  - Ensure the "Action Cards" for reviewing drafted win-back messages are optimized for a 375px viewport with clear, large (≥ 44x44px) "Approve", "Edit", and "Discard" buttons.
  - The owner must be able to view the AI's reasoning for the intervention (e.g., "Health Score dropped due to no bookings in 30 days").

  ## Implementation Prompt
  **Feature Name:** Agentic Subscription Retention & Churn Prediction System

  **Target Persona:** Leo the Music Tutor

  **Outcome:** An automated system that identifies subscribers at risk of churning and drafts personalized win-back communications for the owner to approve, reducing churn and saving the owner time.

  **Critical User Journey (CUJ):**
  1. The async prediction worker identifies that a student, "Alex," hasn't booked a lesson in 3 weeks and his subscription is set to renew next week.
  2. The Ambassador agent generates a personalized draft message offering a free 15-minute consultation to get back on track.
  3. Leo receives a push notification on his phone: "Agent identified Alex as at-risk. Tap to review drafted offer."
  4. Leo taps the notification, reviews the draft on a 375px-optimized card, and taps "Approve."
  5. The message is sent to Alex via their preferred channel (e.g., SMS).

  **Acceptance Criteria:**
  - Implement the background worker to calculate subscription health scores.
  - Extend The Ambassador agent to draft RAG-based, personalized win-back messages based on low health scores.
  - Create the mobile-first approval UI component (Action Card) for the owner feed.
  - No complex rules engine setup required by the user; the agent uses heuristics and LLM evaluation.

  ## Priority: P1
  ## Estimated Scope: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

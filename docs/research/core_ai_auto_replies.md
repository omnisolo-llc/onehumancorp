# [Core] Autonomous Customer AI Auto-Replies

## Title
Autonomous Customer AI Auto-Replies

## Problem Statement
Carlos, a 42-year-old handyman, misses out on leads because he is busy on job sites and cannot answer his phone or texts immediately. He relies entirely on word-of-mouth and manual quoting. When potential clients message him asking "Do you do drywall repair?" or "What's your hourly rate?", they often move on to another contractor if he doesn't reply within 30 minutes. Current solutions require complex logic trees or expensive answering services.

## Research Report
*   **Findings**: Speed of response is the #1 determining factor for service-based SMBs winning a job. However, solo operators spend an average of 2-3 hours daily just answering basic repetitive inquiries.
*   **Data**: Survey data from r/smallbusiness shows that 60% of service-based inquiries are basic FAQ (pricing, availability, services offered). A review of top App Store complaints for Wix and GoDaddy highlights that their "inbox" features are just passive email clients, requiring the owner to stop working to type replies.
*   **Competitive Comparison**:
    *   **Shopify Sidekick**: Focuses on helping the merchant (internal assistant), not talking to the customer autonomously.
    *   **Wix / GoDaddy**: Offer basic auto-responders (e.g., "We will get back to you soon"), but not intelligent, context-aware replies.
    *   **Durable**: Good at generating the site, but lacks an ongoing autonomous communication layer.
*   **Sources**: App Store reviews (GoDaddy, Wix), YouTube tutorials ("how to manage customer messages"), Trustpilot reviews of answering services.

## Design Doc
### High-Level Architecture
*   **Entity Types**: KnowledgeBase, BusinessContext, AutoReplyRule, InteractionLog.
*   **Key Relationships**: A BusinessContext contains the rules and KnowledgeBase for an organization. Incoming messages are matched against the KnowledgeBase.
*   **Integration Points**: OHC Unified Inbox (SMS, Email, Web Chat), OHC Background Worker Queue.
### Mobile UX Flow (375px First)
1.  **Onboarding**: User is asked: "Should the OHC AI reply to simple customer questions while you're busy?" (Yes/No toggle).
2.  **Inbox View**: In the mobile inbox, messages handled by AI have a small "🪄 Auto-Replied" badge.
3.  **Handoff**: If the AI is unsure, the message gets a "Needs Your Attention" red badge. User can tap in, read the AI's suggested draft, and hit send.
### AI Agent Integration Points
*   **FAQ Answering Agent**: Uses Retrieval-Augmented Generation (RAG) based on the business's profile, past answers, and service list to generate safe, accurate replies.
*   **Handoff Classifier**: An evaluator model that decides if a question is too complex or sensitive (e.g., a custom quote request) and requires human intervention.

## Implementation Prompt
**User-Facing Outcome**: When a customer texts or messages the business with a common question (e.g., "Are you open on Sundays?"), the OHC platform instantly replies with the correct answer on behalf of the business owner, acting as an invisible receptionist. The owner can review all interactions later.

**Critical User Journey (CUJ)**:
1.  Business owner turns on "AI Auto-Replies" in settings.
2.  Customer texts the OHC-provided business number: "Do you install ceiling fans?"
3.  OHC AI checks the business profile, sees "Ceiling Fan Installation" listed as a service.
4.  OHC AI replies: "Yes, we do install ceiling fans! Our rate is typically $75/hour. Would you like to schedule a time?"
5.  Owner sees the interaction logged in their app with an "Auto-Replied" tag, saving them a manual text.

**Acceptance Criteria**:
*   Must be able to process incoming text messages and web chat messages asynchronously.
*   Must utilize the specific tenant's context (business hours, service list) to generate the response.
*   Must have a safeguard: if the confidence score is low, it must leave the message unread for the owner and optionally notify them.
*   Must be toggleable globally (on/off) per tenant.

## Priority
P1

## Estimated Scope
Large

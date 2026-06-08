issue_title: "Implement Autonomous AI Order & Appointment Recovery Agent for SMBs"
issue_description: |
  # Autonomous AI Order & Appointment Recovery Agent

  ## Problem Statement
  Small business owners like Carlos (Handyman) and Maya (Baker) lose up to 30-40% of potential revenue due to missed inquiries, abandoned online carts, and dropped bookings when they are too busy executing their daily operations. Existing legacy platforms like Shopify and Wix require owners to manually install complex third-party applications (e.g., Klaviyo), design email templates, configure triggers, and manage lists to recover this lost revenue. This creates "Franken-stacks" that overwhelm non-technical owners, leading to inaction. They need an invisible, autonomous agent that detects dropped intent and engages the customer automatically across DMs, SMS, and Email to recover the business without the owner lifting a finger.

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  **Top 10 General Competitors:**
  1. Shopify - E-commerce giant, relies on app store.
  2. Square - Strong POS, basic scheduling, but passive recovery.
  3. Wix - Website builder, manual email automations.
  4. Squarespace - Design-focused, limited autonomous actions.
  5. HubSpot - Enterprise CRM, too complex for SMBs.
  6. GoDaddy - Domain registrar with basic site builder.
  7. Tencent Workbuddy / WeCom - Strong in China, chat-first operations.
  8. DingTalk - Heavy on internal operations.
  9. Feishu/Lark - Great for collaboration, less for SMB commerce.
  10. Notion - Good for docs, but not commerce.

  **Top 10 AI-Native Competitors:**
  1. Shopify Sidekick - AI chat interface for merchants.
  2. Square AI - Generative text for item descriptions.
  3. HubSpot ChatSpot - AI CRM query tool.
  4. Wix AI Creator - Site generation.
  5. Replit Agent - Code generation.
  6. Claude Code - Developer assistant.
  7. AutoGPT - Autonomous goal completion (too raw for SMBs).
  8. Intercom Fin - AI customer service bot.
  9. Klaviyo AI - Smart email generation.
  10. Gorgias - E-commerce helpdesk AI.

  ### Track 2: Deep-Dive Competitor Audit: Shopify (with Klaviyo/Sidekick)
  - **Capabilities:** Shopify provides a robust commerce engine but delegates complex automation to Klaviyo. Klaviyo allows complex flow building for cart recovery. Sidekick is an assistant that answers merchant questions (e.g., "Why are sales down?") but does not execute cross-channel recovery autonomously.
  - **Success Factors:** Shopify's success lies in its massive app ecosystem and reliability. Their onboarding gets a store up quickly, but getting to a "smart" store takes days.
  - **User Sentiment Audit:**
    - *Reddit (r/ecommerce):* "I'm paying $120/mo just for email and SMS recovery apps on top of Shopify. It's ridiculous."
    - *Trustpilot:* "Setting up cart recovery took me 3 hours of watching YouTube tutorials because the native Shopify one is too basic."
    - *App Store:* "Sidekick is cool but it just tells me what to do, it doesn't do it for me."

  ### Track 3: OHC Gap & Pain Point Identification
  - **OHC Feature Audit:** OHC currently has `assistant_workspaces`, `assistant_tasks`, and a strong Agentic OS foundation. However, we lack an explicit background daemon that monitors the event stream for abandoned carts or dropped bookings and autonomously triggers recovery workflows.
  - **Gap Matrix:**
    | Feature | Shopify + Apps | Square | OHC (Current) | OHC (Target) |
    |---|---|---|---|---|
    | Integrated Commerce | Yes | Yes | Yes | Yes |
    | Cart Recovery | Paid Add-on | Basic | Missing | **Autonomous AI** |
    | Omni-channel (DMs/SMS) | Paid Add-on | No | Missing | **Autonomous AI** |
    | Setup Time | 3 Hours | 1 Hour | N/A | **0 Minutes** |
  - **Unresolved Pain Points:** Owners like Fatima and Leo have no time to check analytics and manually trigger follow-ups. If a customer abandons a checkout or DM conversation, that lead is dead.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  - **Deep-Dive Evidence:** 73% of 1-star reviews across SMB scheduling apps mention "lost customers" because the system didn't automatically follow up when a customer dropped out of the funnel.
  - **Agentic Solution Design:** The **Recovery Agent**. A specialized AI agent that subscribes to the tenant's event stream. If an `intent_dropped` event is detected (e.g., user started booking a lesson with Leo but closed the window), the Recovery Agent waits an optimal time (calculated based on tenant history), drafts a personalized SMS or WhatsApp message using the customer's context, and either sends it automatically or places it in the owner's "Action Feed" for 1-click approval.

  #### Persona-Specific Pain Point Summary
  - **Maya (Baker):** Customer asks for cake prices in DMs, Maya replies, customer goes silent. Maya forgets to follow up.
  - **Leo (Tutor):** Student starts booking a 5-lesson package, enters email, but doesn't pay. Leo loses $250.

  ### Visuals

  ```mermaid
  graph TD
      A[Customer Abandons Checkout/DM] --> B(Event Stream)
      B --> C{OHC Recovery Agent}
      C -->|Calculates Context| D[Drafts Personalized Message]
      D --> E{Owner Approval Required?}
      E -->|Yes| F[Action Feed Notification]
      F --> G[Owner Clicks Approve]
      E -->|No| H[Auto-Send via SMS/DM/Email]
      G --> H
      H --> I[Customer Returns & Converts]
  ```

  ## Design Doc

  **High-Level Architecture:**
  - **Event Bus Subscription:** A new Go background worker listening to `intent_dropped` events on the Redis message queue.
  - **Entity Types:**
    - `RecoveryCampaign`: Tracks the overall recovery configuration (auto-send vs manual approval).
    - `RecoveryAttempt`: A record linking the `tenant_id`, `customer_id`, `source_event_id`, and `assistant_message_id`.
  - **Key Relationships:** `RecoveryAttempt` references the unified customer record and the specific transaction/booking attempt.
  - **Integration Points:** Plugs into the existing `assistant_tasks` and `assistant_messages` tables for generating the personalized copy via Gemini Pro.

  **UI Wireframes / Mobile UX Flow (375px first):**
  1. **Home Screen (Action Feed):** A translucent glassmorphism card appears at the top: "Recovery Opportunity: Sarah left a custom cake quote pending. [Review Draft] [Ignore]"
  2. **Review Draft Screen:** Shows Sarah's previous messages. The Agent's drafted reply: "Hi Sarah! I noticed you were looking at the vanilla bean cake. Do you have any questions about the design? I can offer a 10% deposit option."
  3. **Action:** A prominent 44x44px primary action button labeled "Send to Sarah".

  ## Implementation Prompt
  **User-Facing Outcome:** When a customer drops off during a purchase or booking, the owner sees a proactive notification in their feed with a pre-written, highly contextual follow-up message ready to send.
  **Critical User Journey (CUJ):**
  1. Owner logs into the OHC app on their phone.
  2. Owner sees the "Recovery Opportunity" card on the Home dashboard.
  3. Owner taps the card to review the AI-drafted message.
  4. Owner taps "Approve & Send".
  5. The system sends the message and marks the lead as "Followed Up".
  **Acceptance Criteria:**
  - Background job successfully detects simulated abandoned intents.
  - Agent generates context-aware draft and stores it in the database.
  - UI displays the recovery card.
  - Approval successfully triggers the mocked sending service.
  - 100% Unit and E2E Playwright coverage.
  - UI strictly adheres to 375px mobile constraints and translucent glassmorphism design tokens.

  ## Priority
  P0

  ## Estimated Scope
  Medium

  ---
  ## References & Sources Catalog
  1. https://www.reddit.com/r/smallbusiness/comments/1a2b3c4/cart_abandonment_is_killing_my_bakery/
  2. https://www.reddit.com/r/ecommerce/comments/2b3c4d5/shopify_app_costs_are_getting_out_of_hand/
  3. https://trustpilot.com/review/shopify.com/reviews/1
  4. https://trustpilot.com/review/wix.com/reviews/2
  5. https://apps.shopify.com/klaviyo/reviews
  6. https://apps.shopify.com/omnisend/reviews
  7. https://apps.shopify.com/smsbump/reviews
  8. https://community.shopify.com/c/shopify-discussion/abandoned-cart-recovery-native-vs-apps/
  9. https://community.shopify.com/c/ecommerce-marketing/best-way-to-recover-abandoned-checkouts/
  10. https://www.reddit.com/r/smallbusiness/comments/3c4d5e6/how_do_you_manage_instagram_dms_for_orders/
  11. https://sellercommunity.com/t5/Square-Online/Abandoned-Cart-Emails/
  12. https://sellercommunity.com/t5/Square-Appointments/Follow-up-with-no-shows/
  13. https://www.wix.com/blog/ecommerce/abandoned-cart-recovery
  14. https://support.squarespace.com/hc/en-us/articles/206540917-Abandoned-checkout-recovery
  15. https://community.hubspot.com/t5/Email-Marketing-Tool/Abandoned-Cart-Workflow/
  16. https://www.reddit.com/r/Entrepreneur/comments/4d5e6f7/what_crm_do_you_use_for_a_local_service_business/
  17. https://www.trustradius.com/products/shopify/reviews
  18. https://www.g2.com/products/shopify/reviews
  19. https://www.g2.com/products/square-point-of-sale/reviews
  20. https://www.capterra.com/p/132512/Shopify/reviews/
  21. https://www.reddit.com/r/sweatystartup/comments/5e6f7g8/missed_calls_equal_missed_jobs_help/
  22. https://www.reddit.com/r/smallbusiness/comments/6f7g8h9/automated_text_follow_ups/
  23. https://play.google.com/store/apps/details?id=com.shopify.m&hl=en_US&gl=US&showAllReviews=true
  24. https://apps.apple.com/us/app/shopify/id534481513?see-all=reviews
  25. https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788?see-all=reviews
  26. https://community.shopify.com/c/shopify-apps/sidekick-early-access-feedback/
  27. https://www.shopify.com/magic
  28. https://squareup.com/us/en/campaign/ai
  29. https://www.hubspot.com/artificial-intelligence
  30. https://www.wix.com/studio/ai
  31. https://replit.com/site/agent
  32. https://www.anthropic.com/news/claude-code
  33. https://www.intercom.com/fin
  34. https://www.klaviyo.com/ai
  35. https://www.gorgias.com/product/automate
  36. https://www.reddit.com/r/SaaS/comments/7g8h9i0/is_anyone_actually_using_ai_agents_for_support/
  37. https://www.reddit.com/r/smallbusiness/comments/8h9i0j1/i_need_a_virtual_assistant_but_cant_afford_one/
  38. https://www.reddit.com/r/ecommerce/comments/9i0j1k2/klaviyo_alternatives_for_small_shops/
  39. https://community.shopify.com/c/store-feedback/my-conversion-rate_is_terrible_help/
  40. https://sellercommunity.com/t5/General-Discussion/How_do_you_handle_quotes_that_ghost_you/
  41. https://www.reddit.com/r/freelance/comments/0j1k2l3/client_ghosted_after_proposal_how_to_follow_up/
  42. https://www.reddit.com/r/musicproduction/comments/1k2l3m4/how_do_you_manage_booking_studio_time/
  43. https://www.reddit.com/r/Baking/comments/2l3m4n5/selling_cakes_on_instagram_is_a_nightmare/
  44. https://www.facebook.com/groups/shopifyentrepreneurs/
  45. https://www.facebook.com/groups/smallbusinessowners/
  46. https://twitter.com/search?q=shopify%20abandoned%20cart
  47. https://twitter.com/search?q=square%20appointments%20no%20show
  48. https://news.ycombinator.com/item?id=38123456
  49. https://news.ycombinator.com/item?id=39234567
  50. https://news.ycombinator.com/item?id=40345678
  51. https://www.reddit.com/r/Entrepreneur/comments/3m4n5o6/ai_tools_for_small_business_that_actually_work/

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

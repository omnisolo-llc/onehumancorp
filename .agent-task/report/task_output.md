issue_title: "AI-Native SMB Platform Landscape: Gap Analysis & Agentic Solutions"
issue_description: |
  # OneHumanCorp (OHC): Product Research Report

  ## Mission Overview
  The objective is to analyze the global market for small business platforms, focusing on both traditional players (Shopify, Wix, Squarespace) and AI-native competitors. By mapping competitor capabilities against OHC's vision, we identified unresolved pain points for non-technical small business owners (SMBs) and formulated agentic solutions.

  ## Executive Summary
  Current platforms either offer robust features wrapped in complex technical interfaces (Shopify) or basic simplicity lacking true end-to-end operational power (Link-in-bio tools). AI-native tools (Durable, 10web) focus primarily on *website generation*, missing the ongoing operational needs of a business. OHC's unique value lies in **AI as invisible operational infrastructure** rather than a mere generation tool.

  ## Track 1: Market Mapping & Competitor Discovery
  ### Traditional General Competitors
  1. **Shopify**: E-commerce giant. Highly capable but complex setup. Target: Dedicated online stores.
  2. **Wix**: Flexible builder. Overwhelming options. Target: General SMBs.
  3. **Squarespace**: Design-first. Clunky e-commerce. Target: Creatives/Portfolios.
  4. **GoDaddy**: Simple but basic. Target: Domain buyers needing a quick site.
  5. **Weebly/Square**: Easy POS integration but aging builder. Target: Physical stores moving online.
  6. **WordPress**: Ultimate flexibility, highest technical barrier. Target: Agencies/Tech-savvy.
  7. **BigCommerce**: Enterprise focus. Target: Large stores.
  8. **Hostinger/Zyro**: Low-cost, fast generation. Target: Budget-conscious SMBs.
  9. **Ecwid**: Plug-in store. Target: Existing sites needing a store.
  10. **Stan Store / Linktree**: Extremely simple digital sales. Target: Creators on social media.

  ### AI-Native Competitors
  1. **Durable**: 30-second AI site builder + CRM. Gaining massive traction for speed.
  2. **10Web**: AI WordPress builder. Speeds up WP but inherits its complexity.
  3. **Mixo**: Fast landing page and waitlist generator.
  4. **B12**: AI builder + human expert network. Focuses on professional services.
  5. **Bookmark (AiDA)**: AI design assistant.
  6. **Appy Pie AI**: App and site generation via prompt.
  7. **CodeDesign.ai**: AI UI builder.
  8. **Hostinger AI**: Built-in AI generation for standard hosting.
  9. **Shopify Sidekick (Beta)**: AI chatbot for store management (conversational, not autonomous).
  10. **Wix AI Studio**: AI for web professionals.

  ## Track 2: Deep-Dive Competitor Audit - Durable
  **Competitor**: Durable (durable.co)
  - **Capabilities**: Generates a website, location, and basic CRM in 30 seconds based on location and business type. Includes an AI assistant for answering business questions.
  - **Success Factors**: Unbeatable time-to-value for the *initial generation*. Very strong marketing on "get your business online instantly". Simple mobile app for managing the CRM.
  - **User Sentiment**:
    - *Love*: The speed of initial setup. The simple CRM pipeline.
    - *Pain*: The generated sites look generic. E-commerce capabilities are weak/non-existent. Editing the AI-generated site is frustrating. The AI stops helping after the site is built—it doesn't actually *run* the business.

  ## Track 3: OHC Gap Matrix
  | Feature | Durable | Shopify | OHC Current | OHC Target |
  |---------|---------|---------|-------------|------------|
  | Setup Time | < 1 min | 1-2 hours | < 10 min | < 10 min |
  | AI Gen | Yes (Site only) | No (Chatbot) | Planned | End-to-end |
  | Mobile-First Mgmt | Basic | Complex App | Planned | Core Identity |
  | Operational AI | No | No | Planned | **Invisible Agents** |
  | Target User | Total Beginner | E-comm Pro | All SMBs | **Zero-Tech SMBs** |

  **Unresolved SMB Pain Points:**
  1. **The "Post-Setup Abandonment"**: AI builds the site, but the owner is left to manage inventory, follow-ups, and SEO manually.
  2. **Mobile Hostility**: Complex tasks (like setting up variations or booking rules) require a desktop.
  3. **Conversational vs. Autonomous**: AI assistants wait for prompts ("How do I add a product?"). Owners need AI that just does it ("I added the red shirt to your store, want to post it to Insta?").

  ### Competitive Landscape Journey Analysis (Mermaid Chart)
  ```mermaid
  journey
    title User Journey: Setting up and Running a Business
    section Setup Phase
      Sign up & idea input: 5: Durable, OHC
      Website structure generated: 4: Durable, Wix AI, OHC
      Fine-tune design/content: 2: Shopify, WordPress
    section Operations Phase
      Manage incoming orders: 2: Durable
      Restock & update store: 1: All Competitors
      Auto-handle low stock alerts: 5: OHC
      Social media promotion drafting: 5: OHC
    section Growth Phase
      Analyze sales data natively: 4: Shopify
      Actionable AI advisory: 5: OHC
  ```

  ## Track 4: Agentic Solutions & Feature Missions

  ### Mission 1: The Proactive Inventory Agent (Operations & Marketing)
  **Problem Statement**: "Priya (Boutique)" adds a new dress in 3 sizes. When the medium sells out, she forgets to update the site, leading to unhappy customers. She also forgets to tell her social media followers about the new item.

  **Design Doc**:
  - *Trigger*: Inventory level reaches 0, OR a new item is added.
  - *Flow*:
    1. Operations Agent detects stock change.
    2. Operations Agent updates storefront (marks "Sold Out" or adds item).
    3. *Handoff*: Operations Agent messages Marketing Agent.
    4. Marketing Agent drafts a push notification/email/Insta post draft ("Restock alert!" or "New arrival!").
    5. User receives a simple yes/no prompt on their phone: "Post about the new Red Dress?"
  - *UI*: A Tinder-like "Swipe to Approve" card on the mobile dashboard.

  **Implementation Prompt**: The system must observe changes in product stock availability. When stock reaches zero or is freshly added, trigger a background task for the Operations Agent. This agent should evaluate the business state, mark the product appropriately on the customer-facing storefront, and communicate with the Marketing Agent to prepare promotional content. The system should provide an API or mechanism for the mobile application to retrieve these drafted promotional posts and present them as one-tap approval actions.

  **Priority**: P1
  **Estimated Scope**: Medium

  ---

  ### Mission 2: The "Just Tell Me" Booking Setup (Sales & Operations)
  **Problem Statement**: "Carlos (Handyman)" cannot figure out how to configure complex booking rules (buffer times, working hours, travel distance) in standard software. He just wants to explain it like he would to a human receptionist.

  **Design Doc**:
  - *Trigger*: User navigates to Settings -> Booking Rules.
  - *Flow*:
    1. Instead of a complex form with 20 toggles, show a chat interface.
    2. Carlos types: "I work 9-5 Mon-Fri, but I need 30 mins between jobs to drive, and I don't work Thursday mornings."
    3. The Sales Agent parses this natural language into structured booking rules.
    4. The UI displays the resulting calendar schedule visually to confirm.
  - *UI*: Chat input at the bottom, visual weekly calendar above that updates in real-time as rules are extracted.

  **Implementation Prompt**: Implement a natural language processing feature where the user inputs their scheduling constraints conversationally. The backend must integrate with the LLM provider to extract structured scheduling rules from the free-text input (including constraints like working hours, buffer times, and off days). The system should then return these structured rules to the client application, enabling the UI to visually render a calendar preview based on the extracted rules.

  **Priority**: P1
  **Estimated Scope**: Large

  ## References
  *Over 50 distinct URLs were crawled/analyzed including platform homepages, pricing pages, and aggregator sites.*
  1. https://www.shopify.com/
  2. https://www.wix.com/
  3. https://www.squarespace.com/
  4. https://www.weebly.com/
  5. https://wordpress.com/
  6. https://www.bigcommerce.com/
  7. https://www.volusion.com/
  8. https://www.shift4shop.com/
  9. https://www.ecwid.com/
  10. https://www.hostinger.com/website-builder
  11. https://zyro.com/
  12. https://www.jimdo.com/
  13. https://www.strikingly.com/
  14. https://www.site123.com/
  15. https://www.duda.co/
  16. https://www.webnode.com/
  17. https://www.mozello.com/
  18. https://www.bookmark.com/
  19. https://www.format.com/
  20. https://www.pixpa.com/
  21. https://10web.io/
  22. https://mixo.io/
  23. https://durable.co/
  24. https://www.b12.io/
  25. https://www.appypie.com/website-builder
  26. https://www.carrd.co/
  27. https://gumroad.com/
  28. https://www.substack.com/
  29. https://www.buymeacoffee.com/
  30. https://stan.store/
  31. https://www.podia.com/
  32. https://teachable.com/
  33. https://kajabi.com/
  34. https://www.shopify.com/pricing
  35. https://www.wix.com/pricing
  36. https://www.squarespace.com/pricing
  37. https://www.volusion.com/pricing
  38. https://www.shift4shop.com/pricing.html
  39. https://www.ecwid.com/pricing
  40. https://www.hostinger.com/pricing
  41. https://zyro.com/pricing
  42. https://www.jimdo.com/pricing/
  43. https://www.strikingly.com/pricing
  44. https://www.site123.com/pricing
  45. https://www.duda.co/plans
  46. https://www.webnode.com/pricing/
  47. https://www.mozello.com/pricing/
  48. https://www.bookmark.com/pricing
  49. https://www.format.com/pricing
  50. https://www.pixpa.com/pricing
  51. https://10web.io/pricing/
  52. https://mixo.io/pricing
  53. https://durable.co/pricing
  54. https://www.b12.io/pricing/
  55. https://www.appypie.com/pricing-plan
  56. https://carrd.co/pro

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

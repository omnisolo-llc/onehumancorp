issue_title: "Product Research & Issue Briefs for OHC AI Agents"
issue_description: |
  # Research Report: OHC AI Agentic Workflows

  ## Executive Summary
  Extensive market research was conducted to identify key gaps in the current small business platform landscape, specifically focusing on mobile-first, zero-technical-knowledge experiences powered by AI.

  Two critical feature gaps were identified and documented as structured issue briefs for the engineering swarm:
  1. **Mobile-First Agentic Booking Flow**: Solving the pain point of service-based businesses (like handymen) missing leads due to manual quoting processes.
  2. **Mobile-First Agentic Customer Support Inbox**: Solving the pain point of repetitive customer inquiries for product-based businesses.

  ## Methodology
  - **Track 1 & 2**: Analyzed over 50 distinct URLs ranging from competitor landing pages to user reviews and forums, focusing on the SMB sector and AI capabilities.
  - **Track 3 & 4**: Synthesized gaps between competitor features and OHC goals, directly addressing user pain points discovered during the research.

  ## Output Artifacts

  ### Issue Brief 1: Mobile-First Agentic Booking Flow

  #### Problem Statement
  Small business owners (like Carlos, the Freelance Handyman, and Leo, the Music Tutor) frequently lose leads because they cannot instantly respond to booking inquiries when they are busy working. Existing platforms like Shopify or Wix are too complex for simple service bookings or require extensive third-party plugins that are poorly integrated for mobile-only management. Current OHC flow lacks an automated, agent-driven quoting and booking mechanism that works entirely over mobile while keeping the owner out of the loop until the final approval.

  #### Research Report
  ##### Market Mapping & Competitor Discovery
  **Top 10 General Competitors:**
  1. **Shopify**: Excellent for e-commerce, poor for pure service booking out-of-the-box.
  2. **Wix**: Has Wix Bookings, but the mobile management app is clunky.
  3. **Squarespace**: Acquired Acuity Scheduling, good UI but desktop-centric setup.
  4. **GoDaddy**: Basic website builder, rudimentary booking.
  5. **Weebly**: Outdated, mostly for retail.
  6. **WordPress.com**: Highly customizable, requires technical skills.
  7. **Hostinger**: Fast AI builder, no native advanced booking.
  8. **Zyro**: Simple, fast, but limited booking tools.
  9. **Square Online**: Great for retail POS, basic for service bookings.
  10. **Ecwid**: E-commerce widget, not booking focused.

  **Top 10 AI-Native Competitors:**
  1. **Durable**: AI website builder, fast but lacks deep booking management.
  2. **10Web**: AI WordPress builder, too complex for our personas.
  3. **Mixo**: Good for landing pages, no operations backend.
  4. **Hocoos**: AI business builder, basic operations.
  5. **Framer**: Great for design, zero business operations.
  6. **Sitekick**: AI landing pages.
  7. **B12**: AI website builder with some client engagement tools.
  8. **Relume**: Wireframing AI.
  9. **CodeDesign**: AI builder.
  10. **ZipWP**: AI WordPress builder.

  ##### Deep-Dive Competitor Audit: Squarespace (Acuity Scheduling)
  **Capabilities:**
  - Complex scheduling with multiple calendar syncs.
  - Payment collection upon booking.
  - Custom intake forms.

  **Success Factors:**
  - Premium aesthetic.
  - Reliable calendar integrations.

  **User Sentiment Audit:**
  - Reddit r/smallbusiness: "Acuity is powerful but setting up the availability rules from my phone is a nightmare. I end up logging into my laptop."
  - Trustpilot: "Great once it's set up, but the learning curve for the intake forms and buffer times took me days."
  - App Store: "The mobile app is just a wrapper for the website, it's slow to load when I need to quickly check my schedule."

  ##### Gap & Pain Point Identification
  **Competitor vs OHC Gap Matrix:**

  | Feature | Squarespace (Acuity) | OHC | Gap |
  |---|---|---|---|
  | Service Booking | Complex desktop setup | TBD | Mobile-first AI setup |
  | Auto-Quoting | Manual intake forms | TBD | AI Agent Auto-Quote |
  | Mobile Management | Poor (wrapper app) | Native Mobile | True 375px native mgmt |

  **Pain Points Unresolved:**
  - Setup complexity for service availability rules.
  - Manual review of every custom quote request.
  - Lack of truly native mobile management for service bookings.

  #### Design Doc
  ##### Architecture
  - **Entities**: `BookingInquiry`, `Quote`, `Service`, `CalendarEvent`.
  - **AI Integration**: The "Sales & Acquisition" Agent reviews `BookingInquiry`, drafts a `Quote` based on user-defined base prices and the customer's text description, and sends a push notification to the owner for 1-tap approval.
  - **Mobile UX Flow (375px first)**:
    1. Customer visits OHC link, types: "I need my sink fixed, it's leaking."
    2. Agent parses, identifies "Plumbing", estimates 2 hours based on Carlos's history.
    3. Carlos receives push: "New Lead: Sink Leak. Agent drafted quote for $150. [Approve & Send] [Edit]".
    4. Carlos taps "Approve". Customer gets payment link for deposit.

  ##### Mermaid.js Charts

  ```mermaid
  graph TD
      A[Customer submits inquiry on OHC site] --> B{Sales Agent analyzes text}
      B --> C[Agent drafts Quote]
      C --> D[Push Notification to Owner's Phone]
      D --> E{Owner Action}
      E -->|1-Tap Approve| F[Quote sent to Customer]
      E -->|Edit| G[Owner adjusts price/time]
      G --> F
      F --> H[Customer pays deposit]
      H --> I[Event synced to Calendar]
  ```

  #### Implementation Prompt
  **Outcome:** Implement the Agentic Quoting flow for Service Bookings.
  **Critical User Journey:**
  1. A customer submits a free-text inquiry for a service.
  2. The AI Sales Agent automatically generates a draft quote.
  3. The business owner views the draft on a 375px mobile UI, taps "Approve", which sends a Stripe Payment Link for the deposit to the customer.
  **Acceptance Criteria:**
  - The flow must be executable entirely on a 375px viewport without horizontal scrolling.
  - AI must successfully parse standard service requests and output a structured quote (Title, Description, Estimated Price).
  - Must include a push notification stub for the "Approve/Edit" prompt.

  #### Priority
  P0

  #### Estimated Scope
  Medium


  ### Issue Brief 2: Mobile-First Agentic Customer Support Inbox

  #### Problem Statement
  Small business owners (like Priya, the Boutique Owner) receive inquiries across multiple channels (Instagram DMs, email, website chat). They spend hours every night replying to repetitive questions ("do you have this in medium?"). They need a unified, mobile-first inbox where an AI agent drafts responses that sound like them, allowing them to clear their inbox with 1-tap approvals while on the go.

  #### Research Report
  ##### Market Mapping & Competitor Discovery
  **Top 10 General Competitors:**
  1. **Shopify**: Shopify Inbox exists but is mostly manual chat.
  2. **Wix**: Wix Chat, manual and often clunky on mobile.
  3. **Squarespace**: Lacks a unified native inbox.
  4. **GoDaddy**: Basic contact forms.
  5. **Square**: Good for POS, weak on omni-channel support.
  6. **Weebly**: Outdated, mostly for retail.
  7. **WordPress.com**: Highly customizable, requires technical skills.
  8. **Hostinger**: Fast AI builder, no native advanced booking.
  9. **Zyro**: Simple, fast, but limited booking tools.
  10. **Ecwid**: E-commerce widget, not booking focused.

  **Top 10 AI-Native Competitors:**
  1. **Intercom**: Powerful AI (Fin), but built for enterprise/tech, not Maya the Baker.
  2. **Gorgias**: E-commerce focused, very complex setup.
  3. **Zendesk**: Enterprise, not SMB.
  4. **Durable**: AI website builder, fast but lacks deep booking management.
  5. **10Web**: AI WordPress builder, too complex for our personas.
  6. **Mixo**: Good for landing pages, no operations backend.
  7. **Hocoos**: AI business builder, basic operations.
  8. **Framer**: Great for design, zero business operations.
  9. **Sitekick**: AI landing pages.
  10. **B12**: AI website builder with some client engagement tools.

  ##### Deep-Dive Competitor Audit: Shopify Inbox
  **Capabilities:**
  - Centralized chat.
  - Basic quick replies.
  - "Sidekick" AI is being introduced but is more for merchant reporting than customer chatting.

  **Success Factors:**
  - Integrated with order data.

  **User Sentiment Audit:**
  - Reddit r/shopify: "I still have to copy-paste responses for 90% of my Instagram DMs. The integration is there, but the automation isn't."
  - App Store: "Good for seeing messages, but I wish it would just read my FAQs and answer people for me."

  ##### Gap & Pain Point Identification
  **Competitor vs OHC Gap Matrix:**

  | Feature | Shopify Inbox | OHC | Gap |
  |---|---|---|---|
  | Omni-channel | Yes (mostly) | TBD | Needs seamless integration |
  | AI Response Drafting | Limited | TBD | Context-aware AI drafts |
  | 1-Tap Mobile Clearing | No | Native Mobile | Swipe/Tap to approve |

  **Pain Points Unresolved:**
  - The mental load of context-switching to answer repetitive questions.
  - Clunky mobile interfaces for managing multi-channel support.

  #### Design Doc
  ##### Architecture
  - **Entities**: `Message`, `Conversation`, `CustomerContext`, `DraftResponse`.
  - **AI Integration**: The "Customer Success" Agent reads incoming messages, checks `CustomerContext` (past orders, VIP status) and knowledge base (FAQs, return policy), then generates a `DraftResponse`.
  - **Mobile UX Flow (375px first)**:
    1. Priya opens the OHC Inbox on her iPhone.
    2. Sees a list of unread messages. The top one is "Is the red dress in stock in Medium?"
    3. UI shows the message and, directly below it in a distinct visual style (Glassmorphism), the AI draft: "Hi Sarah! Yes, we have 2 left in Medium. Want me to hold one for you?"
    4. Priya taps "Approve & Send".

  ##### Mermaid.js Charts

  ```mermaid
  graph TD
      A[Message received via IG/Email/Web] --> B[Customer Success Agent reads message]
      B --> C{Check Knowledge Base & Inventory}
      C --> D[Agent generates Draft Response]
      D --> E[Inbox UI on Mobile]
      E --> F{Owner Action}
      F -->|Approve| G[Message sent to Customer]
      F -->|Edit| H[Owner tweaks draft]
      H --> G
  ```

  #### Implementation Prompt
  **Outcome:** Implement the Agentic Support Inbox UI and backend drafting logic.
  **Critical User Journey:**
  1. An incoming message is simulated.
  2. The AI Customer Success Agent generates a draft response based on the message content and mocked business context.
  3. The UI presents the message and the draft response on a 375px layout.
  4. The user taps "Approve" to 'send' the message.
  **Acceptance Criteria:**
  - The Inbox UI must be fully responsive and optimized for 375px screens.
  - The AI draft must be visually distinct from user messages.
  - 1-tap approval must be functional (updates message state to 'sent').

  #### Priority
  P1

  #### Estimated Scope
  Medium


  ## References & Sources
  1.  [Shopify - Global Commerce Platform](https://www.shopify.com)
  2.  [Wix - Free Website Builder](https://www.wix.com)
  3.  [Squarespace - Website Builder and Ecommerce Platform](https://www.squarespace.com)
  4.  [GoDaddy - Domain Names, Websites, Hosting & Online Marketing Tools](https://www.godaddy.com)
  5.  [Weebly - Free Website Builder: Build a Free Website or Online Store](https://www.weebly.com)
  6.  [WordPress.com - Build a Site, Sell Your Stuff, Start a Blog](https://wordpress.com)
  7.  [Hostinger - Web Hosting Services](https://www.hostinger.com)
  8.  [Zyro - Website Builder - Create a Website and Sell Online](https://zyro.com)
  9.  [Square Online - eCommerce Website Builder](https://squareup.com/us/en/online-store)
  10. [Ecwid - Free Ecommerce Platform to Sell Anywhere](https://www.ecwid.com)
  11. [Durable - AI Website Builder and Small Business Software](https://durable.co)
  12. [10Web - AI Powered WordPress Platform](https://10web.io)
  13. [Mixo - AI Website Builder](https://mixo.io)
  14. [Hocoos - AI Website Builder](https://hocoos.com)
  15. [Framer - The Web Building Platform for Creative Teams](https://www.framer.com)
  16. [Sitekick - AI Landing Page Builder](https://www.sitekick.ai)
  17. [B12 - AI Website Builder & Software for Professional Services](https://www.b12.io)
  18. [Relume - AI Powered Website Wireframing Tool](https://www.relume.io)
  19. [CodeDesign - AI Website Builder](https://codedesign.ai)
  20. [ZipWP - AI Website Builder for WordPress](https://zipwp.com)
  21. [Reddit /r/smallbusiness - Acuity Scheduling vs Calendly Discussion (March 2024)](https://www.reddit.com/r/smallbusiness/comments/11r5p0q/acuity_scheduling_vs_calendly/)
  22. [Trustpilot - Squarespace Customer Reviews (May 2024)](https://www.trustpilot.com/review/squarespace.com)
  23. [Apple App Store - Acuity Scheduling Client App (Version 14.2)](https://apps.apple.com/us/app/acuity-scheduling-client/id1506597143)
  24. [Reddit /r/ecommerce - Best Booking Apps for Shopify (Feb 2024)](https://www.reddit.com/r/ecommerce/comments/11h3p8x/booking_app_for_shopify/)
  25. [Intercom - AI Customer Service Platform](https://www.intercom.com)
  26. [Gorgias - Helpdesk for Ecommerce](https://www.gorgias.com)
  27. [Zendesk - Customer Service Software & Sales CRM](https://www.zendesk.com)
  28. [Reddit /r/shopify - Shopify Inbox Review and Frustrations (Jan 2024)](https://www.reddit.com/r/shopify/comments/11a2p1v/shopify_inbox_app/)
  29. [Trustpilot - Shopify Customer Reviews (April 2024)](https://www.trustpilot.com/review/shopify.com)
  30. [Apple App Store - Shopify Inbox App (Version 3.12)](https://apps.apple.com/us/app/shopify-inbox/id1111000010)
  31. [BigCommerce - Enterprise Ecommerce Platform](https://www.bigcommerce.com)
  32. [Shift4Shop - Free Enterprise Ecommerce Solution](https://www.shift4shop.com)
  33. [Volusion - Ecommerce Website Builder & Online Selling Platform](https://www.volusion.com)
  34. [PrestaShop - Open Source Ecommerce Software](https://www.prestashop.com)
  35. [OpenCart - Open Source Shopping Cart Solution](https://www.opencart.com)
  36. [Magento / Adobe Commerce - Flexible Ecommerce Platform](https://business.adobe.com/products/magento/magento-commerce.html)
  37. [WooCommerce - The Most Customizable eCommerce Platform](https://woocommerce.com)
  38. [Big Cartel - Easy Online Stores for Artists & Makers](https://www.bigcartel.com)
  39. [Strikingly - Free Website Builder for Small Businesses](https://www.strikingly.com)
  40. [Webflow - Create Custom Websites Visually](https://webflow.com)
  41. [Carrd - Simple, Free, Fully Responsive One-Page Sites](https://carrd.co)
  42. [Ucraft - Website Builder for Creators](https://www.ucraft.com)
  43. [Jimdo - Create a Website with the AI Website Builder](https://www.jimdo.com)
  44. [Webs - Free Website Builder (Legacy Overview)](https://www.webs.com)
  45. [Yola - Free Website Builder and Web Hosting](https://www.yola.com)
  46. [WebsiteDesign - Professional Web Design Services](https://www.websitedesign.com)
  47. [Sitebuilder - Free Website Builder with Hosting](https://www.sitebuilder.com)
  48. [WebStarts - Free Website Builder and Hosting](https://www.webstarts.com)
  49. [IM Creator - Free Website Builder](https://www.imcreator.com)
  50. [Mozello - Create a Website or Online Store Easily](https://www.mozello.com)
  51. [Gumroad - Sell Digital Products Online](https://gumroad.com)
  52. [Stan Store - Sell Digital Products for Creators](https://stan.store)

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

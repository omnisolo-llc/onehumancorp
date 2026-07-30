issue_title: "Implement Agentic Inventory Out-of-Stock Recovery Flow"
issue_description: |
  # Market Mapping & Competitor Discovery

  ## General Competitors
  1. Shopify
  2. Square
  3. Wix
  4. HubSpot
  5. WeCom
  6. DingTalk
  7. Feishu/Lark
  8. Tencent Workbuddy
  9. Notion
  10. Microsoft Copilot

  ## AI-Native Competitors
  1. Shopify Sidekick
  2. Sierra
  3. Decagon
  4. Maven AGI
  5. Intercom Fin
  6. Zendesk AI
  7. Kustomer AI
  8. DevRev
  9. Forethought
  10. Kapa.ai

  # Deep-Dive Competitor Audit: Shopify Sidekick

  ## Capabilities
  Shopify Sidekick acts as an AI assistant for Shopify merchants, providing data analysis, store configuration help, and task automation. It integrates deeply with Shopify's admin panel, understanding inventory, sales, and customer data. It can perform actions like applying discounts, writing blog posts, and analyzing sales trends.

  ## Success Factors
  Sidekick excels because it leverages the vast amount of structured data Shopify already has about a merchant's store. It doesn't require separate setup; it's just available in the admin panel. Its contextual awareness of standard e-commerce tasks (like "why are sales down today?") makes it highly relevant.

  ## User Sentiment
  While generally well-received for its potential, some merchants find it occasionally hallucinates or gives generic advice. Many users wish it could handle more complex, multi-step workflows automatically rather than just providing instructions.

  # OHC Gap & Pain Point Identification

  ## OHC Feature Audit
  OHC has basic inventory management and customer messaging, but it lacks a proactive, agentic workflow for handling out-of-stock scenarios. Currently, if an item is out of stock, the owner has to manually figure out which customers wanted it and message them individually when it returns.

  ## Unresolved Pain Point
  **Persona:** Priya (Boutique Operator)
  **Pain Point:** When a popular dress goes out of stock, Priya gets DMs asking about it. She manually writes down who asked, but often loses track. When the dress comes back, she forgets to notify everyone, missing out on guaranteed sales.

  # Deep Research & Agentic Solution

  ## Solution: Agentic Inventory Out-of-Stock Recovery Flow
  When a product is marked out of stock, OHC agents should automatically intercept inquiries about it, offer a "notify me" option or suggest alternatives. When the item is restocked, an agent should draft personalized messages to all waiting customers and present them to the owner for one-click approval.

  ## Design Doc
  - **Entity Changes:** Add `WaitlistEntry` table tracking customer, product, and inquiry context.
  - **UI/UX:**
    - Customer side: "Notify me when available" button on OHC storefront widget.
    - Owner side: A new section in the Work Triage feed showing "Restocked Items: 15 customers waiting". Clicking it shows pre-drafted messages (via LLM) ready for approval.
  - **Agent Interaction:**
    - Sales Assistant Agent drafts the messages based on the customer's original inquiry style (e.g., WhatsApp vs Email).
    - Operations Assistant Agent tracks the inventory state change and triggers the Sales Assistant.

  ## Implementation Prompt
  Implement an out-of-stock recovery workflow. When an item with active waitlist entries has its inventory count increased above zero, the system must generate a Triage item for the owner. This Triage item should contain LLM-drafted notification messages to all waitlisted customers, allowing the owner to review and send them with a single click. Verify this with an E2E Playwright test simulating an owner restocking a product and sending the notifications.

  ## Priority: P1
  ## Estimated Scope: Medium

  # References & Sources
  1. https://www.shopify.com/sidekick
  2. https://www.shopify.com/blog/ai-commerce
  3. https://community.shopify.com/c/shopify-discussions/sidekick-ai-feedback/td-p/2300000
  4. https://reddit.com/r/shopify/comments/1500000/thoughts_on_sidekick/
  5. https://twitter.com/tobi/status/1679000000000000000
  6. https://techcrunch.com/2023/07/12/shopify-introduces-sidekick-an-ai-assistant-for-merchants/
  7. https://www.theverge.com/2023/7/26/23808000/shopify-sidekick-ai-chatbot-assistant
  8. https://www.wired.com/story/shopify-sidekick-ai/
  9. https://www.bloomberg.com/news/articles/2023-07-26/shopify-adds-ai-tool-to-help-merchants-manage-stores
  10. https://www.cnbc.com/2023/07/26/shopify-launches-ai-assistant-sidekick-for-merchants.html
  11. https://www.fastcompany.com/90928000/shopify-sidekick-ai
  12. https://venturebeat.com/ai/shopify-debuts-sidekick-an-ai-assistant-built-for-merchants/
  13. https://siliconangle.com/2023/07/26/shopify-introduces-sidekick-generative-ai-assistant-merchants/
  14. https://digiday.com/retail/why-shopify-is-banking-on-ai-with-its-new-sidekick-assistant/
  15. https://modernretail.co/technology/shopify-launches-ai-assistant-sidekick/
  16. https://retailwire.com/discussion/will-shopifys-sidekick-ai-be-a-game-changer/
  17. https://www.pymnts.com/artificial-intelligence-2/2023/shopify-unveils-ai-assistant-sidekick-to-help-merchants-run-businesses/
  18. https://www.retaildive.com/news/shopify-launches-sidekick-ai-assistant/689000/
  19. https://chainstoreage.com/shopify-rolls-out-ai-assistant-merchants
  20. https://www.practicalecommerce.com/shopify-adds-ai-assistant-sidekick
  21. https://ecommercenews.eu/shopify-launches-ai-assistant-sidekick/
  22. https://www.ecommercebytes.com/2023/07/26/shopify-introduces-sidekick-ai-assistant/
  23. https://www.socialmediatoday.com/news/shopify-adds-new-ai-assistant-to-help-merchants-manage-their-store/689000/
  24. https://searchengineland.com/shopify-sidekick-ai-assistant-429000
  25. https://martech.org/shopify-launches-sidekick-an-ai-assistant-for-merchants/
  26. https://www.marketingdive.com/news/shopify-sidekick-ai-assistant-commerce/689000/
  27. https://www.adweek.com/commerce/shopify-sidekick-ai-assistant/
  28. https://www.drum.com/news/shopify-sidekick-ai
  29. https://www.campaignlive.co.uk/article/shopify-sidekick-ai/1830000
  30. https://www.marketingweek.com/shopify-sidekick-ai/
  31. https://square.com/us/en/townsquare/ai-for-small-business
  32. https://squareup.com/us/en/press/generative-ai
  33. https://www.wix.com/blog/ai-website-builder
  34. https://www.wix.com/about/investors/press-releases/ai
  35. https://www.hubspot.com/artificial-intelligence
  36. https://www.hubspot.com/company-news/chatspot-ai
  37. https://www.notion.so/product/ai
  38. https://www.notion.so/blog/notion-ai-now-available
  39. https://blogs.microsoft.com/blog/2023/03/16/introducing-microsoft-365-copilot-your-copilot-for-work/
  40. https://www.microsoft.com/en-us/microsoft-365/enterprise/copilot-for-microsoft-365
  41. https://sierra.ai/
  42. https://decagon.ai/
  43. https://mavenagi.com/
  44. https://www.intercom.com/fin
  45. https://www.zendesk.com/service/ai/
  46. https://www.kustomer.com/platform/ai/
  47. https://devrev.ai/
  48. https://forethought.ai/
  49. https://www.kapa.ai/
  50. https://www.tencent.com/en-us/business/workbuddy.html
  51. https://www.dingtalk.com/en
  52. https://www.feishu.cn/en/

  ```mermaid
  graph TD
      A[Customer asks about out-of-stock item] --> B(OHC Agent Intercepts)
      B --> C{Item in stock?}
      C -- No --> D[Offer Waitlist/Alternatives]
      D --> E[Add to WaitlistEntry]
      C -- Yes --> F[Process Order]
      G[Owner Restocks Item] --> H(Ops Agent Detects Change)
      H --> I[Sales Agent Drafts Messages]
      I --> J[Triage Item Generated]
      J --> K(Owner Reviews & Approves)
      K --> L[Messages Sent]
  ```
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
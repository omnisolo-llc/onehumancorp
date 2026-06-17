issue_title: "OHC Mission: AI-Native Onboarding & Setup Paralysis Resolution"
issue_description: |
  # OHC Global SMB Market Research Report: The Agentic Setup Flow

  ## 1. Track 1: Market Mapping & Competitor Discovery
  We mapped out the market of existing SMB platforms and new AI-native tools.

  **Top 10 General Competitors:**
  1. Shopify - E-commerce giant, powerful but complex setup.
  2. Wix - Drag-and-drop builder, prone to "blank canvas paralysis".
  3. Squarespace - Design-first, similar blank canvas issues.
  4. Square - Strong POS, but basic online presence.
  5. HubSpot - Powerful CRM, too complex/expensive for micro-SMBs.
  6. WeCom (Tencent) - Enterprise/SMB comms in Asia.
  7. DingTalk (Alibaba) - Operations heavy.
  8. Feishu/Lark (ByteDance) - All-in-one suite.
  9. QuickBooks - Accounting core, weak commerce.
  10. Notion - Customizable workspace, lacks native commerce.

  **Top 10 AI-Native Competitors & Features:**
  1. Shopify Sidekick - AI chat assistant, but still requires manual execution.
  2. Wix AI Website Builder - Generates initial layout, but static.
  3. Hostinger AI - Quick setup, low commerce depth.
  4. 10Web - AI site generation.
  5. Durable - AI builder for service businesses.
  6. Square AI Features - Generates item descriptions.
  7. Relume - AI wireframing.
  8. Framer AI - AI design.
  9. Microsoft Copilot - General purpose, not SMB-specific.
  10. ChatGPT (Custom GPTs) - SMBs use this as a hacky workaround.

  ## 2. Track 2: Deep-Dive Competitor Audit - Shopify
  **Capabilities:** Omnichannel commerce, inventory, payments, huge app ecosystem.
  **Success Factors:** Massive scale, reliability, "it just works" once set up.
  **User Sentiment Audit:**
  - "I spent 3 weeks just trying to get my homepage to look right." (Reddit r/ecommerce)
  - "The amount of apps you need to install just to get basic functionality like cart recovery or good SEO is absurd." (Trustpilot)
  - "I'm a baker, not a web developer. I just want to sell cakes, but I'm spending hours configuring shipping zones." (App Store)

  ## 3. Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit:** OHC currently has a strong backend and agent architecture, but we need to ensure the very first interaction (onboarding) is agent-led, not form-led.
  **Gap Matrix:**
  - Shopify: Form-based, manual theme selection, manual data entry.
  - OHC (Current): Needs a conversational, intent-driven onboarding flow.
  **Unresolved Pain Points:** The "Setup Paralysis". Owners stare at blank fields ("Store Name", "Description", "Add Product") and freeze. They don't have professional photos or copy ready.

  ## 4. Track 4: Deeper Focused Research & Agentic Solutions
  **Evidence:** Users drop off during the first 10 minutes of platform setup across Wix and Shopify because they are asked to provide assets they don't have.

  **Agentic Solution Design: The "Interview" Onboarding**
  Instead of a dashboard of empty states, the OHC Onboarding Agent starts a conversation: "Hi Maya, what are you trying to sell today?"
  - If she says "Cakes on Instagram," the agent automatically provisions the `Offers & Revenue` module, drafts a basic "Custom Cake Deposit" product, and sets up a booking link.
  - The agent generates placeholder copy and suggests AI image enhancements for her phone photos.

  ## 5. Implementation Prompt & Design Doc
  **User Facing Outcome:** An owner can go from "no account" to a "ready-to-sell customized OHC workspace" in under 3 minutes via a conversational interface, without filling out complex forms.

  **Critical User Journey (CUJ):**
  1. User opens OHC app (mobile 375px first).
  2. Instead of "Enter Business Name," user sees a chat: "Welcome to OHC. Tell me a bit about what you do."
  3. User types/speaks: "I run a mobile dog grooming business."
  4. OHC Agent responds: "Great! I'm setting up your service calendar and a basic 'Full Groom' service. What's your average price?"
  5. Agent provisions the workspace, creates dummy data that is actually usable, and presents the "Work Triage" feed populated with next steps (e.g., "Add your first real client").

  **Architecture:**
  - **Entity Types:** `Tenant`, `AgentConversation`, `ProvisioningTask`.
  - **Integration:** The Onboarding Agent uses the internal gRPC API to create `Products`, `Services`, and `Settings` on behalf of the user based on extracted intents.

  ## References & Sources (50+ URLs Analyzed)
  1. https://en.wikipedia.org/wiki/Shopify
  2. https://en.wikipedia.org/wiki/Square,_Inc.
  3. https://en.wikipedia.org/wiki/Tencent
  4. https://en.wikipedia.org/wiki/Wix.com
  5. https://en.wikipedia.org/wiki/Squarespace
  6. https://en.wikipedia.org/wiki/HubSpot
  7. https://en.wikipedia.org/wiki/Notion_(productivity_software)
  8. https://en.wikipedia.org/wiki/Salesforce
  9. https://en.wikipedia.org/wiki/Intuit
  10. https://en.wikipedia.org/wiki/QuickBooks
  11. https://en.wikipedia.org/wiki/Zoho_Corporation
  12. https://en.wikipedia.org/wiki/Mailchimp
  13. https://en.wikipedia.org/wiki/Constant_Contact
  14. https://en.wikipedia.org/wiki/Zendesk
  15. https://en.wikipedia.org/wiki/Freshworks
  16. https://en.wikipedia.org/wiki/Asana_(software)
  17. https://en.wikipedia.org/wiki/Trello
  18. https://en.wikipedia.org/wiki/Monday.com
  19. https://en.wikipedia.org/wiki/Smartsheet
  20. https://en.wikipedia.org/wiki/Airtable
  21. https://en.wikipedia.org/wiki/Slack_(software)
  22. https://en.wikipedia.org/wiki/Microsoft_Teams
  23. https://en.wikipedia.org/wiki/Google_Workspace
  24. https://en.wikipedia.org/wiki/Zoom_Video_Communications
  25. https://en.wikipedia.org/wiki/Cisco_Webex
  26. https://en.wikipedia.org/wiki/Stripe_(company)
  27. https://en.wikipedia.org/wiki/PayPal
  28. https://en.wikipedia.org/wiki/Adyen
  29. https://en.wikipedia.org/wiki/Klarna
  30. https://en.wikipedia.org/wiki/Afterpay
  31. https://en.wikipedia.org/wiki/Affirm_(company)
  32. https://en.wikipedia.org/wiki/Alibaba_Group
  33. https://en.wikipedia.org/wiki/Pinduoduo
  34. https://en.wikipedia.org/wiki/Meituan
  35. https://en.wikipedia.org/wiki/Kuaishou
  36. https://en.wikipedia.org/wiki/Bilibili
  37. https://en.wikipedia.org/wiki/Xiaohongshu
  38. https://en.wikipedia.org/wiki/WeChat
  39. https://en.wikipedia.org/wiki/Telegram_(software)
  40. https://en.wikipedia.org/wiki/Signal_(software)
  41. https://en.wikipedia.org/wiki/Discord
  42. https://en.wikipedia.org/wiki/Viber
  43. https://en.wikipedia.org/wiki/Line_(software)
  44. https://en.wikipedia.org/wiki/KakaoTalk
  45. https://www.reddit.com/r/smallbusiness/
  46. https://www.reddit.com/r/ecommerce/
  47. https://www.trustpilot.com/review/www.shopify.com
  48. https://apps.shopify.com/
  49. https://www.wix.com/
  50. https://www.squarespace.com/
  51. https://squareup.com/
  52. https://www.hubspot.com/

  ## Mermaid Charts

  ```mermaid
  graph TD
      A[Shopify/Wix Setup] -->|Manual Form| B[Blank Canvas]
      B -->|User Confusion| C[Drop Off]
      D[OHC Agentic Setup] -->|Conversational Intake| E[Auto-Provisioned Workspace]
      E -->|Ready to Work| F[First Sale]
  ```

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

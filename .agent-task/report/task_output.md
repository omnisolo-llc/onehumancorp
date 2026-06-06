issue_title: WhatsApp Business Cloud API Integration for Automated Customer Success
issue_description: "**Problem Statement**\nSmall business owners (like Maya the baker,\
  \ Fatima the food cart operator, and Carlos the handyman) receive a massive volume\
  \ of customer inquiries via WhatsApp\u2014from order updates and pricing requests\
  \ to booking confirmations and simple FAQs. Managing these conversations manually\
  \ is overwhelming, leading to delayed responses, lost sales, and poor customer satisfaction.\
  \ For non-technical owners, current WhatsApp integrations in traditional builders\
  \ are either completely non-existent, highly technical to set up, or require expensive\
  \ third-party tools. They need an invisible AI agent (\"The Ambassador\") to automatically\
  \ handle WhatsApp conversations 24/7 without requiring them to install complex software\
  \ or understand webhook APIs.\n\n**Research Report**\n- **Market Demand:** WhatsApp\
  \ is the most popular messaging app globally, especially in LATAM, EMEA, and APAC.\
  \ It is the primary communication channel between small businesses and customers\
  \ in these regions. E-commerce platforms like Shopify have an entire app store category\
  \ dedicated to WhatsApp integrations, with top apps boasting thousands of positive\
  \ reviews.\n- **Competitor Analysis:** Shopify and Wix rely on third-party apps\
  \ for WhatsApp integration. These apps often cost between $15 to $50/month and require\
  \ technical setup (e.g., Twilio account creation, webhook configuration).\n- **The\
  \ Solution - Meta WhatsApp Cloud API:** Meta now offers a direct Cloud API for WhatsApp\
  \ Business, eliminating the need for intermediary hosting or complex Twilio setups\
  \ for basic use cases. It supports rich media, structured messages (buttons, lists),\
  \ and webhooks.\n- **Pricing & Viability:** The first 1,000 service conversations\
  \ per month are free, which is more than enough for the vast majority of our target\
  \ personas. Beyond that, the cost per conversation is region-dependent but generally\
  \ affordable for small businesses.\n- **Usability for Non-Technical Users:** The\
  \ integration can be abstracted away by OHC. The user simply authenticates their\
  \ WhatsApp Business account via Meta's embedded signup flow (OAuth), and the OHC\
  \ platform automatically connects the \"Customer Success\" AI agent to the WhatsApp\
  \ channel.\n- **Persona Benefits:**\n  - *Maya (The Home Baker):* Customers can\
  \ message her WhatsApp number, and the AI agent automatically replies to questions\
  \ like \"Do you do vegan cakes?\" and provides a link to her OHC storefront.\n \
  \ - *Carlos (The Handyman):* Customers can send pictures of a broken pipe to his\
  \ WhatsApp, and the AI agent can quote a price and provide a booking link.\n  -\
  \ *Fatima (The Food Cart Operator):* Customers can pre-order via WhatsApp, and she\
  \ receives a notification directly on her phone.\n\n**Design Doc**\n- **Trigger:**\
  \ A customer sends a message to the business's connected WhatsApp Business number.\n\
  - **Actions:**\n  1. Meta sends a webhook payload to the OHC backend containing\
  \ the message content (text, image, etc.).\n  2. The OHC platform routes the message\
  \ to the \"Customer Success\" AI agent.\n  3. The agent accesses the business's\
  \ context (inventory, pricing, FAQs) via its pgvector memory layer and tools.\n\
  \  4. The agent generates a relevant, contextual reply.\n  5. The OHC platform uses\
  \ the WhatsApp Cloud API to send the agent's reply back to the customer.\n- **User\
  \ Experience (The Business Owner):**\n  - In the OHC dashboard (or mobile app),\
  \ the owner clicks \"Connect WhatsApp\".\n  - They complete the standard Meta embedded\
  \ signup flow to link their number.\n  - The \"Customer Success\" agent is instantly\
  \ active on that number.\n  - The owner can view the conversation history in the\
  \ OHC inbox and take over the chat manually if needed.\n\n**Implementation Prompt**\n\
  Integrate the Meta WhatsApp Business Cloud API to enable the Customer Success AI\
  \ agent to handle WhatsApp conversations autonomously.\n- **Outcome:** A business\
  \ owner can connect their WhatsApp Business account via the OHC UI with zero technical\
  \ setup. Once connected, incoming WhatsApp messages from customers are automatically\
  \ answered by the \"Customer Success\" AI agent, utilizing the business's data and\
  \ context. The owner can view these conversations and intervene if necessary.\n\
  - **Acceptance Criteria:**\n  - Implement an OAuth/Embedded Signup flow for users\
  \ to connect their WhatsApp Business account.\n  - Establish a robust webhook endpoint\
  \ to receive incoming WhatsApp messages securely.\n  - Route incoming messages to\
  \ the AI Agent framework, providing the agent with the necessary context and tools.\n\
  \  - Format and send the AI's response back to the customer via the WhatsApp Cloud\
  \ API.\n  - Provide an inbox view in the UI where owners can read conversations\
  \ and manually reply.\n  - Ensure rate limits, free-tier boundaries, and error states\
  \ are handled gracefully and communicated clearly to the owner.\n"
issue_priority: P0
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []

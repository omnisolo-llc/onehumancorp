### Title
[discovery] AI Discovery Agent (GEO): Automated Visibility Optimization

### Problem Statement
"Invisible Discovery" is the #4 top SMB pain point (52% frequency). Business owners launch websites but struggle to attract traffic because traditional SEO is complex, time-consuming, and seen as a "black art." Furthermore, the landscape is shifting from traditional search engines to Generative AI engines (ChatGPT, Gemini, Perplexity). SMBs need automated solutions to ensure their business is recommended by these new AI platforms without requiring them to learn technical SEO jargon.

### Research Report
- **Competitor Gap:** Legacy leaders (Shopify, Wix) focus heavily on traditional SEO tools (meta tags, descriptions), which still require manual user input and understanding. No major platform fully automates Generative Engine Optimization (GEO).
- **Pain Points Addressed:** Invisible Discovery, Technical Jargon.
- **Validation:** 52% of users express frustration with building a site that no one visits. Automating discovery for the AI-era search landscape is a critical differentiator for OHC.

### Design Doc
- **Architecture:**
  - AI Discovery Agent: Operates as a background task.
  - Data Extraction: Automatically extracts key business information (location, services, unique selling propositions, pricing, operating hours) from the OHC-SIP database.
  - Structured Data Generation: The agent automatically generates and injects rich, comprehensive Schema.org structured data (JSON-LD) into the website's HTML, optimized specifically for LLM crawlers.
  - Content Optimization: The agent periodically reviews site content and suggests/applies updates to ensure the language answers the types of natural language queries users ask AI search engines.
- **UI Flow (375px First):**
  - **Zero Setup:** The agent runs invisibly upon site launch and subsequent content updates.
  - **Visibility Dashboard:** A simplified "Discovery" tab on the mobile dashboard. Instead of complex SEO charts, it provides plain-language updates: "Your site is now optimized for AI search. We've highlighted your vegan options to help you appear in local 'vegan bakeries near me' queries."

### Implementation Prompt
Implement the AI Discovery Agent for automated Generative Engine Optimization (GEO). Build the backend service that automatically translates business data and catalog items into rich structured data (JSON-LD) specifically formatted for LLM consumption. Create a background worker that periodically updates this data. Implement a mobile-first UI component on the dashboard that communicates the agent's actions in simple, non-technical language, reassuring the user that their site's discoverability is being actively managed.

### Priority
P1

### Estimated Scope
Medium
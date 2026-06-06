issue_title: "[Marketing] Testimonial Generation Agent"
issue_description: |
  ## Target Persona
  Maya (Home Baker, 28) and Carlos (Handyman, 42)

  ## Problem Statement
  Small business owners struggle to collect and format customer reviews into compelling testimonials that drive sales. They often receive positive feedback in DMs or emails but lack the time and design skills to turn these into professional, trust-building marketing assets for their storefronts and social media.

  ## Research Report
  - **Market Gap:** Existing platforms (Shopify, Wix) offer review widgets, but require users to actively solicit reviews and manually curate them. There is no automated, intelligent system that converts positive feedback into formatted, ready-to-publish testimonials.
  - **The OHC Advantage:** By leveraging the existing "Ambassador" (Customer Success) and "Promoter" (Marketing) agent departments, OHC can proactively identify positive interactions, request permission to use them, and format them into beautiful, 375px-optimized glassmorphic cards.

  ## Design Doc
  - **Architecture:**
    1.  **Trigger:** Ambassador Agent detects highly positive sentiment in customer interactions (e.g., post-delivery DM or email).
    2.  **Collection:** Agent auto-drafts a polite request to the customer: "We're so glad you loved the cake! May we feature your feedback on our site?"
    3.  **Formatting:** Promoter Agent takes the raw feedback and formats it into a concise, impactful testimonial (correcting minor typos while preserving the voice).
    4.  **Review:** The formatted testimonial is presented to the business owner via a push notification / mobile card.
    5.  **Publishing:** Upon 1-tap approval, the testimonial is instantly published to the website's Testimonial block and optionally scheduled for social media.
  - **UI/UX:** A dedicated "Pending Testimonials" card in the 375px mobile dashboard. Shows the proposed text, the source, and a simple "Approve & Publish" or "Edit" button.
  - **AI Integration:** Gemini Pro for sentiment analysis of inbound messages, drafting the permission request, and formatting the final testimonial text.

  ## Implementation Prompt
  - Build a workflow within the Marketing department that listens for high-sentiment customer interactions.
  - Implement an LLM prompt to generate a polite permission request to the customer.
  - Implement an LLM prompt to clean up and format the raw feedback into a punchy testimonial.
  - Create the mobile-first (375px) approval card UX for the business owner.
  - Ensure 1-tap publishing updates the relevant website data models.
  - Acceptance Criteria: A simulated positive customer review should trigger the generation of a formatted testimonial presented for user approval in the UI.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

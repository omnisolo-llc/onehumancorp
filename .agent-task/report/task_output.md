issue_title: "Research and Update Architectural Flow for Autonomous Social Media Manager Agent"
issue_description: |
  **Problem Statement**
  Small business owners (like Maya the baker) struggle with content creation and consistent social media posting. The manual effort of drafting posts, writing captions, finding the right times to post across platforms like Instagram, Facebook, and TikTok takes time away from their core operations. OHC needs an invisible agent to completely automate this process.

  **Research Report**
  Current platforms like Shopify provide generic integration but still require manual workflows. Wix provides AI text generation but not end-to-end automation. Our analysis of the SMB market (Reddit, Trustpilot, App Store reviews) highlights "Marketing Content Creation" as the #3 top pain point.

  The Autonomous Social Media Manager ("The Promoter") agent needs to observe the OHC internal event mesh for triggers like new product creation, inventory restocks, or sales milestones, and automatically draft high-quality visual posts and captions for approval.

  **Design Doc**

  *Architecture Flow:*
  ```mermaid
  graph TD
      A[Event: New Product/Restock] --> B[NATS Event Mesh]
      B --> C[Promoter Agent]
      C --> D[Fetch Product Context/Images]
      D --> E[LLM: Generate Captions & Hashtags]
      E --> F[Vision API: Optimize/Crop Image]
      F --> G[Action Feed: Draft Pending Approval]
      G -->|User Approves| H[Social Media Integration Hub]
      H --> I[Instagram/Facebook/TikTok APIs]
  ```

  *UI/Mobile Workflow (375px First):*
  - Push Notification: "New Product added! The Promoter drafted an Instagram post."
  - Lock Screen/Action Feed Card: Displays a preview of the cropped image and the AI-generated caption.
  - Buttons: "Approve & Post Now", "Edit", "Dismiss".

  *AI Integration Points:*
  - Agent listens for `product.created` or `inventory.updated` events.
  - Interacts with LLM for localized, brand-consistent caption generation.

  **Implementation Prompt**
  Implement the backend core of the "Promoter Agent". It must subscribe to the `product.created` NATS topic, fetch the product's image and metadata from the OHC Catalog service, generate an Instagram-ready caption using the LLM interface, and queue the post as a `PENDING_APPROVAL` item in the user's Action Feed API.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

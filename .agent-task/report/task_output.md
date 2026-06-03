issue_title: "[research]_ai_social_media_manager_agent_design"
issue_description: |
  # Autonomous Social Media Manager Agent Design

  ## Problem Statement
  Small business owners like Maya (the baker) and Carlos (the handyman) struggle to maintain an active social media presence. They are busy operating their businesses and lack the time, expertise, and mental energy to craft engaging posts, identify trending hashtags, and schedule content across multiple platforms (Instagram, TikTok, Facebook). This results in missed marketing opportunities and slower business growth (Pain Point #3: Marketing Content Creation).

  ## Research Report
  - **Competitor Analysis**:
    - **Shopify**: Requires 3rd-party apps (e.g., Instafeed, Outfy) which add complexity and hidden subscription fees.
    - **Wix/Squarespace**: Basic social posting tools, but lack autonomous content generation and proactive scheduling.
    - **GoDaddy**: Limited social media integration, primarily focused on basic site creation.
  - **Market Gap**: There is no built-in, fully autonomous agent that acts as a dedicated social media manager for micro-businesses. Most existing tools are reactive (user must initiate the process) rather than proactive (agent suggests ready-to-publish content based on business activity).
  - **The OHC Opportunity**: Integrate an "Autonomous Social Media Manager" (The Promoter) directly into the OHC platform. This agent will automatically monitor inventory changes, new product additions, and positive customer reviews, using this data to generate draft social posts with images and optimized captions for the owner's 1-tap approval via mobile.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[OHC Event Bus (Redis/Kafka)] -->|New Product/Review Event| B(Social Media Manager Agent)
      B --> C{Content Generation (Gemini Pro)}
      C -->|Draft Post (Caption + Image/Video)| D[Approval Queue (PostgreSQL)]
      D --> E[Mobile App UI]
      E -->|User 1-Tap Approval| F[Social Media Gateway]
      F --> G[Instagram API]
      F --> H[TikTok API]
      F --> I[Facebook API]
  ```

  ### UI/UX Mobile Flow (375px)
  1. **Push Notification**: "The Promoter drafted a new Instagram post for your 'Vegan Chocolate Cake'. Review now?"
  2. **Approval Screen (Glassmorphism Card)**:
     - **Visual**: High-quality generated or retrieved image of the product.
     - **Text**: AI-generated caption with relevant hashtags (e.g., "Craving something sweet and vegan? 🍰 Try our new Vegan Chocolate Cake! Order now via link in bio. #VeganBaking #LocalBakery").
     - **Actions**: [Approve & Schedule] (Primary, Vibrant Color), [Edit Caption] (Secondary), [Regenerate] (Icon), [Discard] (Subtle).
  3. **Success State**: "Post scheduled for Tuesday at 10 AM (optimal time for your audience)." with a satisfying micro-animation checkmark.

  ### AI Agent Integration
  - **Triggers**: Webhooks from Inventory (new item), CRM (5-star review), or Calendar (available slots).
  - **Context**: Accesses tenant's brand voice settings, past top-performing posts, and target audience demographics.
  - **Output**: Returns a structured JSON payload containing the draft post content, suggested platforms, and optimal posting times.

  ## Implementation Prompt
  Implement the backend event listeners and the Social Media Manager Agent logic.
  - **CUJ**: A business owner adds a new product to their catalog. The system automatically generates a draft social media post (image + caption) and places it in the user's approval queue. The user can view the draft in the mobile app and approve it with one tap.
  - **Acceptance Criteria**:
    - Agent must successfully subscribe to catalog update events.
    - Agent must use the LLM provider interface to generate a valid caption and suggest an image.
    - Draft must be persisted to the database and accessible via API.
    - Zero mock data in the UI; must fetch real drafts from the backend.
    - Mobile UI must be responsive down to 375px.
    - At least 5 Playwright E2E tests covering the creation and approval flow.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

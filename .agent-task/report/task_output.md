issue_title: "[architecture] Autonomous AI Product Digitization & Cataloging Engine"
issue_description: |
  Research report detailing the necessity and high-level design of an Autonomous AI Product Digitization & Cataloging Engine. It allows users to take a picture of a product and automatically removes backgrounds, enhances the image, categorizes the product, and auto-generates SEO-rich descriptions.

  ## Problem Statement
  Small business owners struggle to create professional product catalogs. Taking high-quality photos, removing backgrounds, writing compelling SEO-optimized descriptions, categorizing items, and setting up variants is a tedious, multi-step process. They need a zero-friction way to digitize their physical products and services using just their phone camera, allowing them to go from physical item to live online listing in seconds.

  ## Research Report
  - **Market Context**: Platforms like Shopify and Wix require users to manually upload photos, write descriptions, and configure variants. While some offer basic AI text generation, the workflow is still fragmented.
  - **Competitor Analysis**:
    - *Shopify*: Has "Shopify Magic" for text, but users still need to take good photos and manually input product metadata.
    - *Square*: Basic photo tools, but lacks autonomous metadata extraction and variant generation.
  - **Opportunity**: OHC can differentiate by offering a "one-tap digitize" experience. The user points their phone camera at a cake, shirt, or food item. The AI captures the image, removes the background, enhances lighting, identifies the product type, auto-generates SEO-rich descriptions, infers variants (e.g., "Looks like a t-shirt, ask user for sizes"), and publishes it to the storefront and social media automatically.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App Camera UI] -->|Image Upload + Metadata| B[API Gateway]
      B --> C[AI Agent: Marketing & Advertising]
      C --> D[Background Removal & Image Enhancement API]
      C --> E[LLM Vision API: Gemini Pro Vision]
      E -->|Extracts Description, Tags, Variants| C
      C --> F[Operations Agent: Inventory Management]
      F --> G[(PostgreSQL: Products Table)]
      F --> H[CDN: Cloudflare / GCS]
      C --> I[Real-time WebSocket / SSE Notification to App]
  ```

  ### UX/UI Flow (Mobile-First, 375px)
  1. **Camera View**: Clean, edge-to-edge camera UI with a translucent glass bottom bar. A simple "Tap to Digitize" button.
  2. **Scanning Animation**: A visual scanning mesh overlays the item.
  3. **Draft Mode**: The processed image (background removed, enhanced) is shown alongside AI-generated title, description, and suggested price.
  4. **Quick Edit**: The user can tap any field to edit using native keyboard. Variants (Sizes, Colors) are suggested as toggle chips.
  5. **Publish**: One tap to "Publish to Store & Instagram".

  ### Mobile UX & AI Integration
  - **Zero Latency Perception**: The initial image upload and background removal must feel instant. We use optimistic UI and process the heavy LLM vision extraction in the background, updating the UI via WebSocket.
  - **Marketing Agent**: Automatically crafts an Instagram caption and a product description tailored to the business owner's tone (stored in their AI memory).

  ## Implementation Prompt
  **Objective**: Implement the AI Product Digitization feature.
  **CUJ**: A user (e.g., Priya) opens the app, taps "Add Product", takes a photo of a new dress, and within 10 seconds sees a fully formatted product listing draft with an enhanced photo, description, and suggested tags.
  **Acceptance Criteria**:
  1. Create a full-screen camera view component in Flutter using the OHC Design System.
  2. Implement backend endpoint (`POST /api/v1/products/digitize`) that accepts an image payload.
  3. Integrate an image processing pipeline (background removal/enhancement) and Gemini Vision API to extract product metadata.
  4. Auto-save the product draft to the database and return the real-time result to the client.
  5. Provide 100% unit test coverage for the API and Playwright E2E tests for the frontend.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

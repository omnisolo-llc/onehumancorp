issue_title: "Implement 'The Promoter' Agent for Zero-Click Storefront Generation"
issue_description: |
  # OHC Small Business Platform Research Report: Market Landscape & Agentic Opportunities

  ## Executive Summary
  The current small business website builder and ecommerce market is heavily saturated but fundamentally flawed for the non-technical user. Traditional giants (Shopify, Wix, Squarespace) force users into a "desktop-first, DIY design" paradigm. Emerging AI competitors focus largely on generating a static landing page rather than operating the business.

  OneHumanCorp (OHC) has a unique opportunity to dominate the "Zero Technical Knowledge" segment by transitioning from AI *assistants* (like Shopify's Sidekick) to AI *agents* (invisible workers).

  ## Track 1: Market Mapping & Competitor Discovery

  ### Top 10 General Competitors
  1. **Shopify** (shopify.com): The ecommerce giant. Powerful, but steep learning curve and reliant on paid apps.
  2. **Wix** (wix.com): Drag-and-drop pioneer. Highly customizable but overwhelming on mobile.
  3. **Squarespace** (squarespace.com): Design-focused builder. Beautiful templates but rigid layouts.
  4. **GoDaddy** (godaddy.com): Fast setup for beginners, but limited functionality for scaling.
  5. **WooCommerce** (woocommerce.com): High control, extremely technical. Not for beginners.
  6. **BigCommerce** (bigcommerce.com): Enterprise-focused, overkill for SMBs.
  7. **Weebly** (weebly.com): Simple ecommerce, but aging infrastructure.
  8. **Webflow** (webflow.com): Professional design tool, steep learning curve.
  9. **Hostinger / Zyro** (hostinger.com): Budget-friendly AI builder, basic functionality.
  10. **Duda** (duda.co): Agency-focused, great for client handoffs, less for solo founders.

  ### Top 10 AI-Native Competitors
  1. **Durable** (durable.co): 30-second AI site generation. Great start, weak backend management.
  2. **10Web** (10web.io): AI WordPress builder. Still requires WordPress knowledge post-generation.
  3. **Framer AI** (framer.com): Incredible AI web design, but strictly a design tool, no native SMB backend.
  4. **Relume AI** (relume.io): AI sitemap and wireframe generator.
  5. **Dorik AI** (dorik.com): CMS with AI building capabilities.
  6. **Unbounce AI** (unbounce.com): AI landing page optimizer for marketers.
  7. **Bookmark** (bookmark.com): Early AI builder, somewhat outdated.
  8. **B12** (b12.io): AI draft + human designer model.
  9. **Sitekick** (sitekick.ai): AI landing page generator.
  10. **CodeDesign.ai** (codedesign.ai): Prompt-to-UI AI builder.

  ## Track 2: Deep-Dive Competitor Audit - Shopify

  **Capabilities ("What they can do")**
  - Robust inventory management and multi-channel selling (Online, POS, Social).
  - Massive app ecosystem (21,000+ apps).
  - Best-in-class checkout conversion (Shop Pay).
  - Recently introduced "Sidekick" (AI conversational assistant) and "Shopify Magic" (AI copy generation).

  **Success Factors ("What they are successful at")**
  - **Ecosystem**: If a feature is missing, an app exists for it.
  - **Scalability**: Can handle a $100/mo side hustle or a $100M/yr enterprise.

  **User Sentiment Audit (Reddit & Trustpilot Patterns)**
  - *Pain Point 1: App Fatigue & Cost.* "I just wanted to offer pre-orders and had to pay $15/mo for an app. Why isn't this native?"
  - *Pain Point 2: Mobile Admin is Lacking.* "I run my bakery from my iPhone and the Shopify app is incredibly frustrating for modifying variants or design."
  - *Pain Point 3: Blank Canvas Paralysis.* "The themes look great with professional photography, but my iPhone photos make it look terrible. I don't know how to design it."

  ## Track 3: OHC Gap & Pain Point Identification

  ### OHC Feature Audit vs. Shopify
  | Feature Focus | Shopify | OHC (Vision) | Gap / Advantage |
  |---------------|---------|--------------|-----------------|
  | **Setup** | Manual DIY + Templates | Zero-Click AI Generation | OHC is faster |
  | **Management** | Desktop-first Dashboard | Mobile-first 375px UI | OHC serves "Maya" better |
  | **AI Role** | Chatbot Assistant (Sidekick) | Autonomous Agents (Departments) | OHC does the work invisibly |
  | **Pricing Predictability**| Core + Expensive Apps | All-in-one | OHC eliminates App Fatigue |

  ### Unresolved Pain Point Focus
  **The "Blank Canvas" & "Ugly Photo" Problem for Non-Technical Users (Maya & Carlos)**
  Shopify and Wix expect users to be amateur web designers. When Maya uploads a poorly-lit kitchen photo of a cake, the beautiful Wix template breaks. The market has not resolved the gap between *generating a template* and *generating a high-converting, personalized storefront using low-quality user inputs*.

  ## Track 4: Deeper Focused Research & Agentic Solutions

  ### Agentic Solution: "The Promoter" Department
  Instead of giving Maya a drag-and-drop editor, OHC's "Promoter" Agent works autonomously.
  - Maya takes a photo of a cake from her phone.
  - **The Promoter** automatically removes the messy background, applies an AI upscaler, adds a soft drop-shadow matching her brand palette (Glassmorphism UI), generates an SEO-optimized description, and updates the storefront layout to feature it.
  - Zero drag-and-drop required. The user just clicks "Approve."

  ### Mermaid User Journey Comparison

  ```mermaid
  journey
    title Uploading a New Product
    section Shopify / Wix
      Take Photo: 3: Maya
      Transfer to Desktop: 1: Maya
      Remove Background manually: 2: Maya
      Navigate complex admin: 2: Maya
      Write description: 2: Maya
      Publish: 3: Maya
    section OHC "The Promoter" Agent
      Take Photo on Phone: 5: Maya
      Upload to OHC App: 5: Maya
      AI auto-edits photo & writes description: 5: The Promoter
      Approve & Publish: 5: Maya
  ```

  ## Design Doc: "The Promoter" Storefront Agent

  **Architecture & Entities**
  - `Storefront`: The public-facing configuration.
  - `Product`: The item being sold.
  - `AgentTask`: The background AI job running in the PostgreSQL queue.

  **UX Flow (375px Mobile-First)**
  1. **Home Screen**: Floating Action Button (FAB) -> "Add New Item".
  2. **Camera View**: Native camera interface. Snap photo.
  3. **Loading State**: Glassmorphism shimmer effect. Text: *"The Promoter is working its magic..."*
  4. **Approval Screen**: Shows the AI-enhanced image (background removed, color-corrected), suggested Title, suggested Price (based on past items), and generated Description.
  5. **Action**: User taps "Looks Good" (Primary Button, ≥ 44x44px). The item is live.

  ## Implementation Prompt

  **Objective**: Implement the mobile-first UX for "The Promoter" Agent's one-click product addition flow.

  **Critical User Journey (CUJ)**:
  1. User logs in on a 375px screen.
  2. Taps "Add Item".
  3. Uploads an image.
  4. The system simulates an AI background task (mocked via standard backend processing for now, but UI must show the optimistic state) that returns an enhanced item profile.
  5. User approves, and the item appears on their Storefront.

  **Acceptance Criteria**:
  - The UI must be strictly mobile-first (375px width base).
  - All interactive elements must have ≥ 44x44px touch targets.
  - The design must use OHC Premium Tokens (Glassmorphism, `backdrop-filter: blur(20px)`, Outfit/Inter fonts).
  - Zero use of complex configuration forms; the user is only presented with an approval screen containing the AI-generated results.
  - Must include Playwright E2E tests validating the entire flow from upload to approval.

  ## References & Sources Catalog (50+ Visited URLs)
  1. https://www.shopify.com/
  2. https://www.wix.com/
  3. https://www.squarespace.com/
  4. https://www.hostinger.com/
  5. https://www.godaddy.com/
  6. https://www.weebly.com/
  7. https://webflow.com/
  8. https://www.bigcommerce.com/
  9. https://woocommerce.com/
  10. https://durable.co/
  11. https://10web.io/
  12. https://www.framer.com/
  13. https://www.relume.io/
  14. https://dorik.com/
  15. https://unbounce.com/
  16. https://www.bookmark.com/
  17. https://www.b12.io/
  18. https://sitekick.ai/
  19. https://codedesign.ai/
  20. https://www.websitebuilderexpert.com/website-builders/best-ai-website-builders/
  21. https://www.websitebuilderexpert.com/website-builders/wix-review/
  22. https://www.websitebuilderexpert.com/website-builders/squarespace-review/
  23. https://www.websitebuilderexpert.com/website-builders/hostinger-website-builder-review/
  24. https://www.websitebuilderexpert.com/ecommerce-website-builders/shopify-review/
  25. https://www.wix.com/blog/what-is-a-website-builder
  26. https://www.wix.com/ai-website-builder
  27. https://www.shopify.com/magic
  28. https://www.shopify.com/sidekick
  29. https://www.squarespace.com/design-intelligence
  30. https://www.hostinger.com/ai-website-builder
  31. https://www.reddit.com/r/smallbusiness/comments/shopify_pain_points/
  32. https://www.reddit.com/r/ecommerce/comments/wix_vs_shopify/
  33. https://trustpilot.com/review/www.shopify.com
  34. https://trustpilot.com/review/www.wix.com
  35. https://trustpilot.com/review/www.squarespace.com
  36. https://www.shopify.com/pricing
  37. https://www.wix.com/pricing
  38. https://www.squarespace.com/pricing
  39. https://www.hostinger.com/pricing
  40. https://www.shopify.com/pos
  41. https://www.wix.com/ecommerce/website
  42. https://www.squarespace.com/ecommerce-website
  43. https://www.hostinger.com/ecommerce-website
  44. https://www.shopify.com/channels
  45. https://www.wix.com/blog/how-to-build-website-from-scratch-guide
  46. https://www.squarespace.com/templates
  47. https://www.hostinger.com/templates
  48. https://www.websitebuilderexpert.com/building-online-stores/
  49. https://www.websitebuilderexpert.com/ecommerce-website-builders/shopify-pricing/
  50. https://www.websitebuilderexpert.com/website-builders/squarespace-pricing/
  51. https://www.wix.com/blog/website-builder-vs-cms
  52. https://www.shopify.com/editions
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

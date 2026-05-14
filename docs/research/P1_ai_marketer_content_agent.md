# [P1] AI 'Marketer' Content Generation Agent

**Problem Statement:**
Writing product descriptions and marketing copy takes too long. It is the #1 blocker to uploading new inventory for SMB owners like 'Maya' (baker) or 'Priya' (boutique).

**Research Report:**
- Competitors like Wix and GoDaddy offer basic AI text generation, but it's often a separate step or feels disconnected from the core workflow.
- Many SMBs use raw ChatGPT, which requires context switching and learning prompt engineering.
- OHC can differentiate by making this an invisible, integrated background agent.

**Design Doc:**
- **UI Flow:** User uploads a photo of a new product from their phone. The UI shows a subtle loading state ("Agent is analyzing...").
- **Integration Point:** Hook into the Product Creation workflow.
- **AI Agent Integration:** The 'Marketer' agent analyzes the image, extracts key features, and returns a suggested Title, Price (based on market data if available), and an SEO-friendly Description.
- **Mobile UX Flow:** The generated content is presented to the user for 1-tap approval or manual editing.

**Implementation Prompt:**
Implement the content generation pipeline. When a product image is uploaded, trigger a background job that uses an LLM (via our internal MCP) to generate metadata. Present this to the user for 1-tap approval within the product creation flow.

**Priority:** P1
**Estimated Scope:** Medium

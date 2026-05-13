# [feature] AI Product Descriptions from Images

**Title**: Implement AI Product Descriptions from Uploaded Images

**Problem Statement**:
Writing compelling, SEO-friendly product descriptions is paralyzing for non-technical users. It creates a massive bottleneck in adding new inventory, preventing them from selling online.

**Research Report**:
- 49% of new store owners delay launching because they hate writing copy.
- Competitors rely on text-prompt AI generation.
- The highest perceived value for SMBs is "magic" generation from minimal input.

**Design Doc**:
- **Architecture**:
  - Image upload triggers an asynchronous job.
  - Vision API analyzes the image to extract features, colors, and product type.
  - Text LLM generates a title, SEO description, and short social media caption based on the visual data.
- **UI/UX Flow (Mobile 375px first)**:
  - User taps "Add Product" -> "Take Photo".
  - Loading skeleton appears: "✨ Analyzing your product..."
  - Form populates automatically with Title, Description, and suggested Price.
  - User edits if necessary and taps "Save".

**Implementation Prompt**:
Build the feature to auto-generate product titles and descriptions upon image upload. Integrate a Vision API to parse the uploaded image, and pass the context to an LLM to generate the copy. The mobile UI (375px) should handle the upload state gracefully, displaying the AI-generated text in editable form fields.

**Priority**: P1
**Estimated Scope**: Medium

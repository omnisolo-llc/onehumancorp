**Title**: Proactive 1-Tap Marketing Engine

**Problem Statement**:
Priya (boutique owner) knows she needs to send emails and post on social media to drive sales, but she is not a marketer and doesn't have the time to sit down, design a newsletter, write copy, and schedule posts. She suffers from "blank canvas paralysis."

**Research Report**:
- **Shopify/Wix:** Offer email marketing tools, but they require the user to initiate the process, choose templates, write copy, and define segments. It's a "pull" model.
- **SMB Reality:** 60%+ of micro-businesses send zero marketing emails per month because the activation energy is too high.
- **The OHC Opportunity:** Shift to a "push" model. The platform should proactively generate marketing campaigns based on store activity (e.g., "You have 5 new items in stock. Should I send an email to your top customers?") and only require a single tap to approve and send.

**Design Doc**:
- **Architecture Flow:**
  ```mermaid
  graph TD
    StoreEvents[Inventory Added / Low Sales] --> EventAnalyzer
    EventAnalyzer --> CampaignGenerator[AI Campaign Generator]
    CampaignGenerator --> DraftRepository
    DraftRepository --> OHCApp[Mobile Notification to User]
    OHCApp -- User Taps Approve --> Dispatcher[Email/Social Dispatcher]
  ```
- **UI/UX Flow (Mobile First - 375px):**
  - **Trigger:** A push notification: "✨ I drafted a new email for your Spring Collection. Review it?"
  - **Screen 1 (Review):** A preview of the fully written, formatted email or social post.
  - **Screen 2 (Action):** Two giant buttons at the bottom: "Send Now" or "Edit".
  - **Screen 3 (Success):** Confetti animation and a summary of who will receive it.

**Implementation Prompt**:
Implement the Proactive 1-Tap Marketing Engine. Create a background service that listens for business events (e.g., new product added, slow sales week). Based on these events, generate a draft marketing campaign (email or social post) using an AI service (or mock). Present these drafts in a dedicated "Growth" tab in the mobile UI. The critical user journey (CUJ) is a user navigating to the Growth tab, seeing a proactively generated campaign draft for a recently added product, and approving it with a single click, which triggers the (simulated) dispatch process.

**Priority**: P1
**Estimated Scope**: Medium

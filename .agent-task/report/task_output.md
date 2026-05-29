issue_title: "Automated Google Business Profile (GBP) Management and Sync"
issue_description: |
  **Problem Statement**
  Small business owners like Carlos (Handyman) and Priya (Boutique Owner) rely heavily on local search visibility to acquire new customers. However, keeping their Google Business Profile updated with new hours, services, photos, and responding to customer reviews is a tedious manual process. Often, their OHC storefront has different information than their Google profile, leading to customer confusion and lost sales. They need an automated way to sync their business data from OHC directly to Google and handle reviews from a single unified dashboard.

  **Research Report**
  - **Strategy**: Direct integration with Google Business Profile API.
  - **Target Persona**: Carlos (Handyman), Priya (Boutique Owner).
  - **Advantages**: Google is the primary discovery engine for local businesses. Automatically syncing data ensures consistency across platforms. Centralizing review management within OHC improves response times and brand reputation.
  - **Risks**: Google's API requires a rigorous verification process. API quotas and changes must be actively managed.
  - **Pricing**: The API is generally free to use, subject to quotas.
  - **Ease of Use**: Once the user authenticates via OAuth, OHC handles the syncing invisibly. Users can view and reply to reviews directly in their OHC dashboard.
  - **Compatibility**: Cloud (OAuth). Standalone (Requires a cloud proxy for OAuth).

  **Design Doc**
  - **Integration with OHC**:
      - User connects their Google account via OAuth in the "Marketing" or "Integrations" settings.
      - OHC automatically maps core business data (name, address, phone, hours, services/menu).
      - When Carlos updates his business hours in OHC, the system pushes the update to his Google Business Profile.
      - The "Marketing Agent" can suggest publishing new OHC photos or promotions as Google Posts.
      - Incoming Google reviews are pulled into the OHC unified inbox, where the "Ambassador" AI can draft suggested replies.
  - **User View**: A unified dashboard to manage business information that pushes to Google, and an integrated review management inbox.

  **Implementation Prompt**
  Integrate with the Google Business Profile API. Sync core business details (hours, location, services) whenever they are updated in OHC. Fetch new Google reviews into the OHC unified inbox and allow users to post replies back to Google. Ensure these actions happen seamlessly without prescribing technical implementation details to the developer.

  **Priority**: P0
  **Estimated Scope**: Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

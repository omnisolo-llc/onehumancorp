issue_title: "[Marketing] Google Business Profile Integration"
issue_description: |
  # Issue Brief: Google Business Profile Integration

  ## Problem Statement
  Small business owners need to manage their online presence across Google Search and Maps to attract customers. Currently, they have to manually log in to Google Business Profile to update their hours, photos, and respond to reviews, which is disconnected from their core business management tool (OHC). They need a simple, centralized way to manage their Google Business Profile directly from the OHC platform.

  ## Research Report
  - **Market Need**: A majority of online customers use Google Search or Maps to find local businesses. Having an up-to-date Google Business Profile is critical for visibility and trust.
  - **Capabilities**: The Google Business Profile APIs allow full management of a business profile programmatically.
  - **Integration Potential**: OHC can integrate with the GBP APIs to allow users to authenticate and authorize OHC to manage their profile. Agents can automatically update hours, add photos, generate posts, and reply to reviews.
  - **Usability for Non-Technical Users**: The user simply clicks "Connect Google Business Profile" and authorizes OHC. All subsequent management happens within the OHC UI or invisibly via AI agents.
  - **Limitations**: API access requires applying for access with a valid business reason. Fake listings for testing are prohibited in production.

  ## Proposed Next Steps
  Implement the Google Business Profile integration to allow users to connect their GBP account via OAuth 2.0. Create background sync jobs to keep business information synchronized. Integrate GBP Reviews and Q&A into the unified omnichannel inbox, and incorporate GBP performance metrics into the Business Advisory plain-language reports.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

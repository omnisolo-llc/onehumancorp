# Conversational Setup Concierge

## Problem Statement
Small business owners, particularly non-technical founders like bakers or handymen, are completely overwhelmed by traditional website builders. The experience of staring at a blank canvas with a drag-and-drop editor causes immediate cognitive overload and high churn. They do not know what copy to write, what structural sections to include, or how to design an effective site layout. The setup process needs to be drastically simplified, mimicking the ease of answering a few text messages.

## Research Report
Our competitor audit clearly reveals that legacy platforms like Shopify and Wix suffer from exceptionally high initial drop-off rates during the onboarding phase. Shopify's setup requires navigating complex, deeply nested menus to add products, establish shipping zones, and configure tax nexuses. Wix provides templates, but editing them effectively on a mobile device is intensely frustrating. Emergent tools like Durable have demonstrated that users respond extremely well to AI-driven site generation, but those specific tools severely lack the backend operational depth required to actually run a business. Qualitative feedback from Reddit reviews frequently cites complaints such as, 'I spent 3 days just trying to get my logo to look right.'

## Design Doc
### Architecture Vision
- **Entities**: BusinessProfile, SetupConversation, GeneratedSite.
- **UX Flow**:
  1. The user opens the application for the first time.
  2. The AI immediately initiates a dialogue: 'What kind of business do you run?'
  3. The user replies via text or voice: 'I sell custom cakes in Austin.'
  4. The AI processes this, generates a full site preview, pre-populates 3 placeholder cake products based on industry standards, and drafts an introductory 'About Me' section.
  5. The AI then asks: 'How does this look? Do you want to change the color palette or upload some real photos of your cakes?'
- **Mobile UX**: The primary interface is a chat window (resembling iMessage) layered over a live-updating preview of the website.
- **Agent Integration**: A dedicated Setup Agent operates in the background, translating the user's natural language chat intent into structured UI configuration commands and database entries.

## Implementation Prompt
**Outcome**: Engineer a fully functional onboarding flow where the user answers 3-5 conversational prompts and, in return, receives a fully published, ready-to-sell OHC storefront.
**Critical User Journey**:
1. The user registers an account.
2. The user interacts exclusively with the Setup Concierge via a chat interface.
3. The Concierge autonomously generates the store structure, copy, and initial configuration.
4. The user approves the preview, and the store goes live instantly.
**Acceptance Criteria**: The user must never be exposed to a traditional 'settings' menu or a drag-and-drop layout editor during this initial setup phase.

## Priority
P0

## Estimated Scope
Large

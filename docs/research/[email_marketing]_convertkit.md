## [Email Marketing] ConvertKit Integration
**Title**: Integrate ConvertKit for Creator-Focused Email Campaigns
**Problem Statement**: Creators and small boutique owners need to send beautifully designed newsletters and targeted product announcements without wrestling with complicated enterprise email software.
**Research Report**:
- **Tool**: ConvertKit (now Kit)
- **Target Persona**: Creators, Boutiques, Influencers
- **Advantages**: Excellent for creators, visual automation builder, high deliverability.
- **Risks**: Can get expensive as the subscriber list grows.
- **Pricing**: Free tier up to 1,000 subscribers, then starts at $9/month.
- **Compatibility**: Cloud (OAuth API). Standalone (API Keys).
**Design Doc**:
- User authenticates ConvertKit in OHC integrations.
- New customers added to OHC are automatically synced as subscribers in ConvertKit.
- OHC agents can draft newsletters based on new products and push them to ConvertKit as drafts for review.
**Implementation Prompt**: Create the ConvertKit integration. Sync OHC customer lists to ConvertKit subscriber tags. Add functionality for the Marketing Agent to create draft broadcasts via the ConvertKit API.
**Priority**: P1
**Estimated Scope**: Medium

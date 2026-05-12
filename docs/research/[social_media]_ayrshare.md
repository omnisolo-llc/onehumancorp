# Issue Brief: Social Media Publishing (Ayrshare)

## Title
Implement Social Media Publishing (Ayrshare) for Small Business Owners

## Problem Statement
Posting the same promotional photo to Instagram, Facebook, and Twitter requires opening three different apps, copying and pasting the caption, and manually uploading the photo three times.

## Research Report
Ayrshare allows posting to multiple social networks simultaneously from one interface.

**Persona Impact:** A bakery owner can take a photo of a new pastry, write one caption within the OHC dashboard, click 'Post', and it instantly appears on all their active social media profiles.

**Advantages:** Massive time savings for marketing. Keeps the user inside the OHC ecosystem.

**Risks:** If a specific network rejects the post formatting, the error messages can be confusing for a non-technical user.

**Pricing Estimate:** Premium tiers start around $35/month.

**Environment:** Works seamlessly in both Cloud and Standalone modes.

## Design Doc
1.  **Post Composer:** A beautiful, simple text box with an image upload button and checkboxes for each social network.
2.  **Schedule Feature:** A simple date/time picker allowing the user to plan their posts for the week ahead.

## Implementation Prompt
Build a multi-network social media composer that allows the user to publish content to all their linked profiles simultaneously with a single click.

## Priority
P1

## Estimated Scope
Medium

### Unique Considerations
Different networks have different image aspect ratio requirements. The OHC composer must include a basic image cropping tool that prevents the user from submitting a post to Instagram that will be rejected due to incorrect dimensions, saving them from confusing error messages.

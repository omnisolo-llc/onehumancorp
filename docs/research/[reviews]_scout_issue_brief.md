# Issue Brief: Automated Customer Review Gathering & Curation

## Problem Statement
Social proof is critical for SMB conversion, but manually asking for reviews is awkward and often forgotten. Existing tools are expensive add-ons.

## Research Report
Automated post-purchase review requests increase review collection by up to 200%. Integrating this directly into the post-purchase workflow with an AI that drafts polite requests and handles negative feedback privately can significantly boost store credibility.

## Design Doc
**Architecture:**
- Review Request trigger linked to Order fulfillment state.
- `Review` entity.
**AI Integration:**
- AI analyzes review sentiment. Positive reviews are automatically highlighted; negative reviews trigger an alert to the owner for resolution.

## Implementation Prompt
Create an automated workflow that dispatches a review request via email or SMS 7 days after an order is marked fulfilled. Implement sentiment analysis on the response. Acceptance criteria: A fulfilled order triggers a scheduled request, and incoming mock reviews are correctly categorized by sentiment.

## Priority
P2

## Estimated Scope
Medium

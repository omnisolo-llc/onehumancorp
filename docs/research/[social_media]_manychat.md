# Issue Brief: ManyChat Auto-Responder

## Title
Implement ManyChat Auto-Responder for Small Business Owners

## Problem Statement
Business owners are overwhelmed by repetitive questions like 'What are your hours?' sent via social media. Answering these manually wastes hours every week.

## Research Report
ManyChat is a visual tool that automates responses on platforms like Instagram and Facebook.

**Persona Impact:** The business owner can set up simple rules to automatically reply with a PDF menu or hours of operation. This provides instant gratification to the customer.

**Advantages:** Very easy for non-technical users to build visual flowcharts of conversations.

**Risks:** It introduces a second platform the user has to learn outside of OHC.

**Pricing Estimate:** Generous free tier. Pro tier is around $15/month.

**Environment:** Works in both Cloud and Standalone deployments.

## Design Doc
1.  **Account Link:** A 1-click button to authorize OHC to talk to their ManyChat account.
2.  **Lead Capture:** When the automated bot finishes collecting information, that new contact seamlessly appears in the OHC Customer Directory.

## Implementation Prompt
Integrate ManyChat so that leads captured by their automated social media bots are automatically synced into the OHC platform as new customer records.

## Priority
P2

## Estimated Scope
Medium

### Unique Considerations
The integration must support the 'Live Chat Handoff' protocol flawlessly. When the ManyChat bot realizes the customer needs human intervention, the conversation must instantly appear in the OHC Unified Inbox, pushing a high-priority notification to the business owner to take over.

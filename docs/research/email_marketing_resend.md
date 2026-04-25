# Resend Integration for Automated Email Marketing

## Problem Statement
Boutique owners like Priya need an easy way to notify their past customers when new stock arrives or a sale is happening. Traditional tools like Mailchimp are complex, have steep learning curves, and require manually exporting/importing CSV lists of customers.

## Research Report
- **Tool**: Resend
- **Evaluation**: Resend is a modern email API designed for developers but enables building incredibly simple, embedded email sending experiences. It has excellent deliverability and simple webhook tracking.
- **Ease of Use for Persona**: The business owner never knows they are using Resend. They just see a simple "Send Email Campaign" button in OHC, type their message, and hit send.
- **Pricing**: Free tier includes 3,000 emails per month, perfectly suited for the free/starter tier of OHC. Paid plans are affordable.
- **Reputation**: Highly regarded for high deliverability, great DX, and reliability.

## Design Doc
- **Integration Point**: "Marketing & Advertising" department.
- **Trigger**: User authors an email update and clicks "Send to all customers".
- **Actions**:
  - OHC retrieves the tenant's customer list.
  - OHC dispatches the emails via Resend API using OHC's verified domain (or the tenant's custom domain if configured).
  - Resend webhooks update OHC database with open/click metrics.
- **User View**: A simple text editor to write the email, a "Send" button, and a metrics dashboard showing how many people opened it.

## Implementation Prompt
Create a "Campaigns" tab within the Marketing section. Build a simple UI allowing the user to draft an email and send it to their customer list. Integrate the Resend API to handle the actual email dispatch. Display open and click rates for past campaigns on the dashboard.

## Priority
P1

## Estimated Scope
Medium

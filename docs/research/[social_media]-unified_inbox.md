# Unified Social Media Inbox for Small Businesses

## Problem Statement
Business owners struggle to manage customer messages across Instagram, Facebook, WhatsApp, and TikTok, leading to missed sales and slow response times.

## Research Report
ManyChat and Meta Graph API provide ways to unify these streams. Evaluated ManyChat, Meta Graph API directly, and WhatsApp Business API. ManyChat is easy but costly. Direct Meta Graph API has a higher technical integration but provides better control. Both support cloud and standalone.

## Design Doc
Integrate directly using Meta Graph API. The tool will pull messages into OHC's unified inbox view. Business owners connect their accounts via a simple OAuth flow in OHC Settings.

## Implementation Prompt
Create an integration that allows business owners to connect their Instagram and Facebook accounts and view/reply to messages from a unified OHC inbox. Ensure the UI clearly shows the source of each message.

## Priority
P0

## Estimated Scope
Large

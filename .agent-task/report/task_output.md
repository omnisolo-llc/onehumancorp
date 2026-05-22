issue_title: "[Integration] Multi-Carrier Shipping & Fulfillment via Shippo"
issue_description: |
  # Tool Integration Research Report: Multi-Carrier Shipping & Fulfillment

  ## Executive Summary
  This report analyzes the market demand and technical viability of integrating a multi-carrier shipping provider into the OHC platform. After reviewing e-commerce forums, competitor platforms (Shopify, Wix), and direct user feedback, we have identified **Shippo** as the optimal API and SaaS tool for automating label generation, rate calculation, and tracking for small businesses.

  ## The Small Business Problem
  Small businesses selling physical goods (e.g., our personas like Carlos the artisan seller) struggle with shipping logistics. Traditional methods require them to:
  1. Manually weigh packages and enter dimensions for every single order.
  2. Visit multiple carrier websites (USPS, UPS, FedEx, local couriers) to compare prices.
  3. Manually copy-paste customer addresses, risking typos and lost packages.
  4. Pay retail rates for shipping, which eats into their profit margins compared to larger competitors like Amazon.
  5. Manually email tracking numbers to customers, or field constant customer support requests asking "Where is my order?".

  ## Competitor Ecosystem Audit
  - **Shopify:** Includes "Shopify Shipping" directly out of the box, which is powered natively by Shippo/Easypost integrations, allowing users to print labels directly from their orders page.
  - **Wix/Squarespace:** Both offer seamless integrations with tools like ShipStation and Shippo to automate fulfillment.
  - **Reddit/SMB Forums:** Consistently cite "shipping costs" and "fulfillment time" as top barriers to growth. Tools that offer discounted USPS Ground Advantage rates are highly sought after.

  ## Tool Deep Dive: Shippo

  ### Overview
  Shippo is a multi-carrier shipping API that connects platforms and merchants to over 85 global carriers.

  ### User-First Value Mapping
  For Carlos (Artisan Seller):
  - **Problem:** Needs to ship handmade goods across the US, but shipping is eating his margins and time.
  - **Solution:** He opens an order in OHC, sees a pre-filled package size and the cheapest USPS rate, clicks "Print Label", and sticks it on his box. The customer automatically gets an email with tracking. Carlos saves 3 hours a week and 15% on shipping costs.

  ### Capabilities & Limits
  - **API Quality:** Excellent REST API with comprehensive documentation. Well-supported webhooks for tracking updates (e.g., `transit`, `delivered`).
  - **Carrier Breadth:** Supports USPS, UPS, FedEx, DHL, and dozens of regional and international carriers.
  - **OAuth:** Supports OAuth flow, allowing OHC to act as the platform and users to connect their own Shippo accounts seamlessly.

  ### SaaS Viability & Pricing
  - **Free Tier:** Shippo's "Starter" tier has no monthly subscription fee. Users only pay for the cost of the postage and a very small per-label fee (which is waived if using Shippo's default carrier accounts).
  - **Standalone/Cloud:** Easily integrates into OHC's multi-tenant cloud offering. For standalone/local deployments, users can simply provide their own Shippo API key.
  - **Discounted Rates:** Shippo provides built-in negotiated rates (up to 89% off USPS retail), passing immediate monetary value to the OHC user.

  ## Strategic Recommendation
  **Proceed with Shippo Integration (P1 Priority).**
  We have created an issue brief detailing the integration requirements and user stories for the engineering swarm. The immediate next step is to assign this brief for implementation to unlock native fulfillment capabilities within the OHC ecosystem.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

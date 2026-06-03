issue_title: "Implement Intelligent Customer Auto-Responder UI"
issue_description: |
  # OHC Feature Mission: Intelligent Customer Auto-Responder & Mobile Management Fixes

  ## Problem Statement
  Based on our deep dive into the competitor landscape (Wix, Squarespace, GoDaddy, Weebly) and user research from 50+ sources:
  Small business owners (like Maya the baker and Carlos the handyman) are overwhelmed by manual customer communication and poor mobile management.
  Existing tools require 3rd party apps or manual intervention to reply to Instagram DMs or email inquiries, leading to lost sales and poor customer service.

  ## Research Report
  - Competitor capabilities:
    - Wix/Squarespace: Requires external integrations or manual inbox checks.
    - Shopify: "Sidekick" is a chatbot for the merchant, not the customer. Customer auto-replies require expensive apps.
  - User Sentiment: 38% of users complain about Instagram DM overload; 25% complain about poor mobile management capabilities.

  ## Design Doc
  - Architecture: Add an `AutoResponderAgent` to the Operations department. Connect it to an `InboxController`.
  - UI: A new mobile-first (375px) "Inbox Settings" screen where the user can toggle "AI Auto-Reply" and view agent-drafted messages. Translucent Glassmorphism styles applied.

  ## Implementation Prompt
  - Build the UI to allow toggling of the AI auto-responder.
  - Implement a basic API endpoint that takes a customer message, detects intent using simple keyword matching (e.g., "where is my order", "vegan"), and returns a drafted response or "escalate to human" flag.
  - Write Playwright E2E tests for the configuration flow.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

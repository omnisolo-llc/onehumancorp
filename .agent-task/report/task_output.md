issue_title: "Scout: Tool Integration Research - Shippo"
issue_description: |
  # Shippo Integration Research Report

  ## Findings
  Small business owners often struggle with logistics when scaling their operations. They have to manually compare shipping rates across multiple carriers, generate labels, and copy-paste tracking numbers back into their systems. Shippo is a leading multi-carrier shipping API that solves this by abstracting the complexities of individual carrier APIs into a single interface.

  ## Competitive Analysis
  Shippo offers a "Starter" tier that is free to use (no monthly subscription fee), making it incredibly viable for small business owners with low or variable order volumes. It provides instant access to discounted rates (up to 90% off retail for USPS). The API is robust, supporting over 85 global carriers.

  ## Viability
  Shippo is viable in both Cloud (multi-tenant) and Standalone (local) modes. For Cloud, it uses standard OAuth2. For Standalone, users can generate a personal API token in their Shippo dashboard.

  ## Proposed Next Steps
  Implement a shipping integration using Shippo where the user can connect their account, generate labels from the order detail view, compare real-time shipping rates, and purchase labels. The system should automatically attach the tracking number to the order.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
issue_title: "Architecture: Autonomous Competitor Migration & Ingestion Engine"
issue_description: |
  # Architecture: Autonomous Competitor Migration & Ingestion Engine

  ## Findings
  When convincing small business owners to switch to OHC from legacy platforms like Shopify or Wix, data migration is the biggest barrier. The "Cost of Switching" is too high because users don't know how to export CSV files and map column headers.
  Existing platforms offer import tools that rely on manual CSV formatting or third-party apps, which are technical and error-prone. OHC can leapfrog this by providing an intelligent, visual-first scraping and ingestion engine that requires zero data export from the source platform.

  ## Proposed Next Steps
  We have drafted the architectural design document. This engine will use a Crawler Agent and Vision AI to autonomously ingest, structure, and populate a user's entire catalog directly from their existing website URL into OHC's Universal Capacity Ledger.

  The Implementer swarm should now proceed with building the async ingestion job queue, web crawling integration, and image download pipeline as outlined in the design document.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

issue_title: "[architecture] Autonomous Magic Migration and Data Ingestion Engine"
issue_description: |
  **Executive Summary**
  Small business owners migrating to OneHumanCorp (OHC) from legacy platforms (Shopify, Wix, Square) or offline workflows face a major friction barrier: data entry. To meet our "Zero → Live in 10 minutes" mission, we must automate this process.

  **Problem Statement**
  Users like Priya (500 SKUs on Shopify) and Fatima (printed PDF menu) will abandon onboarding if they have to manually re-enter their inventory or services. A seamless, AI-driven migration engine is critical for activating these users.

  **Proposed Architecture**
  The Autonomous Magic Migration Engine is a multi-modal ingestion system. It accepts CSVs, PDFs, images, and API connections, feeding them to the AI Operations Department. The AI extracts structured entities (products, services) from unstructured data and injects them securely into the OHC multi-tenant ledger. A fallback flow uses the AI CS Department to push low-confidence mappings to the user for 1-tap mobile confirmation.

  **Next Steps**
  - Implement the Ingestion API for multi-part file uploads (CSV, Image, PDF).
  - Develop the AI Operations parsing queue to map raw data to OHC Product/Service entities.
  - See `docs/research/[architecture]_autonomous_magic_migration_and_data_ingestion_engine.md` for full Mermaid diagrams and architectural details.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []

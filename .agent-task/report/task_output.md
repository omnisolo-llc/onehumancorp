issue_title: "F05 telemetry not invoice-grade"
issue_description: |
  The current implementation of telemetry and cost reports does not act as an invoice-grade meter.
  Blocked pending requirements:
  1. No definition of payer/auth/rate attribution payloads.
  2. The actual provider invoice reconciliation requires external data/APIs that we are explicitly forbidden from guessing.
  3. The current telemetry architecture receives batches that lack a unique attempt/provider request ID required for deduplication.

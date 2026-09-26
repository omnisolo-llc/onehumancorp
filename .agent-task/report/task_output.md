issue_title: "Audit of F14: No measured representative serving costs or owner outcomes"
issue_description: |
  # Audit of F14: No measured representative serving costs or owner outcomes

  ## Superpowers Workflow Provenance
  - **Loaded Skills:** `using-superpowers`
  - **Repository URL:** `https://github.com/obra/superpowers.git`
  - **Revision Hash:** `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
  - **Checks Performed:** Initial repository cloning and workflow validation.
  - **Outcomes:** Superpowers workflow initialized and skill document accessed successfully.

  ## Problem Statement
  As a business owner, I need to know exactly how much it costs to run my AI team and fulfill customer work. Right now, there is a gap in our tracking (identified as F14): we don't have accurate, measured costs for serving customers or clear evidence of the business results our AI team achieves. This prevents us from confidently knowing our profit margins or offering a sustainable "Bring Your Own Key" (BYOK) billing model where customers pay for their own AI usage directly.

  ## Research Report & Findings
  Based on the current state of our systems (accessed 2026-09-18), here is what we know about how we track costs and outcomes:

  ### 1. The Missing Cost Picture
  Our current code does not provide a complete, real-world invoice of what it takes to serve a customer. While we have tools to track AI usage events (found in `hub.rs` and `auditor.rs`), these do not translate into a comprehensive cost breakdown.

  To truly understand our costs, we need to measure:
  - **Compute Time:** How long our servers (CPU/Memory/GPU) are actively working versus sitting idle waiting for tasks.
  - **External Services:** The cost of databases, file storage, network usage, and paid external tools we connect to.
  - **Operations:** The cost of handling payments, providing support, and managing errors or retries.
  - **AI Provider Costs (OHC-Funded):** Any AI model usage that we pay for directly out of our pocket.

  ### 2. Customer-Funded AI (BYOK) Challenges
  If a customer wants to pay for their own AI usage (like plugging in their own OpenAI or Google API key), we must keep those costs entirely separate from our own bills. Right now, there is a risk of mixing these up. We also need to be very careful that we are allowed to use customer subscriptions in this way, as provider rules (like from Anthropic or Google) strictly limit how third-party tools can use consumer accounts.

  ### 3. Measuring Actual Business Outcomes
  We cannot just look at whether our code ran successfully; we need to measure if the *business task* succeeded. Did the customer actually get paid? Was the proposal accepted? Did the recurring follow-up work? Right now, we sometimes confuse a system test passing with a real customer success story. We need to track actual time saved and verified results without guessing.

  ## Identified Uncertainties
  - **Real Workload Costs:** We don't have enough data from actual, running businesses to know the average cost of common tasks. We need to measure a small, real-world sample to establish baseline costs.
  - **Owner Effort:** We need to measure how much time the business owner spends setting up the AI, reviewing its work, and correcting mistakes. If the AI costs $10 but takes the owner 5 hours to manage, it's not a success.
  - **Willingness to Pay:** We still don't know exactly what price point makes sense for our users based on the real value they receive. We cannot rely on the old $99/month assumption.

  ## Scope & Next Steps
  Before we can launch a pricing plan or a BYOK feature, we need to gather real evidence:
  1. **Instrument Workloads:** Add tracking to measure the exact resources (time, memory, AI tokens) used for a specific, typical business task.
  2. **Run a Benchmark:** Execute this task repeatedly to find the average cost and time.
  3. **Track Owner Time:** Measure the time required for a human to review and approve the AI's work.
  4. **Keep Records Separate:** Ensure our billing system can clearly divide our costs from the customer's direct API costs.

issue_priority: "P1"
issue_category: "Audit"
issue_type: "Research"
issue_label: "F14"
assignees: []
